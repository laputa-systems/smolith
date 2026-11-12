# Platforms

[rust-toolchain.toml](../rust-toolchain.toml) pins the development toolchain. [Cargo.toml](../Cargo.toml) owns the workspace minimum Rust version.

## Recorded results and current checks

Earlier recorded runs passed `cargo test --workspace` on macOS aarch64 and Linux glibc aarch64. The recorded musl aarch64 run excluded `laputa-smol` because its examples then required OpenSSL. Those dependencies and examples have since been removed; that historical exclusion is not a current OpenSSL requirement. A new full musl run is still needed.

For x86_64 musl, the workflow checks workspace libraries and tests excluding `laputa-smol`, then checks the facade library separately. These are compile checks. Cross-compilation needs a C compiler for the selected target wherever a dependency builds C code. A host's `musl-gcc` name alone does not establish its target architecture.

The recorded Linux glibc aarch64 sanitizer runs covered `wake_during_drop` with AddressSanitizer and `close_survives_force_push` with ThreadSanitizer, both using `-Zbuild-std`. Darwin sanitizers were not run. These narrow results do not qualify the entire runtime under sanitizers.

Windows is unsupported. Its target reaches `compile_error!` in polling, async-io, and async-signal.

## h12tiny

[fixtures/h12tiny.toml](../fixtures/h12tiny.toml) pins the consumer revision. `fixtures/h12tiny.sh` retargets its runtime edges to this checkout and runs the HTTP, TLS, cancellation, reuse, timeout, and shutdown gates. `H12TINY_MIRROR` selects the clone source.

The fixture's backpressure tests live in Tokio harnesses outside that workspace. `fixtures/h12tiny.py` records that gate as a gap rather than a pass.

## Execution gaps

- BSD beyond macOS: run `cargo test -p laputa-polling -p laputa-async-io` on the actual target.
- Linux musl x86_64 execution and a current full musl aarch64 suite, including `laputa-smol`, remain unverified by the recorded runs.
- Run `fixtures/h12tiny.sh` on a host with access to its pinned repository and dependency sources before claiming the HTTP/2 and TLS gates passed.
