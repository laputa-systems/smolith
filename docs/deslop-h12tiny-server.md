# HTTP server and web utilities audit

Base `31e6041519af7a3736cfab82b2babc1bd86ba8f9`; Darwin, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. The three crates are published; no public API or runtime code was changed. Set `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6` for the commands below. Contracts are documented in `docs/h12tiny/server.md`, `application.md` and `limits.md`.

## DSLP-16 · Types

`cargo clippy -p h12tiny-server -p h12tiny-util -p h12tiny-web --lib --all-features --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code` exited 0. Counts are diagnostic occurrences whose primary span is in these three crates, including repeated macro diagnostics.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 3 | Reject: existing pin projection and stream implementation require their invariants; no safe replacement established. |
| `wildcard_enum_match_arm` | 3 | Reject: wildcard matches do not establish a defect. |
| `match_wildcard_for_single_variants` | 0 | Reject: no witness. |
| `cast_possible_truncation` / `cast_sign_loss` | 0 / 0 | Reject: no witness. |
| `unwrap_used` / `expect_used` | 0 / 16 | Reject: no removable escape hatch established; includes validated defaults and fixed HTTP response builders. |
| `redundant_clone` / `needless_pass_by_value` | 0 / 1 | Reject: no type defect established. |

Adopted 0 of 9 settings; rejected 9. Counts are leads, not proofs of safety. No typing change or escape-hatch removal was justified.

## DSLP-28 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Admission, setup and drain deadlines | `crates/h12tiny/tests/server_bounds.rs:75`, `:139`, `:183`, `:253`; `crates/h12tiny-server/src/lifecycle.rs:1025`, `:1046`, `:1067` | Retain: capacity rejection, slot release, established streams and draining have separate observable outcomes. |
| Protocol detection and TLS shutdown | `crates/h12tiny-server/src/conn/auto.rs:1154`, `:1179`; `crates/h12tiny/tests/server_tls_shutdown.rs:135`, `:301`, `:459` | Retain: preface replay, setup cancellation and established stream draining. |
| HTTP upgrade and WebSocket handshake | `crates/h12tiny-server/src/upgrade.rs:118`; `crates/h12tiny-web/src/lib.rs:3238`, `:3254`, `:3274`; `crates/h12tiny/tests/websocket_interop.rs:69` | Retain: local transport, valid response and malformed handshake decisions. |
| Body streaming and limits | `crates/h12tiny-util/src/lib.rs:787`, `:857`, `:870`, `:897`, `:911`; `crates/h12tiny-web/src/lib.rs:2800`, `:2823`, `:2854` | Retain: pending reads, errors, limit and timeout behavior. |
| Routing, composition and web wire formats | `crates/h12tiny-web/src/lib.rs:2762`, `:2962`, `:3015`, `:3174`, `:3322`, `:3576`, `:3691`; `crates/h12tiny/tests/web_transport.rs:104` | Retain: dispatch, composition, defaults and wire formats; H1/H2 integration. |

Fault injection: `cp crates/h12tiny-web/src/lib.rs .work/tmp/h12tiny-web-original`; replace the single `decoded.len() == 16` with `decoded.len() != 16`; run `cargo test -p h12tiny-web --features websocket --lib websocket_upgrade_rejects_invalid_and_ambiguous_keys`. Exit 101: `tests::websocket_upgrade_rejects_invalid_and_ambiguous_keys` failed at `crates/h12tiny-web/src/lib.rs:3256`, accepting an invalid key (`None` instead of `Some(InvalidKey)`). Restore with `cp .work/tmp/h12tiny-web-original crates/h12tiny-web/src/lib.rs`; `cmp .work/tmp/h12tiny-web-original crates/h12tiny-web/src/lib.rs` exited 0. This is one fault, not a mutation sweep.

Needs environment: `command -v cargo-mutants` and `command -v cargo-llvm-cov` each exited 1. No branch coverage, automated mutation survivors or warm before/after cost and flake rate are claimed. The unit and in-memory tier is supplemented by local-loopback protocol integration; workspace suite costs are recorded in `AGENTS.md`. Coverage and mutation recipes and CI are outside this task. No tests were changed.

## DSLP-40 · Dead code

The Types command enabled rustc `dead_code`: 0 primary diagnostics in the scoped crates with all features. `rg -n 'allow\(dead_code|allow\(unused' crates/h12tiny-{server,util,web}` exited 1. `rg -n '(^|[^[:alnum:]_])(static_bytes|BodyFactory|ReplayableRequest|ReaderStream|IdleTimeoutBody|collect_bytes_limited|WebSocketUpgrade|RouterBounds|ProductionBounds)([^[:alnum:]_]|$)' --glob '!Cargo.lock'` searched the workspace, including docs and tests. `collect_bytes_limited` serves the web extractor (`crates/h12tiny-web/src/lib.rs:514`); body factories have client consumers (`crates/h12tiny/tests/client_compat.rs:190`, `:239`); `ReaderStream` and idle timeout are documented (`docs/h12tiny/limits.md:42`, `:44`). Retain.

`rg -n 'static_bytes\s*\(' --glob '!Cargo.lock'` found only its definition (`crates/h12tiny-util/src/lib.rs:69`); `rg -n 'static_bytes|RequestFactory|replayable_value' --glob '!Cargo.lock'` likewise found only their definitions. Consumer question: do published-crate users rely on these public conveniences? Local absence does not establish external absence; retain all three. No private code was proven dead or removed.

Needs environment: `command -v cargo-machete`, `command -v cargo-udeps`, `command -v cargo-bloat`, and `command -v debloat` each exited 1. Dependency liveness, other platforms and external consumers remain unverified; no tool-clean claim is made.
