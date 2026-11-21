# HTTP/2 client, server and test support audit

Base `11f2345a92ff7935cf766316d1f93046726de989`; Darwin aarch64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. `UPSTREAM.toml:277` pins hyperium/h2 0.4.20. The requested remote sibling refs were absent; the local `deslop/h2-framing` and `deslop/h2-proto` branches contain their reports. No API or runtime change is proposed.

## DSLP-10 · Types

With `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6`, `cargo clippy -p laputa-h2-futures-lite --lib --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code` exited 0. Counts are primary diagnostics in scoped top-level library files, default features; dependencies and test targets are excluded.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 1 | Reject: `lib.rs:188` is read-only and no safe replacement was established. |
| `wildcard_enum_match_arm` | 3 | Reject: error accessors group other kinds; no defect witnessed. |
| `match_wildcard_for_single_variants` | 0 | Reject: no witness. |
| `cast_possible_truncation` | 2 | Reject: stream capacity casts need range proof; no defect witnessed. |
| `cast_sign_loss` | 0 | Reject: no witness. |
| `unwrap_used` / `expect_used` | 6 / 2 | Reject: state and SETTINGS invariants need individual proof; no escape hatch safely removed. |
| `redundant_clone` / `needless_pass_by_value` | 2 / 1 | Reject: no type defect or removed trust demonstrated; public signature cannot change in this scope. |

Adopted 0 of 9 candidates; rejected 9. Counts are leads, not proof every assertion or cast is safe. No escape hatches were added.

## DSLP-22 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Client request scheme, authority and CONNECT pseudo headers | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/client_request.rs:545`, `:577`, `:611` | Retain: distinguishes HTTP/1 fallback, HTTP/2 rejection and CONNECT encoding. |
| Client rejects missing response status | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/client_request.rs:857` | Retain: checks invalid response handling. |
| Server accepts fragmented preface and serves request | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/server.rs:11`, `:184` | Retain: fragmented handshake and request/response path. |
| Server rejects malformed request headers | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/server.rs:616`, `:1488`, `:1792` | Retain: invalid authority, response status in request and non-CONNECT protocol. |
| Graceful shutdown and in-flight streams | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/server.rs:881`, `:958` | Retain: GOAWAY, ping and completed requests, including peer GOAWAY. |
| Fuzz bridge | `crates/laputa-h2-futures-lite/fuzz/fuzz_targets/fuzz_hpack.rs:5` | Fuzz entry, not a regression oracle; automated coverage unavailable. |

No tests changed. Empty `client_request.rs:653` and `server.rs:613` are not guards. Test tiers and observed suite costs are in `AGENTS.md`; warm before/after cost and flake rate were not measured.

Fault injection: baseline `cargo test -p h2-tests --test server graceful_shutdown -- --exact` exited 0 (1 passed). `cp crates/laputa-h2-futures-lite/src/server.rs .work/tmp/h2-api-server-original`; replace the single `self.connection.go_away_gracefully();` with `self.connection.go_away_from_user(Reason::NO_ERROR);`. The same test command exited 101: `graceful_shutdown` failed at `server.rs:932` with `User(UnexpectedFrameType)`. `cp .work/tmp/h2-api-server-original crates/laputa-h2-futures-lite/src/server.rs`; `cmp .work/tmp/h2-api-server-original crates/laputa-h2-futures-lite/src/server.rs` exited 0. The test detects abrupt shutdown substituted for graceful GOAWAY.

Needs environment: `command -v cargo-mutants`, `command -v cargo-llvm-cov`, and `command -v test-pyramid` each exited 1. No branch coverage or automated mutation survivor count is claimed; the single fault is not a mutation sweep. Coverage and mutation recipes and CI are outside this task.

## DSLP-34 · Dead code

The DSLP-10 command enabled rustc `dead_code`; scoped default-feature library diagnostics: 0. `rg -n 'allow\(dead_code|allow\(unused' crates/laputa-h2-futures-lite/src/{client,server,share,error,ext,fuzz_bridge}.rs crates/laputa-h2-futures-lite/tests/{h2-support,h2-fuzz} crates/laputa-h2-futures-lite/{fuzz,util}` found `src/error.rs:28`: the `Reset` variant is constructed in `error.rs:125` and queried at `:54`, `:95`, `:104`, `:114` and displayed at `:165`. Retain: its stream ID field is carried from the protocol error; safe removal and effects on debug output are unproven. No code removed.

Workspace searches: `rg -n 'h2::(client|server)|release_capacity|send_request' crates/h12tiny crates/h12tiny-server crates/laputa-hyper-futures-lite/src` and `rg -n 'h2::|http2::Builder' crates/h12tiny-server/src/conn/auto.rs` find Hyper client and server uses at `crates/laputa-hyper-futures-lite/src/proto/h2/client.rs:15` and `server.rs:9`, flow-control use at `crates/laputa-hyper-futures-lite/src/body/incoming.rs:188`, and h12tiny's Hyper HTTP/2 builder at `crates/h12tiny-server/src/conn/auto.rs:307`. Public API and external consumers cannot be ruled out by a workspace search.

`rg -n 'genfixture|genhuff|h2-fuzz|honggfuzz|fuzz_client|fuzz_hpack|fuzz_e2e' --hidden --glob '!.git/**' --glob '!target/**' --glob '!.work/**' --glob '!Cargo.lock' --glob '!TODO-DESLOP.md'` found `genhuff` documented as generator of `src/hpack/huffman/table.rs:1`; the fuzz bridge is called by `fuzz/fuzz_targets/fuzz_hpack.rs:5`. The `fuzz/Cargo.toml:30` binary registrations establish build entrypoints, but not external execution. `tests/h2-fuzz` and `util/genfixture` have no workspace caller or documented use in this search; liveness is unproven, not claimed live. No removal is proven, particularly for external tooling and upstream-derived entrypoints.

Needs environment: `command -v cargo-machete`, `cargo-udeps`, `cargo-bloat`, and `debloat` each exited 1. Dependency liveness, non-default features and other platforms remain unverified; no clean-or-allowlisted claim is made.
