import sys
from pathlib import Path

from inventory import load


def graph(root):
    inventory = load(root)
    return inventory.workspace_version, dict(inventory.manifests), list(inventory.edges)


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
