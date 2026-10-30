Prefix: SMOL
Next: 7

### SMOL-2 · P1 · Miri and Loom cover a sliver of the unsafe and lock surface

- **Area:** Tests
- **Where:** `.github/workflows/ci.yml:147-177`, `crates/async-lock/src/rwlock.rs:120`, `crates/event-listener/src/intrusive.rs:194`, `crates/polling/src/poll.rs:79`
- **Problem:** CI proves Miri on four `async-task` tests plus executor `shutdown_test`, and Loom on one `concurrent-queue` close test. No Miri for `async-lock`, `event-listener` intrusive list, `concurrent-queue` bounded/unbounded, `blocking`, or `polling`; no Loom for `async-lock`/`event-listener-strategy`. The dense `SAFETY:` comments in `rwlock.rs`, `intrusive.rs`, and `poll.rs` name behaviors with no check.
- **Recommendation:** Enumerate the named unsafe/lock behaviors (rwlock downgrade/upgrade, intrusive link insert/remove, queue close-vs-push) and add one Miri plus one Loom test per behavior.
- **Fix with:** `renovate-tests`
- **Effort:** M

### SMOL-3 · P2 · Blocking pool silently falls back, caches first, and hides errors

- **Area:** Config
- **Where:** `crates/blocking/src/lib.rs:15`, `crates/blocking/src/lib.rs:234`, `crates/blocking/src/lib.rs:340`, `crates/blocking/src/lib.rs:361`, `crates/blocking/src/lib.rs:575`
- **Problem:** Invalid `BLOCKING_MAX_THREADS` (`""`, non-integer, `0`, `50000`) silently becomes a clamped default with no log or `Result`. The value is cached on first `grow_pool` via `get_or_insert_with`, so later env or setter changes do not re-read. `schedule` pushes onto an unbounded `VecDeque` with thread growth as the only backpressure. `main_loop` discards panic payloads with `.ok()`, and `get_mut`/`with_mut` discard `poll_stop` I/O errors with `.ok()` before assuming idle.
- **Recommendation:** Parse once into a validated config with a loud fallback, inject it instead of reading env inside `max_threads`, bound or observe the queue, and propagate `poll_stop` errors and panic payloads.
- **Fix with:** `isolate-effects`
- **Effort:** M

### SMOL-4 · P2 · Async-io reactor is a global singleton with wall-clock logic and abort-or-swallow errors

- **Area:** Errors
- **Where:** `crates/async-io/src/reactor.rs:90`, `crates/async-io/src/reactor.rs:180`, `crates/async-io/src/reactor.rs:210`, `crates/async-io/src/driver.rs:29`, `crates/async-io/src/driver.rs:70`
- **Problem:** Timer expiry and `react` timeout math read `Instant::now()` and `Reactor::get()` directly, so deterministic tests are impossible. Poisoned mutexes, `poller.notify()`, and driver-thread spawn use `expect` and abort the process, while `react(...).ok()` in `main_loop` and `block_on` silently drops real reactor `io::Error`s (only `Interrupted` is handled).
- **Recommendation:** Inject a clock/timer trait into `Timer`/`Reactor`, convert `expect` on poison/notify/spawn to typed errors or documented aborts, and surface `react` errors to callers or a hook.
- **Fix with:** `error-model`
- **Effort:** M

### SMOL-5 · P2 · Raw-fd registration and ambient process env push safety to callers

- **Area:** Security
- **Where:** `crates/polling/src/lib.rs:519`, `crates/polling/src/lib.rs:538`, `crates/async-process/src/lib.rs:709-757`
- **Problem:** `Poller::add`/`add_with_mode` are `unsafe` over raw fds with a use-after-close or fd-reuse hazard re-implemented per backend (epoll/kqueue/port/poll); there is no owned-fd wrapper where most callers live, and h12tiny inherits this through async-io. `async-process::Command` wraps `std::process::Command` with ambient `PATH` resolution and inherited env, and the crate docs demonstrate `sh -c`, so downstream use with untrusted args is one copy-paste from injection or PATH hijack.
- **Recommendation:** Provide an owned-fd registration API and keep raw `unsafe` for the integer-fd escape hatch; document the `Command` trust contract at `Command::new` with an argv-without-shell example.
- **Fix with:** `os-boundaries`
- **Effort:** M

### SMOL-6 · P2 · Same wake, flush, and lifetime classes recur with only point-fix tests

- **Area:** Tests
- **Where:** `crates/async-lock/src/rwlock.rs:120`, `crates/event-listener-strategy/src/lib.rs:483`, `crates/polling/src/poll.rs:79`
- **Problem:** `git log` shows the same fix shape 3-5 times in a row: wake/notify under lock, blocking write/flush accounting, task shutdown/detach/waker lifetime. Each adds one narrow test. Nothing makes the class impossible: no guard fails on `MutexGuard` held across `wake`/`notify`/`execute`, no flush-state model, no shutdown-transition table. The `should never be None` and `we hold the lock` comments name invariants nothing checks, and fork-adjacent `XXX`/`T_T` notes have no ticket link.
- **Recommendation:** Add one class-level guard per family (lock-held wake audit, flush-after-write sequence model, shutdown-transition table) instead of the next point fix; convert the top `never`/`assume` comments into `debug_assert!` plus a named test.
- **Fix with:** `test-audit`
- **Effort:** M
