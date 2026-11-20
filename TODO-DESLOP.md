Prefix: DSLP
Next: 42

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier D). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After the component tickets (DSLP-3 to DSLP-41, ordered Types, Tests, Dead code), work the open P1 and P2 findings, then fill in `## Report` below.

## Report

Not started.

## Tickets

### DSLP-2 · P2 · Diagnose: ultrareview into this backlog

- **Area:** deslop · diagnose
- **Where:** whole repository
- **Problem:** No review has ranked this repository's risks for the deslop pass.
- **Recommendation:** Run the deslop Diagnose step. Accept: the diff touches only docs, comments, and this file, and every finding names its Fix with.
- **Fix with:** deslop
- **Effort:** M

### DSLP-3 · P2 · Types: runtime scheduling and synchronization

- **Area:** deslop · types · runtime scheduling and synchronization
- **Where:** crates/async-task, async-executor, async-lock, async-channel, async-broadcast, concurrent-queue, event-listener, event-listener-strategy, atomic-waker, parking, waker-fn
- **Problem:** Scope: 28.8k Rust lines, about 3.7k changed against the pinned releases (async-task 1.7k, async-executor 1.3k, async-broadcast 0.35k; async-lock, async-channel, parking and waker-fn are near pristine). Contracts: runtime task, executor, lock, channel and wake APIs. Consumers: laputa-smol and downstream runtime consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-async-task -p laputa-async-executor -p laputa-async-lock -p laputa-async-channel -p laputa-async-broadcast -p laputa-concurrent-queue -p laputa-event-listener -p laputa-event-listener-strategy -p laputa-atomic-waker -p laputa-parking -p laputa-waker-fn; cargo clippy -p laputa-async-task -p laputa-async-executor -p laputa-async-lock -p laputa-async-channel -p laputa-async-broadcast -p laputa-concurrent-queue -p laputa-event-listener -p laputa-event-listener-strategy -p laputa-atomic-waker -p laputa-parking -p laputa-waker-fn --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-4 · P2 · Tests: runtime scheduling and synchronization

- **Area:** deslop · tests · runtime scheduling and synchronization
- **Where:** crates/async-task, async-executor, async-lock, async-channel, async-broadcast, concurrent-queue, event-listener, event-listener-strategy, atomic-waker, parking, waker-fn
- **Problem:** Scope: 28.8k Rust lines, about 3.7k changed against the pinned releases (async-task 1.7k, async-executor 1.3k, async-broadcast 0.35k; async-lock, async-channel, parking and waker-fn are near pristine). Contracts: runtime task, executor, lock, channel and wake APIs. Consumers: laputa-smol and downstream runtime consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-async-task -p laputa-async-executor -p laputa-async-lock -p laputa-async-channel -p laputa-async-broadcast -p laputa-concurrent-queue -p laputa-event-listener -p laputa-event-listener-strategy -p laputa-atomic-waker -p laputa-parking -p laputa-waker-fn; cargo clippy -p laputa-async-task -p laputa-async-executor -p laputa-async-lock -p laputa-async-channel -p laputa-async-broadcast -p laputa-concurrent-queue -p laputa-event-listener -p laputa-event-listener-strategy -p laputa-atomic-waker -p laputa-parking -p laputa-waker-fn --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-5 · P2 · Dead code: runtime scheduling and synchronization

- **Area:** deslop · dead code · runtime scheduling and synchronization
- **Where:** crates/async-task, async-executor, async-lock, async-channel, async-broadcast, concurrent-queue, event-listener, event-listener-strategy, atomic-waker, parking, waker-fn
- **Problem:** Scope: 28.8k Rust lines, about 3.7k changed against the pinned releases (async-task 1.7k, async-executor 1.3k, async-broadcast 0.35k; async-lock, async-channel, parking and waker-fn are near pristine). Contracts: runtime task, executor, lock, channel and wake APIs. Consumers: laputa-smol and downstream runtime consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-async-task -p laputa-async-executor -p laputa-async-lock -p laputa-async-channel -p laputa-async-broadcast -p laputa-concurrent-queue -p laputa-event-listener -p laputa-event-listener-strategy -p laputa-atomic-waker -p laputa-parking -p laputa-waker-fn; cargo clippy -p laputa-async-task -p laputa-async-executor -p laputa-async-lock -p laputa-async-channel -p laputa-async-broadcast -p laputa-concurrent-queue -p laputa-event-listener -p laputa-event-listener-strategy -p laputa-atomic-waker -p laputa-parking -p laputa-waker-fn --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-6 · P2 · Types: runtime I/O, processes and blocking

- **Area:** deslop · types · runtime I/O, processes and blocking
- **Where:** crates/async-io, polling, async-net, async-fs, async-process, async-signal, blocking, piper
- **Problem:** Scope: 20.9k Rust lines, about 1.9k changed against the pinned releases (async-io 0.64k, async-process 0.29k, blocking 0.29k, async-fs 0.28k, polling 0.2k). Contracts: runtime reactor, socket, file, process, signal, blocking-pool and pipe APIs. Consumers: laputa-smol, h12tiny and downstream runtime consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-async-io -p laputa-polling -p laputa-async-net -p laputa-async-fs -p laputa-async-process -p laputa-async-signal -p laputa-blocking -p laputa-piper; cargo clippy -p laputa-async-io -p laputa-polling -p laputa-async-net -p laputa-async-fs -p laputa-async-process -p laputa-async-signal -p laputa-blocking -p laputa-piper --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-7 · P2 · Types: runtime facade and utilities

- **Area:** deslop · types · runtime facade and utilities
- **Where:** crates/smol, smol-macros, futures-lite, async-dup, easy-parallel, fastrand, fastrand-contrib
- **Problem:** Scope: 12.4k Rust lines, about 0.55k changed against the pinned releases (futures-lite 0.47k); smol-macros, easy-parallel and fastrand-contrib (1.9k) have no pinned release to diff and count as ours. Contracts: facade re-exports, futures-lite traits, macro and random-number APIs. Consumers: laputa-smol consumers, h12tiny and downstream runtime consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-smol -p laputa-smol-macros -p laputa-futures-lite -p laputa-async-dup -p laputa-easy-parallel -p laputa-fastrand -p laputa-fastrand-contrib; cargo clippy -p laputa-smol -p laputa-smol-macros -p laputa-futures-lite -p laputa-async-dup -p laputa-easy-parallel -p laputa-fastrand -p laputa-fastrand-contrib --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-8 · P2 · Types: HTTP/2 framing and compression

- **Area:** deslop · types · HTTP/2 framing and compression
- **Where:** crates/laputa-h2-futures-lite/src/{codec,frame,futures_codec,hpack} and the codec tests (tests/h2-tests/tests/codec_read.rs, codec_write.rs)
- **Problem:** Scope: 11.5k Rust lines; futures_codec (2.0k) is wholly ours, the rest changes about 0.4k lines against h2 0.4.20. Contracts: HTTP/2 frame parsing and encoding, HPACK, and the futures-io codec that replaces tokio-util. Consumers: hyper, h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-h2-futures-lite; cargo clippy -p laputa-h2-futures-lite --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-9 · P2 · Types: HTTP/2 connection and streams

- **Area:** deslop · types · HTTP/2 connection and streams
- **Where:** crates/laputa-h2-futures-lite/src/proto and the stream-state tests (tests/h2-tests/tests/{flow_control,stream_states,prioritization,push_promise,trailers,informational_responses,ping_pong,h2_975,hammer}.rs, tests/flow_control_futures.rs)
- **Problem:** Scope: 18k Rust lines (proto 9.1k, tests 8.9k); changes about 0.4k lines against h2 0.4.20 (connection.rs control-response queue and read/write interleave, recv, send, state, streams). Contracts: HTTP/2 streams, flow control, reset accounting, control responses and connection lifecycle. Consumers: hyper, h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-h2-futures-lite -p h2-tests; cargo clippy -p laputa-h2-futures-lite -p h2-tests --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-10 · P2 · Types: HTTP/2 client, server and test support

- **Area:** deslop · types · HTTP/2 client, server and test support
- **Where:** crates/laputa-h2-futures-lite/src top-level files (client, server, share, error, lib, ext, fuzz_bridge), tests/h2-tests/tests/{client_request,server}.rs, tests/h2-support, tests/h2-fuzz, fuzz, util
- **Problem:** Scope: 11.6k Rust lines; src changes are the tokio-to-futures-io port and the tracing shim in lib.rs; the 1.6k h2-support harness and the ported tests are ours. Contracts: h2 client and server public API, the in-process test harness and the fuzz bridge. Consumers: hyper, h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-h2-futures-lite -p h2-tests -p h2-support; cargo clippy -p laputa-h2-futures-lite -p h2-tests -p h2-support --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-11 · P2 · Types: HTTP/1 protocol

- **Area:** deslop · types · HTTP/1 protocol
- **Where:** crates/laputa-hyper-futures-lite/src/proto/h1 and tests (h1_shutdown_while_buffered, h1_flush_before_yield, ready_on_poll_stream, unbuffered_stream, h1_server)
- **Problem:** Scope: 10.3k Rust lines; changes about 0.5k lines against hyper 1.12.0 (dispatch.rs pending_result and poll_close, conn.rs, io.rs, mock-io test port). Contracts: HTTP/1 request and response parsing, framing, chunked decoding and connection lifecycle. Consumers: h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-hyper-futures-lite; cargo clippy -p laputa-hyper-futures-lite --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-12 · P2 · Types: Hyper client, server, body and HTTP/2 glue

- **Area:** deslop · types · Hyper client, server, body and HTTP/2 glue
- **Where:** crates/laputa-hyper-futures-lite/src/{client,server,body,service,proto/h2} and tests/{client,server,integration}.rs
- **Problem:** Scope: 16k Rust lines (src 8k, tests 8.2k); changes about 0.4k lines against hyper 1.12.0. Contracts: Hyper client and server connection builders, Incoming body and the HTTP/2 adapter. Consumers: h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-hyper-futures-lite; cargo clippy -p laputa-hyper-futures-lite --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-13 · P2 · Types: Hyper runtime traits, extensions and FFI

- **Area:** deslop · types · Hyper runtime traits, extensions and FFI
- **Where:** crates/laputa-hyper-futures-lite/src/{rt,common,ext,ffi,upgrade,error,trace,lib,test_support}.rs and directories, tests/support, capi
- **Problem:** Scope: 7.5k Rust lines; changes about 0.2k lines against hyper 1.12.0 (rt timer, io compat, rewind, ffi task and client). Contracts: hyper::rt Read/Write/Timer/Executor traits, upgrade handling, C API. Consumers: h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-hyper-futures-lite; cargo clippy -p laputa-hyper-futures-lite --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-14 · P2 · Types: websocket and protocol test support

- **Area:** deslop · types · websocket and protocol test support
- **Where:** crates/laputa-fastwebsockets-futures-lite and crates/laputa-protocol-test-support
- **Problem:** Scope: 3.5k Rust lines; fastwebsockets has no pinned upstream version in UPSTREAM.toml, so all of it counts. Contracts: websocket framing, handshake and mask contracts; the pipe, executor and budget harness used by protocol suites. Consumers: h12tiny-web, h12tiny and downstream websocket consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p laputa-fastwebsockets-futures-lite -p laputa-protocol-test-support; cargo clippy -p laputa-fastwebsockets-futures-lite -p laputa-protocol-test-support --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-15 · P2 · Types: HTTP facade and clients

- **Area:** deslop · types · HTTP facade and clients
- **Where:** crates/h12tiny, h12tiny-core, h12tiny-client, h12tiny-client-normalize, h12tiny-client-sync, h12tiny-tls-system
- **Problem:** Scope: 16.7k Rust lines, all first-party (includes h12tiny integration tests, examples and the TLS support harness). Contracts: HTTP client, TLS, facade and request/response APIs. Consumers: stuff and other published h12tiny consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p h12tiny -p h12tiny-core -p h12tiny-client -p h12tiny-client-normalize -p h12tiny-client-sync -p h12tiny-tls-system; cargo clippy -p h12tiny -p h12tiny-core -p h12tiny-client -p h12tiny-client-normalize -p h12tiny-client-sync -p h12tiny-tls-system --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-16 · P2 · Types: HTTP server and web utilities

- **Area:** deslop · types · HTTP server and web utilities
- **Where:** crates/h12tiny-server, h12tiny-util, h12tiny-web
- **Problem:** Scope: 7.5k Rust lines, all first-party. Contracts: HTTP server lifecycle, bounds, upgrade, body utilities, routing and websocket APIs. Consumers: stuff and other published h12tiny consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: cargo test -p h12tiny-server -p h12tiny-util -p h12tiny-web; cargo clippy -p h12tiny-server -p h12tiny-util -p h12tiny-web --all-targets; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-17 · P2 · Types: repository tools, benchmarks and CI

- **Area:** deslop · types · repository tools, benchmarks and CI
- **Where:** tools, scripts, tests (Python suites, tests/consumer, tests/qualify), benchmarks/twelve-dollar, fixtures, Makefile and .github
- **Problem:** Scope: about 15k lines of Rust, Python, shell and workflow files, all first-party. Contracts: release, staging, graph-gate, consumer and benchmark command contracts. Consumers: maintainers, CI and the external consumer fixture; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Measure the applicable strict candidates; fix witnessed typing defects; keep behavior and record adopted and rejected settings with counts. Acceptance: python3 -m unittest discover -s tests -p "test_*.py"; tools/graph-gate.sh; cargo test -p smolith-qualify; then make test and make lint; the deslop Types acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-18 · P2 · Tests: runtime I/O, processes and blocking

- **Area:** deslop · tests · runtime I/O, processes and blocking
- **Where:** crates/async-io, polling, async-net, async-fs, async-process, async-signal, blocking, piper
- **Problem:** Scope: 20.9k Rust lines, about 1.9k changed against the pinned releases (async-io 0.64k, async-process 0.29k, blocking 0.29k, async-fs 0.28k, polling 0.2k). Contracts: runtime reactor, socket, file, process, signal, blocking-pool and pipe APIs. Consumers: laputa-smol, h12tiny and downstream runtime consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-async-io -p laputa-polling -p laputa-async-net -p laputa-async-fs -p laputa-async-process -p laputa-async-signal -p laputa-blocking -p laputa-piper; cargo clippy -p laputa-async-io -p laputa-polling -p laputa-async-net -p laputa-async-fs -p laputa-async-process -p laputa-async-signal -p laputa-blocking -p laputa-piper --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-19 · P2 · Tests: runtime facade and utilities

- **Area:** deslop · tests · runtime facade and utilities
- **Where:** crates/smol, smol-macros, futures-lite, async-dup, easy-parallel, fastrand, fastrand-contrib
- **Problem:** Scope: 12.4k Rust lines, about 0.55k changed against the pinned releases (futures-lite 0.47k); smol-macros, easy-parallel and fastrand-contrib (1.9k) have no pinned release to diff and count as ours. Contracts: facade re-exports, futures-lite traits, macro and random-number APIs. Consumers: laputa-smol consumers, h12tiny and downstream runtime consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-smol -p laputa-smol-macros -p laputa-futures-lite -p laputa-async-dup -p laputa-easy-parallel -p laputa-fastrand -p laputa-fastrand-contrib; cargo clippy -p laputa-smol -p laputa-smol-macros -p laputa-futures-lite -p laputa-async-dup -p laputa-easy-parallel -p laputa-fastrand -p laputa-fastrand-contrib --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-20 · P2 · Tests: HTTP/2 framing and compression

- **Area:** deslop · tests · HTTP/2 framing and compression
- **Where:** crates/laputa-h2-futures-lite/src/{codec,frame,futures_codec,hpack} and the codec tests (tests/h2-tests/tests/codec_read.rs, codec_write.rs)
- **Problem:** Scope: 11.5k Rust lines; futures_codec (2.0k) is wholly ours, the rest changes about 0.4k lines against h2 0.4.20. Contracts: HTTP/2 frame parsing and encoding, HPACK, and the futures-io codec that replaces tokio-util. Consumers: hyper, h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-h2-futures-lite; cargo clippy -p laputa-h2-futures-lite --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-21 · P2 · Tests: HTTP/2 connection and streams

- **Area:** deslop · tests · HTTP/2 connection and streams
- **Where:** crates/laputa-h2-futures-lite/src/proto and the stream-state tests (tests/h2-tests/tests/{flow_control,stream_states,prioritization,push_promise,trailers,informational_responses,ping_pong,h2_975,hammer}.rs, tests/flow_control_futures.rs)
- **Problem:** Scope: 18k Rust lines (proto 9.1k, tests 8.9k); changes about 0.4k lines against h2 0.4.20 (connection.rs control-response queue and read/write interleave, recv, send, state, streams). Contracts: HTTP/2 streams, flow control, reset accounting, control responses and connection lifecycle. Consumers: hyper, h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-h2-futures-lite -p h2-tests; cargo clippy -p laputa-h2-futures-lite -p h2-tests --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-22 · P2 · Tests: HTTP/2 client, server and test support

- **Area:** deslop · tests · HTTP/2 client, server and test support
- **Where:** crates/laputa-h2-futures-lite/src top-level files (client, server, share, error, lib, ext, fuzz_bridge), tests/h2-tests/tests/{client_request,server}.rs, tests/h2-support, tests/h2-fuzz, fuzz, util
- **Problem:** Scope: 11.6k Rust lines; src changes are the tokio-to-futures-io port and the tracing shim in lib.rs; the 1.6k h2-support harness and the ported tests are ours. Contracts: h2 client and server public API, the in-process test harness and the fuzz bridge. Consumers: hyper, h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-h2-futures-lite -p h2-tests -p h2-support; cargo clippy -p laputa-h2-futures-lite -p h2-tests -p h2-support --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-23 · P2 · Tests: HTTP/1 protocol

- **Area:** deslop · tests · HTTP/1 protocol
- **Where:** crates/laputa-hyper-futures-lite/src/proto/h1 and tests (h1_shutdown_while_buffered, h1_flush_before_yield, ready_on_poll_stream, unbuffered_stream, h1_server)
- **Problem:** Scope: 10.3k Rust lines; changes about 0.5k lines against hyper 1.12.0 (dispatch.rs pending_result and poll_close, conn.rs, io.rs, mock-io test port). Contracts: HTTP/1 request and response parsing, framing, chunked decoding and connection lifecycle. Consumers: h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-hyper-futures-lite; cargo clippy -p laputa-hyper-futures-lite --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-24 · P2 · Tests: Hyper client, server, body and HTTP/2 glue

- **Area:** deslop · tests · Hyper client, server, body and HTTP/2 glue
- **Where:** crates/laputa-hyper-futures-lite/src/{client,server,body,service,proto/h2} and tests/{client,server,integration}.rs
- **Problem:** Scope: 16k Rust lines (src 8k, tests 8.2k); changes about 0.4k lines against hyper 1.12.0. Contracts: Hyper client and server connection builders, Incoming body and the HTTP/2 adapter. Consumers: h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-hyper-futures-lite; cargo clippy -p laputa-hyper-futures-lite --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-25 · P2 · Tests: Hyper runtime traits, extensions and FFI

- **Area:** deslop · tests · Hyper runtime traits, extensions and FFI
- **Where:** crates/laputa-hyper-futures-lite/src/{rt,common,ext,ffi,upgrade,error,trace,lib,test_support}.rs and directories, tests/support, capi
- **Problem:** Scope: 7.5k Rust lines; changes about 0.2k lines against hyper 1.12.0 (rt timer, io compat, rewind, ffi task and client). Contracts: hyper::rt Read/Write/Timer/Executor traits, upgrade handling, C API. Consumers: h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-hyper-futures-lite; cargo clippy -p laputa-hyper-futures-lite --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-26 · P2 · Tests: websocket and protocol test support

- **Area:** deslop · tests · websocket and protocol test support
- **Where:** crates/laputa-fastwebsockets-futures-lite and crates/laputa-protocol-test-support
- **Problem:** Scope: 3.5k Rust lines; fastwebsockets has no pinned upstream version in UPSTREAM.toml, so all of it counts. Contracts: websocket framing, handshake and mask contracts; the pipe, executor and budget harness used by protocol suites. Consumers: h12tiny-web, h12tiny and downstream websocket consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p laputa-fastwebsockets-futures-lite -p laputa-protocol-test-support; cargo clippy -p laputa-fastwebsockets-futures-lite -p laputa-protocol-test-support --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-27 · P2 · Tests: HTTP facade and clients

- **Area:** deslop · tests · HTTP facade and clients
- **Where:** crates/h12tiny, h12tiny-core, h12tiny-client, h12tiny-client-normalize, h12tiny-client-sync, h12tiny-tls-system
- **Problem:** Scope: 16.7k Rust lines, all first-party (includes h12tiny integration tests, examples and the TLS support harness). Contracts: HTTP client, TLS, facade and request/response APIs. Consumers: stuff and other published h12tiny consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p h12tiny -p h12tiny-core -p h12tiny-client -p h12tiny-client-normalize -p h12tiny-client-sync -p h12tiny-tls-system; cargo clippy -p h12tiny -p h12tiny-core -p h12tiny-client -p h12tiny-client-normalize -p h12tiny-client-sync -p h12tiny-tls-system --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-28 · P2 · Tests: HTTP server and web utilities

- **Area:** deslop · tests · HTTP server and web utilities
- **Where:** crates/h12tiny-server, h12tiny-util, h12tiny-web
- **Problem:** Scope: 7.5k Rust lines, all first-party. Contracts: HTTP server lifecycle, bounds, upgrade, body utilities, routing and websocket APIs. Consumers: stuff and other published h12tiny consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: cargo test -p h12tiny-server -p h12tiny-util -p h12tiny-web; cargo clippy -p h12tiny-server -p h12tiny-util -p h12tiny-web --all-targets; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-29 · P2 · Tests: repository tools, benchmarks and CI

- **Area:** deslop · tests · repository tools, benchmarks and CI
- **Where:** tools, scripts, tests (Python suites, tests/consumer, tests/qualify), benchmarks/twelve-dollar, fixtures, Makefile and .github
- **Problem:** Scope: about 15k lines of Rust, Python, shell and workflow files, all first-party. Contracts: release, staging, graph-gate, consumer and benchmark command contracts. Consumers: maintainers, CI and the external consumer fixture; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Audit boundary coverage, run a bounded mutation pass on the critical contracts (cargo-mutants and cargo-llvm-cov are not installed here; record Needs environment if still absent), disposition critical survivors and hold warm test cost. Acceptance: python3 -m unittest discover -s tests -p "test_*.py"; tools/graph-gate.sh; cargo test -p smolith-qualify; then make test and make lint; the deslop Tests acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-30 · P2 · Dead code: runtime I/O, processes and blocking

- **Area:** deslop · dead code · runtime I/O, processes and blocking
- **Where:** crates/async-io, polling, async-net, async-fs, async-process, async-signal, blocking, piper
- **Problem:** Scope: 20.9k Rust lines, about 1.9k changed against the pinned releases (async-io 0.64k, async-process 0.29k, blocking 0.29k, async-fs 0.28k, polling 0.2k). Contracts: runtime reactor, socket, file, process, signal, blocking-pool and pipe APIs. Consumers: laputa-smol, h12tiny and downstream runtime consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-async-io -p laputa-polling -p laputa-async-net -p laputa-async-fs -p laputa-async-process -p laputa-async-signal -p laputa-blocking -p laputa-piper; cargo clippy -p laputa-async-io -p laputa-polling -p laputa-async-net -p laputa-async-fs -p laputa-async-process -p laputa-async-signal -p laputa-blocking -p laputa-piper --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-31 · P2 · Dead code: runtime facade and utilities

- **Area:** deslop · dead code · runtime facade and utilities
- **Where:** crates/smol, smol-macros, futures-lite, async-dup, easy-parallel, fastrand, fastrand-contrib
- **Problem:** Scope: 12.4k Rust lines, about 0.55k changed against the pinned releases (futures-lite 0.47k); smol-macros, easy-parallel and fastrand-contrib (1.9k) have no pinned release to diff and count as ours. Contracts: facade re-exports, futures-lite traits, macro and random-number APIs. Consumers: laputa-smol consumers, h12tiny and downstream runtime consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-smol -p laputa-smol-macros -p laputa-futures-lite -p laputa-async-dup -p laputa-easy-parallel -p laputa-fastrand -p laputa-fastrand-contrib; cargo clippy -p laputa-smol -p laputa-smol-macros -p laputa-futures-lite -p laputa-async-dup -p laputa-easy-parallel -p laputa-fastrand -p laputa-fastrand-contrib --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-32 · P2 · Dead code: HTTP/2 framing and compression

- **Area:** deslop · dead code · HTTP/2 framing and compression
- **Where:** crates/laputa-h2-futures-lite/src/{codec,frame,futures_codec,hpack} and the codec tests (tests/h2-tests/tests/codec_read.rs, codec_write.rs)
- **Problem:** Scope: 11.5k Rust lines; futures_codec (2.0k) is wholly ours, the rest changes about 0.4k lines against h2 0.4.20. Contracts: HTTP/2 frame parsing and encoding, HPACK, and the futures-io codec that replaces tokio-util. Consumers: hyper, h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-h2-futures-lite; cargo clippy -p laputa-h2-futures-lite --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-33 · P2 · Dead code: HTTP/2 connection and streams

- **Area:** deslop · dead code · HTTP/2 connection and streams
- **Where:** crates/laputa-h2-futures-lite/src/proto and the stream-state tests (tests/h2-tests/tests/{flow_control,stream_states,prioritization,push_promise,trailers,informational_responses,ping_pong,h2_975,hammer}.rs, tests/flow_control_futures.rs)
- **Problem:** Scope: 18k Rust lines (proto 9.1k, tests 8.9k); changes about 0.4k lines against h2 0.4.20 (connection.rs control-response queue and read/write interleave, recv, send, state, streams). Contracts: HTTP/2 streams, flow control, reset accounting, control responses and connection lifecycle. Consumers: hyper, h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-h2-futures-lite -p h2-tests; cargo clippy -p laputa-h2-futures-lite -p h2-tests --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-34 · P2 · Dead code: HTTP/2 client, server and test support

- **Area:** deslop · dead code · HTTP/2 client, server and test support
- **Where:** crates/laputa-h2-futures-lite/src top-level files (client, server, share, error, lib, ext, fuzz_bridge), tests/h2-tests/tests/{client_request,server}.rs, tests/h2-support, tests/h2-fuzz, fuzz, util
- **Problem:** Scope: 11.6k Rust lines; src changes are the tokio-to-futures-io port and the tracing shim in lib.rs; the 1.6k h2-support harness and the ported tests are ours. Contracts: h2 client and server public API, the in-process test harness and the fuzz bridge. Consumers: hyper, h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-h2-futures-lite -p h2-tests -p h2-support; cargo clippy -p laputa-h2-futures-lite -p h2-tests -p h2-support --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-35 · P2 · Dead code: HTTP/1 protocol

- **Area:** deslop · dead code · HTTP/1 protocol
- **Where:** crates/laputa-hyper-futures-lite/src/proto/h1 and tests (h1_shutdown_while_buffered, h1_flush_before_yield, ready_on_poll_stream, unbuffered_stream, h1_server)
- **Problem:** Scope: 10.3k Rust lines; changes about 0.5k lines against hyper 1.12.0 (dispatch.rs pending_result and poll_close, conn.rs, io.rs, mock-io test port). Contracts: HTTP/1 request and response parsing, framing, chunked decoding and connection lifecycle. Consumers: h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-hyper-futures-lite; cargo clippy -p laputa-hyper-futures-lite --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-36 · P2 · Dead code: Hyper client, server, body and HTTP/2 glue

- **Area:** deslop · dead code · Hyper client, server, body and HTTP/2 glue
- **Where:** crates/laputa-hyper-futures-lite/src/{client,server,body,service,proto/h2} and tests/{client,server,integration}.rs
- **Problem:** Scope: 16k Rust lines (src 8k, tests 8.2k); changes about 0.4k lines against hyper 1.12.0. Contracts: Hyper client and server connection builders, Incoming body and the HTTP/2 adapter. Consumers: h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-hyper-futures-lite; cargo clippy -p laputa-hyper-futures-lite --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-37 · P2 · Dead code: Hyper runtime traits, extensions and FFI

- **Area:** deslop · dead code · Hyper runtime traits, extensions and FFI
- **Where:** crates/laputa-hyper-futures-lite/src/{rt,common,ext,ffi,upgrade,error,trace,lib,test_support}.rs and directories, tests/support, capi
- **Problem:** Scope: 7.5k Rust lines; changes about 0.2k lines against hyper 1.12.0 (rt timer, io compat, rewind, ffi task and client). Contracts: hyper::rt Read/Write/Timer/Executor traits, upgrade handling, C API. Consumers: h12tiny and downstream HTTP consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-hyper-futures-lite; cargo clippy -p laputa-hyper-futures-lite --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-38 · P2 · Dead code: websocket and protocol test support

- **Area:** deslop · dead code · websocket and protocol test support
- **Where:** crates/laputa-fastwebsockets-futures-lite and crates/laputa-protocol-test-support
- **Problem:** Scope: 3.5k Rust lines; fastwebsockets has no pinned upstream version in UPSTREAM.toml, so all of it counts. Contracts: websocket framing, handshake and mask contracts; the pipe, executor and budget harness used by protocol suites. Consumers: h12tiny-web, h12tiny and downstream websocket consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p laputa-fastwebsockets-futures-lite -p laputa-protocol-test-support; cargo clippy -p laputa-fastwebsockets-futures-lite -p laputa-protocol-test-support --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-39 · P2 · Dead code: HTTP facade and clients

- **Area:** deslop · dead code · HTTP facade and clients
- **Where:** crates/h12tiny, h12tiny-core, h12tiny-client, h12tiny-client-normalize, h12tiny-client-sync, h12tiny-tls-system
- **Problem:** Scope: 16.7k Rust lines, all first-party (includes h12tiny integration tests, examples and the TLS support harness). Contracts: HTTP client, TLS, facade and request/response APIs. Consumers: stuff and other published h12tiny consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p h12tiny -p h12tiny-core -p h12tiny-client -p h12tiny-client-normalize -p h12tiny-client-sync -p h12tiny-tls-system; cargo clippy -p h12tiny -p h12tiny-core -p h12tiny-client -p h12tiny-client-normalize -p h12tiny-client-sync -p h12tiny-tls-system --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-40 · P2 · Dead code: HTTP server and web utilities

- **Area:** deslop · dead code · HTTP server and web utilities
- **Where:** crates/h12tiny-server, h12tiny-util, h12tiny-web
- **Problem:** Scope: 7.5k Rust lines, all first-party. Contracts: HTTP server lifecycle, bounds, upgrade, body utilities, routing and websocket APIs. Consumers: stuff and other published h12tiny consumers; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: cargo test -p h12tiny-server -p h12tiny-util -p h12tiny-web; cargo clippy -p h12tiny-server -p h12tiny-util -p h12tiny-web --all-targets; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M

### DSLP-41 · P2 · Dead code: repository tools, benchmarks and CI

- **Area:** deslop · dead code · repository tools, benchmarks and CI
- **Where:** tools, scripts, tests (Python suites, tests/consumer, tests/qualify), benchmarks/twelve-dollar, fixtures, Makefile and .github
- **Problem:** Scope: about 15k lines of Rust, Python, shell and workflow files, all first-party. Contracts: release, staging, graph-gate, consumer and benchmark command contracts. Consumers: maintainers, CI and the external consumer fixture; run their tests against the candidate for any API-visible change. Upstream-derived crates are reviewed only where they diverge from UPSTREAM.toml; pristine upstream stays untouched.
- **Recommendation:** Run the dead-code tools (rustc dead_code, cargo-machete, cargo +nightly udeps where installed), then debloat; prove callers and consumers before each cut and file walk-backs as Needs owner tickets. Acceptance: python3 -m unittest discover -s tests -p "test_*.py"; tools/graph-gate.sh; cargo test -p smolith-qualify; then make test and make lint; the deslop Dead code acceptance holds within this component.
- **Fix with:** deslop
- **Effort:** M
