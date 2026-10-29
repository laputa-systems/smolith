#!/usr/bin/env python3
"""Resolve and test the external consumer. No workspace patches."""

import json
import os
import subprocess
import sys

CONSUMER = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(CONSUMER, "..", ".."))
MANIFEST = os.path.join(CONSUMER, "Cargo.toml")


def jaq(expression, path):
    result = subprocess.run(
        ["jaq", "--from", "toml", "-r", expression, path],
        check=True,
        text=True,
        capture_output=True,
    )
    return [line for line in result.stdout.splitlines() if line]


def fail(message):
    print(f"consumer check failed: {message}", file=sys.stderr)
    sys.exit(1)


def manifest_text():
    with open(MANIFEST, encoding="utf-8") as handle:
        return handle.read()


def assert_isolated():
    text = manifest_text()
    if "[patch" in text:
        fail("consumer manifest contains a [patch] table")
    members = jaq(".workspace.members[]", os.path.join(ROOT, "Cargo.toml"))
    if "tests/consumer" in members or "tests/consumer/" in members:
        fail("tests/consumer is a smolith workspace member")
    if "[workspace]" not in text:
        fail("consumer must be its own workspace so it does not inherit smolith")


def metadata():
    env = os.environ.copy()
    env["TMPDIR"] = os.path.join(ROOT, ".work", "tmp")
    env["CARGO_TARGET_DIR"] = os.path.join(ROOT, "target")
    os.makedirs(env["TMPDIR"], exist_ok=True)
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--manifest-path", MANIFEST],
        check=False,
        text=True,
        capture_output=True,
        env=env,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        fail("cargo metadata failed")
    return json.loads(result.stdout)


def closure(meta):
    nodes = {node["id"]: node for node in meta["resolve"]["nodes"]}
    packages = {package["id"]: package for package in meta["packages"]}
    roots = set(meta["workspace_members"])
    seen = set()
    stack = list(roots)
    while stack:
        current = stack.pop()
        if current in seen:
            continue
        seen.add(current)
        node = nodes.get(current)
        if node is None:
            continue
        for dep in node["deps"]:
            kinds = [item.get("kind") for item in dep.get("dep_kinds", [])]
            if any(kind in (None, "build") for kind in kinds):
                stack.append(dep["pkg"])
    return [packages[item] for item in seen if item in packages]


def assert_graph(meta):
    upstream = set(jaq(".components[].upstream_name", os.path.join(ROOT, "UPSTREAM.toml")))
    laputa = set(jaq(".components[].package", os.path.join(ROOT, "UPSTREAM.toml")))
    if len(upstream) != 26 or len(laputa) != 26:
        fail(f"expected 26 upstream components, got {len(upstream)} names and {len(laputa)} packages")
    found = {package["name"] for package in closure(meta)}
    leaked = sorted(found & upstream)
    if leaked:
        fail(f"upstream packages in the consumer graph: {leaked}")
    tokio = sorted(name for name in found if "tokio" in name)
    if tokio:
        fail(f"tokio packages in the consumer graph: {tokio}")
    owned = sorted(found & laputa)
    if "laputa-smol" not in owned:
        fail(f"laputa-smol did not resolve; owned packages: {owned}")
    by_id = {package["id"]: package for package in meta["packages"]}
    for package in closure(meta):
        if package["name"] not in laputa:
            continue
        if package["source"] is not None:
            fail(f"{package['name']} resolved from {package['source']}")
        manifest = package.get("manifest_path", "")
        if "/crates/" not in manifest.replace("\\", "/"):
            fail(f"{package['name']} manifest is not under crates/: {manifest}")
    for trait_crate in ("futures-io", "futures-core"):
        matches = [package for package in by_id.values() if package["name"] == trait_crate]
        versions = {package["version"] for package in matches}
        if len(versions) != 1:
            fail(f"{trait_crate} should resolve to one version, got {sorted(versions)}")
    print("consumer graph: laputa path sources, shared futures-io and futures-core, no tokio")
    print("owned packages:", ", ".join(owned))


def run_tests():
    env = os.environ.copy()
    env["TMPDIR"] = os.path.join(ROOT, ".work", "tmp")
    env["CARGO_TARGET_DIR"] = os.path.join(ROOT, "target")
    os.makedirs(env["TMPDIR"], exist_ok=True)
    result = subprocess.run(
        ["cargo", "test", "--manifest-path", MANIFEST, "--locked"],
        env=env,
    )
    if result.returncode != 0:
        fail("cargo test failed")


def main():
    assert_isolated()
    meta = metadata()
    assert_graph(meta)
    run_tests()
    print("consumer check passed")


if __name__ == "__main__":
    main()
