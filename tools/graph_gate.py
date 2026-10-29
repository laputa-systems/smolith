#!/usr/bin/env python3
import json
import os
import subprocess
import sys

UPSTREAM_NAMES = [
    "smol", "smol-macros", "async-executor", "async-task", "async-io",
    "polling", "blocking", "async-net", "async-fs", "async-process",
    "async-signal", "async-channel", "async-broadcast", "async-lock",
    "event-listener", "event-listener-strategy", "concurrent-queue",
    "parking", "atomic-waker", "piper", "futures-lite", "fastrand",
    "fastrand-contrib", "easy-parallel", "waker-fn", "async-dup"
]
EXPECTED_LAPUTA = {f"laputa-{n}" for n in UPSTREAM_NAMES}

def check_no_patches(root_dir):
    root_toml = os.path.join(root_dir, "Cargo.toml")
    with open(root_toml, "r") as f:
        content = f.read()
    if "[patch" in content:
        raise ValueError(f"Root Cargo.toml contains forbidden [patch] table")

    crates_dir = os.path.join(root_dir, "crates")
    for c in os.listdir(crates_dir):
        cp = os.path.join(crates_dir, c, "Cargo.toml")
        if os.path.isfile(cp):
            with open(cp, "r") as f:
                c_content = f.read()
            if "[patch" in c_content:
                raise ValueError(f"{cp} contains forbidden [patch] table")

def check_member_manifests(root_dir):
    crates_dir = os.path.join(root_dir, "crates")
    for name in UPSTREAM_NAMES:
        cp = os.path.join(crates_dir, name, "Cargo.toml")
        if not os.path.isfile(cp):
            raise ValueError(f"Missing member crate manifest: {cp}")
        with open(cp, "r") as f:
            lines = f.readlines()

        in_dev = False
        in_section = None
        for line in lines:
            line_s = line.strip()
            if line_s.startswith("["):
                in_section = line_s.strip("[]")
                in_dev = "dev-dependencies" in in_section

            # Versionless path check: only dev-dependencies may have versionless path edges
            if not in_dev and in_section and ("dependencies" in in_section):
                for dep in UPSTREAM_NAMES:
                    laputa_dep = f"laputa-{dep}"
                    if (f'"{laputa_dep}"' in line_s or f"'{laputa_dep}'" in line_s or line_s.startswith(f"{dep} ") or line_s.startswith(f"{dep}=")) and "path" in line_s:
                        if "version" not in line_s and "workspace" not in line_s:
                            raise ValueError(
                                f"{cp} has versionless path edge in non-dev section [{in_section}]: {line_s}"
                            )

def check_metadata(metadata_json, label=""):
    meta = json.loads(metadata_json)
    packages_by_id = {p["id"]: p for p in meta["packages"]}
    workspace_members = set(meta["workspace_members"])

    if len(workspace_members) != 26:
        raise ValueError(f"[{label}] Expected 26 workspace members, got {len(workspace_members)}")

    # Check each workspace member
    for pid in workspace_members:
        p = packages_by_id[pid]
        if p["name"] not in EXPECTED_LAPUTA:
            raise ValueError(f"[{label}] Member {p['name']} not in expected laputa-* set")
        if p["version"] != "0.1.0":
            raise ValueError(f"[{label}] Member {p['name']} has version {p['version']} != 0.1.0")
        if p["source"] is not None:
            raise ValueError(f"[{label}] Member {p['name']} source is {p['source']}, expected path (None)")

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
