# Runtime I/O, processes and blocking audit

Base `aff562aa90024a4e2219a06ec342e502b46a8969`; Darwin aarch64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. The eight crates and upstream revisions are in `UPSTREAM.toml`. No runtime or public API change is proposed.

All commands below use `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=4`.

## DSLP-6 · Types

Run `cpu-guard --cores 4 -- cargo clippy -p laputa-async-io -p laputa-polling -p laputa-async-net -p laputa-async-fs -p laputa-async-process -p laputa-async-signal -p laputa-blocking -p laputa-piper --lib --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code`: completed successfully (`build-finished: success`). Counts are warning diagnostics attributed to scoped crate files, default features and library targets, not dependencies.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 61 | Rejected: platform and reactor machinery use unsafe; forbidding it does not itself remove that trust. |
| `wildcard_enum_match_arm` | 0 | Rejected: no witness. |
| `match_wildcard_for_single_variants` | 0 | Rejected: no witness. |
| `cast_possible_truncation` | 2 | Rejected: kqueue error conversion and 32-bit sleep counter need range proof; no defect witnessed. |
| `cast_sign_loss` | 0 | Rejected: no witness. |
| `unwrap_used` | 33 | Rejected: invariant and lock handling need individual proof; no escape hatch removed. |
| `expect_used` | 17 | Rejected: no unsound escape hatch witnessed. |
| `redundant_clone` | 0 | Rejected: no witness. |
| `needless_pass_by_value` | 4 | Rejected: no defect witnessed; some signatures are public. |

Adopted 0 of 9 candidates; rejected 9. These diagnostics are leads, not a claim that every instance is harmless. No escape hatches were added.

## DSLP-18 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Reactor readiness and hangup | `crates/async-io/tests/async.rs:63`, `:85`; `crates/async-io/tests/waker_reentry.rs:69` | Retain: peek, hangup and reentrant wake. |
| Poller notification | `crates/polling/tests/notify.rs:10`, `:24` | Retain: notification and concurrent wait. |
| Datagram peek and consumption | `crates/async-net/tests/peek_from.rs:9` | Retain: repeated peek preserves bytes and receive consumes them. |
| File read-ahead and truncation | `crates/async-fs/tests/read_ahead.rs:52`, `:85`, `:96` | Retain: unread suffix, truncation and cancellation. |
| Process status and repeated wait | `crates/async-process/tests/std.rs:140`, `:203` | Retain: success, failure and repeated status. |
| Signal delivery | `crates/async-signal/tests/sigint.rs:13` | Retain: pending stream receives SIGINT. |
| Blocking I/O errors and idle exit | `crates/blocking/tests/write_error.rs:29`, `:57`; `crates/blocking/tests/idle.rs:8` | Retain: errors reach caller and idle thread exits. |
| Pipe close and buffered data | `crates/piper/tests/pipe.rs:111`, `:135`, `:215` | Retain: close and buffered data. Synchronize peer drop before the close assertions. |

The parent reported a macOS CI failure in `dropping_reader_wakes_writer`: a delayed drop left the poll pending. The sibling test also ordered drop by sleep. Channels now order the full write, reader drop and poll, and the writer drop and poll, preserving the `Ready(0)` and `is_closed()` assertions. The other sleeps in this file stage data delivery, not the order of close assertions.

With one concurrent `yes` process, run the following before and after the test change. Replace `before` with `after` for the second run:

```sh
cpu-guard --cores 4 -- sh -c 'yes > /dev/null & load_pid=$!; trap '\''kill "$load_pid" 2>/dev/null; wait "$load_pid" 2>/dev/null'\'' EXIT HUP INT TERM; pass=0; fail=0; for i in $(seq 1 50); do if cargo test -p laputa-piper --test pipe > "$TMPDIR/piper-before-$i.log" 2>&1; then pass=$((pass + 1)); else fail=$((fail + 1)); fi; done; printf "before pass=%s fail=%s\n" "$pass" "$fail"; test "$fail" -eq 0'
```

Both runs exited 0: before 50 passed, after 50 passed. The local baseline did not reproduce the CI failure. An initial run with four `yes` processes exited 124 when `cpu-guard` stopped the saturated process tree before any iteration completed.

Fault injection: copy `crates/async-process/src/reaper/signal.rs` to `.work/tmp/signal-original`; replace both `return Ok(status);` with `return Err(io::Error::other(status.to_string()));`. `cpu-guard --cores 4 -- cargo test -p laputa-async-process --test std test_process_status -- --exact` exited 101: `test_process_status` failed at `crates/async-process/tests/std.rs:142`, unwrapping the injected error. Restore with `cp .work/tmp/signal-original crates/async-process/src/reaper/signal.rs`; `cmp .work/tmp/signal-original crates/async-process/src/reaper/signal.rs` exited 0.

Verification tiers: edit uses the affected integration test; change uses the eight-package test and clippy commands; full uses `make test` and `make lint` as explicitly requested. Existing suite costs are in `AGENTS.md`; warm before/after cost was not measured. No automated mutant survivor or coverage count is claimed.

Needs environment: `command -v cargo-mutants` and `command -v cargo-llvm-cov` returned 1. Coverage and bounded automated mutation remain unverified. Linux epoll, io_uring and pidfd paths were not executed on Darwin.

## DSLP-30 · Dead code

The Types command enabled `dead_code`; no scoped diagnostic was emitted for default-feature libraries. `rg -n 'dead_code|allow\(unused' crates/{async-io,polling,async-net,async-fs,async-process,async-signal,blocking,piper}/src` found two polling allowances. Workspace search `rg -n 'unsupported_error|PollerSealed' . --glob '!Cargo.lock'` finds `unsupported_error` used by `crates/polling/src/poll.rs:452` and `crates/polling/src/port.rs:78`, and `PollerSealed` used by the kqueue extension at `crates/polling/src/os/kqueue.rs:26`. Retain both: the first is platform-dependent, the second seals a public extension. Public facade consumers include `crates/smol/src/lib.rs:48` and `:54`. No deletion was proven.

Needs environment: `command -v cargo-machete`, `command -v cargo-udeps`, `command -v cargo-debloat` and `command -v debloat` returned 1. Dependency liveness and non-default feature and platform configurations remain unverified; no clean-or-allowlisted claim is made.
