#!/usr/bin/env python3
"""Publish the workspace to a local sparse registry and consume laputa-smol from it.

`cargo package` on this toolchain has no `--dry-run`. Verification is `cargo publish`
without `--no-verify`: Cargo builds the packaged crate before upload. Cargo rewrites
path dependencies to crates.io version requirements, so this command replaces the
crates.io source with the staging index for that process only. The staging server
stores Laputa crates and proxies every other index entry and download.
"""

import hashlib
import io
import json
import os
import shutil
import struct
import subprocess
import sys
import tarfile
import threading
import tomllib
import urllib.error
import urllib.parse
import urllib.request
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from identity import load_release

ROOT = Path(__file__).resolve().parent.parent
VERSION, COMPONENTS = load_release(ROOT)
UPSTREAM_NAMES = {spec["upstream_name"] for spec in COMPONENTS.values()}


class RegistryState:
    def __init__(self):
        self.lock = threading.Lock()
        self.crates = {}
        self.lines = {}
        self.proxied_index = {}
        self.proxied_crate = {}

    def put(self, name, version, crate, line):
        with self.lock:
            self.crates[(name, version)] = crate
            self.lines.setdefault(name, [])
            self.lines[name] = [existing for existing in self.lines[name] if f'"vers":"{version}"' not in existing.replace(" ", "")]
            self.lines[name].append(line)

    def crate(self, name, version):
        with self.lock:
            return self.crates.get((name, version))

    def index(self, name):
        with self.lock:
            rows = self.lines.get(name)
            if not rows:
                return None
            return ("\n".join(rows) + "\n").encode()


def proxy_bytes(state, cache, url):
    with state.lock:
        if url in cache:
            return cache[url]
    request = urllib.request.Request(url, headers={"User-Agent": "smolith-staging"})
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            body = response.read()
    except urllib.error.HTTPError as exc:
        if exc.code == 404:
            with state.lock:
                cache[url] = None
            return None
        raise
    with state.lock:
        cache[url] = body
    return body


def index_prefix(name):
    lowered = name.lower()
    if len(lowered) <= 1:
        return "1"
    if len(lowered) == 2:
        return "2"
    if len(lowered) == 3:
        return f"3/{lowered[0]}"
    return f"{lowered[0:2]}/{lowered[2:4]}"


def make_handler(state, origin):
    class Handler(BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def log_message(self, fmt, *args):
            sys.stderr.write("staging: " + (fmt % args) + "\n")

        def do_GET(self):
            path = urllib.parse.urlsplit(self.path).path
            if path == "/index/config.json":
                body = json.dumps({"dl": f"{origin}/crates", "api": origin}).encode()
                return self._send(200, "application/json", body)
            if path.startswith("/index/"):
                name = path[len("/index/") :].split("/")[-1]
                expected = f"/index/{index_prefix(name)}/{name}"
                if path != expected:
                    return self._send(404, "text/plain", b"")
                body = state.index(name)
                if body is None:
                    body = proxy_bytes(state, state.proxied_index, f"https://index.crates.io/{index_prefix(name)}/{name}")
                if body is None:
                    return self._send(404, "text/plain", b"")
                return self._send(200, "text/plain; charset=utf-8", body)
            parts = path.strip("/").split("/")
            if len(parts) == 4 and parts[0] == "crates" and parts[3] == "download":
                crate = state.crate(parts[1], parts[2])
                if crate is None:
                    crate = proxy_bytes(
                        state,
                        state.proxied_crate,
                        f"https://static.crates.io/crates/{parts[1]}/{parts[2]}/download",
                    )
                if crate is None:
                    return self._send(404, "text/plain", b"")
                return self._send(200, "application/octet-stream", crate)
            self._send(404, "text/plain", b"")

        def do_PUT(self):
            path = urllib.parse.urlsplit(self.path).path
            if path != "/api/v1/crates/new":
                return self._send(404, "application/json", b'{"errors":[{"detail":"unknown route"}]}')
            length = int(self.headers.get("Content-Length", "0"))
            payload = self.rfile.read(length)
            try:
                meta, crate = parse_publish(payload)
                line = index_line(meta, crate)
            except (ValueError, KeyError, struct.error) as exc:
                detail = json.dumps({"errors": [{"detail": str(exc)}]}).encode()
                return self._send(400, "application/json", detail)
            state.put(meta["name"], meta["vers"], crate, line)
            body = json.dumps(
                {
                    "ok": True,
                    "warnings": {"invalid_categories": [], "invalid_badges": [], "other": []},
                }
            ).encode()
            self._send(200, "application/json", body)

        def _send(self, status, content_type, body):
            self.send_response(status)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

    return Handler


def parse_publish(payload):
    json_len = struct.unpack_from("<I", payload, 0)[0]
    meta = json.loads(payload[4 : 4 + json_len])
    rest = payload[4 + json_len :]
    crate_len = struct.unpack_from("<I", rest, 0)[0]
    crate = rest[4 : 4 + crate_len]
    if len(crate) != crate_len:
        raise ValueError("crate bytes truncated")
    return meta, crate


def index_line(meta, crate):
    deps = []
    for dep in meta.get("deps", []):
        kind = dep.get("kind") or "normal"
        if kind == "dev":
            continue
        name = dep["name"]
        alias = dep.get("explicit_name_in_toml")
        entry = {
            "name": alias if alias and alias != name else name,
            "req": dep.get("version_req") or dep["req"],
            "features": dep.get("features") or [],
            "optional": bool(dep.get("optional", False)),
            "default_features": bool(dep.get("default_features", True)),
            "target": dep.get("target"),
            "kind": kind,
            "registry": dep.get("registry"),
        }
        if alias and alias != name:
            entry["package"] = name
        deps.append(entry)
    line = {
        "name": meta["name"],
        "vers": meta["vers"],
        "deps": deps,
        "cksum": hashlib.sha256(crate).hexdigest(),
        "features": meta.get("features") or {},
        "yanked": False,
    }
    if meta.get("features2"):
        line["features2"] = meta["features2"]
        line["v"] = 2
    if meta.get("links"):
        line["links"] = meta["links"]
    if meta.get("rust_version"):
        line["rust_version"] = meta["rust_version"]
    return json.dumps(line, separators=(",", ":"))


def metadata():
    raw = subprocess.check_output(["cargo", "metadata", "--format-version", "1", "--no-deps"], cwd=ROOT)
    return json.loads(raw)


def publish_order():
    meta = metadata()
    packages = {pkg["id"]: pkg for pkg in meta["packages"]}
    names = {}
    for member in meta["workspace_members"]:
        pkg = packages[member]
        if pkg["name"].startswith("laputa-"):
            names[pkg["name"]] = member
    full = subprocess.check_output(["cargo", "metadata", "--format-version", "1"], cwd=ROOT)
    resolved = json.loads(full)
    nodes = {node["id"]: node for node in resolved["resolve"]["nodes"]}
    incoming = {name: set() for name in names}
    for name, member in names.items():
        for dep in nodes[member]["deps"]:
            for kind in dep["dep_kinds"]:
                if kind["kind"] not in (None, "build"):
                    continue
                dep_name = packages_by_resolve_id(resolved, dep["pkg"])
                if dep_name in names:
                    incoming[name].add(dep_name)
    ready = sorted(name for name, deps in incoming.items() if not deps)
    order = []
    while ready:
        name = ready.pop(0)
        order.append(name)
        for other in sorted(incoming):
            if name in incoming[other]:
                incoming[other].remove(name)
                if not incoming[other] and other not in order and other not in ready:
                    ready.append(other)
        ready.sort()
    if len(order) != len(names):
        raise SystemExit(f"dependency cycle among {sorted(set(names) - set(order))}")
    return order


def packages_by_resolve_id(meta, pkg_id):
    for pkg in meta["packages"]:
        if pkg["id"] == pkg_id:
            return pkg["name"]
    raise KeyError(pkg_id)


def git_dirty():
    result = subprocess.run(["git", "status", "--porcelain"], cwd=ROOT, text=True, capture_output=True, check=True)
    return bool(result.stdout.strip())


def cargo_config(index):
    return [
        "--config",
        f'registries.staging.index="{index}"',
        "--config",
        'source.crates-io.replace-with="staging"',
        "--config",
        f'source.staging.registry="{index}"',
    ]


def publish_all(order, index, dirty):
    for name in order:
        cmd = [
            "cargo",
            "publish",
            "-p",
            name,
            "--registry",
            "staging",
            "--token",
            "local",
            *cargo_config(index),
        ]
        if dirty:
            cmd.append("--allow-dirty")
        print(f"publish {name}", flush=True)
        subprocess.check_call(cmd, cwd=ROOT)


def iter_dep_tables(table):
    if not isinstance(table, dict):
        return
    for key, value in table.items():
        if key in ("dependencies", "build-dependencies", "dev-dependencies") and isinstance(value, dict):
            for dep_name, spec in value.items():
                yield key, dep_name, spec
        elif key == "target" and isinstance(value, dict):
            for target_table in value.values():
                yield from iter_dep_tables(target_table)


def package_name(dep_name, spec):
    if isinstance(spec, dict) and "package" in spec:
        return spec["package"]
    return dep_name


def manifest_paths():
    meta = metadata()
    return {
        pkg["name"]: Path(pkg["manifest_path"])
        for pkg in meta["packages"]
        if pkg["name"].startswith("laputa-")
    }


def disabled_default_features(manifest):
    found = set()
    for kind, dep_name, spec in iter_dep_tables(manifest):
        if kind == "dev-dependencies":
            continue
        if isinstance(spec, dict) and spec.get("default-features") is False:
            found.add((kind, dep_name))
    return found


def check_artifacts(state, order):
    sources = manifest_paths()
    for name in order:
        crate = state.crate(name, VERSION)
        if crate is None:
            raise SystemExit(f"missing uploaded crate {name} {VERSION}")
        source = tomllib.loads(sources[name].read_text())
        required = disabled_default_features(source)
        with tarfile.open(fileobj=io.BytesIO(crate), mode="r:gz") as tar:
            names = tar.getnames()
            manifest_name = next(item for item in names if item.endswith("/Cargo.toml") and item.count("/") == 1)
            manifest = tomllib.loads(tar.extractfile(manifest_name).read().decode())
            license_text = manifest["package"]["license"]
            if "MIT" in license_text and not any(item.endswith("/LICENSE-MIT") for item in names):
                raise SystemExit(f"{name} package is missing LICENSE-MIT")
            if "Apache" in license_text and not any(item.endswith("/LICENSE-APACHE") for item in names):
                raise SystemExit(f"{name} package is missing LICENSE-APACHE")
            if "patch" in manifest:
                raise SystemExit(f"{name} packaged manifest contains [patch]")
            packaged = {}
            for kind, dep_name, spec in iter_dep_tables(manifest):
                if kind == "dev-dependencies":
                    continue
                pkg = package_name(dep_name, spec)
                if pkg in UPSTREAM_NAMES:
                    raise SystemExit(f"{name} packaged {kind} names upstream package {pkg}")
                if pkg == "tokio" or pkg.startswith("tokio"):
                    raise SystemExit(f"{name} packaged {kind} names {pkg}")
                packaged.setdefault((kind, dep_name), spec)
            for key in required:
                spec = packaged.get(key)
                if not isinstance(spec, dict) or spec.get("default-features") is not False:
                    raise SystemExit(f"{name} packaged {key[0]} {key[1]} dropped default-features = false")
        print(f"artifact {name} licenses and dependency names ok", flush=True)


def fresh_consumer(index):
    consumer = ROOT / ".work" / "fresh-consumer"
    if consumer.exists():
        shutil.rmtree(consumer)
    (consumer / "src").mkdir(parents=True)
    package = COMPONENTS["smol"]["package"]
    (consumer / "Cargo.toml").write_text(
        f"""[package]
name = "fresh-consumer"
version = "{VERSION}"
edition = "2021"
rust-version = "1.85"
publish = false

[workspace]
resolver = "2"

[dependencies]
smol = {{ package = "{package}", version = "{VERSION}", registry = "staging" }}
""",
        encoding="utf-8",
    )
    (consumer / "src" / "lib.rs").write_text(
        """pub fn channel_roundtrip() {
    smol::block_on(async {
        let (sender, receiver) = smol::channel::bounded(1);
        sender.send(7u8).await.unwrap();
        assert_eq!(receiver.recv().await.unwrap(), 7);
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn roundtrip() {
        super::channel_roundtrip();
    }
}
""",
        encoding="utf-8",
    )
    manifest = consumer / "Cargo.toml"
    subprocess.check_call(
        ["cargo", "test", "--manifest-path", str(manifest), *cargo_config(index)],
        cwd=consumer,
    )
    raw = subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--manifest-path", str(manifest), *cargo_config(index)],
        cwd=consumer,
    )
    meta = json.loads(raw)
    smol_pkgs = [pkg for pkg in meta["packages"] if pkg["name"] == "laputa-smol"]
    if len(smol_pkgs) != 1:
        raise SystemExit(f"fresh consumer resolved {len(smol_pkgs)} laputa-smol packages")
    source = smol_pkgs[0]["source"] or ""
    if "127.0.0.1" not in source or source.startswith("path+") or "crates.io" in source:
        raise SystemExit(f"laputa-smol resolved from {source}")
    for pkg in meta["packages"]:
        if "tokio" in pkg["name"]:
            raise SystemExit(f"fresh consumer resolved {pkg['name']}")
    print(f"fresh consumer laputa-smol source {source}", flush=True)


def main(only=None):
    os.chdir(ROOT)
    tmp = ROOT / ".work" / "tmp"
    tmp.mkdir(parents=True, exist_ok=True)
    os.environ["TMPDIR"] = str(tmp)
    order = publish_order()
    if only:
        unknown = sorted(set(only) - set(order))
        if unknown:
            raise SystemExit(f"unknown packages {unknown}")
        order = [name for name in order if name in set(only)]
    state = RegistryState()
    server = ThreadingHTTPServer(("127.0.0.1", 0), make_handler(state, "http://placeholder"))
    port = server.server_address[1]
    origin = f"http://127.0.0.1:{port}"
    server.RequestHandlerClass = make_handler(state, origin)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    index = f"sparse+{origin}/index/"
    os.environ["CARGO_REGISTRIES_STAGING_INDEX"] = index
    os.environ["CARGO_REGISTRIES_STAGING_TOKEN"] = "local"
    config_arg = f'registries.staging.index="{index}"'
    print(f"staging index {index}", flush=True)
    try:
        dirty = git_dirty()
        publish_all(order, index, dirty)
        check_artifacts(state, order)
        if only is None:
            fresh_consumer(index)
    finally:
        server.shutdown()
    print("STAGING PASSED", flush=True)


if __name__ == "__main__":
    main(sys.argv[1:] or None)
