# Runtime scheduling and synchronization audit

Base `aff562aa90024a4e2219a06ec342e502b46a8969`; Darwin aarch64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. The 11 crates are the scope of DSLP-3; their fork provenance is in `UPSTREAM.toml`. No API or runtime code change is proposed here.

## DSLP-3 · Types

With `TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=4`, run `cpu-guard --cores 4 -- cargo clippy -p laputa-async-task -p laputa-async-executor -p laputa-async-lock -p laputa-async-channel -p laputa-async-broadcast -p laputa-concurrent-queue -p laputa-event-listener -p laputa-event-listener-strategy -p laputa-atomic-waker -p laputa-parking -p laputa-waker-fn --lib -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code`: exit 0. Counts below are diagnostics attributed to scoped crates, not dependencies; default features, library targets. They are leads, not proven defects.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 222 | Rejected: existing synchronization and task machinery uses unsafe; forbidding it cannot remove that trust by itself. |
| `wildcard_enum_match_arm` | 2 | Rejected: grouped states in event-listener; no demonstrated defect. |
| `match_wildcard_for_single_variants` | 1 | Rejected: no demonstrated defect. |
| `cast_possible_truncation` | 1 | Rejected: broadcast position calculation needs separate range proof; no demonstrated defect. |
| `cast_sign_loss` | 0 | Rejected: no witness. |
| `unwrap_used` / `expect_used` | 28 / 9 | Rejected: invariant and poisoned-lock handling require individual proof; no witnessed unsound escape hatch removed. |
| `redundant_clone` / `needless_pass_by_value` | 0 / 0 | Rejected: no witness. |

Adopted 0 of 9 candidates; rejected 9. The profile is not adopted and the audit does not claim all warnings are false positives. No escape hatches were added.

## DSLP-4 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Task cancellation and detach | `crates/async-task/tests/basic.rs:81`, `:200`; `crates/async-task/tests/detach_panic.rs:18` | Retain; cancellation and scheduler panic paths. |
| Executor wake and shutdown | `crates/async-executor/tests/spawn_reentrancy.rs:34`; `crates/async-executor/src/shutdown_test.rs:208` | Retain; wake reentrancy and cancellation during shutdown. |
| Mutex acquisition | `crates/async-lock/tests/mutex.rs:38`, `:58` | Retain; availability and contention. |
| Channel close and wake | `crates/async-channel/tests/bounded.rs:233`, `:249`, `:353`, `:370` | Retain; closed errors, draining, and waking blocked operations. |
| Broadcast admission wake | `crates/async-broadcast/tests/admission_wake.rs:28` | Retain; pending sender notification. |
| Queue close and drain | `crates/concurrent-queue/tests/bounded.rs:123`, `:166` | Retain; result, closed push, drain and force push. |
| Event notification | `crates/event-listener/tests/notify.rs:54`; `crates/event-listener/tests/reentry.rs:29` | Retain; notification and reentrant listener registration. |
| Atomic wake and parking | `crates/atomic-waker/tests/basic.rs:11`; `crates/parking/tests/parking.rs:64` | Retain; registration/wake and cross-thread unpark. |
| Strategy and waker-fn | `crates/event-listener-strategy/src/lib.rs`; `crates/waker-fn/src/lib.rs:38` | No independent boundary test identified in this audit; coverage not established. |

Fault injection: copied `crates/concurrent-queue/src/lib.rs` to `.work/tmp/queue-original`, changed only `Inner::Bounded(q) => q.close()` to `Inner::Bounded(q) => !q.close()`, and ran `cpu-guard --cores 4 -- cargo test -p laputa-concurrent-queue --test bounded close -- --exact`. Exit 101: `close` failed at `crates/concurrent-queue/tests/bounded.rs:128` on `assert!(q.close())`. Restored with `cp .work/tmp/queue-original crates/concurrent-queue/src/lib.rs`; `cmp .work/tmp/queue-original crates/concurrent-queue/src/lib.rs` exited 0.

Needs environment: `cargo-mutants` and `cargo-llvm-cov` are not installed (`command -v` returned 1 for each). No coverage or automated survivor counts are claimed. A hand fault is a single sensitivity check, not a mutation sweep. Existing timing observations and verification tiers are in `AGENTS.md`; a before/after warm-cost comparison was not measured. No tests were changed.

## DSLP-5 · Dead code

The Types command also enabled rustc `dead_code` for default-feature library targets; no scoped dead-code diagnostic was emitted. Search `rg -n 'dead_code|allow\(unused' crates/{async-task,async-executor,async-lock,async-channel,async-broadcast,concurrent-queue,event-listener,event-listener-strategy,atomic-waker,parking,waker-fn}/src` finds explicit allowances. `rg -n 'AssertSync' crates/atomic-waker` finds the local compile-time `Waker: Sync` assertion at `crates/atomic-waker/src/lib.rs:227`; retain it as a compile-time constraint. `rg -n 'into_inner\(' crates/event-listener/src` finds the wrapper at `crates/event-listener/src/lib.rs:1433` and a separate cell call at `crates/event-listener/src/intrusive.rs:339`; the wrapper is conditional on feature configuration, so default-build silence alone is insufficient removal evidence. No cuts were proven; public exports can have downstream consumers (`crates/smol/src/lib.rs:47`, `:54`).

Needs environment: `cargo-machete`, `cargo-udeps`, and `debloat` were not found by `command -v`. Dependency liveness and non-default feature/platform configurations remain unverified; no clean-or-allowlisted claim is made.
