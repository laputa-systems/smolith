//! Benchmarks for a variety of I/O operations.

use async_io::Async;
use futures_lite::{future, prelude::*};
use rustybench::{black_box, Bencher};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};

fn main() {
    rustybench::main();
}

const DRIVEN: &[bool] = &[false, true];

/// Block on a future, either using the I/O driver or simple parking.
fn block_on<R>(fut: impl Future<Output = R>, drive: bool) -> R {
    if drive {
        async_io::block_on(fut)
    } else {
        future::block_on(fut)
    }
}

#[rustybench::bench(args = DRIVEN)]
fn read_and_write_tcp_stream(bencher: Bencher, exec: &bool) {
    const TCP_AMOUNT: usize = 1024 * 1024;

    let listener = TcpListener::bind("localhost:12345").unwrap();
    let read_stream = TcpStream::connect("localhost:12345").unwrap();
    let (write_stream, _) = listener.accept().unwrap();

    let mut reader = Async::new(read_stream).unwrap();
    let mut writer = Async::new(write_stream).unwrap();
    let buf = vec![0x42; TCP_AMOUNT];

    bencher.bench_local(|| {
        let mut received = vec![0; TCP_AMOUNT];

        block_on(
            async {
                let (written, read) =
                    future::zip(writer.write_all(&buf), reader.read_exact(&mut received)).await;
                black_box((written.ok(), read.ok()));
            },
            *exec,
        );
    });
}

#[cfg(unix)]
#[rustybench::bench(args = DRIVEN)]
fn read_and_write_unix_stream(bencher: Bencher, exec: &bool) {
    use std::os::unix::net::UnixStream;
    const UNIX_AMOUNT: usize = 1024;

    let (mut reader, mut writer) = Async::<UnixStream>::pair().unwrap();
    let mut buf = vec![0x42; UNIX_AMOUNT];

    bencher.bench_local(|| {
        let buf = &mut buf;
        block_on(
            async {
                black_box(writer.write_all(&*buf).await.ok());
                black_box(reader.read_exact(buf).await.ok());
            },
            *exec,
        );
    });
}

#[rustybench::bench(args = DRIVEN)]
fn connect_and_accept_tcp_stream(bencher: Bencher, exec: &bool) {
    let socket_addr = SocketAddr::new("127.0.0.1".parse::<Ipv4Addr>().unwrap().into(), 12345);
    let listener = Async::<TcpListener>::bind(socket_addr).unwrap();

    bencher.bench_local(|| {
        block_on(
            async {
                let _reader = Async::<TcpStream>::connect(socket_addr).await.ok();
                black_box(listener.accept().await.ok());
                drop(black_box(_reader));
            },
            *exec,
        );
    });
}

#[cfg(unix)]
#[rustybench::bench(args = DRIVEN)]
fn connect_and_accept_unix_stream(bencher: Bencher, exec: &bool) {
    use std::os::unix::net::{UnixListener, UnixStream};

    let mut id = [0u8; 8];
    getrandom::fill(&mut id).unwrap();
    let id = u64::from_ne_bytes(id);

    let socket_addr = format!("/tmp/async-io-bench-{id}.sock");
    let listener = Async::<UnixListener>::bind(&socket_addr).unwrap();

    bencher.bench_local(|| {
        block_on(
            async {
                let _reader = Async::<UnixStream>::connect(&socket_addr).await.ok();
                black_box(listener.accept().await.ok());
                drop(black_box(_reader));
            },
            *exec,
        );
    });

    drop(listener);
}

#[rustybench::bench(args = DRIVEN)]
fn udp_send_recv(bencher: Bencher, exec: &bool) {
    const UDP_AMOUNT: usize = 1024;

    let socket_addr = |port| SocketAddr::new("127.0.0.1".parse::<Ipv4Addr>().unwrap().into(), port);
    let socket_addr1 = socket_addr(12345);
    let socket_addr2 = socket_addr(12346);

    let reader = Async::<UdpSocket>::bind(socket_addr1).unwrap();
    let writer = Async::<UdpSocket>::bind(socket_addr2).unwrap();

    let mut buf = vec![0x42; UDP_AMOUNT];

    bencher.bench_local(|| {
        let buf = &mut buf;

        block_on(
            async {
                black_box(writer.send_to(&*buf, socket_addr1).await.ok());
                black_box(reader.recv_from(buf).await.ok());
            },
            *exec,
        );
    });
}
