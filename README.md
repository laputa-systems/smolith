# smolith

smolith is the smol async runtime and the crates it is built from. The facade package is `laputa-smol`. Its library name is `smol`, so application code writes `smol::`.

The other packages are `laputa-<upstream-name>` and keep the original library names. `UPSTREAM.toml` records each upstream repository, commit, and version.

These crates are not on crates.io. Depend on a checkout.

## Add the runtime

In a crate outside this workspace:

```toml
[dependencies]
smol = { package = "laputa-smol", version = "0.1.0", path = "../smolith/crates/smol" }
```

`smol::block_on` runs a future to completion. `smol::spawn` schedules work on the global executor. `smol::net` is TCP, UDP, and Unix sockets. `smol::fs` is the filesystem. `smol::channel` is a multi-producer multi-consumer channel.

`smol::io::AsyncRead` and `smol::io::AsyncWrite` are the `futures-io` traits. `smol::prelude::Stream` is `futures_core::stream::Stream`.

The workspace `rust-version` is 1.85. A Windows target stops at `compile_error!` in `crates/polling`, `crates/async-io`, and `crates/async-signal`.

## Upstream interfaces

`futures-core`, `futures-io`, and `futures-sink` are the shared trait crates. Smolith implements those traits and leaves the crates upstream.

`futures-channel` and `futures-util` are futures-rs crates. An application keeps them upstream. `futures-rustls` is the TLS adapter that connects those I/O traits to Rustls, and it stays upstream. Upstream archived `async-rustls` in favor of `futures-rustls`.

## License

Each crate is Apache-2.0 OR MIT. The texts are `LICENSE-APACHE` and `LICENSE-MIT` in that crate's directory.

## Security

Report a vulnerability through the form linked from [SECURITY.md](SECURITY.md).
