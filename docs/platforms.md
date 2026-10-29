# Platforms

The development toolchain is `nightly-2026-09-15` from `rust-toolchain.toml`. The workspace `rust-version` is 1.85.

## Recorded results

macOS aarch64 runs `cargo test --workspace`. Linux glibc aarch64 ran the same command in Docker on this nightly.

Linux musl aarch64 runs `cargo test --workspace --exclude laputa-smol`. The full workspace does not build there: `openssl-sys` has no musl sysroot, and that crate is a dev-dependency of `laputa-smol`. `laputa-smol` tests did not run on musl.

Linux musl x86_64 is compile-only on this host. `cargo check --workspace --lib --tests --target x86_64-unknown-linux-musl --exclude laputa-smol` is the check that fits. That check builds criterion's `alloca`, which needs a musl C compiler. CI installs `musl-tools` and sets `CC_x86_64_unknown_linux_musl` to `musl-gcc`. `cargo check -p laputa-smol --lib --target x86_64-unknown-linux-musl` builds the facade library.

On Linux glibc aarch64, AddressSanitizer covered `wake_during_drop` and ThreadSanitizer covered `close_survives_force_push`, both with `-Zbuild-std`. Darwin sanitizers were not run.

A Windows target hits `compile_error!` in `crates/polling/src/lib.rs`, `crates/async-io/src/reactor.rs`, and `crates/async-signal/src/lib.rs`.

`crates/smol/examples/async-h1-server.rs`, `crates/smol/examples/async-h1-client.rs`, and `crates/smol/examples/web-crawler.rs` have an empty `main` on Linux. `rustix` 0.37, through `async-h1` and `surf`, does not compile on this nightly for that OS. Those examples still build on macOS.

## h12tiny

`fixtures/h12tiny.toml` pins the fixture commit. `fixtures/h12tiny.sh` clones that commit, retargets the smol edges at these crates, and runs the HTTP, TLS, cancellation, reuse, timeout, and shutdown gates. The backpressure tests live in tokio harnesses outside that workspace. `fixtures/h12tiny.py` records that gate as a gap and does not treat it as a pass.

## Still needs a machine

- BSD, beyond the macOS kqueue path: `cargo test -p laputa-polling -p laputa-async-io`.
- Linux musl x86_64 execution, and `laputa-smol` tests on musl aarch64. The first needs qemu or a musl cross C toolchain. The second needs a musl OpenSSL sysroot for `openssl-sys`.
- The h12tiny TLS and HTTP/2 gates on hosts this checkout cannot reach. Run `fixtures/h12tiny.sh` there.
