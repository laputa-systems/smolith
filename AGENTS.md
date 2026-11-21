# smolith

Workspace for the smol runtime and the h12tiny HTTP stack. Runtime package names are `laputa-*`; the facade is `laputa-smol` (library `smol`). HTTP package names stay `h12tiny` and `h12tiny-*`. The protocol forks keep the library names `h2`, `hyper`, and `fastwebsockets`.

Runtime crates share the version in `Cargo.toml`. HTTP crates share a separate version. Protocol forks keep their own versions. `UPSTREAM.toml` records upstream provenance for runtime and protocol code. First-party HTTP crates have no upstream identity.

## Commands

- `make test` runs the workspace suite, the h12tiny suite with all features, and the maintained Hyper tests.
- `make lint` is clippy for the workspace and for the maintained HTTP and Hyper targets. This repo has no rustfmt.
- `python3 -m unittest discover -s tests -p 'test_*.py'` checks release-graph validation and staging isolation without compiling Rust.
- `tools/graph-gate.sh` checks owned edges and rejects `tokio` on all dependency kinds.
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
- No `tokio` crate on any dependency edge, including development and target-specific edges.
- Windows is unsupported.

## Docs

- [docs/FEATURES.md](docs/FEATURES.md) lists crate features.
- [docs/platforms.md](docs/platforms.md) records hosts, gaps, and what still needs a machine.
- [docs/release.md](docs/release.md) separates the version, a git tag, packaged crates, and publication.
- [docs/BASELINES.md](docs/BASELINES.md) records the idle, wake, and TCP figures.
- [docs/LEDGER.md](docs/LEDGER.md) is the historical upstream issue, pull request, and advisory snapshot.
- Deslop component audits: [runtime scheduling](docs/deslop-runtime-scheduling.md), [runtime I/O](docs/deslop-runtime-io.md), [runtime facade](docs/deslop-runtime-facade.md), [h2 framing](docs/deslop-h2-framing.md), [h2 proto](docs/deslop-h2-proto.md), [h2 api](docs/deslop-h2-api.md), [Hyper h1](docs/deslop-hyper-h1.md), [Hyper glue](docs/deslop-hyper-glue.md), [Hyper runtime](docs/deslop-hyper-rt.md), [websocket](docs/deslop-websocket.md), [h12tiny client](docs/deslop-h12tiny-client.md), [h12tiny server](docs/deslop-h12tiny-server.md), [repo tools](docs/deslop-repo-tools.md); findings from the audits' diagnosis are in [diagnose](docs/deslop-diagnose.md).
- [docs/h12tiny/](docs/h12tiny/) holds the HTTP stack's guides: architecture, client, server, TLS, application layer, features, limits, and fork verification. Its router is [docs/h12tiny/AGENTS.md](docs/h12tiny/AGENTS.md).
- [TODO.md](TODO.md) holds current findings and deferred work.
- [UPSTREAM.toml](UPSTREAM.toml) is the provenance map.
- [SECURITY.md](SECURITY.md) is the reporting path.

## Verification

- `make test` (workspace, h12tiny all features, maintained Hyper suites) and `make lint` (clippy; exits 0 with about 2,400 warning diagnostics, none denied). CI runs each as one step in job `verify` of `.github/workflows/ci.yml`; the macOS job runs `cargo test --workspace`.
- Costs, macOS aarch64 18 cores, busy host (load 12-15), observations: `make test` cold 1,757 s (exit 0; a duplicate run shared the target directory, so an upper bound), warm 210 s (2,562 passed, 32 ignored); `make lint` cold 1,329 s, warm 2 s. Cold means an empty target directory with the kache wrapper active. Set `CARGO_BUILD_JOBS=4` and `TMPDIR=$PWD/.work/tmp`.
- Runtime facade, traits and package graph: `tests/consumer/check.sh`, `tools/graph-gate.sh`, `sh scripts/check-features.sh`, `sh scripts/check-normal-dependencies.sh`, and each crate's tests.
- HTTP client, server and body APIs: `crates/h12tiny/tests/client_compat.rs`, `server_bounds.rs`, `lifecycle_regressions.rs`; TLS and ALPN: `dogfood_tls.rs`, `server_tls_shutdown.rs`; H2 control and lifecycle: `h2_lifecycle.rs`.
- HTTP/1 wire parsing and framing: `crates/h12tiny/tests/adversarial_h1.rs`, `raw_h1.rs`, and the Hyper `proto/h1` unit tests; HTTP/2 framing, HPACK and flow control: `crates/laputa-h2-futures-lite/tests/h2-tests/` and `tests/flow_control_futures.rs`; fork parity: `sh scripts/verify-protocols.sh`, with [the ledger](docs/h12tiny/fork-test-parity.md).
- Websocket wire and web routing: `crates/h12tiny/tests/web_transport.rs`, `websocket_interop.rs`, and the `laputa-fastwebsockets-futures-lite` tests. Release graph and staging: `python3 -m unittest discover -s tests -p 'test_*.py'`.
- Performance workloads: `cargo run --release -p smolith-qualify --bin baseline` (idle CPU, wake latency, TCP loopback; [docs/BASELINES.md](docs/BASELINES.md)) for the runtime, and `benchmarks/twelve-dollar/README.md` for HTTP serving.
- Components, each audited as one deslop slice (about 20k lines or less; records in `docs/deslop-*.md`): runtime scheduling and synchronization; runtime I/O, processes and blocking; runtime facade and utilities; HTTP/2 framing and compression; HTTP/2 connection and streams; HTTP/2 client, server and test support; HTTP/1 protocol; Hyper client, server, body and HTTP/2 glue; Hyper runtime traits, extensions and FFI; websocket and protocol test support; HTTP facade and clients; HTTP server and web utilities; repository tools, benchmarks and CI.
- Upstream-derived trees are recorded with repository, branch, commit and released version in [UPSTREAM.toml](UPSTREAM.toml). Review only local divergence: about 8k changed lines across the forks, most in async-task, async-executor, hyper (1.1k) and h2 (0.9k plus the 2k-line `futures_codec`). Without a registry copy to diff, smol-macros, easy-parallel, fastrand-contrib and fastwebsockets count as ours.
- Consumers: stuff (published `h12tiny-*` crates), the external runtime consumer in `tests/consumer/`, and the fleet repositories bctl, ctrs, laputa, soulseek-rs, stats, tansu and xsh.
