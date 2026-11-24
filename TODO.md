Prefix: SMOL
Next: 28

HTTP findings use the H12T prefix and stay in [docs/h12tiny/TODO.md](docs/h12tiny/TODO.md). Their numbers are unchanged.

### SMOL-14 · P3 · h12tiny-server: an established HTTP/2 connection with no streams has no idle bound

- **Area:** HTTP server lifecycle
- **Where:** crates/h12tiny-server/src/lifecycle.rs (production), crates/h12tiny-server/src/conn/auto.rs (Builder::http2 configuration), docs/h12tiny/limits.md
- **Problem:** A cleartext peer that sends the full HTTP/2 preface and then nothing was observed to retain the only connection slot beyond a 50 ms setup deadline, because the deadline ends after the preface. HTTP/1 has header_read_timeout, which production() sets to 10 s and which also bounds an idle keep-alive wait; HTTP/2 has no equivalent, and production() configures no HTTP/2 keep-alive interval or idle timeout. This is a missing idle bound rather than a setup deadline regression.
- **Recommendation:** Decide the production bound for an HTTP/2 connection with no open streams (an idle timeout, or hyper HTTP/2 keep-alive pings with a timeout, which need the timer `production()` already installs) and record it in docs/h12tiny/limits.md. Revalidate that hyper exposes a suitable knob before choosing; if it does not, the choice belongs to the owner. Acceptance: a regression test shows an idle HTTP/2 peer is closed under `production()` while a connection with an active stream is not; make test and make lint.
- **Fix with:** lifecycle-concurrency
- **Effort:** M

### SMOL-15 · P3 · Hyper FFI and capi are compiled but neither executed nor consumed

- **Area:** Needs owner · Hyper FFI
- **Where:** crates/laputa-hyper-futures-lite/src/ffi/, crates/laputa-hyper-futures-lite/capi/, .github/workflows/ci.yml:42
- **Problem:** CI compiles the ffi feature under RUSTFLAGS=--cfg hyper_unstable_ffi (cargo check, ci.yml:42-43) but no repository check runs its tests or builds capi, and no workspace crate or documented consumer uses the C API. It is inherited upstream surface with no evidence that it works in this fork.
- **Recommendation:** Owner decision: either remove the ffi feature, src/ffi and capi from the fork, or keep them and add an executed FFI test lane (cargo test --features ffi under the unstable cfg) to CI.
- **Fix with:** dead-code-archaeology
- **Effort:** S

### SMOL-16 · P3 · CI verify job has no timeout

- **Area:** Needs owner · CI
- **Where:** .github/workflows/ci.yml:23
- **Problem:** The verify job runs make test with no timeout-minutes. A hung accept_descriptor_exhaustion test before #24 held a Linux runner for over an hour before it was cancelled by hand.
- **Recommendation:** Owner decision on CI work: add timeout-minutes to the verify and macOS test jobs, sized from recent run durations (about 6 minutes) with headroom.
- **Fix with:** none
- **Effort:** S

### SMOL-18 · P3 · HTTP/2 DATA frames lack malformed padding regression coverage

- **Area:** HTTP/2 framing
- **Where:** crates/laputa-h2-futures-lite/src/frame/util.rs; crates/laputa-h2-futures-lite/tests/h2-tests/tests/codec_read.rs
- **Problem:** The DATA frame tests cover valid padding but do not exercise an absent padding length or a padding length exceeding the payload; the frame parser rejects these in strip_padding, leaving those protocol boundaries unguarded.
- **Recommendation:** Add wire-level tests for both invalid boundaries and verify the resulting protocol error.
- **Fix with:** none
- **Effort:** M

### SMOL-19 · P3 · HTTP/2 receive window expansion lacks update coverage

- **Area:** HTTP/2 flow control
- **Where:** crates/laputa-h2-futures-lite/tests/h2-tests/tests/flow_control.rs
- **Problem:** The inherited expand_window_sends_window_update and expand_window_calls_are_coalesced tests are ignored empty functions, so they do not guard window update emission or coalescing.
- **Recommendation:** Add observable flow-control tests for window expansion and update coalescing.
- **Fix with:** none
- **Effort:** M

### SMOL-20 · P3 · HTTP/2 connection window update overflow lacks coverage

- **Area:** HTTP/2 flow control
- **Where:** crates/laputa-h2-futures-lite/tests/h2-tests/tests/flow_control.rs; crates/laputa-h2-futures-lite/src/proto/streams/flow_control.rs
- **Problem:** The inherited recv_window_update_causes_overflow test is ignored and empty. poll_capacity_woken_on_library_reset covers stream-window overflow, but no connection-window overflow assertion was found.
- **Recommendation:** Add a peer connection WINDOW_UPDATE that overflows the permitted window and assert the connection-level protocol error.
- **Fix with:** none
- **Effort:** M

### SMOL-21 · P3 · HTTP/2 received reset capacity release lacks coverage

- **Area:** HTTP/2 flow control
- **Where:** crates/laputa-h2-futures-lite/tests/h2-tests/tests/flow_control.rs
- **Problem:** The inherited stream_close_by_recv_reset_frame_releases_capacity test is ignored and empty; the neighboring test covers a sent reset, leaving received-reset capacity release unguarded.
- **Recommendation:** Add a test showing that a received reset releases connection capacity for other streams.
- **Fix with:** none
- **Effort:** M

### SMOL-22 · P3 · HTTP/2 client request version boundary lacks coverage

- **Area:** HTTP/2 client
- **Where:** crates/laputa-h2-futures-lite/tests/h2-tests/tests/client_request.rs
- **Problem:** The inherited request_with_h1_version test is ignored and empty, so it does not establish the behavior of an HTTP/1-version request passed to the HTTP/2 client.
- **Recommendation:** Determine the intended public behavior and add a test that asserts it.
- **Fix with:** none
- **Effort:** M

### SMOL-23 · P3 · HTTP/2 server pending accepts after socket close lack coverage

- **Area:** HTTP/2 server
- **Where:** crates/laputa-h2-futures-lite/tests/h2-tests/tests/server.rs
- **Problem:** The inherited accept_with_pending_connections_after_socket_close test is ignored and empty, leaving server behavior for pending connections after socket closure unguarded.
- **Recommendation:** Determine the intended behavior and add a test covering pending accepts after socket closure.
- **Fix with:** none
- **Effort:** M

### SMOL-25 · P3 · Shared async I/O forwarding lacks a runtime boundary test

- **Area:** runtime I/O
- **Where:** crates/async-dup/src/lib.rs
- **Problem:** The crate checks Send and Sync but has no runtime test of forwarding reads and writes through its shared wrappers.
- **Recommendation:** Exercise shared reads and writes, including contention or wakeup behavior that callers depend on.
- **Fix with:** none
- **Effort:** M

### SMOL-17 · P4 · Lint baseline: the Hyper fork inherits the upstream clippy restriction group and make lint reports about 2,400 diagnostics

- **Area:** lint baseline
- **Where:** crates/laputa-hyper-futures-lite/Cargo.toml ([lints.clippy] restriction = warn), crates/laputa-h2-futures-lite/Cargo.toml and crates/h12tiny-client/Cargo.toml (manifest warnings)
- **Problem:** `make lint` exits 0 but prints about 2,400 warning diagnostics (65 distinct messages) at base 5c794c6: 673 `unwrap()` on a Result, 408 `#[test]` outside a `#[cfg(test)]` module, 301 `allow` without a reason, 284 `#[allow]`, mostly in the Hyper fork and its tests because the fork keeps hyper's `restriction` group at warn. The manifests also warn about `package.readme` (`cargo::manual_readme`) in both protocol forks and about unused dependencies (h12tiny-client: `futures-util`, `h12tiny-client-normalize`, `h12tiny-core`; h12tiny-server: `async-net`, `futures-util`, `h12tiny-core`). No lint is denied, so a real warning cannot stand out and CI cannot gate on a clean run.
- **Recommendation:** Check each feature set before removing the six reported unused dependencies; the compiler reports them for the default make lint feature set only. Drop or silence readme only if the owner wants it, and decide whether the fork keeps the restriction group; if it stays, allow upstream test style groups with reasoned entries. Do not edit pristine upstream files for style. Record the make lint warning count before and after in the PR.
- **Fix with:** none
- **Effort:** M

### SMOL-24 · P4 · Decide whether manually invoked benchmark fixtures remain supported

- **Area:** benchmark tooling
- **Where:** benchmarks/twelve-dollar/tests/cancel-macos.sh; benchmarks/twelve-dollar/tests/cancel-smoke.sh; benchmarks/twelve-dollar/tests/summarize-latency.sh
- **Problem:** These executable test fixtures have no documented or automated invocation found by name; they may be manually used, so their intended maintenance status is unclear.
- **Recommendation:** Ask the owner whether manual invocation is still wanted; document and maintain the entry points if so, otherwise remove them after confirming no external users.
- **Fix with:** dead-code-archaeology
- **Effort:** S

### SMOL-26 · P4 · Check external use of the authenticated system TLS helper

- **Area:** TLS client
- **Where:** crates/h12tiny-tls-system/src/lib.rs
- **Problem:** system_client_config_with_auth is public and has no identified workspace caller; published consumers may rely on it.
- **Recommendation:** Check published consumer use and retain the API unless its removal is justified.
- **Fix with:** dead-code-archaeology
- **Effort:** S

### SMOL-27 · P4 · Check external use of HTTP utility conveniences

- **Area:** HTTP utilities
- **Where:** crates/h12tiny-util/src/lib.rs
- **Problem:** The public static_bytes, RequestFactory and replayable_value helpers have no identified workspace callers; published consumers may rely on them.
- **Recommendation:** Check published consumer use and retain the APIs unless removal is justified.
- **Fix with:** dead-code-archaeology
- **Effort:** S

