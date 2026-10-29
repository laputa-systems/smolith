#!/usr/bin/env python3
"""Pin h12tiny, record its baseline, retarget smol edges at laputa, and run gates."""

import json
import os
import re
import shutil
import subprocess
import sys
import tomllib

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
PIN = os.path.join(ROOT, "fixtures", "h12tiny.toml")
CHECKOUT = os.path.join(ROOT, ".work", "h12tiny")
BASELINE_PATH = os.path.join(ROOT, ".work", "h12tiny-baseline.json")
GATES_PATH = os.path.join(ROOT, ".work", "h12tiny-gates.txt")
FEATURES = "client,client-sync,server,http1,http2,tls"
GATE_TIMEOUT_SECONDS = 1200

EDGES = {
    "async-io": ("laputa-async-io", "crates/async-io"),
    "async-net": ("laputa-async-net", "crates/async-net"),
    "futures-lite": ("laputa-futures-lite", "crates/futures-lite"),
    "smol": ("laputa-smol", "crates/smol"),
    "atomic-waker": ("laputa-atomic-waker", "crates/atomic-waker"),
    "fastrand": ("laputa-fastrand", "crates/fastrand"),
}

REQUIRED = (
    "laputa-async-io",
    "laputa-async-net",
    "laputa-atomic-waker",
    "laputa-fastrand",
    "laputa-futures-lite",
    "laputa-polling",
    "laputa-smol",
)

EDGE_RE = re.compile(
    r"^(\s*)(" + "|".join(re.escape(name) for name in EDGES) + r")\s*=\s*(.*)$"
)

GATES = (
    (
        "http1",
        [
            "cargo",
            "test",
            "--locked",
            "--features",
            FEATURES,
            "--test",
            "dogfood_h1",
            "--",
            "h1_client_and_auto_server_interoperate_over_plaintext",
            "--exact",
        ],
    ),
    (
        "http2",
        [
            "cargo",
            "test",
            "--locked",
            "--features",
            FEATURES,
            "--test",
            "dogfood_h2_prior",
            "--",
            "h2_client_and_auto_server_interoperate_over_cleartext_prior_knowledge",
            "--exact",
        ],
    ),
    (
        "tls",
        [
            "cargo",
            "test",
            "--locked",
            "--features",
            FEATURES,
            "--test",
            "dogfood_tls",
            "--",
            "tls_alpn_http11_validates_fixture_certificate_and_streams_bodies",
            "--exact",
        ],
    ),
    (
        "cancellation",
        [
            "cargo",
            "test",
            "--locked",
            "--features",
            FEATURES,
            "--test",
            "h2_lifecycle",
            "--",
            "cancelling_one_h2_response_future_does_not_poison_the_shared_session",
            "--exact",
        ],
    ),
    (
        "reuse",
        [
            "cargo",
            "test",
            "--locked",
            "--features",
            FEATURES,
            "--test",
            "dogfood_h1_matrix",
            "--",
            "concurrent_h1_requests_get_unique_connections_then_reuse_them",
            "--exact",
        ],
    ),
    (
        "timeout",
        [
            "cargo",
            "test",
            "--locked",
            "-p",
            "h12tiny-client",
            "--features",
            "http1",
            "--lib",
            "--",
            "connect::tests::connect_timeout_cancels_a_pending_tcp_dialer",
            "--exact",
        ],
    ),
    (
        "shutdown",
        [
            "cargo",
            "test",
            "--locked",
            "--features",
            FEATURES,
            "--test",
            "unix_lifecycle",
            "--",
            "unix_listener_serves_h1_and_drains_on_shutdown",
            "--exact",
        ],
    ),
)

BACKPRESSURE_GAP = (
    "backpressure tests exist only in the tokio hyper and h2 harnesses "
    "(h2_connect_backpressure_respected, consecutive_refusals_survive_write_backpressure), "
    "which are outside the h12tiny workspace and are not migrated"
)


def fail(message):
    print(f"h12tiny fixture failed: {message}", file=sys.stderr)
    sys.exit(1)


def cargo_env():
    env = os.environ.copy()
    env["TMPDIR"] = os.path.join(ROOT, ".work", "tmp")
    env["CARGO_TERM_COLOR"] = "never"
    env["GIT_TERMINAL_PROMPT"] = "0"
    os.makedirs(env["TMPDIR"], exist_ok=True)
    return env


def read_toml(path):
    with open(path, "rb") as handle:
        return tomllib.load(handle)


def read_pin():
    pin = read_toml(PIN)
    repo = pin.get("repo")
    commit = pin.get("commit")
    if not isinstance(repo, str) or not repo or not isinstance(commit, str) or not commit:
        fail("fixtures/h12tiny.toml must set repo and commit")
    return repo, commit


def has_commit(repo, commit):
    result = subprocess.run(
        ["git", "-C", repo, "cat-file", "-e", f"{commit}^{{commit}}"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    return result.returncode == 0


def run_git(args, cwd=None):
    result = subprocess.run(["git", *args], cwd=cwd, env=cargo_env(), text=True)
    if result.returncode != 0:
        fail(f"git {' '.join(args)} failed")


def acquire(repo, commit):
    os.makedirs(os.path.join(ROOT, ".work"), exist_ok=True)
    if os.path.isdir(os.path.join(CHECKOUT, ".git")) and has_commit(CHECKOUT, commit):
        run_git(["-C", CHECKOUT, "reset", "--hard", commit])
        run_git(["-C", CHECKOUT, "clean", "-fd"])
        head = subprocess.check_output(
            ["git", "-C", CHECKOUT, "rev-parse", "HEAD"], text=True
        ).strip()
        if head != commit:
            fail(f"checkout HEAD {head} != pin {commit}")
        return "reset-existing"
    if os.path.exists(CHECKOUT):
        shutil.rmtree(CHECKOUT)
    mirrors = []
    mirror = os.environ.get("H12TINY_MIRROR")
    if mirror:
        mirrors.append(mirror)
    mirrors.append(os.path.abspath(os.path.join(ROOT, "..", "h12tiny")))
    for candidate in mirrors:
        if candidate and os.path.isdir(candidate) and has_commit(candidate, commit):
            run_git(["clone", "--local", candidate, CHECKOUT])
            run_git(["-C", CHECKOUT, "checkout", "--detach", commit])
            return candidate
    result = subprocess.run(
        ["git", "clone", "--filter=blob:none", repo, CHECKOUT],
        env=cargo_env(),
        text=True,
    )
    if result.returncode != 0:
        fail(f"could not clone {repo}")
    run_git(["-C", CHECKOUT, "checkout", "--detach", commit])
    return repo


def git_status():
    result = subprocess.run(
        ["git", "-C", CHECKOUT, "status", "--porcelain"],
        check=True,
        text=True,
        capture_output=True,
    )
    return result.stdout.strip()


def cargo_metadata(locked, features):
    command = [
        "cargo",
        "metadata",
        "--format-version",
        "1",
        "--manifest-path",
        os.path.join(CHECKOUT, "Cargo.toml"),
    ]
    if locked:
        command.append("--locked")
    if features:
        command.extend(["--features", features])
    result = subprocess.run(
        command,
        cwd=CHECKOUT,
        env=cargo_env(),
        text=True,
        capture_output=True,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        fail("cargo metadata failed")
    return json.loads(result.stdout)


def upstream_names():
    components = read_toml(os.path.join(ROOT, "UPSTREAM.toml")).get("components")
    if not isinstance(components, dict):
        fail("UPSTREAM.toml is missing components")
    names = set()
    for name, item in components.items():
        upstream = item.get("upstream_name") if isinstance(item, dict) else None
        if not isinstance(upstream, str) or not upstream:
            fail(f"UPSTREAM.toml component {name} is missing upstream_name")
        names.add(upstream)
    return names


def registry_versions(meta):
    versions = {}
    for package in meta["packages"]:
        source = package.get("source") or ""
        if source.startswith("registry+"):
            versions.setdefault(package["name"], set()).add(package["version"])
    return versions


def resolved_packages(meta):
    nodes = {node["id"] for node in meta["resolve"]["nodes"]}
    return [package for package in meta["packages"] if package["id"] in nodes]


def record_baseline(source, commit, meta):
    rows = []
    names = upstream_names()
    for package in resolved_packages(meta):
        if package["name"] in names:
            rows.append(
                {
                    "name": package["name"],
                    "version": package["version"],
                    "source": package["source"],
                }
            )
    rows.sort(key=lambda row: row["name"])
    if not rows:
        fail("unmodified baseline resolved no upstream smol packages")
    status = git_status()
    if status:
        fail(f"unmodified checkout is dirty:\n{status}")
    payload = {
        "commit": commit,
        "source": source,
        "packages": rows,
    }
    with open(BASELINE_PATH, "w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2)
        handle.write("\n")
    print(f"recorded unmodified baseline at {commit} from {source}")
    for row in rows:
        print(f"  baseline {row['name']} {row['version']} {row['source']}")
    return payload


def split_comment(value):
    quote = ""
    for index, char in enumerate(value):
        if quote:
            if char == quote:
                quote = ""
            continue
        if char in "\"'":
            quote = char
            continue
        if char == "#":
            return value[:index].rstrip(), value[index:]
    return value.rstrip(), ""


def split_commas(value):
    parts = []
    quote = ""
    depth = 0
    start = 0
    for index, char in enumerate(value):
        if quote:
            if char == quote:
                quote = ""
            continue
        if char in "\"'":
            quote = char
            continue
        if char in "[{":
            depth += 1
            continue
        if char in "]}":
            depth -= 1
            continue
        if char == "," and depth == 0:
            parts.append(value[start:index].strip())
            start = index + 1
    tail = value[start:].strip()
    if tail:
        parts.append(tail)
    return parts


def rewrite_inline(body, package, path):
    inner = body[1:-1].strip()
    kept = []
    for part in split_commas(inner):
        key = part.split("=", 1)[0].strip()
        if key in {"package", "version", "path"}:
            continue
        kept.append(part)
    prefix = f'package = "{package}", version = "0.1.0", path = "{path}"'
    if kept:
        return "{ " + prefix + ", " + ", ".join(kept) + " }"
    return "{ " + prefix + " }"


def relative_crate(manifest, crate_rel):
    absolute = os.path.join(ROOT, crate_rel)
    relative = os.path.relpath(absolute, os.path.dirname(manifest))
    return relative.replace(os.sep, "/")


def rewrite_manifest(path):
    with open(path, encoding="utf-8") as handle:
        lines = handle.read().splitlines()
    changed = False
    rewritten = []
    for line in lines:
        match = EDGE_RE.match(line)
        if not match:
            rewritten.append(line)
            continue
        indent, key, raw = match.groups()
        body, comment = split_comment(raw.strip())
        if re.search(r"\bworkspace\s*=\s*true\b", body):
            rewritten.append(line)
            continue
        package, crate_rel = EDGES[key]
        dep_path = relative_crate(path, crate_rel)
        if body.startswith(("\"", "'")):
            replacement = f'{{ package = "{package}", version = "0.1.0", path = "{dep_path}" }}'
        elif body.startswith("{") and body.endswith("}"):
            replacement = rewrite_inline(body, package, dep_path)
        else:
            fail(f"unrecognized dependency in {path}: {line}")
        suffix = f" {comment}" if comment else ""
        rewritten.append(f"{indent}{key} = {replacement}{suffix}")
        changed = True
    if changed:
        with open(path, "w", encoding="utf-8") as handle:
            handle.write("\n".join(rewritten) + "\n")
    return changed


def manifests():
    found = []
    for dirpath, dirnames, filenames in os.walk(CHECKOUT):
        dirnames[:] = [name for name in dirnames if name not in {".git", "target", "fuzz"}]
        if "Cargo.toml" in filenames:
            found.append(os.path.join(dirpath, "Cargo.toml"))
    return found


def redirect():
    touched = 0
    for manifest in manifests():
        if rewrite_manifest(manifest):
            touched += 1
    if touched == 0:
        fail("redirect changed no manifests")
    leftovers = []
    for manifest in manifests():
        with open(manifest, encoding="utf-8") as handle:
            for number, line in enumerate(handle, start=1):
                match = EDGE_RE.match(line)
                if not match:
                    continue
                body, _comment = split_comment(match.group(3).strip())
                if re.search(r"\bworkspace\s*=\s*true\b", body):
                    continue
                package, _crate = EDGES[match.group(2)]
                if f'package = "{package}"' not in body:
                    leftovers.append(f"{manifest}:{number}: {line.rstrip()}")
    if leftovers:
        fail("edges left on upstream packages:\n" + "\n".join(leftovers))
    print(f"redirected edges in {touched} manifests; aliases kept")


def assert_minimal_lock(before, after):
    names = upstream_names()
    problems = []
    for name, versions in sorted(before.items()):
        if name not in after:
            if name not in names:
                problems.append(f"removed registry package {name} {sorted(versions)}")
            continue
        if after[name] != versions:
            problems.append(
                f"bumped {name} from {sorted(versions)} to {sorted(after[name])}"
            )
    if problems:
        fail("lockfile did not change minimally:\n" + "\n".join(problems))
    removed = sorted(set(before) - set(after))
    added = sorted(set(after) - set(before))
    print("registry packages removed:", ", ".join(removed) or "(none)")
    print("registry packages added:", ", ".join(added) or "(none)")


def assert_redirected(meta):
    names = upstream_names()
    found = {}
    tokio = []
    for package in resolved_packages(meta):
        name = package["name"]
        if name in names:
            fail(f"upstream package still resolved: {name} {package['version']} {package['source']}")
        if "tokio" in name:
            tokio.append(name)
        found[name] = package
    if tokio:
        fail(f"tokio packages in the redirected graph: {sorted(set(tokio))}")
    for name in REQUIRED:
        package = found.get(name)
        if package is None:
            fail(f"missing {name} in the redirected graph")
        if package["source"] is not None:
            fail(f"{name} resolved from {package['source']}")
        manifest = package.get("manifest_path", "").replace("\\", "/")
        if "/crates/" not in manifest or "/.work/h12tiny/" in manifest:
            fail(f"{name} is not a smolith crate: {manifest}")
    print("redirected graph resolves owned crates to laputa path sources and no tokio")


def test_totals(output):
    passed = 0
    failed = 0
    saw = False
    for line in output.splitlines():
        if not line.startswith("test result:"):
            continue
        saw = True
        passed_match = re.search(r"(\d+) passed", line)
        failed_match = re.search(r"(\d+) failed", line)
        if not passed_match or not failed_match:
            fail(f"unparsed test result: {line}")
        passed += int(passed_match.group(1))
        failed += int(failed_match.group(1))
    return saw, passed, failed


def run_gates():
    results = [("backpressure", "gap", BACKPRESSURE_GAP)]
    for name, command in GATES:
        print(f"running gate {name}", flush=True)
        result = subprocess.run(
            command,
            cwd=CHECKOUT,
            env=cargo_env(),
            text=True,
            capture_output=True,
            timeout=GATE_TIMEOUT_SECONDS,
        )
        sys.stdout.write(result.stdout)
        sys.stderr.write(result.stderr)
        combined = result.stdout + result.stderr
        saw, passed, failed = test_totals(combined)
        if result.returncode == 0 and saw and passed >= 1 and failed == 0:
            results.append((name, "pass", f"{passed} passed"))
            print(f"gate {name}: pass ({passed} passed)", flush=True)
            continue
        detail = f"exit {result.returncode}, parsed={saw}, passed={passed}, failed={failed}"
        results.append((name, "fail", detail))
        print(f"gate {name}: fail ({detail})", flush=True)
    with open(GATES_PATH, "w", encoding="utf-8") as handle:
        for name, status, detail in results:
            handle.write(f"{name}\t{status}\t{detail}\n")
    print("gate backpressure: gap: " + BACKPRESSURE_GAP)
    failed_gates = [name for name, status, _detail in results if status == "fail"]
    if failed_gates:
        fail("gates failed: " + ", ".join(failed_gates))
    print("h12tiny fixture passed")


def main():
    repo, commit = read_pin()
    source = acquire(repo, commit)
    head = subprocess.check_output(
        ["git", "-C", CHECKOUT, "rev-parse", "HEAD"], text=True
    ).strip()
    if head != commit:
        fail(f"pin {commit} checked out as {head}")
    baseline_meta = cargo_metadata(locked=True, features=None)
    before = registry_versions(baseline_meta)
    record_baseline(source, commit, baseline_meta)
    redirect()
    redirected = cargo_metadata(locked=False, features=FEATURES)
    assert_minimal_lock(before, registry_versions(redirected))
    assert_redirected(redirected)
    run_gates()


if __name__ == "__main__":
    try:
        main()
    except subprocess.TimeoutExpired as error:
        fail(f"command timed out: {error.cmd}")
