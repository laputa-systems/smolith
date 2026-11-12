"""Release version and owned-crate identity.

[workspace.package].version in the root Cargo.toml owns the runtime family.
[components] in UPSTREAM.toml owns upstream-derived crate identity. Runtime
components share that version and the laputa-{name} package. Protocol
components set family = "protocol" and keep an independent version and
package name. First-party HTTP packages are not components.

A workspace dependency on an owned component must require that component's
version. The external consumer's smol dependency must require the runtime
version.
"""

import tomllib
from pathlib import Path


def family_of(spec):
    family = spec.get("family", "runtime")
    if family not in ("runtime", "protocol"):
        raise ValueError(f"component family {family} is not runtime or protocol")
    return family


def requirement_matches(requirement, version):
    return requirement == version or requirement == f"={version}"


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
    packages = {}
    for key, spec in components.items():
        if not isinstance(spec, dict):
            raise ValueError(f"component {key} is not a table")
        family = family_of(spec)
        for field in ("package", "version", "upstream_name"):
            if not isinstance(spec.get(field), str) or not spec[field]:
                raise ValueError(f"component {key} is missing {field}")
        if family == "runtime":
            if not isinstance(spec.get("upstream_version"), str) or not spec["upstream_version"]:
                raise ValueError(f"component {key} is missing upstream_version")
            if spec["package"] != f"laputa-{key}":
                raise ValueError(f"component {key} package is {spec['package']}")
            if spec["version"] != version:
                raise ValueError(f"component {key} version {spec['version']} != workspace {version}")
        elif spec.get("upstream_version") is not None and (
            not isinstance(spec.get("upstream_version"), str) or not spec["upstream_version"]
        ):
            raise ValueError(f"component {key} upstream_version is empty")
        if spec["upstream_name"] != key:
            raise ValueError(f"component {key} upstream_name is {spec['upstream_name']}")
        if spec["package"] in packages:
            raise ValueError(f"component {key} repeats package {spec['package']}")
        packages[spec["package"]] = spec
    for name, spec in cargo.get("workspace", {}).get("dependencies", {}).items():
        if isinstance(spec, str):
            dep_version = spec
            package = name
        elif isinstance(spec, dict):
            dep_version = spec.get("version")
            package = spec.get("package", name)
        else:
            raise ValueError(f"workspace.dependencies {name} has an unsupported specification")
        owned = packages.get(package)
        if owned is None:
            continue
        if family_of(owned) == "protocol":
            if not requirement_matches(dep_version, owned["version"]):
                raise ValueError(
                    f"workspace.dependencies {name} version {dep_version} != {owned['version']}"
                )
        elif dep_version != version:
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
    protocol_cargo = {
        "workspace": {
            "package": {"version": "0.1.0"},
            "dependencies": {
                "h2": {
                    "package": "laputa-h2-futures-lite",
                    "version": "=0.4.20-beta.2",
                    "path": "crates/laputa-h2-futures-lite",
                },
                "bytes": "1.12.1",
            },
        }
    }
    protocol_upstream = {
        "components": {
            "h2": {
                "family": "protocol",
                "package": "laputa-h2-futures-lite",
                "version": "0.4.20-beta.2",
                "upstream_name": "h2",
                "upstream_version": "0.4.20",
            },
            "fastwebsockets": {
                "family": "protocol",
                "package": "laputa-fastwebsockets-futures-lite",
                "version": "0.10.2",
                "upstream_name": "fastwebsockets",
            },
        }
    }
    protocol_version, protocol_components = validate_release(protocol_cargo, protocol_upstream)
    if protocol_version != "0.1.0" or protocol_components["h2"]["version"] != "0.4.20-beta.2":
        raise ValueError("protocol identity fixture did not round-trip")
    missing_runtime_upstream = {
        "components": {
            "async-io": {
                "package": "laputa-async-io",
                "version": "0.1.0",
                "upstream_name": "async-io",
            }
        }
    }
    try:
        validate_release(cargo, missing_runtime_upstream)
    except ValueError as error:
        if "upstream_version" not in str(error):
            raise
    else:
        raise ValueError("a runtime component without upstream_version was accepted")
