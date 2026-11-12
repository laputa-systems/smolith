---
name: publish-forks
description: "Publish the workspace, including the protocol forks, from the smolith repository when the owner asks."
disable-model-invocation: true
---

# Publish the workspace

The protocol forks are workspace members. `make publish` plans, verifies, and publishes the runtime crates, the HTTP crates, and the three forks. Do that only when the owner explicitly asks for a release. Do not push a release tag unless that request includes the tag.

## Rules

- Run commands from the repository root. `ROOT=$(git rev-parse --show-toplevel)`.
- Require a clean tree before `make publish` or `make publish-dry-run`.
- `make publish-plan` is offline and does not edit the tree.
- `make publish-dry-run` rewrites versions only in a temporary clone and does not upload.
- Keep the root manifest free of `[patch]`.
- Protocol versions are not part of the runtime or HTTP minor bump. A changed fork payload needs a new version on that manifest before publication.
- Public cargo commands name `--registry crates-io` and run from `/` with the pinned `RUSTUP_TOOLCHAIN`. `--registry crates-io` alone does not disable the stuff download mirror.
- The private phase is `stuff publish cargo` with `REGISTRY` defaulting to `mycrates`.

## Report

Report the packages and versions published or skipped, the registries checked, and any release that was not performed.
