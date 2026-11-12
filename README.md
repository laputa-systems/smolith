# smolith

smolith is the smol async runtime and the crates it is built from. The facade package is `laputa-smol`. Its library name is `smol`, so application code writes `smol::`.

The other packages are `laputa-<upstream-name>` and keep the original library names. `UPSTREAM.toml` records each upstream repository, commit, and version.

The dependency example below uses a checkout.

## Add the runtime

In a crate outside this workspace:

```toml
[dependencies]
smol = { package = "laputa-smol", version = "0.3.0", path = "../smolith/crates/smol" }
```

`smol::block_on` runs a future to completion. `smol::spawn` schedules work on the global executor. `smol::net` is TCP, UDP, and Unix sockets. `smol::fs` is the filesystem. `smol::channel` is a multi-producer multi-consumer channel.

`smol::io::AsyncRead` and `smol::io::AsyncWrite` are the `futures-io` traits. `smol::prelude::Stream` is `futures_core::stream::Stream`.

The workspace `rust-version` is 1.85. HTTP crates declare their own minimums, through 1.89 for the facade. A Windows target stops at `compile_error!` in `crates/polling`, `crates/async-io`, and `crates/async-signal`.

## HTTP

h12tiny is the HTTP facade. Its library has no default features. The component crates (`h12tiny-core`, `h12tiny-client`, `h12tiny-client-sync`, `h12tiny-server`, and the others) can be used directly. Protocol forks are `laputa-h2-futures-lite`, `laputa-hyper-futures-lite`, and `laputa-fastwebsockets-futures-lite`.

Registry users keep these package names. Git users clone `https://github.com/laputa-systems/smolith` and select the same packages. Path users point at `crates/h12tiny` and the sibling component directories. Older h12tiny commits, tags, and published crates stay available.

```toml
[dependencies]
h12tiny = { version = "0.9", default-features = false, features = ["client", "http1"] }
smol = { package = "laputa-smol", version = "0.3" }
```

Family notes live in [docs/h12tiny](docs/h12tiny/architecture.md).

## Upstream interfaces

`futures-core`, `futures-io`, and `futures-sink` are the shared trait crates. Smolith implements those traits and leaves the crates upstream.

`futures-channel` and `futures-util` are futures-rs crates. An application keeps them upstream. `futures-rustls` is the TLS adapter that connects those I/O traits to Rustls, and it stays upstream. Upstream archived `async-rustls` in favor of `futures-rustls`.

## License

Runtime crates are Apache-2.0 OR MIT, with `LICENSE-APACHE` and `LICENSE-MIT` in the crate directory. h12tiny crates and the H2 and Hyper forks are MIT. fastwebsockets is Apache-2.0. Each crate carries its own license file.

## Security

Report a vulnerability through the form linked from [SECURITY.md](SECURITY.md).
