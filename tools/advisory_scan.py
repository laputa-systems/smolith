#!/usr/bin/env python3
"""Match RustSec advisories through UPSTREAM.toml names, then Cargo.lock registry packages.

A scan of `laputa-*` names alone is not evidence. Those names are not in the advisory database.
"""

import json
import os
import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DB = ROOT / ".work" / "advisory-db"
ADVISORY_REPO = "https://github.com/RustSec/advisory-db.git"


def parse_version(text):
    text = text.strip()
    if "+" in text:
        text = text.split("+", 1)[0]
    pre = None
    if "-" in text:
        text, pre = text.split("-", 1)
    nums = [int(part) for part in text.split(".") if part != ""]
    original = list(nums)
    while len(nums) < 3:
        nums.append(0)
    return (nums[0], nums[1], nums[2], pre), original


def cmp_version(left, right):
    for index in range(3):
        if left[index] != right[index]:
            return -1 if left[index] < right[index] else 1
    if left[3] == right[3]:
        return 0
    if left[3] is None:
        return 1
    if right[3] is None:
        return -1
    return -1 if left[3] < right[3] else 1


def caret(version, bound, original):
    if cmp_version(version, bound) < 0:
        return False
    if not original or original[0] > 0:
        return version[0] == bound[0]
    if len(original) == 1:
        return version[0] == 0
    if original[1] > 0 or len(original) < 3:
        return version[0] == bound[0] and version[1] == bound[1]
    return version[0] == bound[0] and version[1] == bound[1] and version[2] == bound[2]


def tilde(version, bound, original):
    if cmp_version(version, bound) < 0:
        return False
    if len(original) <= 1:
        return version[0] == bound[0]
    return version[0] == bound[0] and version[1] == bound[1]


def match_one(version_text, req):
    req = req.strip()
    if req == "*":
        return True
    matched = re.match(r"^(>=|<=|>|<|=|\^|~)\s*(.+)$", req)
    if matched:
        op, rest = matched.group(1), matched.group(2).strip()
    else:
        op, rest = "^", req
    if "*" in rest:
        raise ValueError(f"unsupported version requirement {req}")
    bound, original = parse_version(rest)
    version, _unused = parse_version(version_text)
    compared = cmp_version(version, bound)
    if op == ">=":
        return compared >= 0
    if op == ">":
        return compared > 0
    if op == "<=":
        return compared <= 0
    if op == "<":
        return compared < 0
    if op == "=":
        return compared == 0
    if op == "^":
        return caret(version, bound, original)
    if op == "~":
        return tilde(version, bound, original)
    raise ValueError(f"unsupported version requirement {req}")


def matches(version_text, req):
    return all(match_one(version_text, part) for part in req.split(",") if part.strip())


def is_vulnerable(version_text, versions):
    patched = versions.get("patched") or []
    unaffected = versions.get("unaffected") or []
    affected = versions.get("affected") or []
    if any(matches(version_text, req) for req in patched):
        return False
    if any(matches(version_text, req) for req in unaffected):
        return False
    if affected:
        return any(matches(version_text, req) for req in affected)
    return True


def front_matter(text):
    marker = "```toml"
    start = text.find(marker)
    if start < 0:
        return None
    body = text[start + len(marker) :]
    end = body.find("\n```")
    if end < 0:
        raise ValueError("unclosed advisory front matter")
    return body[:end]


def ensure_db():
    DB.parent.mkdir(parents=True, exist_ok=True)
    if (DB / "crates").is_dir():
        return
    if DB.exists():
        raise SystemExit(f"{DB} exists but has no crates directory")
    subprocess.check_call(["git", "clone", "--depth", "1", ADVISORY_REPO, str(DB)])


def load_advisories():
    found = {}
    for path in sorted((DB / "crates").glob("*/*.md")):
        matter = front_matter(path.read_text(encoding="utf-8"))
        if matter is None:
            continue
        try:
            doc = tomllib.loads(matter)
        except tomllib.TOMLDecodeError as exc:
            raise SystemExit(f"{path}: {exc}") from exc
        advisory = doc.get("advisory")
        if not advisory or advisory.get("withdrawn"):
            continue
        affected = doc.get("affected") or {}
        oses = []
        if isinstance(affected, dict):
            oses = list(affected.get("os") or [])
        advisory["_versions"] = doc.get("versions") or {}
        advisory["_affected_os"] = oses
        advisory["_path"] = str(path)
        found.setdefault(advisory["package"], []).append(advisory)
    return found


def self_check(advisories):
    event = next(item for item in advisories["event-listener"] if item["id"] == "RUSTSEC-2026-0221")
    synthetic = {"patched": [">= 2.0.0"], "unaffected": ["< 1.0.0"]}
    checks = (
        (not is_vulnerable("5.4.2", event["_versions"]), "event-listener 5.4.2 should be patched"),
        (is_vulnerable("5.4.1", event["_versions"]), "event-listener 5.4.1 should be vulnerable"),
        (not is_vulnerable("5.0.0", event["_versions"]), "event-listener 5.0.0 should be unaffected"),
        (is_vulnerable("5.1.0", event["_versions"]), "event-listener 5.1.0 should be vulnerable"),
        (not is_vulnerable("2.5.0", synthetic), "2.5.0 should match patched"),
        (is_vulnerable("1.2.4", synthetic), "1.2.4 should be inside the open range"),
        (not is_vulnerable("0.9.0", synthetic), "0.9.0 should be unaffected"),
    )
    for ok, message in checks:
        if not ok:
            raise SystemExit(f"advisory matcher failed: {message}")


def components():
    doc = tomllib.loads((ROOT / "UPSTREAM.toml").read_text(encoding="utf-8"))
    rows = []
    for name, spec in doc["components"].items():
        rows.append((name, spec["package"], spec["upstream_name"], spec["upstream_version"]))
    return rows


def lock_packages():
    doc = tomllib.loads((ROOT / "Cargo.lock").read_text(encoding="utf-8"))
    rows = []
    for pkg in doc["package"]:
        source = pkg.get("source", "")
        if source.startswith("registry+"):
            rows.append((pkg["name"], pkg["version"], source))
    return rows


def classify(advisory, version):
    if not is_vulnerable(version, advisory["_versions"]):
        return None
    oses = set(advisory.get("_affected_os") or [])
    if oses and oses.isdisjoint({"linux", "macos", "unix"}):
        return "note"
    info = advisory.get("informational")
    if info and info != "unsound":
        return "note"
    return "fail"


def report(advisory, version, where):
    return f"{advisory['id']} {advisory['package']} {version} via {where}"


def shipped_registry_packages():
    raw = subprocess.check_output(["cargo", "metadata", "--format-version", "1"], cwd=ROOT)
    meta = json.loads(raw)
    packages = {pkg["id"]: pkg for pkg in meta["packages"]}
    nodes = {node["id"]: node for node in meta["resolve"]["nodes"]}
    stack = []
    for member in meta["workspace_members"]:
        for dep in nodes[member]["deps"]:
            for kind in dep["dep_kinds"]:
                if kind["kind"] in (None, "build"):
                    stack.append(dep["pkg"])
    seen = set()
    while stack:
        pkg_id = stack.pop()
        if pkg_id in seen:
            continue
        seen.add(pkg_id)
        for dep in nodes[pkg_id]["deps"]:
            for kind in dep["dep_kinds"]:
                if kind["kind"] in (None, "build"):
                    stack.append(dep["pkg"])
    shipped = set()
    for pkg_id in seen:
        pkg = packages[pkg_id]
        source = pkg.get("source") or ""
        if source.startswith("registry+"):
            shipped.add((pkg["name"], pkg["version"]))
    return shipped


def main():
    os.chdir(ROOT)
    tmp = ROOT / ".work" / "tmp"
    tmp.mkdir(parents=True, exist_ok=True)
    os.environ["TMPDIR"] = str(tmp)
    ensure_db()
    advisories = load_advisories()
    self_check(advisories)
    failures = []
    notes = []
    owned = components()
    if len(owned) != 26:
        raise SystemExit(f"UPSTREAM.toml has {len(owned)} components, expected 26")
    checked_pairs = set()
    for _key, package, upstream_name, upstream_version in owned:
        checked_pairs.add((upstream_name, upstream_version))
        for advisory in advisories.get(upstream_name, []):
            kind = classify(advisory, upstream_version)
            if kind == "fail":
                failures.append(report(advisory, upstream_version, package))
            elif kind == "note":
                notes.append("info " + report(advisory, upstream_version, package))
    shipped = shipped_registry_packages()
    registry_rows = lock_packages()
    for name, version, _source in registry_rows:
        if (name, version) in checked_pairs:
            continue
        for advisory in advisories.get(name, []):
            kind = classify(advisory, version)
            if kind is None:
                continue
            line = report(advisory, version, "Cargo.lock")
            if kind == "fail" and (name, version) in shipped:
                failures.append(line)
            else:
                notes.append(("dev " if (name, version) not in shipped else "info ") + line)
    if notes:
        print("\n".join(notes))
    if failures:
        print("\n".join(failures), file=sys.stderr)
        raise SystemExit(f"advisory scan failed: {len(failures)} vulnerable package version(s)")
    print(
        f"advisory scan passed: {len(owned)} components via upstream_name, "
        f"{len(registry_rows)} Cargo.lock registry packages, "
        f"{sum(len(items) for items in advisories.values())} advisories"
    )


if __name__ == "__main__":
    main()
