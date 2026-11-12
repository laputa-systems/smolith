"""Publishable package inventory for the runtime, HTTP, and protocol families.

Cargo manifests decide publish eligibility. UPSTREAM.toml names only
upstream-derived runtime and protocol crates. HTTP packages are the other
publishable workspace members and share one version.
"""

import re
import sys
import tomllib
from pathlib import Path

from identity import family_of, load_release, requirement_matches

SEMVER = re.compile(
    r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$"
)
MEMBER_PATH = re.compile(r"^[A-Za-z0-9_./-]+$")


class Inventory:
    def __init__(self, workspace_version, components):
        self.workspace_version = workspace_version
        self.components = components
        self.http_version = None
        self.packages = {}
        self.manifests = {}
        self.edges = []
        self.order = []
        self.member_manifests = []

    @property
    def upstream_names(self):
        return {spec["upstream_name"] for spec in self.components.values()}

    @property
    def owned_names(self):
        return set(self.packages)

    def version_of(self, name):
        return self.packages[name]["version"]

    def family_of_package(self, name):
        return self.packages[name]["family"]


def _package_version(package, workspace_version, family, label):
    raw = package.get("version")
    if isinstance(raw, dict) and raw.get("workspace") is True:
        actual = workspace_version
    elif isinstance(raw, str):
        actual = raw
    elif family == "runtime" and raw is None:
        actual = workspace_version
    else:
        raise ValueError(f"{label} has no package version")
    if not SEMVER.fullmatch(actual):
        raise ValueError(f"{label} version {actual} is not a semver version")
    return actual


def _aliases(workspace_deps):
    aliases = {}
    for key, spec in workspace_deps.items():
        if isinstance(spec, str):
            aliases[key] = key
        elif isinstance(spec, dict):
            aliases[key] = spec.get("package", key)
        else:
            raise ValueError(f"workspace.dependencies {key} has an unsupported specification")
    return aliases


def _dependency_scopes(document):
    scopes = [document]
    targets = document.get("target")
    if isinstance(targets, dict):
        scopes.extend(target for target in targets.values() if isinstance(target, dict))
    return scopes


def _edges(documents, manifests, aliases, owned):
    edges = set()
    for name, document in documents.items():
        for scope in _dependency_scopes(document):
            for kind in ("dependencies", "build-dependencies"):
                for alias, spec in scope.get(kind, {}).items():
                    if isinstance(spec, dict) and str(spec.get("package", "")).startswith("laputa-"):
                        raise ValueError(f"direct owned dependency in {manifests[name]}: {alias}")
                    resolved = aliases.get(alias)
                    declared = spec.get("package", alias) if isinstance(spec, dict) else alias
                    inherited = isinstance(spec, dict) and spec.get("workspace") is True
                    if inherited and alias not in aliases:
                        raise ValueError(
                            f"dependency {alias} sets workspace = true without a workspace.dependencies entry"
                        )
                    if declared in owned and not inherited:
                        raise ValueError(f"owned dependency does not inherit workspace: {alias}")
                    if resolved in owned and resolved != name:
                        if not inherited:
                            raise ValueError(f"owned dependency does not inherit workspace: {alias}")
                        edges.add((name, resolved))
    return sorted(edges)


def order_packages(names, edges):
    incoming = {name: set() for name in names}
    for dependent, dependency in edges:
        if dependent not in incoming or dependency not in incoming:
            raise ValueError(f"unknown package in edge: {dependent} {dependency}")
        incoming[dependent].add(dependency)
    ready = sorted(name for name, deps in incoming.items() if not deps)
    ordered = []
    while ready:
        name = ready.pop(0)
        ordered.append(name)
        for other in sorted(incoming):
            if name in incoming[other]:
                incoming[other].remove(name)
                if not incoming[other] and other not in ordered and other not in ready:
                    ready.append(other)
        ready.sort()
    if len(ordered) != len(names):
        raise ValueError(f"dependency cycle: {sorted(set(names) - set(ordered))}")
    return ordered


def _workspace_requirement(workspace_deps, package_name, expected, required):
    found_entry = False
    for key, spec in workspace_deps.items():
        if isinstance(spec, str):
            found = key
            requirement = spec
        elif isinstance(spec, dict):
            found = spec.get("package", key)
            requirement = spec.get("version")
        else:
            continue
        if found != package_name:
            continue
        found_entry = True
        if not requirement_matches(requirement, expected):
            raise ValueError(f"workspace.dependencies {key} version {requirement} != {expected}")
    if required and not found_entry:
        raise ValueError(f"workspace.dependencies has no entry for {package_name}")


def load(root):
    root = Path(root)
    version, components = load_release(root)
    if not SEMVER.fullmatch(version):
        raise ValueError("invalid workspace version")
    cargo = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    workspace = cargo["workspace"]
    workspace_deps = workspace.get("dependencies", {})
    by_package = {spec["package"]: (key, spec) for key, spec in components.items()}
    inventory = Inventory(version, components)
    documents = {}
    http_versions = {}
    for directory in workspace["members"]:
        if not MEMBER_PATH.fullmatch(directory):
            raise ValueError(f"unsupported member path: {directory}")
        relative = Path(directory) / "Cargo.toml"
        document = tomllib.loads((root / relative).read_text(encoding="utf-8"))
        package = document["package"]
        inventory.member_manifests.append(relative.as_posix())
        if package.get("publish") is False:
            continue
        name = package["name"]
        if name in documents:
            raise ValueError(f"duplicate release package: {name}")
        component = by_package.get(name)
        if component is None:
            family = "http"
            expected = None
        else:
            family = family_of(component[1])
            expected = component[1]["version"]
        actual = _package_version(package, version, family, relative.as_posix())
        if expected is not None and actual != expected:
            raise ValueError(f"{relative.as_posix()} version {actual} != component {expected}")
        if family == "http":
            http_versions[name] = actual
        elif family == "runtime" and actual != version:
            raise ValueError(f"{name} version {actual} != workspace {version}")
        documents[name] = document
        inventory.packages[name] = {
            "version": actual,
            "family": family,
            "manifest": relative.as_posix(),
        }
        inventory.manifests[name] = relative.as_posix()
    component_packages = set(by_package)
    if not component_packages <= set(documents):
        missing = sorted(component_packages - set(documents))
        raise ValueError(f"upstream components are not publishable members: {missing}")
    if http_versions:
        distinct = set(http_versions.values())
        if len(distinct) != 1:
            raise ValueError(f"HTTP packages do not share one version: {sorted(http_versions.items())}")
        inventory.http_version = distinct.pop()
    for name, info in inventory.packages.items():
        if info["family"] == "protocol":
            _workspace_requirement(workspace_deps, name, info["version"], True)
        elif info["family"] == "http":
            _workspace_requirement(workspace_deps, name, info["version"], False)
    aliases = _aliases(workspace_deps)
    inventory.edges = _edges(documents, inventory.manifests, aliases, set(inventory.packages))
    inventory.order = order_packages(list(inventory.packages), inventory.edges)
    return inventory


def core_tuple(version):
    if not SEMVER.fullmatch(version):
        raise ValueError(f"invalid version {version}")
    core = version.split("+", 1)[0].split("-", 1)[0]
    return tuple(int(part) for part in core.split("."))


def at_least(left, right):
    return core_tuple(left) >= core_tuple(right)


def next_release(current, latest=""):
    if not SEMVER.fullmatch(current):
        raise ValueError(f"invalid version {current}")
    base = current
    if latest:
        if not SEMVER.fullmatch(latest):
            raise ValueError(f"invalid version {latest}")
        if core_tuple(current) < core_tuple(latest):
            base = latest
    major, minor, _patch = core_tuple(base)
    return f"{major}.{minor + 1}.0"


def local_plan(inventory):
    planned = {}
    for name, info in inventory.packages.items():
        if info["family"] == "protocol":
            planned[name] = info["version"]
        elif info["family"] == "http":
            planned[name] = next_release(inventory.http_version)
        else:
            planned[name] = next_release(inventory.workspace_version)
    return planned


def exact_version_present(intended, present_versions):
    return intended in set(present_versions)


def main(argv):
    root = Path(__file__).resolve().parent.parent
    command = argv[1]
    if command == "version":
        print(load(root).workspace_version)
        return
    if command == "packages":
        inventory = load(root)
        for name in inventory.order:
            info = inventory.packages[name]
            print(f"{name} {info['family']} {info['version']} {info['manifest']}")
        return
    if command == "plan":
        inventory = load(root)
        planned = local_plan(inventory)
        for index, name in enumerate(inventory.order, start=1):
            info = inventory.packages[name]
            print(
                f"{index} {name} {info['family']} {info['version']} {planned[name]} crates-io mycrates"
            )
        return
    if command == "next":
        latest = argv[3] if len(argv) > 3 else ""
        print(next_release(argv[2], latest))
        return
    if command == "at-least":
        raise SystemExit(0 if at_least(argv[2], argv[3]) else 1)
    if command == "pending":
        intended = argv[2]
        present = argv[3:]
        raise SystemExit(0 if exact_version_present(intended, present) else 1)
    raise ValueError(f"unknown inventory command: {command}")


if __name__ == "__main__":
    try:
        main(sys.argv)
    except Exception as error:
        print(f"inventory: {error}", file=sys.stderr)
        sys.exit(1)
