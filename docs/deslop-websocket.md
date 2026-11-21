# Websocket and protocol test support audit

Base `58f1233ee70eb44af306750eab59f9c8c130992b`; Darwin aarch64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. `UPSTREAM.toml:274` records no fastwebsockets upstream revision, so its whole fork is in scope. The requested remote sibling ref is absent; `git show deslop/h2-api:docs/deslop-h2-api.md` reads the local sibling report. No API or runtime change is proposed.

## DSLP-14 · Types

With `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6`, strict diagnostics were measured on both default-feature library targets with `cargo clippy -p laputa-fastwebsockets-futures-lite -p laputa-protocol-test-support --lib --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code`. Counts exclude dependencies and test targets.


The command exited 0. Counts are primary diagnostics in scoped source files:

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 1 | Reject: aligned mask words in `src/mask.rs:33`; no safe equivalent established here. |
| `wildcard_enum_match_arm` | 7 | Reject: grouped alternatives and forward-compatible matches; no defect witnessed. |
| `match_wildcard_for_single_variants` | 0 | Reject: no witness. |
| `cast_possible_truncation` | 5 | Reject: length-guarded encoding, aligned prefix length, and pointer-width-specific decoding; no defect witnessed. |
| `cast_sign_loss` | 0 | Reject: no witness. |
| `unwrap_used` / `expect_used` | 15 / 2 | Reject: parser and mock state invariants require individual proof; no escape hatch safely removed. |
| `redundant_clone` / `needless_pass_by_value` | 0 / 0 | Reject: no witness. |

Adopted 0 of 9 candidates; rejected 9. Counts are leads, not a proof that every cast or assertion is safe. No escape hatches were added.

## DSLP-26 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Client masks; server rejects unmasked and decodes masked frames | `crates/laputa-fastwebsockets-futures-lite/tests/masking.rs:19`, `tests/fragment.rs:148`, `:162` | Retain: checks role-specific wire bytes and decoded payload. |
| Fragmentation and message size | `crates/laputa-fastwebsockets-futures-lite/tests/fragment.rs:53`, `:78`, `:106`, `:127`, `:201` | Retain: rejects restarts and aggregate overflow, preserves ping and boundary behavior. |
| Text UTF-8 | `crates/laputa-fastwebsockets-futures-lite/tests/fragment_utf8.rs:28`, `:51`, `:79`, `:113` | Retain: incomplete and invalid text rejected, split Unicode and raw fragments preserved. |
| Read and close cancellation | `crates/laputa-fastwebsockets-futures-lite/tests/read_cancellation.rs:136`, `:143`, `:149`, `:188`, `:222`; `tests/double_close.rs:66`, `:247`, `:252` | Retain: partial input and output resume, close is sent once. |
| Frame headers, control frames and close codes | `crates/laputa-fastwebsockets-futures-lite/src/frame.rs:412`, `:421`, `:433`; `tests/fragment.rs:176`; `src/close.rs:187` | Retain: concrete encoded headers, control-size rejection and allowed-code mapping. |
| Upgrade and live wire integration | `crates/h12tiny-web/src/lib.rs:3238`, `:3254`, `:3274`; `crates/h12tiny/tests/websocket_interop.rs:69` | Retain: handshake validation and masked echo over H1. |
| Executor, timer and bounded pipe | `crates/laputa-protocol-test-support/tests/support.rs:24`, `:41`, `:58`, `:66`, `:109` | Retain: lost wakes, repolls, timer, backpressure and timeout. |
| Scripted I/O | `crates/laputa-protocol-test-support/tests/support.rs:91`; `crates/laputa-hyper-futures-lite/src/proto/h1/dispatch.rs:815` | Retain: ordered read/write and consumer integration. |

No tests changed. Test tiers and observed suite cost are in `AGENTS.md`; warm before/after cost and flake rate were not measured.


Fault injection: `cargo test -p laputa-fastwebsockets-futures-lite --test fragment server_rejects_unmasked_client_frames -- --exact` exited 0 (1 passed). `cp crates/laputa-fastwebsockets-futures-lite/src/lib.rs .work/tmp/web-lib-original`; replace the single `if self.role == Role::Server && !masked {` with `if self.role == Role::Client && !masked {`. The same command exited 101: `server_rejects_unmasked_client_frames` failed at `tests/fragment.rs:157`, expecting `UnmaskedFrame`. `cp .work/tmp/web-lib-original crates/laputa-fastwebsockets-futures-lite/src/lib.rs`; `cmp .work/tmp/web-lib-original crates/laputa-fastwebsockets-futures-lite/src/lib.rs` exited 0. The test detects disabling the server's mask requirement.

Needs environment: `command -v cargo-mutants`, `command -v cargo-llvm-cov`, and `command -v test-pyramid` each exited 1. Branch coverage and automated survivor counts are unmeasured; the hand fault is not a mutation sweep. Coverage and mutation recipes and CI are outside scope.

## DSLP-38 · Dead code

The DSLP-14 command enabled rustc `dead_code`: 0 scoped diagnostics for default-feature library targets. Workspace searches: `rg -n 'laputa-protocol-test-support|laputa_protocol_test_support' --glob 'Cargo.toml' --glob '*.rs' --glob '!target/**' --glob '!.work/**'` and `rg -n 'laputa-fastwebsockets|fastwebsockets' --glob 'Cargo.toml' --glob '*.rs' --glob '!target/**' --glob '!.work/**'`. The workspace dependency is consumed by `crates/h12tiny-web/Cargo.toml:20` and `:23`, with calls at `crates/h12tiny-web/src/lib.rs:38` and `crates/h12tiny-web/src/websocket_client.rs:7`. H2 and Hyper declare shared support as dev-dependencies at `crates/laputa-h2-futures-lite/Cargo.toml:53` and `crates/laputa-hyper-futures-lite/Cargo.toml:55`; actual uses include `crates/laputa-h2-futures-lite/tests/h2-support/src/prelude.rs:31`, `crates/laputa-h2-futures-lite/tests/flow_control_futures.rs:5`, and `crates/laputa-hyper-futures-lite/tests/support/mod.rs:341`. Fuzz targets and the unmask benchmark are registered in the fork manifests and call its API.

`rg -n 'poll_once|sleep_until|timer::now|build_with_handle|read_error|write_error' crates/laputa-h2-futures-lite crates/laputa-hyper-futures-lite crates/laputa-protocol-test-support --glob '*.rs'` finds `build_with_handle` used in Hyper's `src/proto/h1/conn.rs:1328` and `src/proto/h1/dispatch.rs:815`. The shared support `executor::poll_once` and `timer::sleep_until` have no demonstrated caller in that search; they are public and removal would change the API, so retain as unproven. No code removed; external consumers and non-default platforms remain unverified.

Needs environment: `command -v cargo-machete`, `cargo-udeps`, `cargo-bloat`, and `debloat` each exited 1. Dependency liveness and binary-size analysis are unverified; no clean-or-allowlisted claim is made.
