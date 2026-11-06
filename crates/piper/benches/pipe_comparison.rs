use async_io::Async;
use futures_lite::{future, prelude::*};
use rustybench::Bencher;
use std::net::{TcpListener, TcpStream};

fn main() {
    rustybench::main();
}

#[rustybench::bench]
fn pipe(bencher: Bencher) {
    let (mut reader, mut writer) = piper::pipe(16834);
    let data = vec![0; 16834];
    let mut buffer = vec![0; 16834];
    bencher.bench_local(move || {
        future::block_on(async {
            let (written, read) = future::zip(writer.write_all(&data), reader.read_exact(&mut buffer)).await;
            written.unwrap();
            read.unwrap();
        });
    });
}

#[rustybench::bench]
fn syspipe_tcp(bencher: Bencher) {
    let (mut tx, mut rx) = future::block_on(self_pipe());
    let data = vec![0; 16834];
    let mut buffer = vec![0; 16834];
    bencher.bench_local(|| {
        future::block_on(async {
            let (written, read) = future::zip(tx.write_all(&data), rx.read_exact(&mut buffer)).await;
            written.unwrap();
            read.unwrap();
        });
    });
}

#[cfg(unix)]
#[rustybench::bench]
fn syspipe_unix(bencher: Bencher) {
    use std::os::unix::net::UnixStream;

    let (mut tx, mut rx) = async_io::Async::<UnixStream>::pair().unwrap();
    let data = vec![0; 16834];
    let mut buffer = vec![0; 16834];
    bencher.bench_local(|| {
        future::block_on(async {
            let (written, read) = future::zip(tx.write_all(&data), rx.read_exact(&mut buffer)).await;
            written.unwrap();
            read.unwrap();
        });
    });
}

async fn self_pipe() -> (Async<TcpStream>, Async<TcpStream>) {
    let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 0)).unwrap();
    let addr = listener.get_ref().local_addr().unwrap();

    let tx = Async::<TcpStream>::connect(addr).await.unwrap();
    let rx = listener.accept().await.unwrap().0;

    (tx, rx)
}
