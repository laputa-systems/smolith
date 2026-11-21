Prefix: DSLP
Next: 47

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier D). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After the component tickets (DSLP-3 to DSLP-41, ordered Types, Tests, Dead code), work the open P1 and P2 findings, then fill in `## Report` below.

## Report

Not started.

## Tickets

### DSLP-1 · P2 · Ground: baseline, boundaries, and CI test entry point

- **Area:** deslop · ground
- **Where:** AGENTS.md, Makefile, .github/workflows/
- **Problem:** Base 347b1d2; code lines Rust 115.2k, Python 2.9k (fleet/loc/v1, vendored included); timings not yet measured. Open fleet lint findings: mdtool 82, gha-syntax 10, linux-musl-toolchain 5, laputa-rustix 4, no-formatters 4, rust-collections 3, audit-upgrades 1, make-clean-target 1, and 2 more. Over 50k lines, so Types, Tests, and Dead code run per component; largest first-party groups: crates/laputa-h2-futures-lite (39k), crates/laputa-hyper-futures-lite (32k), crates/futures-lite (8k), crates/h12tiny (7k), crates/async-task (6k), crates/async-lock (5k), crates/polling (5k), crates/h12tiny-client (5k), and 2 more. Consumers in the fleet: bctl, ctrs, laputa, soulseek-rs, stats, stuff, tansu, xsh; their tests are the consumer check for API changes. Perf gate: decide in Ground whether a user-visible hot path needs a workload.
- **Recommendation:** Run the deslop Ground step. Accept: CI runs make test and is green, and AGENTS.md has a Verification section.
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

### DSLP-45 · P3 · h12tiny-server: an established HTTP/2 connection with no streams has no idle bound

- **Area:** deslop · diagnose · HTTP server lifecycle
- **Where:** crates/h12tiny-server/src/lifecycle.rs (production), crates/h12tiny-server/src/conn/auto.rs (Builder::http2 configuration), docs/h12tiny/limits.md
- **Problem:** Reproduced at base 5c794c6 (docs/deslop-diagnose.md, `repro_cleartext_h2_preface_then_idle_retains_capacity_after_setup_timeout` in `zz_diag`): a cleartext peer that sends the full HTTP/2 preface and then nothing keeps the only slot beyond the 50 ms setup deadline, because the deadline ends when the preface is detected. HTTP/1 has `header_read_timeout`, which `production()` sets to 10 s and which also bounds an idle keep-alive wait; HTTP/2 has no equivalent, and `production()` configures no HTTP/2 keep-alive interval or idle timeout. This is the same slot-exhaustion exposure as the preface case after the preface, with the documented setup deadline behaving as written, so it is a missing bound rather than a regression.
- **Recommendation:** Decide the production bound for an HTTP/2 connection with no open streams (an idle timeout, or hyper HTTP/2 keep-alive pings with a timeout, which need the timer `production()` already installs) and record it in docs/h12tiny/limits.md. Revalidate that hyper exposes a suitable knob before choosing; if it does not, the choice belongs to the owner. Acceptance: a regression test shows an idle HTTP/2 peer is closed under `production()` while a connection with an active stream is not; make test and make lint.
- **Fix with:** lifecycle-concurrency
- **Effort:** M

### DSLP-46 · P4 · Lint baseline: the Hyper fork inherits the upstream clippy restriction group and make lint reports about 2,400 diagnostics

- **Area:** deslop · ground · lint baseline
- **Where:** crates/laputa-hyper-futures-lite/Cargo.toml ([lints.clippy] restriction = warn), crates/laputa-h2-futures-lite/Cargo.toml and crates/h12tiny-client/Cargo.toml (manifest warnings)
- **Problem:** `make lint` exits 0 but prints about 2,400 warning diagnostics (65 distinct messages) at base 5c794c6: 673 `unwrap()` on a Result, 408 `#[test]` outside a `#[cfg(test)]` module, 301 `allow` without a reason, 284 `#[allow]`, mostly in the Hyper fork and its tests because the fork keeps hyper's `restriction` group at warn. The manifests also warn about `package.readme` (`cargo::manual_readme`) in both protocol forks and about unused dependencies (h12tiny-client: `futures-util`, `h12tiny-client-normalize`, `h12tiny-core`; h12tiny-server: `async-net`, `futures-util`, `h12tiny-core`). No lint is denied, so a real warning cannot stand out and CI cannot gate on a clean run.
- **Recommendation:** In the Types and Dead code tickets for the Hyper and h12tiny components: remove the six unused dependency entries after checking each feature set (the compiler reports them for the default `make lint` feature set only), drop or silence `readme` only if the owner wants it, and decide whether the fork keeps the restriction group; if it stays, allow the groups that are upstream style in tests with one reasoned entry each. Do not edit pristine upstream files for style. Acceptance: make lint warning count recorded before and after in the PR.
- **Fix with:** deslop
- **Effort:** M
