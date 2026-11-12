Prefix: SMOL
Next: 13

### SMOL-3 · P1 · Accessing an Unblock handle erases pending write failures

- **Area:** Errors
- **Where:** `crates/blocking/src/lib.rs:574`, `crates/blocking/src/lib.rs:608`, `crates/blocking/src/lib.rs:721`, `crates/blocking/src/lib.rs:1082`
- **Problem:** `get_mut()` and `with_mut()` discard the error returned by `poll_stop()` after it restores `State::Idle`. A subsequent flush can succeed although previously accepted bytes never reached the inner writer. Reproduced with `Unblock::with_capacity(64, writer)`, where `writer.write()` always fails and `writer.flush()` succeeds: `write_all(b"lost data")`, either accessor, and `flush()` all return successfully. Neither accessor documents discarding pending write errors. The caller has followed the documented flush-before-drop requirement.
- **Recommendation:** Preserve a background write or flush error in the existing pending-error state when an accessor stops the task, then return it from the next write, flush, or close. Keep the accessor signatures. Cover write → get_mut/with_mut → flush with a failing inner writer, and keep extraction's flush-before-handoff contract explicit.
- **Fix with:** `error-model`
- **Effort:** S

### SMOL-9 · P1 · File synchronization discards unread buffered bytes

- **Area:** Data
- **Where:** `crates/async-fs/src/lib.rs:968`, `crates/async-fs/src/lib.rs:999`, `crates/async-fs/src/lib.rs:1025`, `crates/async-fs/src/lib.rs:1084`, `crates/async-fs/src/lib.rs:1126`, `crates/blocking/src/lib.rs:709`, `crates/blocking/src/lib.rs:1107`
- **Problem:** `sync_all()`, `sync_data()`, and `set_len()` flush the underlying `Unblock` even when it is reading. This stops the background reader and discards its read-ahead pipe. The OS cursor has advanced, but `File.read_pos` still represents the logical position and the next read does not restore it. Three real-file probes each opened `abcdef`, read `a`, called one of these methods, and then read EOF instead of `bcdef`. Even `set_len(6)`, which changes no file content, loses the unread suffix.
- **Recommendation:** Preserve the logical read position across synchronization and truncation. Drain writes without discarding an active read buffer where possible. If read-ahead must be discarded, retain the cursor-restoration obligation in File-owned state across cancellation and complete it before another read. Reuse the existing repositioning mechanism. Test all three operations between partial reads, including cancellation during the transition.
- **Fix with:** `lifecycle-concurrency`
- **Effort:** M

### SMOL-8 · P1 · Release preflight dirties its own checkout and rejects it

- **Area:** Build
- **Where:** `scripts/bump_minor_release.sh:358`, `scripts/bump_minor_release.sh:395`, `scripts/bump_minor_release.sh:497`, `scripts/bump_minor_release.sh:536`, `tools/release_graph.py:6`, `.gitignore:1`
- **Problem:** Release graph queries import `identity` before the release script checks for a clean tree. With normal Python defaults, this creates unignored `tools/__pycache__/`. In a clean disposable clone, `python3 tools/release_graph.py version` prints `0.3.0`, then `git status --porcelain=v1` reports that directory. Running the unchanged release script with only registry queries stubbed succeeds through those queries and then exits with `release requires a clean working tree`; no tracked files change. This independently blocks a release or resume after the registry-query defect is repaired.
- **Recommendation:** Suppress or redirect bytecode for these tooling entry points, or intentionally ignore their cache directories. Keep rejection of unrelated untracked user files. Exercise release preflight from a clean clone with normal Python defaults and no user-global Git ignore rules.
- **Fix with:** `isolate-effects`
- **Effort:** S

### SMOL-6 · P2 · Waker callbacks run while internal locks are held

- **Area:** Concurrency
- **Where:** `crates/event-listener/src/intrusive.rs:137`, `crates/event-listener/src/intrusive.rs:225`, `crates/event-listener/src/intrusive.rs:347`, `crates/async-executor/src/lib.rs:891`, `crates/async-executor/src/lib.rs:906`, `crates/async-executor/src/lib.rs:1038`, `crates/async-io/src/reactor.rs:685`, `crates/async-io/src/reactor.rs:730`, `crates/async-broadcast/src/lib.rs:728`
- **Problem:** Safe user wakers can reenter the library while it holds a non-reentrant mutex. Three real-crate probes deadlock: poll an Event listener with a waker that calls `event.listen()`, then notify it; drop a pending `Executor::tick()` whose final waker destructor spawns on that executor; or drop a registered `Async<UdpSocket>::readable()` future whose final waker destructor calls `poll_writable()` on that socket. Each probe reaches its callback and cannot return. Event/executor controls that remove only callback reentry complete. Waker replacement also destroys old wakers under these locks. The broadcast channel additionally holds its outer channel mutex while notifying listeners.
- **Recommendation:** Complete state changes while locked, extract callbacks and retired wakers, then invoke or destroy them after releasing every relevant lock. Account for clone, replacement, wake, and final drop, including callers' outer locks. Preserve notification propagation and executor publication accounting. Add safe reentrant-waker regressions for event notification, ticker cancellation, and readiness replacement/cancellation.
- **Fix with:** `lifecycle-concurrency`
- **Effort:** M

### SMOL-10 · P2 · Broadcast admission changes leave pending senders asleep

- **Area:** Concurrency
- **Where:** `crates/async-broadcast/src/lib.rs:274`, `crates/async-broadcast/src/lib.rs:394`, `crates/async-broadcast/src/lib.rs:434`, `crates/async-broadcast/src/lib.rs:631`, `crates/async-broadcast/src/lib.rs:1748`, `crates/async-broadcast/src/lib.rs:1896`
- **Problem:** A blocked broadcast waits on `send_ops`, but four public transitions change its result without notifying that event: adding the first active receiver with `Sender::new_receiver()`, increasing a full channel's capacity, enabling overflow, or disabling `await_active` while all receivers are inactive. Counting-waker probes registered a pending send before each transition. All four delivered zero wakes although a forced repoll immediately returned Ready. The control, `InactiveReceiver::activate_cloned()`, delivered one wake. An executor can therefore leave a now-ready sender asleep indefinitely.
- **Recommendation:** Centralize admission-change notifications across Sender, Receiver, and InactiveReceiver. Notify when a transition permits progress or yields an error, dispatch outside the channel mutex, and retain chaining for multiple waiting senders. Add a public-API table test for the four transitions using a counting waker.
- **Fix with:** `lifecycle-concurrency`
- **Effort:** M

### SMOL-11 · P2 · Floating-point range helpers return excluded endpoints

- **Area:** Types
- **Where:** `crates/fastrand-contrib/src/float_range.rs:124`, `crates/fastrand-contrib/src/float_range.rs:143`, `crates/fastrand-contrib/src/float_range.rs:160`, `crates/fastrand-contrib/src/lib.rs:156`
- **Problem:** Multiplying a sample by the interval width and adding its lower bound can round up to an excluded upper endpoint. This violates the public range contract for valid nonempty ranges. With `Rng::with_seed(42)` and an upper bound one representable float above `1.0`, both `f32_range(1.0..upper)` and `f64_range(1.0..upper)` returned the excluded endpoint in 60 of the first 100 samples. Both current library sources were compiled unchanged for the probe. Existing tests sample broad ranges and miss adjacent representable values.
- **Recommendation:** Make endpoint inclusion hold after floating-point rounding for both widths and both scaling paths. Add adjacent-value, signed-zero, subnormal, and wide-range cases that assert the actual RangeBounds contract. Define rejection of empty or non-finite input separately from correcting valid finite ranges; avoid an unbounded rejection loop when no representable sample exists.
- **Fix with:** `typing-audit`
- **Effort:** S

### SMOL-12 · P3 · Ordered timer storage shifts the whole queue during cancellation

- **Area:** Performance
- **Where:** `crates/async-io/src/reactor.rs:199`, `crates/async-io/src/reactor.rs:369`, `crates/async-io/src/reactor.rs:387`, `crates/async-io/src/reactor.rs:396`, `crates/async-io/src/reactor.rs:410`, `crates/async-io/benches/timer.rs:13`
- **Problem:** Commit `a7a3d7e` replaced the ordered timer map with an FxHashMap plus a sorted Vec. Each interior insertion or removal shifts the Vec while holding the global timer mutex; expiring a prefix shifts the remaining timers. Cancelling N live timers in deadline order therefore moves a quadratic number of keys. This is source-proven scaling cost, not a measured latency regression. The existing registration benchmark uses one-second timers, so a large setup can expire before the timed region and does not establish cancellation cost with a stable live set.
- **Recommendation:** Measure a fixed, still-live timer set through registration, out-of-order insertion, cancellation, and expiry against the previous ordered-map implementation. Restore logarithmic ordered updates or choose an equivalent structure if representative workloads confirm the cost. Keep equal-deadline ordering and cancellation semantics, and report live counts and completed work alongside timings.
- **Fix with:** `make-it-fast`
- **Effort:** M
