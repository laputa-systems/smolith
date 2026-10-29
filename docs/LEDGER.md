# Issue, PR, and Advisory Ledger

**Snapshot Date**: 2026-10-29  
**Monorepo**: `smolith` (`laputa-systems/smolith`)  
**Total Tracked Components**: 26  
**Excluded/Archived Upstreams**: 6  

## Upstream Provenance Inventory

| Component | Published Package | Upstream URL | Branch | Pinned Commit SHA | Upstream Version | License |
|:---|:---|:---|:---|:---|:---|:---|
| `async-broadcast` | `laputa-async-broadcast` | [async-broadcast](https://github.com/smol-rs/async-broadcast) | `master` | `124e9a78a44f` | `0.7.2` | `MIT OR Apache-2.0` |
| `async-channel` | `laputa-async-channel` | [async-channel](https://github.com/smol-rs/async-channel) | `master` | `34e669e05719` | `2.5.0` | `Apache-2.0 OR MIT` |
| `async-dup` | `laputa-async-dup` | [async-dup](https://github.com/smol-rs/async-dup) | `master` | `ff6bad916f6f` | `1.2.4` | `Apache-2.0 OR MIT` |
| `async-executor` | `laputa-async-executor` | [async-executor](https://github.com/smol-rs/async-executor) | `master` | `96a1f6a19ceb` | `1.14.0` | `Apache-2.0 OR MIT` |
| `async-fs` | `laputa-async-fs` | [async-fs](https://github.com/smol-rs/async-fs) | `master` | `8074234d4556` | `2.2.0` | `Apache-2.0 OR MIT` |
| `async-io` | `laputa-async-io` | [async-io](https://github.com/smol-rs/async-io) | `master` | `e94f31c472d6` | `2.6.0` | `Apache-2.0 OR MIT` |
| `async-lock` | `laputa-async-lock` | [async-lock](https://github.com/smol-rs/async-lock) | `master` | `23e181d5cfb9` | `3.4.2` | `Apache-2.0 OR MIT` |
| `async-net` | `laputa-async-net` | [async-net](https://github.com/smol-rs/async-net) | `master` | `b2fea5f9294b` | `2.0.0` | `Apache-2.0 OR MIT` |
| `async-process` | `laputa-async-process` | [async-process](https://github.com/smol-rs/async-process) | `master` | `f4485f156f92` | `2.5.0` | `Apache-2.0 OR MIT` |
| `async-signal` | `laputa-async-signal` | [async-signal](https://github.com/smol-rs/async-signal) | `master` | `79322b0986d7` | `0.2.14` | `Apache-2.0 OR MIT` |
| `async-task` | `laputa-async-task` | [async-task](https://github.com/smol-rs/async-task) | `master` | `f98b30bb0474` | `4.7.1` | `Apache-2.0 OR MIT` |
| `atomic-waker` | `laputa-atomic-waker` | [atomic-waker](https://github.com/smol-rs/atomic-waker) | `master` | `471d0cf3a273` | `1.1.2` | `Apache-2.0 OR MIT` |
| `blocking` | `laputa-blocking` | [blocking](https://github.com/smol-rs/blocking) | `master` | `e29a5e6979f5` | `1.7.0` | `Apache-2.0 OR MIT` |
| `concurrent-queue` | `laputa-concurrent-queue` | [concurrent-queue](https://github.com/smol-rs/concurrent-queue) | `master` | `f41c20d8b942` | `2.5.0` | `Apache-2.0 OR MIT` |
| `easy-parallel` | `laputa-easy-parallel` | [easy-parallel](https://github.com/smol-rs/easy-parallel) | `master` | `cf4a474f7267` | `3.3.1` | `Apache-2.0 OR MIT` |
| `event-listener` | `laputa-event-listener` | [event-listener](https://github.com/smol-rs/event-listener) | `master` | `77004ac77dce` | `5.4.2` | `Apache-2.0 OR MIT` |
| `event-listener-strategy` | `laputa-event-listener-strategy` | [event-listener-strategy](https://github.com/smol-rs/event-listener-strategy) | `main` | `be0f536e7a18` | `0.5.4` | `Apache-2.0 OR MIT` |
| `fastrand` | `laputa-fastrand` | [fastrand](https://github.com/smol-rs/fastrand) | `master` | `7a1cc2c69ebd` | `2.5.0` | `Apache-2.0 OR MIT` |
| `fastrand-contrib` | `laputa-fastrand-contrib` | [fastrand-contrib](https://github.com/smol-rs/fastrand-contrib) | `main` | `5c353ab12a48` | `0.1.0` | `Apache-2.0 OR MIT` |
| `futures-lite` | `laputa-futures-lite` | [futures-lite](https://github.com/smol-rs/futures-lite) | `master` | `3dc587ffdd65` | `2.6.1` | `Apache-2.0 OR MIT` |
| `parking` | `laputa-parking` | [parking](https://github.com/smol-rs/parking) | `master` | `cedb1884e050` | `2.2.1` | `Apache-2.0 OR MIT` |
| `piper` | `laputa-piper` | [piper](https://github.com/smol-rs/piper) | `main` | `ef802bcc97f6` | `0.2.5` | `MIT OR Apache-2.0` |
| `polling` | `laputa-polling` | [polling](https://github.com/smol-rs/polling) | `master` | `35ba7a4fa8bf` | `3.11.0` | `Apache-2.0 OR MIT` |
| `smol` | `laputa-smol` | [smol](https://github.com/smol-rs/smol) | `master` | `4157930176b4` | `2.0.2` | `Apache-2.0 OR MIT` |
| `smol-macros` | `laputa-smol-macros` | [smol-macros](https://github.com/smol-rs/smol-macros) | `main` | `64caeb4ebe46` | `0.1.1` | `Apache-2.0 OR MIT` |
| `waker-fn` | `laputa-waker-fn` | [waker-fn](https://github.com/smol-rs/waker-fn) | `master` | `d44bd05c889e` | `1.2.0` | `Apache-2.0 OR MIT` |

### Upstream Dispositions

| Target | Classification | Disposition & Rational |
|:---|:---|:---|
| `.github` | Excluded / Archived | excluded (organization infrastructure, not a code crate) |
| `async-compat` | Excluded / Archived | excluded (Tokio bridge, outside no-Tokio scope) |
| `async-rustls` | Excluded / Archived | archived upstream (disposition confirmed 2026-10-29) |
| `cache-padded` | Excluded / Archived | archived upstream (disposition confirmed 2026-10-29) |
| `nb-connect` | Excluded / Archived | archived upstream (disposition confirmed 2026-10-29) |
| `vec-arena` | Excluded / Archived | archived upstream (disposition confirmed 2026-10-29) |

## Mandatory Bug Entry Points & S04-S06 Cross-Links

| ID | Issue / Topic | Slice | Component | Severity | Disposition | Local Commit / Evidence |
|:---|:---|:---|:---|:---|:---|:---|
| M01 | [Executor teardown (wake races destruction)](https://github.com/smol-rs/async-executor/issues/163) | **S04** | `laputa-async-executor` | **Critical** | Fixed upstream in pinned commit 96a1f6a19ceb (reverted #146). Verified in S04 with wake-destruction race regression test under TSan/loom. | `ec201de (S01 import), regression test in S04`: TSan heap-use-after-free repro in #163 exercising Waker firing during Executor::drop. |
| M02 | [Task metadata Send/Sync compile-fail bounds](https://github.com/smol-rs/async-task/issues/100) | **S04** | `laputa-async-task` | **High** | Target S04: apply Send/Sync bounds on Metadata to prevent unsound cross-thread access of !Send metadata types. | `Target S04`: Compile-fail test demonstrating !Send metadata leaking across threads without bounds. |
| M03 | [Local-task pinning and Miri UB detection](https://github.com/smol-rs/async-task/issues/102) | **S04** | `laputa-async-task` | **High** | Target S04: reload task state after polling the future; run Miri validation on spawn_local. | `Target S04`: `cargo miri test` on local-task execution passes without stacked borrows / tree borrows violation. |
| M04 | [Task waker vtable identity progress and idle checks](https://github.com/smol-rs/async-task/pull/108) | **S04** | `laputa-async-task` | **Medium** | Target S04: ensure RawWakerVTable identity is consistent across waker clones and comparisons. | `Target S04`: Waker will_wake comparisons and progress/idle assertion test. |
| M05 | [Reactor wakes and tick-snapshot replacement](https://github.com/smol-rs/async-io/issues/78) | **S05** | `laputa-async-io` | **High** | Target S05: validate tick counter snapshotting and wake replacement to avoid missed wakeup events when polling Ready. | `Target S05`: Tick-snapshot replacement test with concurrent poller wakes (issue #220). |
| M06 | [UDP peek loopback on macOS/Darwin](https://github.com/smol-rs/async-net/issues/33) | **S05** | `laputa-async-net` | **Medium** | Target S05: ensure UdpSocket::peek_from handles IPv6 and macOS sockaddr buffers without EINVAL/EFAULT. | `Target S05`: Loopback IPv4/IPv6 peek_from test on macOS and Linux. |
| M07 | [Polling OS contracts (kqueue EV_CLEAR/EINTR, IOCP updates, epoll timerfd)](https://github.com/smol-rs/polling/pull/265) | **S05** | `laputa-polling` | **High** | Target S05: preserve EV_CLEAR on EVFILT_USER notify across EINTR; integrate IOCP update queue fix (#272); avoid timerfd touching on epoll (#250). | `Target S05`: Signal interruption test with concurrent poller wake on kqueue and epoll. |
| M08 | [Blocking thread pool spawn-failure fault injection](https://github.com/smol-rs/blocking/issues/79) | **S06** | `laputa-blocking` | **High** | Target S06: prevent 100% CPU liveloop in grow_pool when OS thread spawn fails persistently. | `Target S06`: Simulated thread exhaustion test verifying executor returns error or throttles instead of spinning. |
| M09 | [Blocking thread idle-retirement and 500ms wakeup elimination](https://github.com/smol-rs/blocking/issues/83) | **S06** | `laputa-blocking` | **Medium** | Target S06: fix condition so idle worker threads properly terminate and stop periodic 500ms wakes when idle. | `Target S06`: Zero wakeups when idle above 1 Hz benchmark gate check. |
| M10 | [Unblock write-error propagation from background writer](https://github.com/smol-rs/blocking/issues/70) | **S06** | `laputa-blocking` | **High** | Target S06: report background writer errors from nonempty writes to prevent infinite write loop. | `Target S06`: Fault-injecting writer test ensuring AsyncWrite returns the I/O error to upper layers. |
| M11 | [Concurrent queue close race with concurrent force_push](https://github.com/smol-rs/concurrent-queue/issues/82) | **S06** | `laputa-concurrent-queue` | **High** | Target S06: ensure bounded queue remains closed when close() interleaves with force_push(). | `Target S06`: Concurrent interleaving stress test modeling close() and force_push() racing. |
| M12 | [Line reader buffer corruption / mid-line error handling](https://github.com/smol-rs/futures-lite/issues/143) | **S06** | `laputa-futures-lite` | **Medium** | Target S06: preserve buffer contents and prevent panic when AsyncBufRead::read_line reader fails mid-line. | `Target S06`: Failing reader test verifying partial buffer state and absence of panic. |
| M13 | [StackSlot unbounded Send/Sync (RUSTSEC-2026-0221)](https://github.com/smol-rs/event-listener/pull/163) | **S06** | `laputa-event-listener` | **High** | Patched in 5.4.2 upstream; S06 adds regression test suite verifying !Send tag types are rejected across thread boundaries. | `ec201de (upstream 5.4.2 import), test in S06`: RUSTSEC-2026-0221 compile-fail and multi-thread tag rejection tests. |

## RustSec Advisory Register

| Advisory ID | Component | Published Package | Severity | Affected Versions | Patched Version | Disposition | Evidence |
|:---|:---|:---|:---|:---|:---|:---|:---|
| [RUSTSEC-2026-0221](https://rustsec.org/advisories/RUSTSEC-2026-0221.html) | `event-listener` | `laputa-event-listener` | High (Unsoundness / Data race) | `>= 5.1.0, < 5.4.2` | `>= 5.4.2` | Pinned import is 5.4.2 (commit `77004ac77dce`). Verified in S06 with regression tests. | `crates/event-listener/tests/listener.rs` compile-fail tag Send test. |
| [RUSTSEC-2021-0145](https://rustsec.org/advisories/RUSTSEC-2021-0145.html) | `concurrent-queue` | `laputa-concurrent-queue` | Low (Unaligned read) | `< 2.0.0` | `>= 2.0.0` | Historical advisory; upstream version is 2.5.0, unaffected. Verified in S06. | Upstream 2.5.0 architecture clean. |

## Component Issue & PR Registers

### `laputa-async-broadcast` (`async-broadcast`)

- **Upstream Repo**: `https://github.com/smol-rs/async-broadcast`
- **Pinned Commit**: `124e9a78a44f7e1fec07e106027cd78a54eb2860`
- **Open Issues**: 5, **Open PRs**: 0

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#80](https://github.com/smol-rs/async-broadcast/issues/80) | Unclear documentation for `InactiveReceiver` and `await_active` | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #80 |
| [#79](https://github.com/smol-rs/async-broadcast/issues/79) | broadcast takes in usize while using NonZeroUsize ergonomics. | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #79 |
| [#75](https://github.com/smol-rs/async-broadcast/issues/75) | Ergonomics: renaming `broadcast*` and `recv*` methods | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #75 |
| [#71](https://github.com/smol-rs/async-broadcast/issues/71) | WASM support | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #71 |
| [#58](https://github.com/smol-rs/async-broadcast/issues/58) | no_std | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #58 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #61](https://github.com/smol-rs/async-broadcast/pull/61) | Fix compiling to wasm | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-async-channel` (`async-channel`)

- **Upstream Repo**: `https://github.com/smol-rs/async-channel`
- **Pinned Commit**: `34e669e0571930913535ee8e50c92f01bce3795c`
- **Open Issues**: 14, **Open PRs**: 2

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#111](https://github.com/smol-rs/async-channel/issues/111) | Document cancel safety guarantees. | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #111 |
| [#110](https://github.com/smol-rs/async-channel/issues/110) | Support Custom Allocator of Channel | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #110 |
| [#99](https://github.com/smol-rs/async-channel/issues/99) | Tests mpmc_stream and mpmc fails on s390x: assertion `left == right` failed | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #99 |
| [#96](https://github.com/smol-rs/async-channel/issues/96) | Sending can yield the thread | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #96 |
| [#87](https://github.com/smol-rs/async-channel/issues/87) | Sending cancel safety | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #87 |
| [#85](https://github.com/smol-rs/async-channel/issues/85) | Add MIRI to CI | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #85 |
| [#82](https://github.com/smol-rs/async-channel/issues/82) | `Recv` can lock on dropping | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #82 |
| [#78](https://github.com/smol-rs/async-channel/issues/78) | mpmc and mpmc_stream tests fail once and then never again | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #78 |
| [#77](https://github.com/smol-rs/async-channel/issues/77) | Allow blocking channels in wasm | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #77 |
| [#58](https://github.com/smol-rs/async-channel/issues/58) | Add zero capacity bounded channel | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #58 |
| [#57](https://github.com/smol-rs/async-channel/issues/57) | Document why we notify all streams when sending a single message | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #57 |
| [#23](https://github.com/smol-rs/async-channel/issues/23) | Drain channel immediately after last receiver is dropped | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #23 |
| [#12](https://github.com/smol-rs/async-channel/issues/12) | Implement Sink for Sender | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #12 |
| [#6](https://github.com/smol-rs/async-channel/issues/6) | Document channel (non)-fairness | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #6 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#112](https://github.com/smol-rs/async-channel/pull/112) | Add cancel safety guarantees to `recv` and `send`. | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #112 |
| [#83](https://github.com/smol-rs/async-channel/pull/83) | added `Receiver::recv_owned` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #83 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #107](https://github.com/smol-rs/async-channel/pull/107) | Fix description of `recv_blocking` | Merged PR | Included in pinned SHA | Local test suite |
| [PR #103](https://github.com/smol-rs/async-channel/pull/103) | ci: Use reusable workflows for fmt and security_audit & fix CI failure | Merged PR | Included in pinned SHA | Local test suite |
| [PR #100](https://github.com/smol-rs/async-channel/pull/100) | Fix grammar in description of `WeakSender` and `WeakReceiver` | Merged PR | Included in pinned SHA | Local test suite |
| [PR #84](https://github.com/smol-rs/async-channel/pull/84) | Fix CI badge in `README.md` | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #104](https://github.com/smol-rs/async-channel/issues/104) | recv panicked at core/src/panicking.rs:223 in Rust 1.84.0 | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-async-dup` (`async-dup`)

- **Upstream Repo**: `https://github.com/smol-rs/async-dup`
- **Pinned Commit**: `ff6bad916f6f96a3d822b59477c5212c7ac9a548`
- **Open Issues**: 1, **Open PRs**: 0

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#13](https://github.com/smol-rs/async-dup/issues/13) | Replace futures with futures-lite in examples | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #13 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #3](https://github.com/smol-rs/async-dup/pull/3) | Fix CI | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-async-executor` (`async-executor`)

- **Upstream Repo**: `https://github.com/smol-rs/async-executor`
- **Pinned Commit**: `96a1f6a19ceb828c8eb8a10cb4b40daefdbdb945`
- **Open Issues**: 12, **Open PRs**: 5

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#158](https://github.com/smol-rs/async-executor/issues/158) | try_tick does not reschedule tasks | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #158 |
| [#156](https://github.com/smol-rs/async-executor/issues/156) | Example for multi-threaded priority executor which doesn't run forever | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #156 |
| [#153](https://github.com/smol-rs/async-executor/issues/153) | Tokio console support | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #153 |
| [#140](https://github.com/smol-rs/async-executor/issues/140) | Is the `StaticExecutor` not updating the `Task`'s reference count correctly? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #140 |
| [#139](https://github.com/smol-rs/async-executor/issues/139) | Feature request: Support `task_local!` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #139 |
| [#119](https://github.com/smol-rs/async-executor/issues/119) | StaticLocalExecutor usage example? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #119 |
| [#100](https://github.com/smol-rs/async-executor/issues/100) | Wake up threads more aggressively if there is known work to do | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #100 |
| [#88](https://github.com/smol-rs/async-executor/issues/88) | try_tick considered harmful | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #88 |
| [#84](https://github.com/smol-rs/async-executor/issues/84) | Allow creating tasks with metadata | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #84 |
| [#64](https://github.com/smol-rs/async-executor/issues/64) | "Next task" optimization | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #64 |
| [#62](https://github.com/smol-rs/async-executor/issues/62) | Push task directly to the local queue | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #62 |
| [#32](https://github.com/smol-rs/async-executor/issues/32) | Consider using st3 for work-stealing | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #32 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#161](https://github.com/smol-rs/async-executor/pull/161) | feat: add minimal scaffolding for `std` feature | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #161 |
| [#157](https://github.com/smol-rs/async-executor/pull/157) | scoped execution | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #157 |
| [#149](https://github.com/smol-rs/async-executor/pull/149) | Directly schedule tasks instead of using Runnable::schedule | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #149 |
| [#144](https://github.com/smol-rs/async-executor/pull/144) | Attempt to optimize LocalExecutor | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #144 |
| [#115](https://github.com/smol-rs/async-executor/pull/115) | Replace the local worker queues with st3's | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #115 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #164](https://github.com/smol-rs/async-executor/pull/164) | Revert "Pin an Executor's State to minimize atomic operations. (#146)" | Merged PR | Included in pinned SHA | Local test suite |
| [PR #152](https://github.com/smol-rs/async-executor/pull/152) | Fix failing docs build | Merged PR | Included in pinned SHA | Local test suite |
| [PR #147](https://github.com/smol-rs/async-executor/pull/147) | Don't explicitly panic or block when avoidable | Merged PR | Included in pinned SHA | Local test suite |
| [PR #143](https://github.com/smol-rs/async-executor/pull/143) | Fix newly added clippy warnings | Merged PR | Included in pinned SHA | Local test suite |
| [PR #128](https://github.com/smol-rs/async-executor/pull/128) | Fix build badge | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #135](https://github.com/smol-rs/async-executor/issues/135) | Executor is unusable after `spawn_many` iterator panics | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #111](https://github.com/smol-rs/async-executor/issues/111) | A leaked Executor does not need to track active tasks | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #99](https://github.com/smol-rs/async-executor/issues/99) | Tick counter can overflow on debug mode | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #89](https://github.com/smol-rs/async-executor/issues/89) | Panic on multithreaded browser Wasm | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #54](https://github.com/smol-rs/async-executor/issues/54) | Panic with version 1.5.3 | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-async-fs` (`async-fs`)

- **Upstream Repo**: `https://github.com/smol-rs/async-fs`
- **Pinned Commit**: `8074234d4556620a084a1cc3e5575f15143cf950`
- **Open Issues**: 6, **Open PRs**: 4

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#58](https://github.com/smol-rs/async-fs/issues/58) | Add `File::create_new` associated function | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #58 |
| [#49](https://github.com/smol-rs/async-fs/issues/49) | into_inner methods to access sync file | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #49 |
| [#41](https://github.com/smol-rs/async-fs/issues/41) | Add "exists" operation | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #41 |
| [#31](https://github.com/smol-rs/async-fs/issues/31) | Web support | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #31 |
| [#24](https://github.com/smol-rs/async-fs/issues/24) | Conditionally use more efficient strategies for polling file I/O | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #24 |
| [#7](https://github.com/smol-rs/async-fs/issues/7) | Support OS-specific FileExt traits. | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #7 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#59](https://github.com/smol-rs/async-fs/pull/59) | Add File::create_new for atomic file creation | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #59 |
| [#55](https://github.com/smol-rs/async-fs/pull/55) | Implement conversion to inner blocking file | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #55 |
| [#54](https://github.com/smol-rs/async-fs/pull/54) | Add `exists` function | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #54 |
| [#43](https://github.com/smol-rs/async-fs/pull/43) | Add try_exists function as wrapper over std::path::Path::try_exists | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #43 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #57](https://github.com/smol-rs/async-fs/pull/57) | Fix documentation of `DirEntry::metadata` | Merged PR | Included in pinned SHA | Local test suite |
| [PR #52](https://github.com/smol-rs/async-fs/pull/52) | Fix documentation issues for src/lib.rs | Merged PR | Included in pinned SHA | Local test suite |
| [PR #39](https://github.com/smol-rs/async-fs/pull/39) | chore: Fix typo in docs | Merged PR | Included in pinned SHA | Local test suite |
| [PR #35](https://github.com/smol-rs/async-fs/pull/35) | fix(docs): `create_dir_all` doesn't error if path already exists | Merged PR | Included in pinned SHA | Local test suite |
| [PR #33](https://github.com/smol-rs/async-fs/pull/33) | Fix minor documentation glitch | Merged PR | Included in pinned SHA | Local test suite |
| [PR #32](https://github.com/smol-rs/async-fs/pull/32) | chore: fix build status badge | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-async-io` (`async-io`)

- **Upstream Repo**: `https://github.com/smol-rs/async-io`
- **Pinned Commit**: `e94f31c472d61101dac0aaa67c4fb42068bd8071`
- **Open Issues**: 12, **Open PRs**: 7

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#253](https://github.com/smol-rs/async-io/issues/253) | CirrusCI will shut down on 2026-06-01 | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #253 |
| [#241](https://github.com/smol-rs/async-io/issues/241) | A yield_now variant | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #241 |
| [#213](https://github.com/smol-rs/async-io/issues/213) | Support IoSafe on VsockStream, VsockListener | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #213 |
| [#190](https://github.com/smol-rs/async-io/issues/190) | Benchmark and configurable pool | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #190 |
| [#89](https://github.com/smol-rs/async-io/issues/89) | Enable usage of timers on WASM | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #89 |
| [#78](https://github.com/smol-rs/async-io/issues/78) | Missing waker.wake events | High | Mandatory target (S05) | S05 | Tick-snapshot replacement test with concurrent poller wakes (issue #220). |
| [#63](https://github.com/smol-rs/async-io/issues/63) | `Timer` is paused on Android in "doze" state | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #63 |
| [#62](https://github.com/smol-rs/async-io/issues/62) | avoid unnecessary poll `source.ready().await` when events happen immediately | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #62 |
| [#40](https://github.com/smol-rs/async-io/issues/40) | Avoid spawning the async-io thread if possible | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #40 |
| [#39](https://github.com/smol-rs/async-io/issues/39) | io_uring support | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #39 |
| [#17](https://github.com/smol-rs/async-io/issues/17) | Use NT reserve objects | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #17 |
| [#14](https://github.com/smol-rs/async-io/issues/14) | Support for Handles on Windows | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #14 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#255](https://github.com/smol-rs/async-io/pull/255) | ci: test on FreeBSD, NetBSD and OpenBSD without Cirrus | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #255 |
| [#254](https://github.com/smol-rs/async-io/pull/254) | feat: add yield_now to drive reactor from sync code | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #254 |
| [#248](https://github.com/smol-rs/async-io/pull/248) | Update getrandom requirement from 0.3 to 0.4 | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #248 |
| [#247](https://github.com/smol-rs/async-io/pull/247) | Update signal-hook requirement from 0.3 to 0.4 | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #247 |
| [#245](https://github.com/smol-rs/async-io/pull/245) | Update criterion requirement from 0.7 to 0.8 | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #245 |
| [#156](https://github.com/smol-rs/async-io/pull/156) | fix very slow poll_writable() | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #156 |
| [#79](https://github.com/smol-rs/async-io/pull/79) | fix: missing waker.wake | High | Mandatory target (S05) | S05 | Tick-snapshot replacement test with concurrent poller wakes (issue #220). |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #252](https://github.com/smol-rs/async-io/pull/252) | Revert "ci: Install ca_root_nss for FreeBSD CI" | Merged PR | Included in pinned SHA | Local test suite |
| [PR #251](https://github.com/smol-rs/async-io/pull/251) | ci: Fix broken tests | Merged PR | Included in pinned SHA | Local test suite |
| [PR #234](https://github.com/smol-rs/async-io/pull/234) | Make tracing optional & fix CI failure & update CI config | Merged PR | Included in pinned SHA | Local test suite |
| [PR #229](https://github.com/smol-rs/async-io/pull/229) | nit: Fix comment in ReactorLock::react | Merged PR | Included in pinned SHA | Local test suite |
| [PR #228](https://github.com/smol-rs/async-io/pull/228) | Revert "Temporarily disable openBSD in testing" | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #205](https://github.com/smol-rs/async-io/issues/205) | Race of Sloths: The Open Source Developer Challenge is about to start | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-async-lock` (`async-lock`)

- **Upstream Repo**: `https://github.com/smol-rs/async-lock`
- **Pinned Commit**: `23e181d5cfb91a59382e5719c682caa58ff43ace`
- **Open Issues**: 6, **Open PRs**: 2

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#100](https://github.com/smol-rs/async-lock/issues/100) | Document `RwLockWriteGuard::downgrade()` guarantees | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #100 |
| [#91](https://github.com/smol-rs/async-lock/issues/91) | RwLock.write() and Mutex.lock() hang forever | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #91 |
| [#84](https://github.com/smol-rs/async-lock/issues/84) | Sporadic panic in async-io thread with esp-sys-svc async TLS example | High | Candidate soundness fix | Evaluate in S04-S06 | Tracker #84 |
| [#65](https://github.com/smol-rs/async-lock/issues/65) | feature request：support cancellable RwLock APIs | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #65 |
| [#24](https://github.com/smol-rs/async-lock/issues/24) | Support acquire_many (and variants) in Semaphore | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #24 |
| [#23](https://github.com/smol-rs/async-lock/issues/23) | Fairness of semaphore | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #23 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#111](https://github.com/smol-rs/async-lock/pull/111) | docs: explain RwLock write guard downgrade guarantees | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #111 |
| [#108](https://github.com/smol-rs/async-lock/pull/108) | semaphore: Add set_size() method | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #108 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #102](https://github.com/smol-rs/async-lock/pull/102) | Fix build badge | Merged PR | Included in pinned SHA | Local test suite |
| [PR #96](https://github.com/smol-rs/async-lock/pull/96) | ci: Use reusable workflows for fmt and security_audit & fix CI failure | Merged PR | Included in pinned SHA | Local test suite |
| [PR #92](https://github.com/smol-rs/async-lock/pull/92) | chore: Fix lints | Merged PR | Included in pinned SHA | Local test suite |
| [PR #89](https://github.com/smol-rs/async-lock/pull/89) | chore: Fix typos | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #90](https://github.com/smol-rs/async-lock/issues/90) | Fix unreachable patterns | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-async-net` (`async-net`)

- **Upstream Repo**: `https://github.com/smol-rs/async-net`
- **Pinned Commit**: `b2fea5f9294be19abe108a44a93c4e1b4c07bf9f`
- **Open Issues**: 6, **Open PRs**: 1

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#38](https://github.com/smol-rs/async-net/issues/38) | Why Iterator<Item = SocketAddr> from ToSocketAddrsFuture can't be Sync? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #38 |
| [#37](https://github.com/smol-rs/async-net/issues/37) | Making the `blocking` dependency optional | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #37 |
| [#34](https://github.com/smol-rs/async-net/issues/34) | Idempotence TCP shutdown, closing, and dropping | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #34 |
| [#33](https://github.com/smol-rs/async-net/issues/33) | `UdpSocket::peek_from` is returning weird error on IPv6 on MacOS | Medium | Mandatory target (S05) | S05 | Loopback IPv4/IPv6 peek_from test on macOS and Linux. |
| [#20](https://github.com/smol-rs/async-net/issues/20) | Provide a `TcpSocket` abstraction | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #20 |
| [#3](https://github.com/smol-rs/async-net/issues/3) | Peer credentials API | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #3 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#35](https://github.com/smol-rs/async-net/pull/35) | fix peek_from for UdpSocket | Medium | Mandatory target (S05) | S05 | Loopback IPv4/IPv6 peek_from test on macOS and Linux. |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #12](https://github.com/smol-rs/async-net/pull/12) | Fix clippy::from_over_into lint | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-async-process` (`async-process`)

- **Upstream Repo**: `https://github.com/smol-rs/async-process`
- **Pinned Commit**: `f4485f156f9294b86a5be37f7236bcf0cf93c76b`
- **Open Issues**: 5, **Open PRs**: 2

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#108](https://github.com/smol-rs/async-process/issues/108) | `Child::output` should have a version that takes a `&mut Child` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #108 |
| [#50](https://github.com/smol-rs/async-process/issues/50) | How does this compare to Tokio's Command? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #50 |
| [#49](https://github.com/smol-rs/async-process/issues/49) | Use a more efficient strategy for waiting on processes | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #49 |
| [#35](https://github.com/smol-rs/async-process/issues/35) | `From<std::process::Command>` ignores pre-configured values for `stdin`/`stdout`/`stderr` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #35 |
| [#22](https://github.com/smol-rs/async-process/issues/22) | `ChildStdin`, et cetera should implement `AsRawFd/AsRawHandle` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #22 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#91](https://github.com/smol-rs/async-process/pull/91) | Freebsd kqueue waitable child | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #91 |
| [#19](https://github.com/smol-rs/async-process/pull/19) | allow waiting on process exit without requiring a mutable borrow and without dropping stdin | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #19 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #109](https://github.com/smol-rs/async-process/pull/109) | Bump MSRV to 1.85 & Fix CI failure | Merged PR | Included in pinned SHA | Local test suite |
| [PR #105](https://github.com/smol-rs/async-process/pull/105) | Fixed CommandExt uid, gid argument type mismatch for target_os = nto | Merged PR | Included in pinned SHA | Local test suite |
| [PR #101](https://github.com/smol-rs/async-process/pull/101) | Make tracing optional & fix CI failure & update CI config | Merged PR | Included in pinned SHA | Local test suite |
| [PR #98](https://github.com/smol-rs/async-process/pull/98) | Fix build badge | Merged PR | Included in pinned SHA | Local test suite |
| [PR #96](https://github.com/smol-rs/async-process/pull/96) | Fix target-specific feature flags for rustix dependency | Merged PR | Included in pinned SHA | Local test suite |
| [PR #93](https://github.com/smol-rs/async-process/pull/93) | ci: Use reusable workflows for fmt and security_audit & fix CI failure | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-async-signal` (`async-signal`)

- **Upstream Repo**: `https://github.com/smol-rs/async-signal`
- **Pinned Commit**: `79322b0986d757225fb8201c6d007f0c77fda720`
- **Open Issues**: 1, **Open PRs**: 0

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#31](https://github.com/smol-rs/async-signal/issues/31) | Stabilize this crate | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #31 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #61](https://github.com/smol-rs/async-signal/pull/61) | Fix build badge | Merged PR | Included in pinned SHA | Local test suite |
| [PR #59](https://github.com/smol-rs/async-signal/pull/59) | Fix build error on haiku | Merged PR | Included in pinned SHA | Local test suite |
| [PR #52](https://github.com/smol-rs/async-signal/pull/52) | Fix CI failure & update CI config | Merged PR | Included in pinned SHA | Local test suite |
| [PR #39](https://github.com/smol-rs/async-signal/pull/39) | fix: clippy std::usize::MAX to usize::MAX | Merged PR | Included in pinned SHA | Local test suite |
| [PR #38](https://github.com/smol-rs/async-signal/pull/38) | docs: fix clippy lint in readme example | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-async-task` (`async-task`)

- **Upstream Repo**: `https://github.com/smol-rs/async-task`
- **Pinned Commit**: `f98b30bb047426104c23b81e1c2fa083b177a567`
- **Open Issues**: 11, **Open PRs**: 10

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#103](https://github.com/smol-rs/async-task/issues/103) | New crates.io release | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #103 |
| [#102](https://github.com/smol-rs/async-task/issues/102) | `miri` detect UB for local task. | High | Mandatory target (S04) | S04 | `cargo miri test` on local-task execution passes without stacked borrows / tree borrows violation. |
| [#100](https://github.com/smol-rs/async-task/issues/100) | Fix Send and Sync bounds for M to avoiding unsound behavior. | High | Mandatory target (S04) | S04 | Compile-fail test demonstrating !Send metadata leaking across threads without bounds. |
| [#92](https://github.com/smol-rs/async-task/issues/92) | Non-blocking non-async cancel | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #92 |
| [#77](https://github.com/smol-rs/async-task/issues/77) | overgeneral documentation | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #77 |
| [#76](https://github.com/smol-rs/async-task/issues/76) | `Builder::spawn` should encode the metadata lifetime contract | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #76 |
| [#53](https://github.com/smol-rs/async-task/issues/53) | fails to test examples without feature std | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #53 |
| [#24](https://github.com/smol-rs/async-task/issues/24) | Custom allocators | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #24 |
| [#22](https://github.com/smol-rs/async-task/issues/22) | Task not get rescheduled randomly | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #22 |
| [#7](https://github.com/smol-rs/async-task/issues/7) | Support for statically allocating tasks (no_alloc usage) | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #7 |
| [#2](https://github.com/smol-rs/async-task/issues/2) | Run `poll_drop` destructors when invoking `Task::cancel` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #2 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#110](https://github.com/smol-rs/async-task/pull/110) | Require std for the spawn-local example | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #110 |
| [#109](https://github.com/smol-rs/async-task/pull/109) | Reload task state after polling the future | High | Mandatory target (S04) | S04 | `cargo miri test` on local-task execution passes without stacked borrows / tree borrows violation. |
| [#108](https://github.com/smol-rs/async-task/pull/108) | Fix task waker vtable identity | Medium | Mandatory target (S04) | S04 | Waker will_wake comparisons and progress/idle assertion test. |
| [#106](https://github.com/smol-rs/async-task/pull/106) | Add `Runnable::is_cancelled` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #106 |
| [#101](https://github.com/smol-rs/async-task/pull/101) | Fix: Make Metadata Send to avoid unsound behavior | High | Mandatory target (S04) | S04 | Compile-fail test demonstrating !Send metadata leaking across threads without bounds. |
| [#98](https://github.com/smol-rs/async-task/pull/98) | Add custom allocator support | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #98 |
| [#97](https://github.com/smol-rs/async-task/pull/97) | Add raw metadata accessor | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #97 |
| [#90](https://github.com/smol-rs/async-task/pull/90) | Require non-closure schedule functions | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #90 |
| [#85](https://github.com/smol-rs/async-task/pull/85) | Miri supports epoll nowadays | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #85 |
| [#79](https://github.com/smol-rs/async-task/pull/79) | Add SAFETY comments and notes for `unsafe` blocks | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #79 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #107](https://github.com/smol-rs/async-task/pull/107) | Fix typo in comment about task detachment | Merged PR | Included in pinned SHA | Local test suite |
| [PR #105](https://github.com/smol-rs/async-task/pull/105) | Fix misspelling inside the comment | Merged PR | Included in pinned SHA | Local test suite |
| [PR #93](https://github.com/smol-rs/async-task/pull/93) | Fix typos | Merged PR | Included in pinned SHA | Local test suite |
| [PR #91](https://github.com/smol-rs/async-task/pull/91) | Fix doctest for Valgrind | Merged PR | Included in pinned SHA | Local test suite |
| [PR #86](https://github.com/smol-rs/async-task/pull/86) | ci: Use reusable workflows for fmt and security_audit & fix CI failure | Merged PR | Included in pinned SHA | Local test suite |
| [PR #82](https://github.com/smol-rs/async-task/pull/82) | Fix build badge | Merged PR | Included in pinned SHA | Local test suite |
| [PR #81](https://github.com/smol-rs/async-task/pull/81) | chore: Fix typo | Merged PR | Included in pinned SHA | Local test suite |
| [PR #73](https://github.com/smol-rs/async-task/pull/73) | Improve panic message for when a task is polled after completion | Merged PR | Included in pinned SHA | Local test suite |
| [PR #72](https://github.com/smol-rs/async-task/pull/72) | Always set #![no_std] to fix redundant import warning | Merged PR | Included in pinned SHA | Local test suite |
| [PR #69](https://github.com/smol-rs/async-task/pull/69) | Fix CI failure & Migrate to Rust 2021 | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #89](https://github.com/smol-rs/async-task/issues/89) | Maybe Multiple unsoundness problems | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #36](https://github.com/smol-rs/async-task/issues/36) | Bug: Unit test with expected panic message randomly fails because of crate-specific panic message | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #20](https://github.com/smol-rs/async-task/issues/20) | Option to avoid panicking when polling a cancelled task | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-atomic-waker` (`atomic-waker`)

- **Upstream Repo**: `https://github.com/smol-rs/atomic-waker`
- **Pinned Commit**: `471d0cf3a2734e62d1eb21a1372fd83e8fbf90d8`
- **Open Issues**: 1, **Open PRs**: 2

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#19](https://github.com/smol-rs/atomic-waker/issues/19) | The memory order `Acquire` in `self.state.swap(WAITING, AcqRel);` is not necessary | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #19 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#29](https://github.com/smol-rs/atomic-waker/pull/29) | Implement UnwindSafe and RefUnwindSafe | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #29 |
| [#1](https://github.com/smol-rs/atomic-waker/pull/1) | Add spin_loop_hint in `register` spinlock | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #1 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #23](https://github.com/smol-rs/atomic-waker/pull/23) | Fix build badge | Merged PR | Included in pinned SHA | Local test suite |
| [PR #22](https://github.com/smol-rs/atomic-waker/pull/22) | ci: Use reusable workflows for fmt and security_audit & fix CI failure | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-blocking` (`blocking`)

- **Upstream Repo**: `https://github.com/smol-rs/blocking`
- **Pinned Commit**: `e29a5e6979f56c7537b04ab17ca7bdceee66ef5f`
- **Open Issues**: 9, **Open PRs**: 2

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#83](https://github.com/smol-rs/blocking/issues/83) | Idle threads never exit since 1.7.0 and wake every 500 ms | Medium | Mandatory target (S06) | S06 | Zero wakeups when idle above 1 Hz benchmark gate check. |
| [#81](https://github.com/smol-rs/blocking/issues/81) | Erroneous condition when stopping threads | Medium | Mandatory target (S06) | S06 | Zero wakeups when idle above 1 Hz benchmark gate check. |
| [#79](https://github.com/smol-rs/blocking/issues/79) | `Executor::grow_pool` livelocks at 100 % CPU when `thread::Builder::spawn` fails persistently | High | Mandatory target (S06) | S06 | Simulated thread exhaustion test verifying executor returns error or throttles instead of spinning. |
| [#77](https://github.com/smol-rs/blocking/issues/77) | Is unblock runtime agnostic? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #77 |
| [#76](https://github.com/smol-rs/blocking/issues/76) | Should clarify the behavior when dropping a Task handle | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #76 |
| [#70](https://github.com/smol-rs/blocking/issues/70) | `poll_write` May Cause an Infinite Loop in the Upper Layer | High | Mandatory target (S06) | S06 | Fault-injecting writer test ensuring AsyncWrite returns the I/O error to upper layers. |
| [#68](https://github.com/smol-rs/blocking/issues/68) | Difference with `tokio::task::spawn_blocking`? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #68 |
| [#65](https://github.com/smol-rs/blocking/issues/65) | Not working with TcpStream | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #65 |
| [#19](https://github.com/smol-rs/blocking/issues/19) | [Discussion] A more ergonomic way to deal with 'blocking' futures | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #19 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#82](https://github.com/smol-rs/blocking/pull/82) | Report background writer errors from nonempty Unblock writes | High | Mandatory target (S06) | S06 | Fault-injecting writer test ensuring AsyncWrite returns the I/O error to upper layers. |
| [#80](https://github.com/smol-rs/blocking/pull/80) | Stop growing the pool when thread creation fails | High | Mandatory target (S06) | S06 | Simulated thread exhaustion test verifying executor returns error or throttles instead of spinning. |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #71](https://github.com/smol-rs/blocking/pull/71) | Fix newly added clippy warning and build failure with minimal-versions | Merged PR | Included in pinned SHA | Local test suite |
| [PR #58](https://github.com/smol-rs/blocking/pull/58) | Propogate panics that occur in unblocked functions | Merged PR | Included in pinned SHA | Local test suite |
| [PR #57](https://github.com/smol-rs/blocking/pull/57) | Fix redundant import warning | Merged PR | Included in pinned SHA | Local test suite |
| [PR #54](https://github.com/smol-rs/blocking/pull/54) | Fix compilation on wasm32 | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-concurrent-queue` (`concurrent-queue`)

- **Upstream Repo**: `https://github.com/smol-rs/concurrent-queue`
- **Pinned Commit**: `f41c20d8b9424eff6c227dead7b062e3b87142bc`
- **Open Issues**: 5, **Open PRs**: 1

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#82](https://github.com/smol-rs/concurrent-queue/issues/82) | Bounded queue can end up open after `close()` if `force_push` is running concurrently | High | Mandatory target (S06) | S06 | Concurrent interleaving stress test modeling close() and force_push() racing. |
| [#81](https://github.com/smol-rs/concurrent-queue/issues/81) | Adaptive backoff in CAS retry and wait loops | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #81 |
| [#69](https://github.com/smol-rs/concurrent-queue/issues/69) | Investigate ways to make `steal` from `async-executor` more efficient | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #69 |
| [#63](https://github.com/smol-rs/concurrent-queue/issues/63) | Improve documentation of internals of `bounded` implementation | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #63 |
| [#50](https://github.com/smol-rs/concurrent-queue/issues/50) | Merge implementation with crossbeam-queue | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #50 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#12](https://github.com/smol-rs/concurrent-queue/pull/12) | bounded: make sure lap does not overflow into index | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #12 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #76](https://github.com/smol-rs/concurrent-queue/pull/76) | chore: fixup workflow's badge | Merged PR | Included in pinned SHA | Local test suite |
| [PR #75](https://github.com/smol-rs/concurrent-queue/pull/75) | chore: fix spurious clippy warning | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-easy-parallel` (`easy-parallel`)

- **Upstream Repo**: `https://github.com/smol-rs/easy-parallel`
- **Pinned Commit**: `cf4a474f7267a0363af0e3b949c0260b86da1ee0`
- **Open Issues**: 0, **Open PRs**: 0

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #13](https://github.com/smol-rs/easy-parallel/pull/13) | Fix clippy::useless_vec warning in test | Merged PR | Included in pinned SHA | Local test suite |
| [PR #2](https://github.com/smol-rs/easy-parallel/pull/2) | Fix CI | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-event-listener` (`event-listener`)

- **Upstream Repo**: `https://github.com/smol-rs/event-listener`
- **Pinned Commit**: `77004ac77dcef7884bf71a99c276a50f83a7fa4e`
- **Open Issues**: 4, **Open PRs**: 2

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#167](https://github.com/smol-rs/event-listener/issues/167) | Website in Readme and documentation no longer valid | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #167 |
| [#159](https://github.com/smol-rs/event-listener/issues/159) | Send singular message without `Clone` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #159 |
| [#124](https://github.com/smol-rs/event-listener/issues/124) | Make the heapless API less prone to accidentally panicking | High | Candidate soundness fix | Evaluate in S04-S06 | Tracker #124 |
| [#43](https://github.com/smol-rs/event-listener/issues/43) | Code coverage | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #43 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#164](https://github.com/smol-rs/event-listener/pull/164) | feat: Re-enable `no_std` support for tags | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #164 |
| [#154](https://github.com/smol-rs/event-listener/pull/154) | feat: Add queuing strategies for listener lists. | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #154 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #163](https://github.com/smol-rs/event-listener/pull/163) | Fix unbounded Send/Sync implementations on StackSlot | Merged PR | Included in pinned SHA | Local test suite |
| [PR #161](https://github.com/smol-rs/event-listener/pull/161) | Fix CI failure and update CI config | Merged PR | Included in pinned SHA | Local test suite |
| [PR #155](https://github.com/smol-rs/event-listener/pull/155) | ci: Re-enable WASM test, fix newly added clippy warning | Merged PR | Included in pinned SHA | Local test suite |
| [PR #146](https://github.com/smol-rs/event-listener/pull/146) | Fix latest CI | Merged PR | Included in pinned SHA | Local test suite |
| [PR #145](https://github.com/smol-rs/event-listener/pull/145) | Fix missed notification in Mutex example | Merged PR | Included in pinned SHA | Local test suite |
| [PR #136](https://github.com/smol-rs/event-listener/pull/136) | Use Cargo.toml lint fix instead of build.rs | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #138](https://github.com/smol-rs/event-listener/issues/138) | During racy initialization, a race can happen where a notification is dropped | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #116](https://github.com/smol-rs/event-listener/issues/116) | Possible racecondition | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #100](https://github.com/smol-rs/event-listener/issues/100) | Unsoundness | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-event-listener-strategy` (`event-listener-strategy`)

- **Upstream Repo**: `https://github.com/smol-rs/event-listener-strategy`
- **Pinned Commit**: `be0f536e7a18776978b03a6079eb823ee3aa9b34`
- **Open Issues**: 1, **Open PRs**: 0

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#8](https://github.com/smol-rs/event-listener-strategy/issues/8) | GAT nature of `Strategy::Future` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #8 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #29](https://github.com/smol-rs/event-listener-strategy/pull/29) | Fix doctest on WASM | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-fastrand` (`fastrand`)

- **Upstream Repo**: `https://github.com/smol-rs/fastrand`
- **Pinned Commit**: `7a1cc2c69ebde6cb3442809213db3fe19a2cb0ba`
- **Open Issues**: 7, **Open PRs**: 1

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#130](https://github.com/smol-rs/fastrand/issues/130) | Global RNG: seed with a slightly more unique identifier? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #130 |
| [#128](https://github.com/smol-rs/fastrand/issues/128) | WyRand fails collisions tests—maybe switching to a generator with more state? | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #128 |
| [#123](https://github.com/smol-rs/fastrand/issues/123) | w1rand | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #123 |
| [#101](https://github.com/smol-rs/fastrand/issues/101) | Auto-vectorization | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #101 |
| [#40](https://github.com/smol-rs/fastrand/issues/40) | matklad's dream fastrand | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #40 |
| [#20](https://github.com/smol-rs/fastrand/issues/20) | 1.5.0 changes random values with seeds | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #20 |
| [#13](https://github.com/smol-rs/fastrand/issues/13) | Performance optimizations | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #13 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#129](https://github.com/smol-rs/fastrand/pull/129) | Replace WyRand with MWC192X, a scrambled multiply-with-carry generator | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #129 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #125](https://github.com/smol-rs/fastrand/pull/125) | Fix build failure with js feature | Merged PR | Included in pinned SHA | Local test suite |
| [PR #119](https://github.com/smol-rs/fastrand/pull/119) | chore: fix grammatical errors in documentation comments | Merged PR | Included in pinned SHA | Local test suite |
| [PR #111](https://github.com/smol-rs/fastrand/pull/111) | Remove manual doc(cfg(..)) & Fix rustdoc::broken_intra_doc_links warning | Merged PR | Included in pinned SHA | Local test suite |
| [PR #98](https://github.com/smol-rs/fastrand/pull/98) | ci: Use reusable workflows for fmt and security_audit & fix CI failure | Merged PR | Included in pinned SHA | Local test suite |
| [PR #97](https://github.com/smol-rs/fastrand/pull/97) | Revert "Bump resolver to version 2 to fix web CI" | Merged PR | Included in pinned SHA | Local test suite |
| [PR #96](https://github.com/smol-rs/fastrand/pull/96) | Bump resolver to version 2 to fix web CI | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #84](https://github.com/smol-rs/fastrand/issues/84) | Windows MSVC compiler sometimes panics. | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-fastrand-contrib` (`fastrand-contrib`)

- **Upstream Repo**: `https://github.com/smol-rs/fastrand-contrib`
- **Pinned Commit**: `5c353ab12a48f6484913fb345a81bbc75ac367e6`
- **Open Issues**: 1, **Open PRs**: 0

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#19](https://github.com/smol-rs/fastrand-contrib/issues/19) | add choice_weighted? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #19 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #16](https://github.com/smol-rs/fastrand-contrib/pull/16) | Fix dead_code warning | Merged PR | Included in pinned SHA | Local test suite |
| [PR #15](https://github.com/smol-rs/fastrand-contrib/pull/15) | ci: Use reusable workflows for fmt and security_audit & fix CI failure | Merged PR | Included in pinned SHA | Local test suite |
| [PR #13](https://github.com/smol-rs/fastrand-contrib/pull/13) | Fix CI failures and seal RngExt trait | Merged PR | Included in pinned SHA | Local test suite |
| [PR #11](https://github.com/smol-rs/fastrand-contrib/pull/11) | Fix badges links in readme | Merged PR | Included in pinned SHA | Local test suite |
| [PR #3](https://github.com/smol-rs/fastrand-contrib/pull/3) | Fix CI issues | Merged PR | Included in pinned SHA | Local test suite |
| [PR #1](https://github.com/smol-rs/fastrand-contrib/pull/1) | Fix macro for defining the extension | Merged PR | Included in pinned SHA | Local test suite |

### `laputa-futures-lite` (`futures-lite`)

- **Upstream Repo**: `https://github.com/smol-rs/futures-lite`
- **Pinned Commit**: `3dc587ffdd652bc6002f34dc24a300643ef498ae`
- **Open Issues**: 13, **Open PRs**: 6

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#143](https://github.com/smol-rs/futures-lite/issues/143) | `read_line` wrong behavior and panic | Medium | Mandatory target (S06) | S06 | Failing reader test verifying partial buffer state and absence of panic. |
| [#142](https://github.com/smol-rs/futures-lite/issues/142) | Add `ready_chunks` method from `futures` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #142 |
| [#129](https://github.com/smol-rs/futures-lite/issues/129) | `StreamExt::or` variant which respects `Poll::Ready(None)` from `stream1` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #129 |
| [#127](https://github.com/smol-rs/futures-lite/issues/127) | Add `get_ref` and `get_mut` to stream combinators | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #127 |
| [#93](https://github.com/smol-rs/futures-lite/issues/93) | Add a trait for abstracing over executors | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #93 |
| [#84](https://github.com/smol-rs/futures-lite/issues/84) | Recreating concurrent futures combinators in smol | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #84 |
| [#83](https://github.com/smol-rs/futures-lite/issues/83) | Possible deadlock when using `smol::io::split` | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #83 |
| [#67](https://github.com/smol-rs/futures-lite/issues/67) | Panic in read_line? | High | Candidate soundness fix | Evaluate in S04-S06 | Tracker #67 |
| [#51](https://github.com/smol-rs/futures-lite/issues/51) | Add `dedupe()` to `StreamExt` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #51 |
| [#48](https://github.com/smol-rs/futures-lite/issues/48) | Accept async functions for `StreamExt::map()` and others | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #48 |
| [#47](https://github.com/smol-rs/futures-lite/issues/47) | question: What is the proper way how to wait for a `BufReader::read_line` to finish? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #47 |
| [#32](https://github.com/smol-rs/futures-lite/issues/32) | add now_or_never | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #32 |
| [#6](https://github.com/smol-rs/futures-lite/issues/6) | join! macro to join more than two futures? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #6 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#148](https://github.com/smol-rs/futures-lite/pull/148) | Fix read_line panic when the reader fails mid-line | Medium | Mandatory target (S06) | S06 | Failing reader test verifying partial buffer state and absence of panic. |
| [#147](https://github.com/smol-rs/futures-lite/pull/147) | feat: Derive `Clone` on `BufReader` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #147 |
| [#144](https://github.com/smol-rs/futures-lite/pull/144) | Release 2.7.0 | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #144 |
| [#137](https://github.com/smol-rs/futures-lite/pull/137) | Add `stream::fallback()` combinator that ends with the first stream | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #137 |
| [#60](https://github.com/smol-rs/futures-lite/pull/60) | Append to string buffer when calling read_line with string buffer | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #60 |
| [#7](https://github.com/smol-rs/futures-lite/pull/7) | Provide an `or!` macro that handles any number of futures | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #7 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #141](https://github.com/smol-rs/futures-lite/pull/141) | Fix read_line panic on non-empty buffer | Merged PR | Included in pinned SHA | Local test suite |
| [PR #139](https://github.com/smol-rs/futures-lite/pull/139) | Fix failing docs build | Merged PR | Included in pinned SHA | Local test suite |
| [PR #138](https://github.com/smol-rs/futures-lite/pull/138) | Fix `stream::Race` to return `None` if both combined streams return `None` | Merged PR | Included in pinned SHA | Local test suite |
| [PR #134](https://github.com/smol-rs/futures-lite/pull/134) | Fix build badge | Merged PR | Included in pinned SHA | Local test suite |
| [PR #131](https://github.com/smol-rs/futures-lite/pull/131) | docs: Fix `stop_after_future` link and `once_future` description | Merged PR | Included in pinned SHA | Local test suite |
| [PR #113](https://github.com/smol-rs/futures-lite/pull/113) | fix: remove `Unpin` from lines | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #124](https://github.com/smol-rs/futures-lite/issues/124) | `read_line` panics on non-empty buffer | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #40](https://github.com/smol-rs/futures-lite/issues/40) | stream::Race never finishes | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-parking` (`parking`)

- **Upstream Repo**: `https://github.com/smol-rs/parking`
- **Pinned Commit**: `cedb1884e0502ad21d4863a21eec9e3f13c3cf5f`
- **Open Issues**: 3, **Open PRs**: 0

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#29](https://github.com/smol-rs/parking/issues/29) | Potential DeadLock Reported by Miri | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #29 |
| [#21](https://github.com/smol-rs/parking/issues/21) | Support passing values through the Mutex inside the parker | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #21 |
| [#20](https://github.com/smol-rs/parking/issues/20) | Add support for WASM targets | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #20 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #9](https://github.com/smol-rs/parking/pull/9) | fix: CI for pull requests | Merged PR | Included in pinned SHA | Local test suite |
| [PR #3](https://github.com/smol-rs/parking/pull/3) | Fix CI | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #23](https://github.com/smol-rs/parking/issues/23) | Loom panics with "inconsistent park_timeout state: 1" | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-piper` (`piper`)

- **Upstream Repo**: `https://github.com/smol-rs/piper`
- **Pinned Commit**: `ef802bcc97f6b93af640ff7d60286ab12875552e`
- **Open Issues**: 2, **Open PRs**: 0

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#33](https://github.com/smol-rs/piper/issues/33) | [compiler bug] piper fails to compile for Xtensa-based ESP32 targets | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #33 |
| [#4](https://github.com/smol-rs/piper/issues/4) | Threads | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #4 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #34](https://github.com/smol-rs/piper/pull/34) | Bump MSRV to 1.63 & Fix CI failure | Merged PR | Included in pinned SHA | Local test suite |
| [PR #31](https://github.com/smol-rs/piper/pull/31) | Fix ordering of available data check. | Merged PR | Included in pinned SHA | Local test suite |
| [PR #26](https://github.com/smol-rs/piper/pull/26) | Fix crates.io description for v0.2 reduced scope | Merged PR | Included in pinned SHA | Local test suite |
| [PR #21](https://github.com/smol-rs/piper/pull/21) | Fix typo | Merged PR | Included in pinned SHA | Local test suite |
| [PR #19](https://github.com/smol-rs/piper/pull/19) | Fix nondeterministic test suite failures in length measurement functions | Merged PR | Included in pinned SHA | Local test suite |
| [PR #15](https://github.com/smol-rs/piper/pull/15) | Revert "msrv: Bump to v1.37.0" | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #30](https://github.com/smol-rs/piper/issues/30) | Race condition in `piper::Reader`. | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-polling` (`polling`)

- **Upstream Repo**: `https://github.com/smol-rs/polling`
- **Pinned Commit**: `35ba7a4fa8bf1e9f33711cb140a015d6bfe84c1b`
- **Open Issues**: 22, **Open PRs**: 7

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#271](https://github.com/smol-rs/polling/issues/271) | IOCP: an update queued just as another thread starts waiting can be missed | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #271 |
| [#267](https://github.com/smol-rs/polling/issues/267) | Update to focused 0.100 windows crates to follow Microsoft guidance | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #267 |
| [#266](https://github.com/smol-rs/polling/issues/266) | `port::Poller::modify` and `delete` might be unsound. | High | Candidate soundness fix | Evaluate in S04-S06 | Tracker #266 |
| [#262](https://github.com/smol-rs/polling/issues/262) | OpenBSD: kqueue fd accounting causing errors in Niri | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #262 |
| [#254](https://github.com/smol-rs/polling/issues/254) | Is it possible to make the internal lock optional? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #254 |
| [#253](https://github.com/smol-rs/polling/issues/253) | poller wait() code example does not delete() on earlier ? error return. | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #253 |
| [#245](https://github.com/smol-rs/polling/issues/245) | add new `io_uring` on linux? should be widely adopted enough since 2025 | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #245 |
| [#227](https://github.com/smol-rs/polling/issues/227) | Support disjoint polls on separate threads | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #227 |
| [#223](https://github.com/smol-rs/polling/issues/223) | Use kqueue on haiku | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #223 |
| [#213](https://github.com/smol-rs/polling/issues/213) | Make it so `Event::is_interrupt` corresponds to EV_EOF on kqueue platforms | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #213 |
| [#208](https://github.com/smol-rs/polling/issues/208) | Use ProcessSocketNotifications on Windows? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #208 |
| [#182](https://github.com/smol-rs/polling/issues/182) | A comparation with mio | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #182 |
| [#181](https://github.com/smol-rs/polling/issues/181) | Flakey NetBSD test | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #181 |
| [#178](https://github.com/smol-rs/polling/issues/178) | FortanixOS Support | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #178 |
| [#165](https://github.com/smol-rs/polling/issues/165) | Use kqueue1 where available | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #165 |
| [#141](https://github.com/smol-rs/polling/issues/141) | Use NtAssociateWaitCompletionPacket for waitable handles on Windows 10 | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #141 |
| [#138](https://github.com/smol-rs/polling/issues/138) | Support the x86_64-uwp-windows-msvc target | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #138 |
| [#121](https://github.com/smol-rs/polling/issues/121) | Figure out the optimal number of sockets to use per AFD handle | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #121 |
| [#102](https://github.com/smol-rs/polling/issues/102) | WASI support? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #102 |
| [#97](https://github.com/smol-rs/polling/issues/97) | Support overlapped operations in Windows | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #97 |
| [#89](https://github.com/smol-rs/polling/issues/89) | Support the AIX operating system | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #89 |
| [#37](https://github.com/smol-rs/polling/issues/37) | make `Poller` fork-safe | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #37 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#272](https://github.com/smol-rs/polling/pull/272) | Don't lose IOCP updates queued while a wait is starting | High | Mandatory target (S05) | S05 | Signal interruption test with concurrent poller wake on kqueue and epoll. |
| [#265](https://github.com/smol-rs/polling/pull/265) | fix(kqueue): preserve EV_CLEAR on EVFILT_USER notify and reset flag on EINTR | High | Mandatory target (S05) | S05 | Signal interruption test with concurrent poller wake on kqueue and epoll. |
| [#264](https://github.com/smol-rs/polling/pull/264) | Replace pipe_with calls on AIX same as haiku,nto | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #264 |
| [#263](https://github.com/smol-rs/polling/pull/263) | Fixed CompletionPort IO | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #263 |
| [#248](https://github.com/smol-rs/polling/pull/248) | Add  file handle support for overlapped I/O | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #248 |
| [#210](https://github.com/smol-rs/polling/pull/210) | feat: add `ProcessSocketNotifications` backend | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #210 |
| [#192](https://github.com/smol-rs/polling/pull/192) | DRAFT: trying to make add safe | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #192 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #270](https://github.com/smol-rs/polling/pull/270) | Fix spurious WouldBlock from the poll() backend | Merged PR | Included in pinned SHA | Local test suite |
| [PR #268](https://github.com/smol-rs/polling/pull/268) | Fix a clippy warning: | Merged PR | Included in pinned SHA | Local test suite |
| [PR #261](https://github.com/smol-rs/polling/pull/261) | Fix issues in CI | Merged PR | Included in pinned SHA | Local test suite |
| [PR #250](https://github.com/smol-rs/polling/pull/250) | fix: epoll: don't touch timerfd unless the timeout was provided | Merged PR | Included in pinned SHA | Local test suite |
| [PR #242](https://github.com/smol-rs/polling/pull/242) | ci: Use reusable workflows for clippy & Fix clippy warnings on Windows | Merged PR | Included in pinned SHA | Local test suite |
| [PR #238](https://github.com/smol-rs/polling/pull/238) | fix: making `tracing` integration optional | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #174](https://github.com/smol-rs/polling/issues/174) | When AFD is unsupported, return an error that can be traced back to the original I/O error | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-smol` (`smol`)

- **Upstream Repo**: `https://github.com/smol-rs/smol`
- **Pinned Commit**: `4157930176b44cb9831283aa4a68ea6d7ab8badf`
- **Open Issues**: 20, **Open PRs**: 4

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#388](https://github.com/smol-rs/smol/issues/388) | Allow cancel a task without consume receiver | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #388 |
| [#375](https://github.com/smol-rs/smol/issues/375) | `async_native_tls` / `native_tls` error when running `hyper-server` example | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #375 |
| [#370](https://github.com/smol-rs/smol/issues/370) | Allow build executor | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #370 |
| [#364](https://github.com/smol-rs/smol/issues/364) | Does a task-cancel interrupt a TcpStream read await? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #364 |
| [#357](https://github.com/smol-rs/smol/issues/357) | Question: What is the difference between using `#![apply(smol_macros::main!)]` and `ex.spawn` as shown in the example, and using `async_global_executor::block_on(async { })` and `async_global_executor::spawn`? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #357 |
| [#344](https://github.com/smol-rs/smol/issues/344) | Update minimal subcrates versions? | High | Candidate soundness fix | Evaluate in S04-S06 | Tracker #344 |
| [#329](https://github.com/smol-rs/smol/issues/329) | Question: does `smol` have something similar to `tokio::task_local!` and `async_std::task_local!`? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #329 |
| [#317](https://github.com/smol-rs/smol/issues/317) | Better docs on multi-threaded executor | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #317 |
| [#311](https://github.com/smol-rs/smol/issues/311) | Equivalent of `tokio::sync::watch` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #311 |
| [#310](https://github.com/smol-rs/smol/issues/310) | Expose async-broadcast in smol | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #310 |
| [#308](https://github.com/smol-rs/smol/issues/308) | Equivalent of `tokio::task::JoinSet` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #308 |
| [#244](https://github.com/smol-rs/smol/issues/244) | Tracking issue for potentially useful features in future Rust versions | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #244 |
| [#237](https://github.com/smol-rs/smol/issues/237) | smol::spawn: BrokenPipe io::copy failed | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #237 |
| [#230](https://github.com/smol-rs/smol/issues/230) | TLS example should use `async_tls` instead of `async-native-tls` | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #230 |
| [#223](https://github.com/smol-rs/smol/issues/223) | It's not clear how to use Executor instead of smol::spawn | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #223 |
| [#220](https://github.com/smol-rs/smol/issues/220) | Parallel async read and write example | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #220 |
| [#210](https://github.com/smol-rs/smol/issues/210) | There is no benchmark Compared to tokio and async_std? | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #210 |
| [#108](https://github.com/smol-rs/smol/issues/108) | Codebase documentation | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #108 |
| [#91](https://github.com/smol-rs/smol/issues/91) | Demo using warp | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #91 |
| [#87](https://github.com/smol-rs/smol/issues/87) | Wasm support | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #87 |

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#386](https://github.com/smol-rs/smol/pull/386) | Document running a multi-threaded executor from main | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #386 |
| [#385](https://github.com/smol-rs/smol/pull/385) | Update minimal versions of dependencies | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #385 |
| [#379](https://github.com/smol-rs/smol/pull/379) | Refactor AsyncWrite implementation and enhance web crawler error handling | Medium | Candidate bug fix | Evaluate in S04-S06 | Tracker #379 |
| [#158](https://github.com/smol-rs/smol/pull/158) | example: netlink crate | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #158 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #382](https://github.com/smol-rs/smol/pull/382) | fix: reject zero SMOL_THREADS value | Merged PR | Included in pinned SHA | Local test suite |
| [Issue #380](https://github.com/smol-rs/smol/issues/380) | RUSTSEC-2026-0097: Rand is unsound with a custom logger using `rand::rng()` | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #377](https://github.com/smol-rs/smol/issues/377) | RUSTSEC-2026-0097: Rand is unsound with a custom logger using `rand::rng()` | Closed Issue | Resolved / Dispositioned | Issue confirmation |
| [Issue #376](https://github.com/smol-rs/smol/issues/376) | RUSTSEC-2026-0097: Rand is unsound with a custom logger using `rand::rng()` | Closed Issue | Resolved / Dispositioned | Issue confirmation |

### `laputa-smol-macros` (`smol-macros`)

- **Upstream Repo**: `https://github.com/smol-rs/smol-macros`
- **Pinned Commit**: `64caeb4ebe4620001edcc031c7615205bc79af65`
- **Open Issues**: 3, **Open PRs**: 0

#### Open Issues

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#14](https://github.com/smol-rs/smol-macros/issues/14) | Release v1.0.0 | Low | Maintenance / Documentation | Tracked (no action needed in core runtime) | Tracker #14 |
| [#13](https://github.com/smol-rs/smol-macros/issues/13) | Support `pub` functions in `main!` | High | Candidate soundness fix | Evaluate in S04-S06 | Tracker #13 |
| [#2](https://github.com/smol-rs/smol-macros/issues/2) | Add the ability to use futures_lite::future::block_on instead of async_io::block_on | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #2 |

### `laputa-waker-fn` (`waker-fn`)

- **Upstream Repo**: `https://github.com/smol-rs/waker-fn`
- **Pinned Commit**: `d44bd05c889e431c5e8cba7599186de263c04ec3`
- **Open Issues**: 0, **Open PRs**: 1

#### Open Pull Requests

| # | Title | Severity | Disposition | Target | Evidence / Notes |
|:---|:---|:---|:---|:---|:---|
| [#14](https://github.com/smol-rs/waker-fn/pull/14) | feat: add 'alloc' feature and `waker_fn_ptr`, zero-allocation waker from fn pointer | Low | Feature / API Enhancement | Deferred (parity before enhancement) | Tracker #14 |

#### Merged-Since-Release / Closed Bug Reports

| Item | Title | Type | Disposition | Evidence |
|:---|:---|:---|:---|:---|
| [PR #2](https://github.com/smol-rs/waker-fn/pull/2) | Fix CI | Merged PR | Included in pinned SHA | Local test suite |

