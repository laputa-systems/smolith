# Platforms

[rust-toolchain.toml](../rust-toolchain.toml) pins the development toolchain. [Cargo.toml](../Cargo.toml) owns the workspace minimum Rust version.

## Recorded results and current checks

Earlier recorded runs passed `cargo test --workspace` on macOS aarch64 and Linux glibc aarch64. The recorded musl aarch64 run excluded `laputa-smol` because its examples then required OpenSSL. Those dependencies and examples have since been removed; that historical exclusion is not a current OpenSSL requirement. A new full musl run is still needed.

For x86_64 musl, the workflow checks workspace libraries and tests excluding `laputa-smol`, then checks the facade library separately. These are compile checks. Cross-compilation needs a C compiler for the selected target wherever a dependency builds C code. A host's `musl-gcc` name alone does not establish its target architecture.

The recorded Linux glibc aarch64 sanitizer runs covered `wake_during_drop` with AddressSanitizer and `close_survives_force_push` with ThreadSanitizer, both using `-Zbuild-std`. Darwin sanitizers were not run. These narrow results do not qualify the entire runtime under sanitizers.

Windows is unsupported. Its target reaches `compile_error!` in polling, async-io, and async-signal.

## h12tiny

HTTP, TLS, cancellation, reuse, timeout, shutdown, and WebSocket behavior is covered by the facade integration tests under `crates/h12tiny/tests`. `make test` runs that suite with its features enabled. `sh scripts/verify-protocols.sh` runs the protocol suites on the host that executes it. A target compile check does not establish that execution.

Protocol examples run through the h12tiny facade. The H2 and WebSocket fuzz targets use futures-I/O.

The WASM browser test harness is removed because its runtime dependency includes Tokio. Its test cases remain native Rust tests. Browser execution is not maintained.

## Execution gaps

- BSD beyond macOS: run `cargo test -p laputa-polling -p laputa-async-io` on the actual target.
- Linux musl x86_64 execution and a current full musl aarch64 suite, including `laputa-smol` and the HTTP crates, remain unverified by the recorded runs.
- The protocol gate and the h12tiny integration tests need to be run on Linux before claiming those hosts passed after this move.
