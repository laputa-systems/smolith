# Hyper runtime traits, extensions and FFI audit

Base `58f1233ee70eb44af306750eab59f9c8c130992b`; Darwin arm64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. Fork provenance is in `UPSTREAM.toml`. No API or runtime change. The requested remote-tracking ref was absent; `git show deslop/hyper-glue:docs/deslop-hyper-glue.md` supplied the local sibling reference.

## DSLP-13 · Types

With `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6`, run `cargo clippy -p laputa-hyper-futures-lite --lib --features full --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code`. Exit 0. Count only diagnostics with a primary span in the scoped source; DSLP-46 tracks the inherited baseline. FFI is excluded from this full-feature profile.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 16 | Reject: includes runtime buffer initialization and pin projections; no safe replacement established. |
| `wildcard_enum_match_arm` | 1 | Reject: `common/io/compat.rs:61` forwards the remaining `Poll` states; no defect established. |
| `match_wildcard_for_single_variants` | 0 | Reject: no witness. |
| `cast_possible_truncation` / `cast_sign_loss` | 0 / 0 | Reject: no witness. |
| `unwrap_used` / `expect_used` | 0 / 4 | Reject: no removable invariant established; `rt/io.rs:347` checks cursor overflow and `error.rs:494` follows `is_io`. |
| `redundant_clone` / `needless_pass_by_value` | 0 / 0 | Reject: no witness. |

Adopted 0 of 9 candidates; rejected 9. No escape hatch removed or added.

## DSLP-25 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Upgrade delivers the stream and buffered bytes | `crates/laputa-hyper-futures-lite/tests/server.rs:1843`; `crates/laputa-hyper-futures-lite/src/upgrade.rs:381` | Retain: handoff, buffered bytes and downcast. |
| Runtime I/O advances the initialized cursor correctly | `crates/h12tiny-core/src/io.rs:354`, `:412`, `:438` | Retain: byte count, oversized read and adapter behavior. |
| Timer reset preserves the timer contract | `crates/h12tiny-core/src/runtime.rs:183` | Retain: sleep and reset. |
| Reason phrases and informational responses | `crates/laputa-hyper-futures-lite/src/ext/h1_reason_phrase.rs:181`, `:210`; `crates/laputa-hyper-futures-lite/tests/client.rs:2355` | Retain: accepted and rejected bytes and response callback. |
| Error classification | `crates/laputa-hyper-futures-lite/tests/client.rs:1844`, `:2316`; `crates/laputa-hyper-futures-lite/tests/server.rs:1601` | Retain: incomplete, oversized and timeout errors. |
| C header case and order | `crates/laputa-hyper-futures-lite/src/ffi/http_types.rs:596`, `:646` | Retain; FFI tests are not executed by the identified repository checks. |

Test tiers and previous suite costs are in `AGENTS.md`: unit, protocol integration and workspace/consumer tests. No test changes; warm before/after cost and flake rate were not measured.

Fault injection: `cp crates/laputa-hyper-futures-lite/src/upgrade.rs .work/tmp/hyper-rt-upgrade-original`; replace `let _ = self.tx.send(Ok(upgraded));` with `let _ = (self.tx, upgraded);`; `cargo test -p laputa-hyper-futures-lite --features full --test server upgrades_new` exited 101: `upgrades_new` failed with `hyper::Error(Canceled, UpgradeExpected)`. Restore with `cp .work/tmp/hyper-rt-upgrade-original crates/laputa-hyper-futures-lite/src/upgrade.rs`; `cmp .work/tmp/hyper-rt-upgrade-original crates/laputa-hyper-futures-lite/src/upgrade.rs` exited 0.

Needs environment: `command -v cargo-mutants` and `command -v cargo-llvm-cov` each exited 1. No automated mutation survivor counts or branch coverage are claimed; one injected fault is not a mutation sweep. Coverage and mutation recipes and CI are outside this task.

## DSLP-37 · Dead code

The Types command enabled rustc `dead_code`: 0 scoped primary diagnostics under `full`. `rg -n 'allow\((dead_code|unused)' crates/laputa-hyper-futures-lite/src/{rt,common,ext,ffi,upgrade.rs,error.rs,trace.rs,cfg.rs,headers.rs,test_support.rs}` found suppressions in `test_support.rs:2`, `trace.rs:3`, `:27`, `error.rs:101` and FFI. `rg -n 'Rewind|OnUpgrade|ReasonPhrase|on_informational|ReadBuf|Timer|Executor' crates --glob '*.rs'` searched workspace callers, including `h12tiny-core/src/io.rs:78`, `h12tiny-core/src/runtime.rs:116`, `h12tiny-client/src/connect.rs:42`, `h12tiny-server/src/upgrade.rs:17`, and `h12tiny-web/src/websocket_client.rs:122`. Retain these interfaces; no code removed.

`Cargo.toml:70` defines `full` without FFI; `:86` defines `ffi`, gated by `src/lib.rs:138` and `src/ffi/mod.rs:39`. `.github/workflows/ci.yml:42` sets `RUSTFLAGS='--cfg hyper_unstable_ffi --cfg hyper_unstable_tracing'` for `cargo check --workspace --all-features --all-targets` at line 43, so CI compiles FFI but does not execute its tests. `capi/README.md:16` documents a cdylib build and `capi/gen_header.sh:40` expands it; `capi/examples/client.c:178` and `upload.c:221` call its executor. Search `rg -n 'hyper_unstable_ffi|features.*ffi|cbuild|gen_header|hyper_(client|executor|task|body)' --glob '!Cargo.lock' --glob '!hyper.h' .` found no repository test command executing the FFI or C examples. External ABI consumers and other platforms remain unknown; retain the public FFI.

Needs environment: `command -v cargo-machete`, `command -v cargo-udeps`, `command -v cargo-bloat`, and `command -v debloat` each exited 1. Dependency liveness, FFI runtime execution, other feature combinations and other platforms remain unverified; no clean-or-allowlisted claim is made.
