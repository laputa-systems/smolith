# smolith

Workspace of the smol ecosystem. Published names are `laputa-*`. The facade is `laputa-smol` (library `smol`). One version, `0.1.0`. Upstream versions live in `UPSTREAM.toml`.

## Commands

- `cargo test --workspace` runs the suite, including doctests.
- `cargo clippy --workspace --all-targets` is the lint. This repo has no rustfmt.
- `tools/graph-gate.sh` checks owned edges: `laputa-*` 0.1.0, one version each, no `tokio` in normal or build dependencies.
- `tools/advisory-scan.sh` maps `UPSTREAM.toml` through the RustSec database, including optional normal features. `--offline` skips the refresh.
- `tools/stage.sh` publishes every crate to a local registry and builds a fresh consumer from that index.
- `tests/consumer/check.sh` checks an external path consumer.
- `fixtures/h12tiny.sh` runs the pinned h12tiny gates. `H12TINY_MIRROR` selects the clone source.

`rust-toolchain.toml` pins `nightly-2026-09-15` with `rust-src` and `clippy`.

## Invariants

- Normal and build edges use `[workspace.dependencies]` with `package`, `path`, and `version`. The workspace has no `[patch]` table.
- A versionless path edge is only a member dev-dependency, and it sets `package = "laputa-..."`.
- A dependency that must build with its own defaults off sets `default-features = false` on the workspace entry. A member line cannot turn workspace defaults off.
- `futures-core` and `futures-io` are the shared trait identities.
- No `tokio` crate on a normal or build edge.
- Windows is unsupported.

## Docs

- [docs/FEATURES.md](docs/FEATURES.md) lists crate features.
- [docs/platforms.md](docs/platforms.md) records hosts, gaps, and what still needs a machine.
- [docs/release.md](docs/release.md) separates the version, a git tag, packaged crates, and publication.
- [docs/BASELINES.md](docs/BASELINES.md) records the idle, wake, and TCP figures.
- [docs/LEDGER.md](docs/LEDGER.md) records issues, pull requests, and advisories.
- [UPSTREAM.toml](UPSTREAM.toml) is the provenance map.
- [SECURITY.md](SECURITY.md) is the reporting path.
