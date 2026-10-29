#!/usr/bin/env python3
"""Match RustSec advisories through UPSTREAM.toml names, then Cargo.lock registry packages.

A scan of `laputa-*` names alone is not evidence. Those names are not in the advisory database.
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tomllib
from datetime import date
from pathlib import Path

from identity import load_release

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


def identifier_is_numeric(text):
    return bool(text) and text.isascii() and text.isdigit()


def cmp_prerelease(left, right):
    left_ids = left.split(".")
    right_ids = right.split(".")
    for left_id, right_id in zip(left_ids, right_ids):
        left_num = identifier_is_numeric(left_id)
        right_num = identifier_is_numeric(right_id)
        if left_num and right_num:
            compared = int(left_id) - int(right_id)
            if compared != 0:
                return -1 if compared < 0 else 1
            continue
        if left_num != right_num:
            return -1 if left_num else 1
        if left_id != right_id:
            return -1 if left_id < right_id else 1
    if len(left_ids) == len(right_ids):
        return 0
    return -1 if len(left_ids) < len(right_ids) else 1


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
    return cmp_prerelease(left[3], right[3])


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


def git_env():
    templates = ROOT / ".work" / "tmp" / "git-templates"
    templates.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    env["GIT_TEMPLATE_DIR"] = str(templates)
    env.setdefault("GIT_AUTHOR_NAME", "advisory-scan")
    env.setdefault("GIT_AUTHOR_EMAIL", "advisory-scan@example.com")
    env.setdefault("GIT_COMMITTER_NAME", "advisory-scan")
    env.setdefault("GIT_COMMITTER_EMAIL", "advisory-scan@example.com")
    return env


def run_git(args, cwd=None):
    subprocess.check_call(["git", *args], cwd=cwd, env=git_env())


def db_revision(db):
    rev = subprocess.check_output(
        ["git", "-C", str(db), "rev-parse", "HEAD"], text=True, env=git_env()
    ).strip()
    committed = subprocess.check_output(
        ["git", "-C", str(db), "log", "-1", "--format=%cs"], text=True, env=git_env()
    ).strip()
    if not rev or not committed:
        raise SystemExit(f"{db} has no commit to identify the advisory snapshot")
    return rev, committed


def refresh_db(db, *, offline, repo):
    db.parent.mkdir(parents=True, exist_ok=True)
    crates = db / "crates"
    if offline:
        if not crates.is_dir():
            raise SystemExit(f"advisory scan --offline requires {crates}")
        rev, committed = db_revision(db)
        return "offline", rev, committed
    if (db / ".git").is_dir():
        if not crates.is_dir():
            raise SystemExit(f"{db} exists but has no crates directory")
        run_git(["-C", str(db), "fetch", "--quiet", "--depth", "1", "origin"])
        run_git(["-C", str(db), "reset", "-q", "--hard", "FETCH_HEAD"])
        rev, committed = db_revision(db)
        return "updated", rev, committed
    if db.exists():
        raise SystemExit(f"{db} exists but is not a git checkout")
    run_git(["clone", "--quiet", "--depth", "1", repo, str(db)])
    rev, committed = db_revision(db)
    return "cloned", rev, committed


def format_db(mode, rev, committed):
    age_days = (date.today() - date.fromisoformat(committed)).days
    return f"advisory db {rev} committed {committed} ({age_days} days old, {mode})"


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


def self_check_versions():
    order = (
        "1.0.0-alpha",
        "1.0.0-alpha.1",
        "1.0.0-alpha.beta",
        "1.0.0-beta",
        "1.0.0-beta.2",
        "1.0.0-beta.11",
        "1.0.0-rc.1",
        "1.0.0",
    )
    for earlier, later in zip(order, order[1:]):
        versions = {"patched": [f">= {later}"]}
        if not is_vulnerable(earlier, versions):
            raise SystemExit(f"advisory matcher failed: {earlier} should be below {later}")
        if is_vulnerable(later, versions):
            raise SystemExit(f"advisory matcher failed: {later} should satisfy >= {later}")
    checks = (
        (
            is_vulnerable("1.2.3-alpha.2", {"patched": [">= 1.2.3-alpha.10"]}),
            "1.2.3-alpha.2 should be below >= 1.2.3-alpha.10",
        ),
        (
            not is_vulnerable("1.2.3-beta.11", {"patched": [">= 1.2.3-beta.2"]}),
            "1.2.3-beta.11 should satisfy >= 1.2.3-beta.2",
        ),
        (
            is_vulnerable("1.0.0-1", {"patched": [">= 1.0.0-alpha"]}),
            "numeric prerelease identifiers precede non-numeric ones",
        ),
    )
    for ok, message in checks:
        if not ok:
            raise SystemExit(f"advisory matcher failed: {message}")


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
    _version, specs = load_release(ROOT)
    return [
        (name, spec["package"], spec["upstream_name"], spec["upstream_version"])
        for name, spec in specs.items()
    ]


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


def registry_packages(meta):
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


def lock_disposition(kind, name, version, shipped):
    if kind is None:
        return None
    if kind == "fail" and (name, version) in shipped:
        return "fail"
    if (name, version) in shipped:
        return "info"
    return "dev"


def self_check_reachability():
    registry = "registry+https://github.com/rust-lang/crates.io-index"
    meta = {
        "packages": [
            {"id": "root", "name": "root", "version": "0.1.0", "source": None},
            {"id": "optin", "name": "libm", "version": "9.9.9", "source": registry},
            {"id": "onlydev", "name": "criterion", "version": "9.9.9", "source": registry},
        ],
        "workspace_members": ["root"],
        "resolve": {
            "nodes": [
                {
                    "id": "root",
                    "deps": [
                        {"pkg": "optin", "dep_kinds": [{"kind": None}]},
                        {"pkg": "onlydev", "dep_kinds": [{"kind": "dev"}]},
                    ],
                },
                {"id": "optin", "deps": []},
                {"id": "onlydev", "deps": []},
            ]
        },
    }
    shipped = registry_packages(meta)
    if lock_disposition("fail", "libm", "9.9.9", shipped) != "fail":
        raise SystemExit("opt-in normal dependency must stay a production advisory")
    if lock_disposition("fail", "criterion", "9.9.9", shipped) != "dev":
        raise SystemExit("dev-only dependency must stay a nonfatal advisory")


def self_check_refresh():
    tmp = ROOT / ".work" / "tmp" / "advisory-self-check"
    shutil.rmtree(tmp, ignore_errors=True)
    tmp.mkdir(parents=True)
    remote = tmp / "remote"
    db = tmp / "db"
    remote.mkdir()
    run_git(["init", "-q", "-b", "main"], cwd=remote)
    crates = remote / "crates"
    crates.mkdir()
    (crates / "marker").write_text("one\n", encoding="utf-8")
    run_git(["add", "crates"], cwd=remote)
    run_git(["commit", "-q", "-m", "one"], cwd=remote)
    run_git(["clone", "--quiet", "--depth", "1", f"file://{remote}", str(db)])
    first, _committed = db_revision(db)
    (crates / "marker").write_text("two\n", encoding="utf-8")
    run_git(["add", "crates"], cwd=remote)
    run_git(["commit", "-q", "-m", "two"], cwd=remote)
    mode, rev, committed = refresh_db(db, offline=False, repo=str(remote))
    if mode != "updated" or rev == first:
        raise SystemExit("an existing advisory database was not refreshed")
    if not committed:
        raise SystemExit("refreshed advisory database has no commit date")
    mode, offline_rev, offline_date = refresh_db(db, offline=True, repo=str(remote))
    if mode != "offline" or offline_rev != rev or offline_date != committed:
        raise SystemExit("offline advisory scan moved or hid the snapshot")
    described = format_db(mode, offline_rev, offline_date)
    if "offline" not in described or offline_date not in described:
        raise SystemExit("offline advisory scan does not report snapshot age and mode")
    shutil.rmtree(tmp)


def shipped_registry_packages():
    raw = subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--all-features", "--locked"],
        cwd=ROOT,
    )
    return registry_packages(json.loads(raw))


def check_real_reachability(shipped):
    if not any(name == "libm" for name, _version in shipped):
        raise SystemExit("libm is an opt-in normal dependency and was classified as development-only")
    if any(name == "criterion" for name, _version in shipped):
        raise SystemExit("criterion is development-only and was classified as shipped")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--offline", action="store_true")
    args = parser.parse_args()
    os.chdir(ROOT)
    tmp = ROOT / ".work" / "tmp"
    tmp.mkdir(parents=True, exist_ok=True)
    os.environ["TMPDIR"] = str(tmp)
    self_check_versions()
    self_check_reachability()
    self_check_refresh()
    mode, rev, committed = refresh_db(DB, offline=args.offline, repo=ADVISORY_REPO)
    print(format_db(mode, rev, committed))
    advisories = load_advisories()
    self_check(advisories)
    failures = []
    notes = []
    owned = components()
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
    check_real_reachability(shipped)
    registry_rows = lock_packages()
    for name, version, _source in registry_rows:
        if (name, version) in checked_pairs:
            continue
        for advisory in advisories.get(name, []):
            kind = classify(advisory, version)
            fate = lock_disposition(kind, name, version, shipped)
            if fate is None:
                continue
            line = report(advisory, version, "Cargo.lock")
            if fate == "fail":
                failures.append(line)
            else:
                notes.append(f"{fate} {line}")
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
