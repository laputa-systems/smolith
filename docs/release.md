# Release

The workspace version is `0.1.0` in the root `Cargo.toml`. Upstream crate versions are only in `UPSTREAM.toml`.

A git tag names a source revision. It does not upload crates. No release tag is part of this tree, and nothing from this tree is published to crates.io.

`tools/stage.sh` is the local publish check. It runs `cargo publish` for all 26 `laputa-*` crates against a localhost sparse index, without `--no-verify`. For that command only, crates.io is replaced by the staging index, which proxies every other crate. The workspace has no `[patch]` table. This nightly has no `cargo package --dry-run`. Publish verifies by packaging and building.

The script then checks each uploaded crate for `LICENSE-MIT`, `LICENSE-APACHE`, the absence of `[patch]`, and dependency names. A normal or build dependency must not use an upstream package name or a `tokio` name. Where the source manifest sets `default-features = false` on a normal or build dependency, the packaged manifest must keep that flag. Dev-dependencies are omitted from the published index. A fresh crate outside the workspace then builds `laputa-smol` `0.1.0` from the staging registry and runs its test. That consumer does not use a path dependency.

`tools/advisory-scan.sh` reads `upstream_name` and `upstream_version` from `UPSTREAM.toml`. RustSec has no entries under the `laputa-*` names, so a scan of those names alone is not evidence. The scan fails an unsound advisory when the pinned version is vulnerable and a normal or build edge reaches it. Dev-only hits are printed and do not fail the scan. `laputa-event-listener` tracks event-listener 5.4.2, which is outside RUSTSEC-2026-0221.

`.github/workflows/ci.yml` pins third-party actions by commit. It runs Clippy, `cargo test --workspace`, `cargo check --workspace --all-features --all-targets`, the graph gate, the advisory scan, the consumer, the h12tiny fixture, the staging publish, a macOS test, and the musl checks from [platforms](platforms.md). The consumer and fixture scripts read TOML with the Python standard library. The musl job installs `musl-tools` and points the musl C compiler at `musl-gcc`. The workflow does not run rustfmt.
