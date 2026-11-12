# Release

Runtime crates share the version in [Cargo.toml](../Cargo.toml). HTTP crates share one separate version. Protocol forks keep the versions declared on their own manifests. [UPSTREAM.toml](../UPSTREAM.toml) maps runtime and protocol identities. A source tag, a packaged crate, and a registry upload are separate artifacts.

## Publication

`make publish-plan` prints every publishable package, its family, its current version, and the version a local minor bump would choose. It does not query a registry or edit the tree. Protocol versions stay put.

`make publish-dry-run` queries crates.io, plans the runtime and HTTP minors from the greater of the local and published versions, and checks that plan in a temporary clone. It does not upload, commit, or tag.

`make publish` performs that release. It rewrites only runtime and HTTP versions, verifies the workspace and the local staging registry, commits, tags the packages whose versions changed, and uploads to crates.io. It then publishes the same packages through `stuff publish cargo`. `REGISTRY` defaults to `mycrates` and `STUFF` selects the executable. `make publish-mycrates` repeats only the private phase for versions already on crates.io.

Public cargo commands name `--registry crates-io` and run outside the repository so the committed source replacement is not the upload destination. That replacement remains the download mirror for ordinary development.

The script requires Cargo credentials before querying or editing. A registry version newer than the intended version does not count as that version. A matching release commit resumes the same versions instead of bumping again. Uploads are not atomic: a later failure leaves earlier packages published, and the same commit is the resume point. Set `TMPDIR` to an existing `.work/tmp` directory for release backups.

Protocol forks are part of this release. They are not bumped with the runtime or HTTP families. Publishing a changed fork payload requires a new version on the fork manifest before `make publish`.

## Local package verification

`tools/stage.sh` publishes the owned crates in dependency order to a localhost sparse registry, using Cargo's package-build verification. For that process, crates.io is replaced by the staging index. The server proxies external dependencies and refuses to obtain an owned crate, including `h12tiny` names, from the public registry.

The verifier checks licenses, dependency identities, the absence of patches and Tokio normal/build edges, and preservation of default features explicitly disabled in member manifests. The staging index omits dev-dependencies. Fresh consumers then build `laputa-smol` and small HTTP, TLS, blocking, web, and component crates from the index without path dependencies.

## Other checks

[AGENTS.md](../AGENTS.md) lists the local commands. [The workflow](../.github/workflows/ci.yml) defines its feature, MSRV, Miri, Loom, protocol, and platform selections. An all-features workspace build does not establish minimal-feature support, and a target check does not establish execution on that target. See [features](FEATURES.md), [platforms](platforms.md), and [security](../SECURITY.md) for those contracts.
