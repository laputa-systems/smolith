# Runtime facade and utilities audit

Base `5f2694c09359747290c83f2399658d5e7f2b2f0d`; Darwin aarch64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. The seven crates and upstream revisions are in `UPSTREAM.toml`. No runtime or public API change is proposed. All commands use `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6`.

## DSLP-7 · Types

Run `cargo clippy -p laputa-smol -p laputa-smol-macros -p laputa-futures-lite -p laputa-async-dup -p laputa-easy-parallel -p laputa-fastrand -p laputa-fastrand-contrib --lib --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code`: exit 0 (`build-finished: success`). Counts are primary diagnostics in scoped crate files, default features and library targets; macro expansions can produce several diagnostics at one source location.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 0 | Rejected: no witness. |
| `wildcard_enum_match_arm` | 3 | Rejected: grouped bounds and result cases; no defect witnessed. |
| `match_wildcard_for_single_variants` | 0 | Rejected: no witness. |
| `cast_possible_truncation` | 23 | Rejected: random generator word arithmetic and conversions need individual range proofs; no defect witnessed. |
| `cast_sign_loss` | 6 | Rejected: integer range macro wraps signed values; no defect witnessed. |
| `unwrap_used` | 22 | Rejected: includes choice from nonempty constants; no unsound escape hatch removed. |
| `expect_used` | 5 | Rejected: includes valid UTF-8 prefix and completed-future invariants; no unsound escape hatch removed. |
| `redundant_clone` | 0 | Rejected: no witness. |
| `needless_pass_by_value` | 0 | Rejected: no witness. |

Adopted 0 of 9 candidates; rejected 9. Diagnostics are leads, not a claim that every instance is harmless. No escape hatches were added.

## DSLP-19 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Facade block_on and I/O | `tests/consumer/tests/interop.rs:5`; `tests/qualify/src/boundaries.rs:77` | Retain: external consumer and TCP boundary; broader wake behavior is not established by these lines alone. |
| Macro drives executor | `crates/smol-macros/tests/macro_usages.rs:24`, `:81` | Retain: spawned tasks rendezvous with the test on shared and local executors. Basic print-only cases at `:13` and `:19` have no behavioral assertion. |
| Future combinators | `crates/futures-lite/src/future.rs:250`, `:414`; `tests/qualify/src/boundaries.rs:414` | Retain: doctest results and channel send/receive through zip. |
| Buffered text and errors | `crates/futures-lite/src/io.rs:3181`, `:3192`, `:3279` | Retain: errors preserve valid and partial bytes. |
| Shared async I/O forwarding | `crates/async-dup/src/lib.rs:490` | Send/Sync compile assertion only; no runtime forwarding boundary test identified. |
| Parallel results and finish | `crates/easy-parallel/tests/smoke.rs:20`, `:29` | Retain: exact ordered results and finish result. |
| Integer range bounds | `crates/fastrand/tests/smoke.rs:31` | Retain: repeated exclusive upper-bound assertions; sensitivity checked below. |
| Floating-point bounds | `crates/fastrand-contrib/src/float_range.rs:337`, `:346`, `:360` | Retain: adjacent endpoints, subnormals, wide ranges and invalid bounds. |

Fault injection: `cp crates/fastrand/src/lib.rs .work/tmp/fastrand-original`, then replace the integer range macro's `Bound::Excluded(&x) => x.checked_sub(1).unwrap_or_else(panic_empty_range),` with `Bound::Excluded(&x) => x,`. `cargo test -p laputa-fastrand --test smoke u32 -- --exact` exited 101: `u32` failed at `crates/fastrand/tests/smoke.rs:37` on the exclusive upper-bound assertion. `cp .work/tmp/fastrand-original crates/fastrand/src/lib.rs` restored the source; `cmp .work/tmp/fastrand-original crates/fastrand/src/lib.rs` exited 0. This is one sensitivity check, not a mutation sweep.

Verification tiers: edit uses the affected focused test; change uses the seven-package test and clippy commands; full uses `make test` and `make lint` as requested. Existing suite costs are in `AGENTS.md`; no before/after warm comparison was measured. No tests were changed. Coverage and automated mutation were unavailable; no survivor count is claimed.

## DSLP-31 · Dead code

The Types command enabled rustc `dead_code`; no scoped diagnostic was emitted for default-feature libraries. `rg -n 'dead_code|allow\(unused' crates/{smol,smol-macros,futures-lite,async-dup,easy-parallel,fastrand,fastrand-contrib}` found no such allowances. Workspace searches `rg -n 'random_seed\(|try_with_rng\(|with_rng\(' --glob '*.rs'` and `rg -n 'Parallel::new\(|future::zip\(|async_dup::(Arc|Mutex)|fastrand_contrib::' --glob '*.rs'` find the random seed and thread-local helpers used at `crates/fastrand/src/global_rng.rs:26`, `:31`, `:73`, `Parallel` consumers at `crates/async-channel/tests/bounded.rs:474`, zip at `tests/qualify/src/boundaries.rs:414`, and async-dup at `crates/smol/examples/chat-server.rs:19`. Retain these; public exports may also have external consumers. No deletion was proven.

Needs environment: `command -v cargo-mutants`, `cargo-llvm-cov`, `cargo-machete`, `cargo-udeps`, `cargo-debloat`, and `debloat` each exited 1. Coverage, bounded mutation, dependency liveness and debloat remain unverified; no clean-or-allowlisted claim is made. Non-default feature and non-Darwin paths remain unverified.
