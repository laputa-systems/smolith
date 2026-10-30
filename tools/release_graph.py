import re
import sys
import tomllib
from pathlib import Path

from identity import load_release


def graph(root):
    root = Path(root)
    version, components = load_release(root)
    if not re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?", version):
        raise ValueError("invalid workspace version")
    cargo = tomllib.loads((root / "Cargo.toml").read_text())
    workspace = cargo["workspace"]
    names = {spec["package"] for spec in components.values()}
    aliases = {key: spec.get("package", key) for key, spec in workspace["dependencies"].items() if isinstance(spec, dict)}
    manifests = {}
    documents = {}
    for directory in workspace["members"]:
        if not re.fullmatch(r"[A-Za-z0-9_./-]+", directory):
            raise ValueError(f"unsupported member path: {directory}")
        path = Path(directory) / "Cargo.toml"
        document = tomllib.loads((root / path).read_text())
        package = document["package"]
        if package.get("publish") is False:
            continue
        name = package["name"]
        if name not in names or name in manifests:
            raise ValueError(f"unknown or duplicate release package: {name}")
        manifests[name] = path.as_posix()
        documents[name] = document
    if set(manifests) != names:
        raise ValueError("release components do not match publishable workspace members")
    edges = set()
    for name, document in documents.items():
        scopes = [document, *document.get("target", {}).values()]
        for scope in scopes:
            for kind in ("dependencies", "build-dependencies"):
                for alias, spec in scope.get(kind, {}).items():
                    if isinstance(spec, dict) and str(spec.get("package", "")).startswith("laputa-"):
                        raise ValueError(f"direct owned dependency in {manifests[name]}: {alias}")
                    package = aliases.get(alias)
                    if package in names and package != name:
                        if not isinstance(spec, dict) or spec.get("workspace") is not True:
                            raise ValueError(f"owned dependency does not inherit workspace: {alias}")
                        edges.add((name, package))
    return version, manifests, sorted(edges)


if __name__ == "__main__":
    version, manifests, edges = graph(Path(__file__).resolve().parent.parent)
    mode = sys.argv[1]
    if mode == "version":
        print(version)
    elif mode == "manifests":
        for name, path in manifests.items():
            print(f"{name}:{path}")
    elif mode == "edges":
        for dependent, dependency in edges:
            print(dependent, dependency)
    else:
        raise ValueError(f"unknown release graph output: {mode}")
