# Diagnose repros

Reproductions behind the DSLP-2 findings, taken at base 5c794c6 on the pinned nightly (nightly-2026-09-15), macOS aarch64. Each is a scratch test: copy it to the named path, run the command, then delete it. None is kept in the tree; a fix adds its own regression test. Loopback only, port 0, no external network.

## Silent HTTP/2 peer holds an admission slot (TLS) and an idle HTTP/2 connection has no bound (cleartext)

Copy to `crates/h12tiny/tests/zz_diag.rs` and run `cargo test -p h12tiny --all-features --test zz_diag`. The tests assert the retained slot, so a pass means the defect reproduces. The control shows the setup deadline works for a silent TCP peer that never starts TLS.

Observed: `repro_tls_h2_silent_preface_retains_capacity_after_setup_timeout ... ok`, `repro_cleartext_h2_preface_then_idle_retains_capacity_after_setup_timeout ... ok`, `control_silent_tcp_before_tls_handshake_releases_capacity ... ok`; 3 passed in 0.50 s.

Cause: `crates/h12tiny-server/src/lifecycle.rs` `run_tls` bounds only `acceptor.accept(stream)` with the setup deadline. `Builder::serve_tls_connection` in `crates/h12tiny-server/src/conn/auto.rs` then builds `ConnState::H2` for ALPN `h2`, and only `ConnState::ReadVersion` carries a `setup_deadline`. The TLS client never has to send the 24-byte preface. For cleartext, the deadline also ends once detection has seen the full preface.

```rust
#![cfg(all(feature = "server", feature = "http1", feature = "http2", feature = "tls"))]

mod support;

use std::convert::Infallible;
use std::future::Future;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;

use async_net::{TcpListener, TcpStream};
use futures_channel::oneshot;
use futures_lite::io::{AsyncReadExt, AsyncWriteExt};
use futures_rustls::{TlsAcceptor, TlsConnector};
use futures_util::future::{self, Either};
use h12tiny::io::FuturesIo;
use h12tiny::runtime::BoxExecutor;
use h12tiny::server::conn::auto;
use http::{Request, Response};
use hyper::service::service_fn;
use support::{FullBody, SmolExecutor, collect, fixture_client_config, fixture_server_config};

type ServerTls = futures_rustls::server::TlsStream<TcpStream>;
type ClientTls = futures_rustls::client::TlsStream<TcpStream>;

async fn bounded<F: Future>(future: F) -> F::Output {
    match future::select(Box::pin(future), async_io::Timer::after(Duration::from_secs(2))).await {
        Either::Left((result, _)) => result,
        Either::Right(_) => panic!("connection operation did not complete"),
    }
}

async fn tls_pair(alpn: Option<&[u8]>) -> (ServerTls, ClientTls) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let protocols = alpn.into_iter().map(<[u8]>::to_vec).collect::<Vec<_>>();
    let mut server_config = fixture_server_config();
    server_config.alpn_protocols = protocols.clone();
    let mut client_config = fixture_client_config();
    client_config.alpn_protocols = protocols;
    let server = async move {
        let (stream, _) = listener.accept().await.unwrap();
        TlsAcceptor::from(Arc::new(server_config)).accept(stream).await.unwrap()
    };
    let client = async move {
        let stream = TcpStream::connect(address).await.unwrap();
        TlsConnector::from(Arc::new(client_config))
            .connect(rustls::pki_types::ServerName::try_from("localhost").unwrap(), stream)
            .await.unwrap()
    };
    bounded(future::join(server, client)).await
}


#[test]
fn repro_tls_h2_silent_preface_retains_capacity_after_setup_timeout() {
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let rejected = Arc::new(AtomicUsize::new(0));
        let hook = rejected.clone();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let server = smol::spawn(async move {
            let mut config = fixture_server_config();
            config.alpn_protocols = vec![b"h2".to_vec()];
            h12tiny::server::serve_tls(
                listener,
                TlsAcceptor::from(Arc::new(config)),
                service_fn(|_| async { Ok::<_, Infallible>(Response::new(FullBody::from_static(b"ok"))) }),
                BoxExecutor::new(SmolExecutor),
            )
            .setup_timeout(Duration::from_millis(50)).unwrap()
            .max_connections(1)
            .on_connection_rejected(move || { hook.fetch_add(1, Ordering::SeqCst); })
            .shutdown_on(shutdown_rx).await.unwrap();
        });
        let stream = TcpStream::connect(address).await.unwrap();
        let mut config = fixture_client_config();
        config.alpn_protocols = vec![b"h2".to_vec()];
        let peer = bounded(TlsConnector::from(Arc::new(config)).connect(
            rustls::pki_types::ServerName::try_from("localhost").unwrap(), stream)).await.unwrap();
        async_io::Timer::after(Duration::from_millis(250)).await;
        let _second = TcpStream::connect(address).await.unwrap();
        for _ in 0..100 {
            if rejected.load(Ordering::SeqCst) != 0 { break; }
            async_io::Timer::after(Duration::from_millis(10)).await;
        }
        assert_eq!(rejected.load(Ordering::SeqCst), 1, "silent H2 peer retained slot");
        drop(peer);
        shutdown_tx.send(()).unwrap();
        bounded(server).await;
    });
}

#[test]
fn repro_cleartext_h2_preface_then_idle_retains_capacity_after_setup_timeout() {
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let rejected = Arc::new(AtomicUsize::new(0));
        let hook = rejected.clone();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let server = smol::spawn(async move {
            h12tiny::server::serve(
                listener,
                service_fn(|_| async { Ok::<_, Infallible>(Response::new(FullBody::from_static(b"ok"))) }),
                BoxExecutor::new(SmolExecutor),
            )
            .setup_timeout(Duration::from_millis(50)).unwrap()
            .max_connections(1)
            .on_connection_rejected(move || { hook.fetch_add(1, Ordering::SeqCst); })
            .shutdown_on(shutdown_rx).await.unwrap();
        });
        let mut peer = TcpStream::connect(address).await.unwrap();
        peer.write_all(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n").await.unwrap();
        peer.flush().await.unwrap();
        async_io::Timer::after(Duration::from_millis(250)).await;
        let _second = TcpStream::connect(address).await.unwrap();
        for _ in 0..100 {
            if rejected.load(Ordering::SeqCst) != 0 { break; }
            async_io::Timer::after(Duration::from_millis(10)).await;
        }
        assert_eq!(rejected.load(Ordering::SeqCst), 1, "idle H2 peer retained slot");
        drop(peer);
        shutdown_tx.send(()).unwrap();
        bounded(server).await;
    });
}

#[test]
fn control_silent_tcp_before_tls_handshake_releases_capacity() {
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let rejected = Arc::new(AtomicUsize::new(0));
        let hook = rejected.clone();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let server = smol::spawn(async move {
            let mut config = fixture_server_config();
            config.alpn_protocols = vec![b"h2".to_vec()];
            h12tiny::server::serve_tls(
                listener,
                TlsAcceptor::from(Arc::new(config)),
                service_fn(|_| async { Ok::<_, Infallible>(Response::new(FullBody::from_static(b"ok"))) }),
                BoxExecutor::new(SmolExecutor),
            )
            .setup_timeout(Duration::from_millis(50)).unwrap()
            .max_connections(1)
            .on_connection_rejected(move || { hook.fetch_add(1, Ordering::SeqCst); })
            .shutdown_on(shutdown_rx).await.unwrap();
        });
        let silent = TcpStream::connect(address).await.unwrap();
        async_io::Timer::after(Duration::from_millis(250)).await;
        let _second = TcpStream::connect(address).await.unwrap();
        async_io::Timer::after(Duration::from_millis(250)).await;
        assert_eq!(rejected.load(Ordering::SeqCst), 0, "control: setup deadline must release the slot");
        drop(silent);
        shutdown_tx.send(()).unwrap();
        bounded(server).await;
    });
}
```

## Descriptor exhaustion ends the whole listener

Copy to `crates/h12tiny/tests/zz_emfile.rs` and run `cargo test -p h12tiny --all-features --test zz_emfile -- --nocapture --test-threads=1`. The parent re-executes itself under `ulimit -n 48`, holds 200 idle connections against the child's `serve` listener (`max_connections(1024)`), and asserts the listener keeps running. The assertion fails.

Observed: `serve exit line: Some("SERVE_EXITED Err(Os { code: 24, kind: TooManyOpenFiles, message: \"Too many open files\" })"); connect after release: Err(... ConnectionRefused ...)`; `test result: FAILED. 1 passed; 1 failed`.

Cause: `is_transient_accept_error` in `crates/h12tiny-server/src/lifecycle.rs` lists Interrupted, WouldBlock, ConnectionAborted, ConnectionReset and TimedOut. EMFILE and ENFILE are not retried, so `run` and `run_tls` drain and return the error.

```rust
#![cfg(all(feature = "server", feature = "http1"))]

mod support;

use std::convert::Infallible;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use async_net::TcpListener;
use h12tiny::runtime::BoxExecutor;
use h12tiny::server::serve;
use http::Response;
use hyper::service::service_fn;
use support::{FullBody, SmolExecutor};

/// Runs only when re-executed by the parent test under a low descriptor limit.
#[test]
fn emfile_child() {
    if std::env::var_os("H12T_EMFILE_CHILD").is_none() {
        return;
    }
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        println!("PORT {}", listener.local_addr().unwrap().port());
        let result = serve(
            listener,
            service_fn(|_| async { Ok::<_, Infallible>(Response::new(FullBody::from_static(b"ok"))) }),
            BoxExecutor::new(SmolExecutor),
        )
        .max_connections(1024)
        .shutdown_on(std::future::pending::<()>())
        .await;
        println!("SERVE_EXITED {result:?}");
    });
}

#[test]
fn repro_descriptor_exhaustion_stops_the_listener() {
    let exe = std::env::current_exe().unwrap();
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("ulimit -n 48; exec \"$0\" --exact emfile_child --nocapture --test-threads=1")
        .arg(&exe)
        .env("H12T_EMFILE_CHILD", "1")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let port: u16 = loop {
        let line = lines.next().unwrap().unwrap();
        if let Some(i) = line.find("PORT ") { break line[i + 5..].trim().parse().unwrap(); }
    };
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || { for line in lines.map_while(Result::ok) { let _ = tx.send(line); } });
    // Hold 200 idle connections open; the child allows 48 descriptors in total.
    let mut held = Vec::new();
    for _ in 0..200 { if let Ok(s) = std::net::TcpStream::connect(("127.0.0.1", port)) { held.push(s); } }
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut exited = None;
    while Instant::now() < deadline {
        if let Ok(line) = rx.recv_timeout(Duration::from_millis(100)) {
            if line.contains("SERVE_EXITED") { exited = Some(line); break; }
        }
    }
    // Connections released: a healthy server would accept again.
    drop(held);
    let after = std::net::TcpStream::connect(("127.0.0.1", port));
    let status = child.try_wait().unwrap();
    let _ = child.kill();
    eprintln!("serve exit line: {exited:?}; connect after release: {after:?}; child status: {status:?}");
    assert!(exited.is_none(), "listener stopped on descriptor exhaustion: {exited:?}");
}
```

## Per-read zeroing makes trickle cost scale with the advertised HTTP/2 frame size

Copy to `crates/laputa-h2-futures-lite/tests/zz_h2_diag.rs` and run `cargo test --release -p laputa-h2-futures-lite --test zz_h2_diag -- --nocapture`. A scripted peer delivers one byte per `poll_read`: preface, empty SETTINGS, then a DATA frame header that declares the advertised maximum length and 200 payload bytes.

Observed (release, busy host):

| `max_frame_size` | 200 trickled bytes | per byte |
| --- | --- | --- |
| 16,384 | 164 us | 0.8 us |
| 262,144 | 322 us | 1.6 us |
| 1,048,576 | 1.07 ms | 5.3 us |
| 16,777,215 | 20.96 ms | 104.8 us |

Attribution: with the zeroing in `poll_next` capped at 8 KiB of spare capacity (scratch edit, reverted), the same run gave 0.8, 0.2, 0.1 and 0.1 us per byte. Cause: `crates/laputa-h2-futures-lite/src/futures_codec/framed_impl.rs` `poll_next` writes zero to the whole spare capacity of the read buffer before every `poll_read`. The length-delimited decoder reserves the declared frame length first, so the spare capacity grows to the advertised maximum.

Caution when adapting the test: an earlier variant that ignored `Err` from `accept()` did not terminate, so the loop must break on the first error.

```rust
use futures_io::{AsyncRead, AsyncWrite};
use laputa_protocol_test_support::executor::block_on_with_limit;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

const PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

/// Delivers a scripted byte stream one byte per `poll_read`, then EOF.
struct Trickle { script: Vec<u8>, pos: usize }
impl AsyncRead for Trickle {
    fn poll_read(mut self: Pin<&mut Self>, _: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        if self.pos == self.script.len() { return Poll::Ready(Ok(0)); }
        buf[0] = self.script[self.pos];
        self.pos += 1;
        Poll::Ready(Ok(1))
    }
}
impl AsyncWrite for Trickle {
    fn poll_write(self: Pin<&mut Self>, _: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> { Poll::Ready(Ok(buf.len())) }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> { Poll::Ready(Ok(())) }
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> { Poll::Ready(Ok(())) }
}

fn script(payload_bytes: usize, frame_len: u32) -> Vec<u8> {
    let mut s = PREFACE.to_vec();
    s.extend_from_slice(&[0, 0, 0, 4, 0, 0, 0, 0, 0]); // empty SETTINGS
    s.extend_from_slice(&frame_len.to_be_bytes()[1..]); // length (24 bit)
    s.extend_from_slice(&[0, 0, 0, 0, 0, 1]);           // DATA, no flags, stream 1
    s.extend(std::iter::repeat(0u8).take(payload_bytes));
    s
}

fn run(max_frame_size: u32, payload_bytes: usize, frame_len: u32) -> Duration {
    let start = Instant::now();
    block_on_with_limit(async move {
        let io = Trickle { script: script(payload_bytes, frame_len), pos: 0 };
        let mut conn = h2::server::Builder::new().max_frame_size(max_frame_size).handshake::<_, bytes::Bytes>(io).await.unwrap();
        while let Some(r) = conn.accept().await { if r.is_err() { break; } }
    }, Duration::from_secs(120)).unwrap();
    start.elapsed()
}

#[test]
fn repro_trickle_cost_scales_with_advertised_max_frame_size() {
    // 200 payload bytes of a frame that declares the advertised maximum length,
    // delivered one byte per read.
    let n = 200;
    for size in [16_384u32, 262_144, 1_048_576, 16_777_215] {
        let t = run(size, n, size);
        eprintln!("max_frame_size={size:>8}: {n} trickled bytes took {t:?} ({:.1} us/byte)", t.as_secs_f64() * 1e6 / n as f64);
    }
}
```
