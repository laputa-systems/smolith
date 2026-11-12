"""Rewrite owned package versions without touching external requirements.

Runtime packages share workspace.package.version. HTTP packages share the
version recorded on their manifests and workspace dependencies. Protocol
forks and third-party crates keep the versions already written down.
"""

import sys
from pathlib import Path

from inventory import load


def _version_assignment(version):
    return f'version = "{version}"'


def _replace_version(line, old, new):
    current = _version_assignment(old)
    if current not in line:
        raise ValueError(f"version assignment {old} was not on the line")
    if line.count(current) != 1:
        raise ValueError(f"version assignment {old} was ambiguous")
    return line.replace(current, _version_assignment(new), 1)


def _section(stripped):
    header = stripped.split("#", 1)[0].strip()
    if len(header) >= 2 and header.startswith("[") and header.endswith("]"):
        return header
    return None


def _dependency_key(stripped):
    if "=" not in stripped or stripped.startswith("#"):
        return None
    return stripped.split("=", 1)[0].strip()


def rewrite_root(text, runtime_old, runtime_new, http_old, http_new, http_names):
    section = ""
    out = []
    for line in text.splitlines(keepends=True):
        stripped = line.strip()
        header = _section(stripped)
        if header is not None:
            section = header
        elif section == "[workspace.package]" and _dependency_key(stripped) == "version":
            line = _replace_version(line, runtime_old, runtime_new)
        elif section == "[workspace.dependencies]":
            key = _dependency_key(stripped)
            if http_old and key in http_names:
                line = _replace_version(line, http_old, http_new)
            elif f'version = "{runtime_old}"' in line and 'package = "laputa-' in line:
                line = _replace_version(line, runtime_old, runtime_new)
        out.append(line)
    return "".join(out)


def rewrite_upstream(text, runtime_old, runtime_new, runtime_count):
    current = _version_assignment(runtime_old)
    matches = 0
    out = []
    for line in text.splitlines(keepends=True):
        if line.strip() == current:
            matches += 1
            prefix = line[: len(line) - len(line.lstrip())]
            ending = "\n" if line.endswith("\n") else ""
            line = f"{prefix}{_version_assignment(runtime_new)}{ending}"
        out.append(line)
    if matches != runtime_count:
        raise ValueError(f"UPSTREAM.toml has {matches} runtime versions, expected {runtime_count}")
    return "".join(out)


def rewrite_manifest(text, package_version, old, new):
    if old == new:
        return text
    section = ""
    replaced = False
    out = []
    for line in text.splitlines(keepends=True):
        stripped = line.strip()
        header = _section(stripped)
        if header is not None:
            section = header
        elif (
            package_version
            and section == "[package]"
            and _dependency_key(stripped) == "version"
            and f'version = "{old}"' in line
        ):
            line = _replace_version(line, old, new)
            replaced = True
        out.append(line)
    if package_version and not replaced:
        raise ValueError("package version was not updated")
    return "".join(out)


def rewrite_requirement_line(line, runtime_names, http_names, runtime_old, runtime_new, http_old, http_new):
    stripped = line.strip()
    if stripped.startswith("#"):
        return line
    key = _dependency_key(stripped)
    if http_old and http_old != http_new and f'version = "{http_old}"' in line:
        if key in http_names or any(f'"{name}"' in line for name in http_names):
            return _replace_version(line, http_old, http_new)
    if runtime_old != runtime_new and f'version = "{runtime_old}"' in line:
        if key in runtime_names or any(f'package = "{name}"' in line for name in runtime_names):
            return _replace_version(line, runtime_old, runtime_new)
    return line


def _toml_files(root):
    for path in root.rglob("Cargo.toml"):
        relative = path.relative_to(root)
        if any(part in {".git", "target", ".work"} for part in relative.parts):
            continue
        yield path


def apply(root, runtime_next, http_next=None):
    root = Path(root)
    inventory = load(root)
    runtime_old = inventory.workspace_version
    http_old = inventory.http_version
    if http_old is None:
        http_next = None
    elif not http_next:
        raise ValueError("HTTP packages need a planned version")
    runtime_names = {
        name for name, info in inventory.packages.items() if info["family"] == "runtime"
    }
    http_names = {name for name, info in inventory.packages.items() if info["family"] == "http"}
    runtime_aliases = set()
    cargo = (root / "Cargo.toml").read_text(encoding="utf-8")
    # Alias keys are the workspace dependency names, which requirement lines use.
    for name in runtime_names:
        runtime_aliases.add(name)
    for line in cargo.splitlines():
        stripped = line.strip()
        if 'package = "laputa-' not in stripped:
            continue
        for name in runtime_names:
            if f'package = "{name}"' in stripped:
                key = _dependency_key(stripped)
                if key:
                    runtime_aliases.add(key)
    changed = []

    def write(path, updated, original):
        if updated != original:
            path.write_text(updated, encoding="utf-8")
            changed.append(path.relative_to(root).as_posix())

    root_manifest = root / "Cargo.toml"
    original = root_manifest.read_text(encoding="utf-8")
    write(
        root_manifest,
        rewrite_root(original, runtime_old, runtime_next, http_old, http_next, http_names),
        original,
    )
    upstream = root / "UPSTREAM.toml"
    original = upstream.read_text(encoding="utf-8")
    write(
        upstream,
        rewrite_upstream(original, runtime_old, runtime_next, len(runtime_names)),
        original,
    )
    http_manifests = {
        info["manifest"] for info in inventory.packages.values() if info["family"] == "http"
    }
    for path in _toml_files(root):
        relative = path.relative_to(root).as_posix()
        if relative == "Cargo.toml":
            continue
        original = path.read_text(encoding="utf-8")
        if relative in http_manifests:
            updated = rewrite_manifest(original, True, http_old, http_next)
        else:
            updated = "".join(
                rewrite_requirement_line(
                    line,
                    runtime_aliases,
                    http_names,
                    runtime_old,
                    runtime_next,
                    http_old,
                    http_next,
                )
                for line in original.splitlines(keepends=True)
            )
        write(path, updated, original)
    return changed


def main(argv):
    root = Path(__file__).resolve().parent.parent
    if argv[1] != "apply":
        raise ValueError(f"unknown release apply command: {argv[1]}")
    http_next = None if len(argv) < 4 or argv[3] == "-" else argv[3]
    for path in apply(root, argv[2], http_next):
        print(path)


if __name__ == "__main__":
    try:
        main(sys.argv)
    except Exception as error:
        print(f"release apply: {error}", file=sys.stderr)
        sys.exit(1)
