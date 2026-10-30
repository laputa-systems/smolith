#!/usr/bin/env sh
# Bump the laputa workspace to its next minor version and publish the
# workspace crates in dependency order.
#
# The workspace keeps one version in the root Cargo.toml
# ([workspace.package].version). Every laputa-* crate inherits it, every
# [workspace.dependencies] entry for a laputa-* crate carries it, every
# [components.*] entry in UPSTREAM.toml carries it, and the smol path
# requirements in tests/qualify and tests/consumer track it.
# The release moves to minor(max(local, registry)) + 1 with the patch reset.
# A registry that ran ahead is honored as the base; a local version ahead
# of the registry without a matching release commit fails instead of
# guessing. Inter-crate requirements track the new version; anything
# unexpected fails loudly before anything is edited.
# One commit carries the release and each crate gets a <name>-v<version> tag.
# Re-running after a partial publish finishes the remaining crates instead
# of failing. A missing registry login fails before any registry query or
# file edit.

set -eu
LC_ALL=C
export LC_ALL

fail() {
    printf '%s\n' "make publish: $*" >&2
    exit 1
}

SCRIPT_DIR=$(CDPATH= cd -P "$(dirname "$0")" && pwd -P) || fail "cannot locate script directory"
ROOT=$(CDPATH= cd -P "$SCRIPT_DIR/.." && pwd -P) || fail "cannot locate repository root"
cd "$ROOT"

for command_name in awk cargo cp dirname git grep mkdir mv python3 rm sed sort; do
    command -v "$command_name" >/dev/null 2>&1 || fail "required command not found: $command_name"
done

temporary_manifest=
backup_dir=
release_started=0
release_committed=0
release_subject="chore(release): bump laputa workspace minor version"
newline=$(printf '\nX')
newline=${newline%X}

RESET_PATHS="Cargo.toml Cargo.lock UPSTREAM.toml tests/qualify/Cargo.toml tests/consumer/Cargo.toml"

on_exit() {
    exit_status=$1
    trap - 0 HUP INT TERM

    if [ "$release_started" -eq 1 ] && [ "$release_committed" -eq 0 ]; then
        if head_subject=$(git log -1 --format=%s 2>/dev/null); then
            :
        else
            head_subject=
        fi
        if [ "$head_subject" != "$release_subject" ]; then
            # RESET_PATHS contains no whitespace.
            # shellcheck disable=SC2086
            if ! git reset -- $RESET_PATHS; then
                printf '%s\n' "make publish: failed to unstage release inputs; backups remain in $backup_dir" >&2
                exit 1
            fi
            restore_failed=0
            if ! cp -p "$backup_dir/Cargo.toml" "$ROOT/Cargo.toml"; then
                restore_failed=1
            fi
            if ! cp -p "$backup_dir/Cargo.lock" "$ROOT/Cargo.lock"; then
                restore_failed=1
            fi
            if ! cp -p "$backup_dir/UPSTREAM.toml" "$ROOT/UPSTREAM.toml"; then
                restore_failed=1
            fi
            if ! cp -p "$backup_dir/qualify.toml" "$ROOT/tests/qualify/Cargo.toml"; then
                restore_failed=1
            fi
            if ! cp -p "$backup_dir/consumer.toml" "$ROOT/tests/consumer/Cargo.toml"; then
                restore_failed=1
            fi
            if [ "$restore_failed" -ne 0 ]; then
                printf '%s\n' "make publish: failed to restore release inputs; backups remain in $backup_dir" >&2
                exit 1
            fi
        fi
    fi

    if [ -n "$temporary_manifest" ]; then
        rm -f "$temporary_manifest" || exit_status=1
    fi
    if [ -n "$backup_dir" ]; then
        rm -rf "$backup_dir" || exit_status=1
    fi
    exit "$exit_status"
}

trap 'on_exit $?' 0
trap 'exit 1' HUP INT TERM

registry_latest() {
    # Prints the latest registry version of $1, or nothing when the crate
    # has never been published. A failed query still fails loudly.
    # Standard error is captured so cargo's informational notes stay out of
    # release output; it is reported when the query itself fails.
    search_result=$(cargo search "$1" --limit 1 2>&1) || fail "could not query the default Cargo registry for $1: $search_result"
    printf '%s\n' "$search_result" | awk -v package_name="$1" '
        $1 == package_name && $2 == "=" && $3 ~ /^".+"$/ {
            version = substr($3, 2, length($3) - 2)
            matches++
        }
        END {
            if (matches > 1) exit 1
            if (matches == 1) print version
        }
    ' || fail "could not parse the registry response for $1"
}

version_at_least() {
    awk -v left="$1" -v right="$2" '
        function components(value, result) {
            sub(/[+].*$/, "", value)
            sub(/-.*/, "", value)
            return split(value, result, "[.]")
        }
        function compare_component(left_part, right_part) {
            if (length(left_part) < length(right_part)) return -1
            if (length(left_part) > length(right_part)) return 1
            if (("x" left_part) < ("x" right_part)) return -1
            if (("x" left_part) > ("x" right_part)) return 1
            return 0
        }
        BEGIN {
            components(left, left_parts)
            components(right, right_parts)
            for (part = 1; part <= 3; part++) {
                comparison = compare_component(left_parts[part], right_parts[part])
                if (comparison != 0) exit (comparison > 0 ? 0 : 1)
            }
            exit 0
        }
    '
}

plan_next() {
    awk -v current="$1" -v latest="$2" '
        function valid_version(value) {
            return value ~ /^(0|[1-9][0-9]*)[.](0|[1-9][0-9]*)[.](0|[1-9][0-9]*)(-[0-9A-Za-z.-]+)?([+][0-9A-Za-z.-]+)?$/
        }
        function components(value, result) {
            sub(/[+].*$/, "", value)
            sub(/-.*/, "", value)
            return split(value, result, "[.]")
        }
        function compare_component(left, right) {
            if (length(left) < length(right)) return -1
            if (length(left) > length(right)) return 1
            if (("x" left) < ("x" right)) return -1
            if (("x" left) > ("x" right)) return 1
            return 0
        }
        function compare_versions(left, right, left_parts, right_parts, part, comparison) {
            components(left, left_parts)
            components(right, right_parts)
            for (part = 1; part <= 3; part++) {
                comparison = compare_component(left_parts[part], right_parts[part])
                if (comparison != 0) return comparison
            }
            return 0
        }
        function increment_decimal(value, result, carry, position, digit, next_digit) {
            result = ""
            carry = 1
            for (position = length(value); position > 0; position--) {
                digit = substr(value, position, 1)
                next_digit = index("0123456789", digit) - 1 + carry
                if (next_digit == 10) {
                    next_digit = 0
                    carry = 1
                } else {
                    carry = 0
                }
                result = substr("0123456789", next_digit + 1, 1) result
            }
            if (carry) result = "1" result
            return result
        }
        BEGIN {
            if (!valid_version(current)) exit 1
            base = current
            if (latest != "") {
                if (!valid_version(latest)) exit 1
                if (compare_versions(current, latest) < 0) base = latest
            }
            components(base, base_parts)
            print base_parts[1] "." increment_decimal(base_parts[2]) ".0"
        }
    '
}

publish_package() {
    # Publishes $1 at $2 unless the registry already has it, so a re-run
    # after a partial publish finishes the remaining crates.
    already_published=$(registry_latest "$1") || exit 1
    if [ -n "$already_published" ] && version_at_least "$already_published" "$2"; then
        printf '%s\n' "$1 $2 is already published; skipping"
        return 0
    fi
    printf '%s\n' "Publishing $1 $2"
    cargo publish -p "$1"
}

ensure_tag() {
    tag=$1-v$2
    case "$newline$tags_at_head$newline" in
        *"$newline$tag$newline"*) ;;
        *) git tag -a "$tag" -m "$1 $2" ;;
    esac
}

ensure_clean_tree() {
    working_changes=$(git status --porcelain=v1) || fail "could not inspect the working tree"
    [ -z "$working_changes" ] || fail "release requires a clean working tree"
}

ensure_registry_auth() {
    # Preflight before any registry query or file edit: a missing login
    # otherwise surfaces only at the first authenticated step, after several
    # registry queries. Ownership of each published crate is still validated
    # by the cargo owner loop below before anything is edited.
    if [ -n "${CARGO_REGISTRY_TOKEN:-}" ] || [ -n "${CARGO_REGISTRIES_CRATES_IO_TOKEN:-}" ]; then
        return 0
    fi
    if [ -n "${CARGO_REGISTRY_GLOBAL_CREDENTIAL_PROVIDERS:-}" ]; then
        return 0
    fi
    cargo_home=${CARGO_HOME:-$HOME/.cargo}
    for config in "$cargo_home/config.toml" "$cargo_home/config"; do
        if [ -f "$config" ] && grep -q 'credential.provider' "$config" 2>/dev/null; then
            return 0
        fi
    done
    for credentials in "$cargo_home/credentials.toml" "$cargo_home/credentials"; do
        if [ -f "$credentials" ] && grep -q '^[[:space:]]*token[[:space:]]*=' "$credentials" 2>/dev/null; then
            return 0
        fi
    done
    fail "no Cargo registry token found; run \`cargo login\` (or set CARGO_REGISTRY_TOKEN) before publishing"
}

# Fail fast without a registry login before any registry query or file edit.
ensure_registry_auth

# Derive the member manifests from the workspace so new crates are picked
# up without editing this script. Members with publish = false stay in the
# workspace but leave the release: they are never queried, tagged, or
# published.
LOCAL_VERSION=$(python3 tools/release_graph.py version) || fail "could not read release version"
MANIFEST_MAP=$(python3 tools/release_graph.py manifests) || fail "could not read release manifests"
PKG_SPACE=$(printf '%s\n' "$MANIFEST_MAP" | awk -F: '{ print $1 }' | tr '\n' ' ')
[ -n "$PKG_SPACE" ] || fail "workspace has no publishable members"
EDGES=$(python3 tools/release_graph.py edges) || fail "could not read release dependencies"

# Order crates dependencies-first. Emitted in sorted order among ready
# crates so the sequence is stable.
TOPO_ORDER=$(printf '%s\n' "$EDGES" | awk -v nodes="$PKG_SPACE" '
    BEGIN {
        count = split(nodes, list, " ")
        for (i = 1; i <= count; i++) {
            node = list[i]
            ranked[i] = node
            depcount[node] = 0
            dependents[node] = ""
        }
        for (i = 1; i <= count; i++)
            for (j = i + 1; j <= count; j++)
                if (ranked[j] < ranked[i]) {
                    tmp = ranked[i]
                    ranked[i] = ranked[j]
                    ranked[j] = tmp
                }
    }
    NF == 2 {
        dependent = $1
        dep = $2
        if (!(dependent in depcount) || !(dep in depcount)) {
            printf "%s\n", "unknown package in edge: " $1 " " $2 > "/dev/stderr"
            bad = 1
            next
        }
        dependents[dep] = dependents[dep] " " dependent
        depcount[dependent]++
    }
    END {
        if (bad) exit 1
        emitted = 0
        while (emitted < count) {
            progress = 0
            for (i = 1; i <= count; i++) {
                node = ranked[i]
                if (!done[node] && depcount[node] == 0) {
                    print node
                    done[node] = 1
                    emitted++
                    progress = 1
                    users = dependents[node]
                    user_count = split(users, user_list, " ")
                    for (k = 1; k <= user_count; k++)
                        if (user_list[k] != "") depcount[user_list[k]]--
                }
            }
            if (!progress) {
                print "dependency cycle detected" > "/dev/stderr"
                exit 1
            }
        }
    }
') || fail "could not order workspace crates"

# Crates without workspace dependencies can be dry-run before the release
# commit; anything else must wait until its dependencies are published.
LEAVES=$(printf '%s\n' "$EDGES" | awk -v nodes="$PKG_SPACE" '
    BEGIN {
        count = split(nodes, list, " ")
        for (i = 1; i <= count; i++) hasdep[list[i]] = 0
    }
    NF == 2 { hasdep[$1] = 1 }
    END {
        for (i = 1; i <= count; i++)
            if (!hasdep[list[i]]) print list[i]
    }
')
[ -n "$LEAVES" ] || LEAVES=$(printf '%s\n' $PKG_SPACE | sort | sed -n '1p')

# Query the registry and check ownership crate by crate before the script
# edits any release file. Ownership of a never-published name cannot be
# queried; the publish step surfaces that instead. Checking ownership right
# after each version query fails fast on a dead token instead of running
# every query first. Cargo output is captured so successful checks stay
# silent; failures report it.
MAX_PUBLISHED=
for name in $PKG_SPACE; do
    latest=$(registry_latest "$name")
    if [ -n "$latest" ]; then
        owner_output=$(cargo owner --list "$name" 2>&1) || fail "could not verify ownership of $name: $owner_output"
        if [ -z "$MAX_PUBLISHED" ]; then
            MAX_PUBLISHED=$latest
        elif version_at_least "$latest" "$MAX_PUBLISHED"; then
            if ! version_at_least "$MAX_PUBLISHED" "$latest"; then
                MAX_PUBLISHED=$latest
            fi
        fi
    fi
done

git_subject=$(git log -1 --format=%s) || fail "could not read the current commit subject"
tags_at_head=$(git tag --points-at HEAD) || fail "could not read tags at HEAD"

if [ "$git_subject" = "$release_subject" ]; then
    ensure_clean_tree
    for name in $TOPO_ORDER; do
        ensure_tag "$name" "$LOCAL_VERSION"
    done
    pending=0
    for name in $TOPO_ORDER; do
        already=$(registry_latest "$name") || exit 1
        if [ -n "$already" ] && version_at_least "$already" "$LOCAL_VERSION"; then
            printf '%s\n' "$name $LOCAL_VERSION is already published; skipping"
        else
            pending=1
        fi
    done
    if [ "$pending" -eq 0 ]; then
        printf '%s\n' "all workspace crates already published; nothing to do"
        exit 0
    fi
    printf '%s\n' "Retrying publish of unpublished workspace crates"
    for name in $TOPO_ORDER; do
        publish_package "$name" "$LOCAL_VERSION"
    done
    exit 0
fi

if [ -n "$MAX_PUBLISHED" ]; then
    version_at_least "$MAX_PUBLISHED" "$LOCAL_VERSION" \
        || fail "local workspace $LOCAL_VERSION is newer than published $MAX_PUBLISHED, but HEAD is not the matching release commit"
fi
NEXT_VERSION=$(plan_next "$LOCAL_VERSION" "$MAX_PUBLISHED") || fail "could not plan next version"
[ -n "$NEXT_VERSION" ] || fail "could not plan next version"
[ "$NEXT_VERSION" != "$LOCAL_VERSION" ] || fail "could not plan next version"

existing_tags=$(git tag) || fail "could not list Git tags"
for name in $TOPO_ORDER; do
    next_tag=$name-v$NEXT_VERSION
    case "$newline$existing_tags$newline" in
        *"$newline$next_tag$newline"*) fail "tag $next_tag already exists away from the release commit" ;;
    esac
done
ensure_clean_tree

release_body=$(for name in $PKG_SPACE; do
    printf '%s %s -> %s\n' "$name" "$LOCAL_VERSION" "$NEXT_VERSION"
done | sort)

backup_dir_candidate=${TMPDIR:-/tmp}/bump_minor_release.$$
(umask 077 && mkdir "$backup_dir_candidate") || fail "could not create private release backups"
backup_dir=$backup_dir_candidate
cp -p "$ROOT/Cargo.toml" "$backup_dir/Cargo.toml" || fail "could not back up Cargo.toml"
cp -p "$ROOT/Cargo.lock" "$backup_dir/Cargo.lock" || fail "could not back up Cargo.lock"
cp -p "$ROOT/UPSTREAM.toml" "$backup_dir/UPSTREAM.toml" || fail "could not back up UPSTREAM.toml"
cp -p "$ROOT/tests/qualify/Cargo.toml" "$backup_dir/qualify.toml" || fail "could not back up tests/qualify/Cargo.toml"
cp -p "$ROOT/tests/consumer/Cargo.toml" "$backup_dir/consumer.toml" || fail "could not back up tests/consumer/Cargo.toml"
release_started=1

bump_workspace_version() {
    temporary_manifest_candidate=$ROOT/.Cargo.toml.bump.$$
    (umask 077 && set -C && : > "$temporary_manifest_candidate") || fail "could not create temporary manifest"
    temporary_manifest=$temporary_manifest_candidate
    cp -p "$1" "$temporary_manifest" || fail "could not preserve Cargo.toml permissions"
    awk -v old="$2" -v new="$3" '
        /^[ \t]*\[workspace\.package\][ \t]*$/ { section = "package"; print; next }
        /^[ \t]*\[workspace\.dependencies\][ \t]*$/ { section = "deps"; print; next }
        /^[ \t]*\[/ { section = ""; print; next }
        section == "package" {
            line = $0
            stripped = line
            sub(/^[ \t]*/, "", stripped)
            sub(/[ \t]*$/, "", stripped)
            if (stripped ~ /^version[ \t]*=/) {
                matches++
                if (stripped != "version = \"" old "\"") exit 1
                print "version = \"" new "\""
                next
            }
        }
        section == "deps" {
            if ($0 ~ /version[ \t]*=[ \t]*"/) {
                want = "version = \"" old "\""
                give = "version = \"" new "\""
                if (index($0, want) == 0) exit 1
                out = ""
                rest = $0
                found = 0
                while ((pos = index(rest, want)) > 0) {
                    found++
                    out = out substr(rest, 1, pos - 1) give
                    rest = substr(rest, pos + length(want))
                }
                if (found != 1) exit 1
                matches++
                print out rest
                next
            }
        }
        { print }
        END { if (matches < 2) exit 1 }
    ' "$1" > "$temporary_manifest" || fail "$1 version changed or could not be updated"
    mv "$temporary_manifest" "$1" || fail "could not install updated $1"
    temporary_manifest=
}

bump_upstream_versions() {
    temporary_manifest_candidate=$ROOT/.UPSTREAM.toml.bump.$$
    (umask 077 && set -C && : > "$temporary_manifest_candidate") || fail "could not create temporary manifest"
    temporary_manifest=$temporary_manifest_candidate
    cp -p "$1" "$temporary_manifest" || fail "could not preserve UPSTREAM.toml permissions"
    awk -v old="$2" -v new="$3" '
        {
            line = $0
            stripped = line
            sub(/^[ \t]*/, "", stripped)
            if (stripped == "version = \"" old "\"") {
                matches++
                print "version = \"" new "\""
                next
            }
            print
        }
        END { if (matches < 1) exit 1 }
    ' "$1" > "$temporary_manifest" || fail "$1 version changed or could not be updated"
    count=$(grep -c "^version = \"$3\"" "$temporary_manifest") || fail "could not verify $1"
    expected=$(printf '%s\n' "$PKG_SPACE" | wc -w | tr -d ' ')
    [ "$count" = "$expected" ] || fail "$1 has $count version $3 entries, expected $expected"
    mv "$temporary_manifest" "$1" || fail "could not install updated $1"
    temporary_manifest=
}

# Track the new version in a smol path requirement. The requirement
# operator (bare, =, ^, ~) is preserved; anything else fails.
rewrite_requirement() {
    awk -v dep="$2" -v old="$3" -v new="$4" '
        BEGIN {
            prefixes[1] = ""
            prefixes[2] = "="
            prefixes[3] = "^"
            prefixes[4] = "~"
        }
        {
            line = $0
            stripped = line
            sub(/^[ \t]*/, "", stripped)
            if (stripped ~ ("^" dep "[ \t]*=")) {
                matched++
                total = 0
                for (k = 1; k <= 4; k++) {
                    want = "version = \"" prefixes[k] old "\""
                    give = "version = \"" prefixes[k] new "\""
                    out = ""
                    rest = line
                    found = 0
                    while ((pos = index(rest, want)) > 0) {
                        found++
                        out = out substr(rest, 1, pos - 1) give
                        rest = substr(rest, pos + length(want))
                    }
                    if (found > 0) {
                        total += found
                        line = out rest
                    }
                }
                if (total != 1) exit 1
                print line
                next
            }
            print
        }
        END { if (matched < 1) exit 1 }
    ' "$1" > "$temporary_manifest" || fail "dependency $2 in $1 changed or could not be updated"
    mv "$temporary_manifest" "$1" || fail "could not install updated $1"
    temporary_manifest=
}

bump_workspace_version "$ROOT/Cargo.toml" "$LOCAL_VERSION" "$NEXT_VERSION"
bump_upstream_versions "$ROOT/UPSTREAM.toml" "$LOCAL_VERSION" "$NEXT_VERSION"

for file in tests/qualify/Cargo.toml tests/consumer/Cargo.toml; do
    temporary_manifest_candidate=$ROOT/.Cargo.toml.bump.$$
    (umask 077 && set -C && : > "$temporary_manifest_candidate") || fail "could not create temporary manifest"
    temporary_manifest=$temporary_manifest_candidate
    cp -p "$ROOT/$file" "$temporary_manifest" || fail "could not preserve Cargo.toml permissions"
    rewrite_requirement "$ROOT/$file" "smol" "$LOCAL_VERSION" "$NEXT_VERSION"
done

cargo check --workspace --quiet
for name in $LEAVES; do
    cargo publish -p "$name" --dry-run --allow-dirty
done
# shellcheck disable=SC2086
git add -- $RESET_PATHS
git commit -m "$release_subject" -m "$release_body"
release_committed=1
for name in $TOPO_ORDER; do
    ensure_tag "$name" "$NEXT_VERSION"
done

# Dependencies must reach the registry before their dependents can even be
# dry-run, so each crate is validated and published in dependency order
# after the release commit.
for name in $TOPO_ORDER; do
    cargo publish -p "$name" --dry-run --allow-dirty
    publish_package "$name" "$NEXT_VERSION"
done

