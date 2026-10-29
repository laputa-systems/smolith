"""Release version and owned-crate identity.

[workspace.package].version in the root Cargo.toml owns the version.
[components] in UPSTREAM.toml owns crate identity. A workspace dependency
on one of those crates, and the external consumer's path dependency, must
use that same version.
"""

import tomllib
from pathlib import Path


def load_release(root):
    root = Path(root)
    cargo = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    upstream = tomllib.loads((root / "UPSTREAM.toml").read_text(encoding="utf-8"))
    version, components = validate_release(cargo, upstream)
    consumer = tomllib.loads((root / "tests" / "consumer" / "Cargo.toml").read_text(encoding="utf-8"))
    spec = consumer.get("dependencies", {}).get("smol")
    package = components["smol"]["package"]
    if not isinstance(spec, dict) or spec.get("package") != package or spec.get("version") != version:
        raise ValueError(f"tests/consumer smol dependency must be {package} {version}")
    return version, components


def validate_release(cargo, upstream):
    version = cargo.get("workspace", {}).get("package", {}).get("version")
    components = upstream.get("components")
    if not isinstance(version, str) or not version:
        raise ValueError("workspace.package.version is missing")
    if not isinstance(components, dict) or not components:
        raise ValueError("UPSTREAM.toml has no components")
    for key, spec in components.items():
        if not isinstance(spec, dict):
            raise ValueError(f"component {key} is not a table")
        for field in ("package", "version", "upstream_name", "upstream_version"):
            if not isinstance(spec.get(field), str) or not spec[field]:
                raise ValueError(f"component {key} is missing {field}")
        if spec["upstream_name"] != key:
            raise ValueError(f"component {key} upstream_name is {spec['upstream_name']}")
        if spec["package"] != f"laputa-{key}":
            raise ValueError(f"component {key} package is {spec['package']}")
        if spec["version"] != version:
            raise ValueError(f"component {key} version {spec['version']} != workspace {version}")
    laputa = {spec["package"] for spec in components.values()}
    for name, spec in cargo.get("workspace", {}).get("dependencies", {}).items():
        if isinstance(spec, str):
            dep_version = spec
            package = name
        elif isinstance(spec, dict):
            dep_version = spec.get("version")
            package = spec.get("package", name)
        else:
            raise ValueError(f"workspace.dependencies {name} has an unsupported specification")
        if package in laputa and dep_version != version:
            raise ValueError(f"workspace.dependencies {name} version {dep_version} != workspace {version}")
    return version, components


def self_check():
    cargo = {
        "workspace": {
            "package": {"version": "0.1.0"},
            "dependencies": {
                "async-io": {"package": "laputa-async-io", "version": "0.1.0", "path": "crates/async-io"},
            },
        }
    }
    upstream = {
        "components": {
            "async-io": {
                "package": "laputa-async-io",
                "version": "0.1.0",
                "upstream_name": "async-io",
                "upstream_version": "2.6.0",
            }
        }
    }
    version, components = validate_release(cargo, upstream)
    if version != "0.1.0" or components["async-io"]["package"] != "laputa-async-io":
        raise ValueError("release identity fixture did not round-trip")
    drifted = {
        "components": {
            "async-io": {**upstream["components"]["async-io"], "version": "0.2.0"}
        }
    }
    try:
        validate_release(cargo, drifted)
    except ValueError as error:
        if "0.2.0" not in str(error):
            raise
    else:
        raise ValueError("a component version drift was accepted")
    dep_drift = {
        "workspace": {
            "package": {"version": "0.1.0"},
            "dependencies": {
                "async-io": {"package": "laputa-async-io", "version": "9.9.9", "path": "crates/async-io"},
            },
        }
    }
    try:
        validate_release(dep_drift, upstream)
    except ValueError as error:
        if "9.9.9" not in str(error):
            raise
    else:
        raise ValueError("a workspace dependency version drift was accepted")
