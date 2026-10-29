#!/usr/bin/env python3
import json
import os
import subprocess
import sys
import tomllib

UPSTREAM_NAMES = [
    "smol", "smol-macros", "async-executor", "async-task", "async-io",
    "polling", "blocking", "async-net", "async-fs", "async-process",
    "async-signal", "async-channel", "async-broadcast", "async-lock",
    "event-listener", "event-listener-strategy", "concurrent-queue",
    "parking", "atomic-waker", "piper", "futures-lite", "fastrand",
    "fastrand-contrib", "easy-parallel", "waker-fn", "async-dup"
]
EXPECTED_LAPUTA = {f"laputa-{n}" for n in UPSTREAM_NAMES}
ALLOWED_EXTRA_MEMBERS = {"smolith-qualify"}
DEPENDENCY_TABLES = ("dependencies", "build-dependencies", "dev-dependencies")

def load_toml(path):
    with open(path, "rb") as handle:
        return tomllib.load(handle)

def iter_dependency_tables(doc):
    yield from dependency_tables(doc)
    targets = doc.get("target")
    if isinstance(targets, dict):
        for target_doc in targets.values():
            if isinstance(target_doc, dict):
                yield from dependency_tables(target_doc)

def dependency_tables(doc):
    for kind in DEPENDENCY_TABLES:
        table = doc.get(kind)
        if isinstance(table, dict):
            yield kind, table

def resolve_dependency(name, spec, workspace_deps):
    if isinstance(spec, str):
        return {"version": spec}
    if not isinstance(spec, dict):
        raise ValueError(f"dependency {name} has an unsupported specification")
    if not spec.get("workspace"):
        return spec
    base = workspace_deps.get(name)
    if isinstance(base, str):
        merged = {"version": base}
    elif isinstance(base, dict):
        merged = dict(base)
    else:
        raise ValueError(f"dependency {name} sets workspace = true without a workspace.dependencies entry")
    for key, value in spec.items():
        if key != "workspace":
            merged[key] = value
    return merged

def is_owned_edge(name, spec, upstream_names, laputa_names):
    package = spec.get("package", name)
    return name in upstream_names or package in upstream_names or package in laputa_names

def versionless_owned_paths(doc, workspace_deps, upstream_names, laputa_names):
    found = []
    for kind, table in iter_dependency_tables(doc):
        if kind == "dev-dependencies":
            continue
        for name, spec in table.items():
            resolved = resolve_dependency(name, spec, workspace_deps)
            if "path" not in resolved or "version" in resolved:
                continue
            if is_owned_edge(name, resolved, upstream_names, laputa_names):
                found.append(f"[{kind}] {name}")
    return found

def check_no_patches(root_dir):
    paths = [os.path.join(root_dir, "Cargo.toml")]
    crates_dir = os.path.join(root_dir, "crates")
    for name in os.listdir(crates_dir):
        path = os.path.join(crates_dir, name, "Cargo.toml")
        if os.path.isfile(path):
            paths.append(path)
    for path in paths:
        if "patch" in load_toml(path):
            raise ValueError(f"{path} contains a [patch] table")

def check_member_manifests(root_dir):
    workspace_deps = load_toml(os.path.join(root_dir, "Cargo.toml")).get("workspace", {}).get("dependencies", {})
    crates_dir = os.path.join(root_dir, "crates")
    upstream_names = set(UPSTREAM_NAMES)
    for name in UPSTREAM_NAMES:
        path = os.path.join(crates_dir, name, "Cargo.toml")
        if not os.path.isfile(path):
            raise ValueError(f"Missing member crate manifest: {path}")
        problems = versionless_owned_paths(load_toml(path), workspace_deps, upstream_names, EXPECTED_LAPUTA)
        if problems:
            raise ValueError(f"{path} has versionless path edges: {', '.join(problems)}")

def self_check_manifests():
    upstream = {"async-io", "futures-lite"}
    laputa = {"laputa-async-io", "laputa-futures-lite"}
    workspace_deps = {
        "async-io": {"package": "laputa-async-io", "version": "0.1.0", "path": "crates/async-io"},
    }
    cases = (
        (
            True,
            """
            # a comment mentioning [patch.crates-io] is not a patch table
            [dependencies.async-io]
            package = "laputa-async-io"
            path = "../async-io"
            """,
        ),
        (
            True,
            """
            [target.'cfg(unix)'.build-dependencies.async-io]
            package = "laputa-async-io"
            path = "../async-io"
            """,
        ),
        (
            False,
            """
            [dependencies]
            async-io = { workspace = true }
            """,
        ),
        (
            False,
            """
            [dev-dependencies]
            futures-lite = { package = "laputa-futures-lite", path = "../futures-lite" }
            """,
        ),
        (
            False,
            """
            [dependencies]
            libc = { path = "vendor/libc" }
            """,
        ),
    )
    for expect_hit, text in cases:
        problems = versionless_owned_paths(tomllib.loads(text), workspace_deps, upstream, laputa)
        if bool(problems) != expect_hit:
            raise ValueError(f"manifest fixture mismatch: {problems!r} from {text.strip()}")
    uncommented = tomllib.loads("# [patch.crates-io]\n[dependencies]\nlibc = \"0.2\"\n")
    if "patch" in uncommented:
        raise ValueError("a comment was parsed as a [patch] table")
    patched = tomllib.loads("[patch.crates-io]\nlibc = { path = \"vendor/libc\" }\n")
    if "patch" not in patched:
        raise ValueError("a [patch] table was ignored")
    bare = {"async-io": {"package": "laputa-async-io", "path": "crates/async-io"}}
    inherited = tomllib.loads("[dependencies]\nasync-io = { workspace = true }\n")
    if not versionless_owned_paths(inherited, bare, upstream, laputa):
        raise ValueError("workspace inheritance hid a versionless path")

def check_metadata(metadata_json, label=""):
    meta = json.loads(metadata_json)
    packages_by_id = {p["id"]: p for p in meta["packages"]}
    workspace_members = set(meta["workspace_members"])

    member_names = set()
    for pid in workspace_members:
        p = packages_by_id[pid]
        member_names.add(p["name"])
        if p["version"] != "0.1.0":
            raise ValueError(f"[{label}] Member {p['name']} has version {p['version']} != 0.1.0")
        if p["source"] is not None:
            raise ValueError(f"[{label}] Member {p['name']} source is {p['source']}, expected path (None)")
        if p["name"] in ALLOWED_EXTRA_MEMBERS:
            manifest = p.get("manifest_path", "")
            with open(manifest, "r") as handle:
                if "publish = false" not in handle.read():
                    raise ValueError(f"[{label}] Extra member {p['name']} must set publish = false")
        elif p["name"] not in EXPECTED_LAPUTA:
            raise ValueError(f"[{label}] Member {p['name']} not in expected laputa-* set")

    missing = EXPECTED_LAPUTA - member_names
    if missing:
        raise ValueError(f"[{label}] Missing owned members: {sorted(missing)}")
    unexpected = member_names - EXPECTED_LAPUTA - ALLOWED_EXTRA_MEMBERS
    if unexpected:
        raise ValueError(f"[{label}] Unexpected members: {sorted(unexpected)}")

    # Check exactly one version per owned component
    for laputa_name in EXPECTED_LAPUTA:
        matching = [p for p in meta["packages"] if p["name"] == laputa_name]
        if len(matching) != 1:
            raise ValueError(f"[{label}] Expected 1 version of {laputa_name}, got {len(matching)}")
        if matching[0]["version"] != "0.1.0":
            raise ValueError(f"[{label}] {laputa_name} version is {matching[0]['version']} != 0.1.0")

    # Traverse normal and build dependency graph
    resolve_nodes = {n["id"]: n for n in meta["resolve"]["nodes"]}
    normal_build_pkgs = set()

    for w_id in workspace_members:
        node = resolve_nodes.get(w_id)
        if not node:
            continue
        for dep in node["deps"]:
            for k in dep["dep_kinds"]:
                if k["kind"] in (None, "build"):
                    normal_build_pkgs.add(dep["pkg"])

    to_visit = list(normal_build_pkgs)
    while to_visit:
        curr = to_visit.pop()
        node = resolve_nodes.get(curr)
        if not node:
            continue
        for dep in node["deps"]:
            for k in dep["dep_kinds"]:
                if k["kind"] in (None, "build"):
                    if dep["pkg"] not in normal_build_pkgs:
                        normal_build_pkgs.add(dep["pkg"])
                        to_visit.append(dep["pkg"])

    # Verify normal/build packages
    for pid in normal_build_pkgs:
        p = packages_by_id[pid]
        pname = p["name"]

        # 1. No upstream un-prefixed smol names
        if pname in UPSTREAM_NAMES:
            raise ValueError(f"[{label}] Upstream package {pname} found in normal/build path: {pid}")

        # 2. No tokio or tokio-util
        if "tokio" in pname:
            raise ValueError(f"[{label}] Tokio package {pname} found in normal/build path: {pid}")

        # 3. Owned components must resolve to path source (None)
        if pname in EXPECTED_LAPUTA:
            if p["source"] is not None:
                raise ValueError(f"[{label}] Owned component {pname} resolved from non-path source: {p['source']}")

def run_gate():
    root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    print(f"Running graph gate on {root_dir}")

    # 1. Check manifests for patches and versionless edges
    print("Checking manifests for forbidden patches and versionless edges...")
    self_check_manifests()
    check_no_patches(root_dir)
    check_member_manifests(root_dir)
    print("Manifest checks passed.")

    # 2. Targets to test
    installed_targets = subprocess.check_output(
        ["rustup", "target", "list", "--installed"],
        text=True
    ).splitlines()

    test_targets = [None]  # Host default
    for t in ["x86_64-unknown-linux-musl", "aarch64-unknown-linux-musl"]:
        if t in installed_targets:
            test_targets.append(t)

    # 3. Feature combinations
    feature_combos = [
        [],
        ["--all-features"],
    ]

    for target in test_targets:
        for fargs in feature_combos:
            label_parts = []
            if target:
                label_parts.append(f"target={target}")
            else:
                label_parts.append("target=host")
            if fargs:
                label_parts.append(" ".join(fargs))
            else:
                label_parts.append("default-features")
            label = ", ".join(label_parts)

            cmd = ["cargo", "metadata", "--format-version", "1"] + fargs
            if target:
                cmd.extend(["--filter-platform", target])

            print(f"Evaluating {label}...")
            output = subprocess.check_output(cmd, cwd=root_dir, text=True)
            check_metadata(output, label)

    print("GRAPH GATE PASSED: all 26 owned components resolved to laputa-* path sources, no upstream packages or tokio in normal/build paths, one version per owned component, no unpublished paths or workspace patches.")

if __name__ == "__main__":
    try:
        run_gate()
    except Exception as e:
        print(f"GRAPH GATE FAILED: {e}", file=sys.stderr)
        sys.exit(1)
