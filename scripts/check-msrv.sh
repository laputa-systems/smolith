#!/usr/bin/env sh
# Check each workspace member with the Rust version declared by its manifest.
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"
installed_toolchains=''

json() {
    if command -v jaq >/dev/null 2>&1; then
        jaq "$@"
    elif command -v jq >/dev/null 2>&1; then
        jq "$@"
    else
        printf '%s\n' 'reading cargo metadata requires jaq or jq' >&2
        exit 1
    fi
}

ensure_toolchain() {
    toolchain=$1
    case " $installed_toolchains " in
        *" $toolchain "*) ;;
        *)
            rustup toolchain install "$toolchain" --profile minimal
            installed_toolchains="$installed_toolchains $toolchain"
            ;;
    esac
}

check() {
    version=$1
    manifest=$2
    if [ -z "$version" ] || [ "$version" = "null" ]; then
        printf '%s does not declare rust-version\n' "$manifest" >&2
        exit 1
    fi
    ensure_toolchain "$version.0"
    (cd / && cargo "+$version.0" check --locked --manifest-path "$manifest" --all-features)
}

metadata=$(cd / && cargo metadata --no-deps --format-version 1 --manifest-path "$root/Cargo.toml")
members=$(printf '%s\n' "$metadata" | json -r '.packages[] | "\(.rust_version)\t\(.manifest_path)"')
if [ -z "$members" ]; then
    printf '%s\n' 'cargo metadata returned no workspace members' >&2
    exit 1
fi

# Each member is checked with its own declared minimum and every feature.
while IFS="$(printf '\t')" read -r version manifest; do
    check "$version" "$manifest"
done <<EOF
$members
EOF

ensure_toolchain 1.85.0
(cd / && cargo +1.85.0 check --locked --manifest-path "$root/tests/consumer/Cargo.toml" --tests)

printf '%s\n' 'minimum supported Rust version checks passed'
