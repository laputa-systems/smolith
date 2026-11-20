# HTTP/2 connection and streams audit

Base `5f2694c09359747290c83f2399658d5e7f2b2f0d`; Darwin aarch64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. `UPSTREAM.toml:277` pins hyperium/h2 0.4.20, commit `1fa08adf223690035f602e899d3bc9be2897eee3`. No API or runtime code change is proposed. The requested `origin/deslop/h2-framing` ref was unavailable; `git show deslop/h2-framing:docs/deslop-h2-framing.md` read the sibling report at `27f97049be3649eb9bdbe7e851b80f48efc0654b` instead.

## DSLP-9 · Types

With `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6`, `cargo clippy -p laputa-h2-futures-lite --lib --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code` exited 0. Counts are compiler diagnostics with primary location in `src/proto`, default features, library target, excluding dependencies and tests.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 0 | Reject: no witness or escape hatch removed. |
| `wildcard_enum_match_arm` | 17 | Reject: grouped cases; no missing variant defect witnessed. |
| `match_wildcard_for_single_variants` | 0 | Reject: no witness. |
| `cast_possible_truncation` | 16 | Reject: window and frame conversions need range proofs; no defect witnessed. |
| `cast_sign_loss` | 5 | Reject: flow-control signed arithmetic needs range proofs; no defect witnessed. |
| `unwrap_used` / `expect_used` | 77 / 19 | Reject: lock poisoning, queue and frame invariants need individual proof; no safe removal demonstrated. |
| `redundant_clone` / `needless_pass_by_value` | 0 / 4 | Reject: no type defect or removed trust demonstrated. |

Adopted 0 of 9 candidates; rejected 9. Diagnostics are leads, not proof each cast or assertion is safe. No escape hatches were added.

## DSLP-21 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Stream and connection capacity returned on consumption, including padding | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/flow_control.rs:154`, `:223`; `crates/laputa-h2-futures-lite/tests/flow_control_futures.rs:58` | Retain: emitted window updates and continued data flow. |
| Reset preserves received end-of-stream and accounting | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/stream_states.rs:620`, `:1233`, `:1240` | Retain: pending-accept counts and completed body/reset reason. |
| Backpressured PING and SETTINGS responses are bounded | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/h2_975.rs:127` | Retain: each flood must result in ENHANCE_YOUR_CALM GOAWAY. |
| Multiple PINGs | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/ping_pong.rs:38` | Retain: multiple PONGs reach the peer. |
| GOAWAY and processed streams | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/stream_states.rs:755` | Retain: processed streams complete across GOAWAY. |
| Idle established connection | None in this component establishes a server idle bound | Known open DSLP-45: `docs/deslop-diagnose.md:5` reproduces slot retention in h12tiny-server; out of scope here. |

The ignored empty tests `flow_control.rs:353`, `:357`, and `:970` are not guards. `flow_control.rs:479` is also ignored. No tests were changed. Test tiers and observed costs are in `AGENTS.md`; warm before/after cost and flake rate were not measured.

Fault injection: `cp crates/laputa-h2-futures-lite/src/proto/connection.rs .work/tmp/h2-proto-connection-original`; replace `const MAX_PENDING_CONTROL_RESPONSES: usize = 32;` with `const MAX_PENDING_CONTROL_RESPONSES: usize = 128;`. Before the edit, `cargo test -p h2-tests --test h2_975 control_response_overflow_closes_without_panicking -- --exact` exited 0 (1 passed). After it, the same command exited 101: `control_response_overflow_closes_without_panicking` failed at `h2_975.rs:170` with `bounded control backlog closes: Stall` after the two-second bound. The test sends 64 frames, below the injected cap. `cp .work/tmp/h2-proto-connection-original crates/laputa-h2-futures-lite/src/proto/connection.rs`; `cmp .work/tmp/h2-proto-connection-original crates/laputa-h2-futures-lite/src/proto/connection.rs` exited 0. This one fault detects the lost bound; it is not a mutation sweep.

Needs environment: `command -v cargo-mutants`, `cargo-llvm-cov`, and `test-pyramid` each exited 1. No branch coverage, automated survivor counts or complete mutation sweep are claimed. Adding recipes or CI is out of scope.

## DSLP-33 · Dead code

The DSLP-9 command enabled rustc `dead_code`; scoped default-feature library diagnostics: 0. `rg -n 'allow\(dead_code|allow\(unused' crates/laputa-h2-futures-lite/src/proto` found `streams/store.rs:139`, an unreachable `Infallible` match arm. The workspace search `rg -n 'for_each\(|try_for_each\(' crates/laputa-h2-futures-lite/src/proto/streams` finds the store iteration callers in `streams.rs:790`, `:819`, `:959`, `recv.rs:611`, `:628` and `send.rs:538`, `:606`; retain it.

`rg -n 'MAX_PENDING_CONTROL_RESPONSES|control_responses|count_pending_refusal|recv_reset|release_capacity' --glob '!TODO-DESLOP.md' --glob '!*.lock'` searches workspace callers: the queue is fed by SETTINGS and PING at `connection.rs:470`, `:681`; reset passes through `connection.rs:655` to `streams/streams.rs:403`; public `share.rs:532` reaches receive capacity, used by Hyper at `crates/laputa-hyper-futures-lite/src/body/incoming.rs:188` and `src/proto/h2/upgrade.rs:195`. `rg -n 'h2::(client|server)|h2::(SendStream|RecvStream)|release_capacity|poll_reset|send_reset' crates/h12tiny crates/h12tiny-server crates/laputa-hyper-futures-lite/src` and `rg -n 'h2::|http2|Http2' crates/h12tiny/src crates/h12tiny-server/src/conn/auto.rs` identify Hyper's h2 client/server entry points at `crates/laputa-hyper-futures-lite/src/proto/h2/client.rs:15` and `server.rs:9`, and h12tiny's Hyper HTTP/2 builder at `crates/h12tiny-server/src/conn/auto.rs:307`. No code removed; caller searches alone do not prove external or feature-specific nonuse.

Needs environment: `command -v cargo-machete`, `cargo-udeps`, `cargo-bloat`, and `debloat` each exited 1. Dependency liveness, non-default features and other platforms remain unverified; no clean-or-allowlisted claim is made.
