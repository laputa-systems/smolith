# smol

A small and fast async runtime.

This crate simply re-exports other smaller async crates (see the source).

To use tokio-based libraries with smol, apply the [`async-compat`] adapter to futures and I/O
types.

See the [`smol-macros`] crate if you want a no proc-macro, fast compiling, easy-to-use
async main and/or multi-threaded Executor setup out of the box.

## Examples

Connect to an HTTP website, make a GET request, and pipe the response to the standard output:

```rust,no_run
use smol::{io, net, prelude::*, Unblock};

fn main() -> io::Result<()> {
    smol::block_on(async {
        let mut stream = net::TcpStream::connect("example.com:80").await?;
        let req = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        stream.write_all(req).await?;

        let mut stdout = Unblock::new(std::io::stdout());
        io::copy(stream, &mut stdout).await?;
        Ok(())
    })
}
```

There's a lot more in the [examples](https://github.com/laputa-systems/smolith/tree/main/crates/smol/examples) directory.

To call Tokio libraries from this runtime, adapt the futures with [async-compat](https://docs.rs/async-compat/latest/async_compat/). That crate is not part of this workspace. [`smol-macros`](https://github.com/laputa-systems/smolith/tree/main/crates/smol-macros) provides the macros.

## Subcrates

- `async-channel`: multi-producer multi-consumer channels
- `async-executor`: composable async executors
- `async-fs`: async filesystem primitives
- `async-io`: async adapter for I/O types, and timers
- `async-lock`: async locks (barrier, mutex, reader-writer lock, semaphore)
- `async-net`: async TCP, UDP, and Unix sockets
- `async-process`: async processes
- `async-task`: task abstraction for executors
- `blocking`: a thread pool for blocking I/O
- `futures-lite`: futures, streams, and async I/O combinators
- `polling`: epoll, kqueue, and event ports

## TLS certificate

Some code examples are using TLS for authentication. The repository
contains a self-signed certificate usable for testing, but it should **not**
be used for real-world scenarios. Browsers and tools like curl will
show this certificate as insecure.

In browsers, accept the security prompt or use `curl -k` on the
command line to bypass security warnings.

The certificate file was generated using
[minica](https://github.com/jsha/minica) and
[openssl](https://www.openssl.org/):

```text
minica --domains localhost -ip-addresses 127.0.0.1 -ca-cert certificate.pem
openssl pkcs12 -export -out identity.pfx -inkey localhost/key.pem -in localhost/cert.pem
```

Another useful tool for making certificates is [mkcert].

[mkcert]: https://github.com/FiloSottile/mkcert

## MSRV Policy

The Minimum Supported Rust Version (MSRV) of this crate is **1.85**. As a **tentative** policy, the MSRV will not advance past the [current Rust version provided by Debian Stable](https://packages.debian.org/stable/rust/rustc). At the time of writing, this version of Rust is *1.85*. However, the MSRV may be advanced further in the event of a major ecosystem shift or a security vulnerability.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
