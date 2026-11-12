#!/usr/bin/env sh
# Publish every publishable workspace package.
#
# Runtime crates share one minor bump. HTTP crates share a separate minor
# bump. Protocol forks stay on the version already declared in the tree.
# Public uploads go to crates.io. The private phase then runs
# `stuff publish cargo` (REGISTRY defaults to mycrates, STUFF selects the
# executable). Downloads still follow the committed source replacement;
# that mirror is not an upload destination.
#
# `make publish-dry-run` rewrites versions only in a temporary clone.
# A matching release commit resumes the same versions instead of bumping
# again. A newer registry version does not count as the intended version.

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

for command_name in awk cargo cp dirname git grep mkdir mktemp mv python3 rm rustup sed sleep sort; do
    command -v "$command_name" >/dev/null 2>&1 || fail "required command not found: $command_name"
done

REGISTRY=${REGISTRY:-mycrates}
STUFF=${STUFF:-stuff}

TOOLCHAIN=$(rustup show active-toolchain | awk 'NR==1 { print $1 }') || fail "could not read the pinned toolchain"
[ -n "$TOOLCHAIN" ] || fail "could not read the pinned toolchain"

if [ -z "${TMPDIR:-}" ]; then
    mkdir -p "$ROOT/.work/tmp" || fail "could not create scratch directory"
    TMPDIR=$ROOT/.work/tmp
    export TMPDIR
fi

temporary_manifest=
backup_dir=
dry_clone_dir=
packages_file=
release_started=0
release_committed=0
release_subject="chore(release): bump laputa workspace minor version"
runtime_latest=
http_latest=
runtime_next=
http_next=
http_current=
newline=$(printf '\nX')
newline=${newline%X}

on_exit() {
    exit_status=$1
    trap - 0 HUP INT TERM

    if [ "$release_started" -eq 1 ] && [ "$release_committed" -eq 0 ]; then
        head_subject=$(git log -1 --format=%s 2>/dev/null || true)
        if [ "$head_subject" != "$release_subject" ]; then
            if ! git checkout -- .; then
                printf '%s\n' "make publish: failed to restore the working tree; backups remain in $backup_dir" >&2
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
    if [ -n "$dry_clone_dir" ]; then
        rm -rf "$dry_clone_dir" || exit_status=1
    fi
    if [ -n "$packages_file" ]; then
        rm -f "$packages_file" || exit_status=1
    fi
    exit "$exit_status"
}

trap 'on_exit $?' 0
trap 'exit 1' HUP INT TERM

is_rate_limited() {
    # crates.io embeds the throttle in the cargo error text.
    printf '%s' "$1" | grep -qi -e 'status 429' -e 'too many requests' -e 'try again after'
}

rate_limit_wait_secs() {
    # "try again after <date>" is the deadline crates.io puts in a 429 body.
    printf '%s' "$1" | python3 -c '
import re
import sys
from datetime import datetime, timezone
from email.utils import parsedate_to_datetime
import math
text = sys.stdin.read()
low = text.lower()
fallback = 610 if "new crate" in low else 70
match = re.search(r"try again after\s+(.+?)(?:\s+and see|\s+or email|\.\s*|\.$|$)", text, re.IGNORECASE | re.DOTALL)
if not match:
    print(fallback)
    sys.exit(0)
date_str = match.group(1).strip().rstrip(".").strip().splitlines()[0].strip()
try:
    target = parsedate_to_datetime(date_str)
    if target.tzinfo is None:
        target = target.replace(tzinfo=timezone.utc)
    wait = (target - datetime.now(timezone.utc)).total_seconds() + 10
    if wait < 5:
        wait = 5
    print(math.ceil(wait))
except Exception:
    print(fallback)
'
}

is_transient_publish_failure() {
    # The sparse index can lag a dependency published seconds earlier.
    printf '%s' "$1" | grep -qi -e 'no matching package' -e 'failed to select a version' -e 'unknown crate' -e 'spurious network' -e 'connection reset' -e 'timed out' -e 'status 5[0-9][0-9]' -e 'failed to get'
}

# crates.io stays the public upload registry. The committed source
# replacement redirects an unnamed registry at dependency downloads and
# rejects search, owner, and publish until this name is passed.
PUBLIC_REGISTRY=crates-io

cargo_public() {
    cargo "$@" --registry "$PUBLIC_REGISTRY"
}

from_root() {
    (cd / && RUSTUP_TOOLCHAIN="$TOOLCHAIN" "$@")
}

registry_latest() {
    # Prints the latest registry version of $1, or nothing when the crate
    # has never been published. A 429 waits for the server retry time.
    attempt=1
    while true; do
        if search_result=$(from_root cargo_public search "$1" --limit 1 2>&1); then
            break
        fi
        if is_rate_limited "$search_result"; then
            wait_secs=$(rate_limit_wait_secs "$search_result") || wait_secs=70
            case "$wait_secs" in ''|*[!0-9]*) wait_secs=70 ;; esac
            printf '%s\n' "Rate limited querying registry for $1 (attempt $attempt); waiting ${wait_secs}s" >&2
            sleep "$wait_secs"
            attempt=$((attempt + 1))
            continue
        fi
        fail "could not query the default Cargo registry for $1: $search_result"
    done
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

exact_published() {
    from_root cargo_public info "$1@$2" >/dev/null 2>&1
}

wait_for_exact() {
    attempt=1
    while [ "$attempt" -le 20 ]; do
        if exact_published "$1" "$2"; then
            return 0
        fi
        printf '%s\n' "waiting for $1 $2 on crates.io (attempt $attempt)" >&2
        sleep 30
        attempt=$((attempt + 1))
    done
    fail "$1 $2 did not become visible on crates.io"
}

dry_run_package() {
    attempt=1
    while true; do
        if dry_output=$(from_root cargo_public publish -p "$1" --manifest-path "$ROOT/Cargo.toml" --dry-run 2>&1); then
            printf '%s\n' "$dry_output"
            return 0
        fi
        if is_rate_limited "$dry_output"; then
            wait_secs=$(rate_limit_wait_secs "$dry_output") || wait_secs=70
            case "$wait_secs" in ''|*[!0-9]*) wait_secs=70 ;; esac
            printf '%s\n' "Rate limited dry-running $1 (attempt $attempt); waiting ${wait_secs}s"
            sleep "$wait_secs"
            attempt=$((attempt + 1))
            continue
        fi
        if [ "$attempt" -lt 20 ] && is_transient_publish_failure "$dry_output"; then
            printf '%s\n' "Transient failure dry-running $1 (attempt $attempt); waiting 30s"
            printf '%s\n' "$dry_output" | sed -n '1,10p'
            sleep 30
            attempt=$((attempt + 1))
            continue
        fi
        printf '%s\n' "$dry_output" >&2
        fail "could not dry-run $1"
    done
}

publish_package() {
    if exact_published "$1" "$2"; then
        printf '%s\n' "$1 $2 is already published; skipping"
        return 0
    fi
    printf '%s\n' "Publishing $1 $2"
    attempt=1
    transient_attempt=1
    while true; do
        if publish_output=$(from_root cargo_public publish -p "$1" --manifest-path "$ROOT/Cargo.toml" 2>&1); then
            printf '%s\n' "$publish_output"
            printf '%s\n' "published $1 $2 crates-io"
            wait_for_exact "$1" "$2"
            return 0
        fi
        case "$publish_output" in
            *'already exists'*|*'already uploaded'*|*'already on crates.io'*)
                if exact_published "$1" "$2"; then
                    printf '%s\n' "$1 $2 is already published; skipping"
                    return 0
                fi
                ;;
        esac
        if is_rate_limited "$publish_output"; then
            wait_secs=$(rate_limit_wait_secs "$publish_output") || wait_secs=70
            case "$wait_secs" in ''|*[!0-9]*) wait_secs=70 ;; esac
            printf '%s\n' "Rate limited publishing $1 $2 (attempt $attempt); waiting ${wait_secs}s"
            printf '%s\n' "$publish_output" | grep -i -e '429' -e 'try again after' | sed -n '1,5p'
            sleep "$wait_secs"
            attempt=$((attempt + 1))
            continue
        fi
        if [ "$transient_attempt" -lt 20 ] && is_transient_publish_failure "$publish_output"; then
            printf '%s\n' "Transient failure publishing $1 $2 (attempt $transient_attempt); waiting 30s"
            printf '%s\n' "$publish_output" | sed -n '1,10p'
            sleep 30
            transient_attempt=$((transient_attempt + 1))
            continue
        fi
        printf '%s\n' "$publish_output" >&2
        fail "could not publish $1 $2"
    done
}

publish_one_private() {
    attempt=1
    while true; do
        if private_output=$(from_root "$STUFF" publish cargo -p "$1" --manifest-path "$ROOT/Cargo.toml" --registry "$REGISTRY" 2>&1); then
            printf '%s\n' "$private_output"
            printf '%s\n' "published $1 $2 $REGISTRY"
            return 0
        else
            status=$?
        fi
        if [ "$status" -eq 3 ] || [ "$status" -eq 5 ]; then
            if [ "$attempt" -ge 5 ]; then
                printf '%s\n' "$private_output" >&2
                fail "could not publish $1 $2 to $REGISTRY"
            fi
            printf '%s\n' "retrying private publish of $1 $2 (attempt $attempt, status $status)" >&2
            sleep 30
            attempt=$((attempt + 1))
            continue
        fi
        printf '%s\n' "$private_output" >&2
        fail "could not publish $1 $2 to $REGISTRY"
    done
}

ensure_tag() {
    tag=$1-v$2
    if git rev-parse -q --verify "refs/tags/$tag" >/dev/null 2>&1; then
        target=$(git rev-list -n 1 "$tag") || fail "could not read tag $tag"
        head_now=$(git rev-parse HEAD) || fail "could not read HEAD"
        [ "$target" = "$head_now" ] || fail "tag $tag already exists away from the release commit"
        return 0
    fi
    git tag -a "$tag" -m "$1 $2" || fail "could not create tag $tag"
}

ensure_clean_tree() {
    working_changes=$(git status --porcelain=v1) || fail "could not inspect the working tree"
    [ -z "$working_changes" ] || fail "release requires a clean working tree"
}

ensure_registry_auth() {
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

set_planned() {
    family=$1
    version=$2
    case "$family" in
        runtime)
            if [ -n "$runtime_next" ]; then
                planned=$runtime_next
            else
                planned=$version
            fi
            ;;
        http)
            if [ -n "$http_next" ]; then
                planned=$http_next
            else
                planned=$version
            fi
            ;;
        protocol)
            planned=$version
            ;;
        *)
            fail "unknown release family $family"
            ;;
    esac
}

note_latest() {
    family=$1
    latest=$2
    [ -n "$latest" ] || return 0
    case "$family" in
        runtime) current_max=$runtime_latest ;;
        http) current_max=$http_latest ;;
        *) return 0 ;;
    esac
    if [ -z "$current_max" ]; then
        current_max=$latest
    elif python3 tools/inventory.py at-least "$latest" "$current_max"; then
        if ! python3 tools/inventory.py at-least "$current_max" "$latest"; then
            current_max=$latest
        fi
    fi
    case "$family" in
        runtime) runtime_latest=$current_max ;;
        http) http_latest=$current_max ;;
    esac
}

load_packages() {
    mkdir -p "$ROOT/.work/tmp" || fail "could not create scratch directory"
    packages_file=$ROOT/.work/tmp/publish-packages.$$
    python3 tools/inventory.py packages > "$packages_file" || fail "could not read the package inventory"
    [ -s "$packages_file" ] || fail "workspace has no publishable members"
    http_current=
    while IFS=' ' read -r name family version manifest; do
        [ -n "$name" ] || continue
        if [ "$family" = "http" ]; then
            if [ -z "$http_current" ]; then
                http_current=$version
            elif [ "$http_current" != "$version" ]; then
                fail "HTTP packages do not share one version"
            fi
        fi
    done < "$packages_file"
}

ensure_release_tags() {
    tag_body=$ROOT/.work/tmp/publish-tags.$$
    git log -1 --format=%b > "$tag_body" || fail "could not read the release commit body"
    while IFS= read -r line; do
        case "$line" in
            *" -> "*)
                name=${line%% *}
                rest=${line#* }
                old=${rest%% -> *}
                new=${rest##* -> }
                if [ "$old" != "$new" ]; then
                    ensure_tag "$name" "$new"
                fi
                ;;
        esac
    done < "$tag_body"
    rm -f "$tag_body"
}

publish_selected() {
    printf '%s\n' "Publishing is not atomic. A failed upload leaves earlier packages on the registry; re-run the same release commit to continue."
    while IFS=' ' read -r name family version manifest; do
        [ -n "$name" ] || continue
        set_planned "$family" "$version"
        if exact_published "$name" "$planned"; then
            printf '%s\n' "$name $planned is already published; skipping"
            continue
        fi
        dry_run_package "$name"
        publish_package "$name" "$planned"
    done < "$packages_file"
}

publish_private() {
    command -v "$STUFF" >/dev/null 2>&1 || fail "required command not found: $STUFF"
    while IFS=' ' read -r name family version manifest; do
        [ -n "$name" ] || continue
        set_planned "$family" "$version"
        if ! exact_published "$name" "$planned"; then
            fail "$name $planned is not on crates.io; refusing private publication"
        fi
        publish_one_private "$name" "$planned"
    done < "$packages_file"
}

backup_release_inputs() {
    backup_dir_candidate=$TMPDIR/bump_minor_release.$$
    (umask 077 && mkdir "$backup_dir_candidate") || fail "could not create private release backups"
    backup_dir=$backup_dir_candidate
    cp -p "$ROOT/Cargo.toml" "$backup_dir/Cargo.toml" || fail "could not back up Cargo.toml"
    cp -p "$ROOT/Cargo.lock" "$backup_dir/Cargo.lock" || fail "could not back up Cargo.lock"
    cp -p "$ROOT/UPSTREAM.toml" "$backup_dir/UPSTREAM.toml" || fail "could not back up UPSTREAM.toml"
    cp -p "$ROOT/tests/qualify/Cargo.toml" "$backup_dir/qualify.toml" || fail "could not back up tests/qualify/Cargo.toml"
    cp -p "$ROOT/tests/consumer/Cargo.toml" "$backup_dir/consumer.toml" || fail "could not back up tests/consumer/Cargo.toml"
    cp -p "$ROOT/tests/consumer/Cargo.lock" "$backup_dir/consumer.lock" || fail "could not back up tests/consumer/Cargo.lock"
}

# Fail fast without a registry login before any registry query or file edit.
ensure_registry_auth
load_packages

if [ "${PUBLISH_PRIVATE_ONLY:-}" = 1 ]; then
    ensure_clean_tree
    publish_private
    exit 0
fi

git_subject=$(git log -1 --format=%s) || fail "could not read the current commit subject"
if [ "$git_subject" = "$release_subject" ]; then
    ensure_clean_tree
    ensure_release_tags
    publish_selected
    publish_private
    exit 0
fi

while IFS=' ' read -r name family version manifest; do
    [ -n "$name" ] || continue
    latest=$(registry_latest "$name") || exit 1
    if [ -n "$latest" ]; then
        owner_output=$(from_root cargo_public owner --list "$name" 2>&1) || fail "could not verify ownership of $name: $owner_output"
    fi
    note_latest "$family" "$latest"
done < "$packages_file"

runtime_current=$(python3 tools/inventory.py version) || fail "could not read release version"
runtime_next=$(python3 tools/inventory.py next "$runtime_current" "$runtime_latest") || fail "could not plan the runtime version"
if [ -n "$http_current" ]; then
    http_next=$(python3 tools/inventory.py next "$http_current" "$http_latest") || fail "could not plan the HTTP version"
    http_arg=$http_next
else
    http_arg=-
fi
[ -n "$runtime_next" ] || fail "could not plan the runtime version"
[ "$runtime_next" != "$runtime_current" ] || fail "could not plan the runtime version"

if [ -n "$runtime_latest" ] && ! python3 tools/inventory.py at-least "$runtime_latest" "$runtime_current"; then
    fail "local runtime $runtime_current is newer than published $runtime_latest, but HEAD is not the matching release commit"
fi
if [ -n "$http_current" ] && [ -n "$http_latest" ] && ! python3 tools/inventory.py at-least "$http_latest" "$http_current"; then
    fail "local http $http_current is newer than published $http_latest, but HEAD is not the matching release commit"
fi

existing_tags=$(git tag) || fail "could not list Git tags"
while IFS=' ' read -r name family version manifest; do
    [ -n "$name" ] || continue
    set_planned "$family" "$version"
    [ "$planned" = "$version" ] && continue
    next_tag=$name-v$planned
    case "$newline$existing_tags$newline" in
        *"$newline$next_tag$newline"*) fail "tag $next_tag already exists away from the release commit" ;;
    esac
done < "$packages_file"

printf '%s\n' "runtime $runtime_current -> $runtime_next"
if [ -n "$http_current" ]; then
    printf '%s\n' "http $http_current -> $http_next"
fi
printf '%s\n' "protocol versions stay at their declared releases"

ensure_clean_tree

if [ "${PUBLISH_DRY_RUN:-}" = 1 ]; then
    mkdir -p "$ROOT/.work/tmp" || fail "could not create scratch directory"
    dry_clone_dir=$(mktemp -d "$ROOT/.work/tmp/publish-dry.XXXXXX") || fail "could not create a dry-run directory"
    git clone --local --quiet "$ROOT" "$dry_clone_dir/checkout" || fail "could not clone for the dry run"
    printf '%s\n' "publish dry-run: runtime $runtime_next http $http_arg"
    (cd "$dry_clone_dir/checkout" && python3 tools/release_apply.py apply "$runtime_next" "$http_arg") || fail "dry-run apply failed"
    (cd "$dry_clone_dir/checkout" && python3 tools/stage.py) || fail "dry-run staging failed"
    printf '%s\n' "publish dry-run complete"
    exit 0
fi

body_unsorted=$ROOT/.work/tmp/publish-body.$$
: > "$body_unsorted" || fail "could not build the release commit body"
while IFS=' ' read -r name family version manifest; do
    [ -n "$name" ] || continue
    if [ "$family" = "runtime" ]; then
        body_planned=$runtime_next
    elif [ "$family" = "http" ]; then
        body_planned=$http_next
    elif [ "$family" = "protocol" ]; then
        body_planned=$version
    else
        fail "unknown release family $family"
    fi
    printf '%s %s -> %s\n' "$name" "$version" "$body_planned" >> "$body_unsorted"
done < "$packages_file"
release_body=$(sort "$body_unsorted") || fail "could not build the release commit body"
rm -f "$body_unsorted"

backup_release_inputs
release_started=1
python3 tools/release_apply.py apply "$runtime_next" "$http_arg" || fail "could not apply release versions"

from_root cargo check --manifest-path "$ROOT/Cargo.toml" --workspace --quiet
from_root cargo update --manifest-path "$ROOT/tests/consumer/Cargo.toml"
"$ROOT/tests/consumer/check.sh"
python3 tools/stage.py

git add -u -- Cargo.toml Cargo.lock UPSTREAM.toml tests benchmarks crates
git commit -m "$release_subject" -m "$release_body"
release_committed=1
ensure_release_tags
publish_selected
publish_private
