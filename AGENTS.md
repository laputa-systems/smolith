# smolith

Workspace for the smol runtime and the h12tiny HTTP stack. Runtime package names are `laputa-*`; the facade is `laputa-smol` (library `smol`). HTTP package names stay `h12tiny` and `h12tiny-*`. The protocol forks keep the library names `h2`, `hyper`, and `fastwebsockets`.

Runtime crates share the version in `Cargo.toml`. HTTP crates share a separate version. Protocol forks keep their own versions. `UPSTREAM.toml` records upstream provenance for runtime and protocol code. First-party HTTP crates have no upstream identity.

## Commands

- `make test` runs the workspace suite, the h12tiny suite with all features, and the maintained Hyper tests.
- `make lint` is clippy for the workspace and for the maintained HTTP and Hyper targets. This repo has no rustfmt.
- `python3 -m unittest discover -s tests -p 'test_*.py'` checks release-graph validation and staging isolation without compiling Rust.
- `tools/graph-gate.sh` checks owned edges and rejects `tokio` on normal or build dependencies.
- `tools/advisory-scan.sh` maps `UPSTREAM.toml` through the RustSec database, including optional normal features. `--offline` skips the refresh.
- `tools/stage.sh` publishes every owned crate to a local registry and builds consumers from that index.
- `sh scripts/check-features.sh` checks HTTP feature isolation outside an all-features build.
- `sh scripts/check-normal-dependencies.sh` checks the facade's normal dependency graph.
- `sh scripts/check-msrv.sh` checks each member with its own `rust-version`.
- `sh scripts/verify-protocols.sh` runs the protocol suites. `--check-ledger` checks [the parity ledger](docs/h12tiny/fork-test-parity.md).
- `make publish-plan` prints the package order and the versions a release would use. `make publish-dry-run` checks that plan in a temporary clone. `make publish` bumps, commits, tags, and publishes publicly, then privately. Details are in [release](docs/release.md).
- `tests/consumer/check.sh` checks an external runtime consumer.

`rust-toolchain.toml` pins the development toolchain. Cargo dependencies resolve through the source replacement in `.cargo/config.toml`.

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
- [docs/LEDGER.md](docs/LEDGER.md) is the historical upstream issue, pull request, and advisory snapshot.
- [TODO.md](TODO.md) holds current findings and deferred work.
- [UPSTREAM.toml](UPSTREAM.toml) is the provenance map.
- [SECURITY.md](SECURITY.md) is the reporting path.
