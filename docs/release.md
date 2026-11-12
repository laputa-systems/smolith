# Release

[Cargo.toml](../Cargo.toml) owns the shared release version. [UPSTREAM.toml](../UPSTREAM.toml) maps package identities and upstream versions. A source tag, a packaged crate, and a registry upload are separate artifacts.

## Publication

`make publish` has two phases. First, `scripts/bump_minor_release.sh` bumps the version, commits, tags, and publishes to the public Cargo registry. Then `make publish-mycrates` invokes `stuff publish cargo` for the private registry. `REGISTRY` defaults to `mycrates` and affects only that second phase; `STUFF` selects its executable. Run these targets only for an intended release.

The public phase names `crates-io` on search, ownership, dry runs, and uploads. Dependency downloads still use the committed source replacement, and that replacement is not the upload destination.

The script requires Cargo credentials before querying or editing. It chooses the next minor after the greater of the local and published versions, resets the patch, and derives dependency order from `tools/release_graph.py`. One commit updates the workspace, component and consumer requirements, and both lockfiles. Components receive `<name>-v<version>` tags before upload; a tag does not prove successful publication. A matching release commit permits resuming partial publication. If a published version exists and the local version is newer without a matching release commit, the script stops. Keep the release identity intact when resuming.

The consumer is a separate workspace. `tests/consumer/check.sh` resolves its requirement and updates its lockfile before the release commit. Set `TMPDIR` to an existing `.work/tmp` directory for release backups. Publish this workspace before an h12tiny release that removes its smolith path dependencies.

[.cargo/config.toml](../.cargo/config.toml) routes dependency downloads through the `stuff` mirror. That source replacement does not select the upload registry. Cargo checks require the mirror to contain the versions in the committed lockfiles.

## Local package verification

`tools/stage.sh` publishes the owned crates in dependency order to a localhost sparse registry, using Cargo's package-build verification. For that process, crates.io is replaced by the staging index. The server proxies external dependencies and refuses to obtain an owned `laputa-*` crate from the public registry.

The verifier checks licenses, dependency identities, the absence of patches and Tokio normal/build edges, and preservation of default features explicitly disabled in member manifests. The staging index omits dev-dependencies. A fresh external consumer then builds and tests `laputa-smol` from the index without path dependencies. This verifies packaged dependencies; the path consumer checks the checkout graph instead.

## Other checks

[AGENTS.md](../AGENTS.md) lists the local commands. [The workflow](../.github/workflows/ci.yml) defines its feature, MSRV, Miri, Loom, and platform selections. An all-features workspace build does not establish minimal-feature support, and a target check does not establish execution on that target. See [features](FEATURES.md), [platforms](platforms.md), and [security](../SECURITY.md) for those contracts.
