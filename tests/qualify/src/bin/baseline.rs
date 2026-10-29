use std::io::{self, Read as _, Write as _};
use std::net::{Shutdown, TcpListener};
use std::thread;
use std::time::{Duration, Instant};

use async_io::{Async, Timer};
use async_net::TcpStream as AsyncTcpStream;
use futures_lite::future::block_on;
use futures_lite::prelude::*;
use qualify::cpu_time;

fn main() {
    println!("host {} {}", std::env::consts::OS, std::env::consts::ARCH);
    idle_timer();
    idle_pool();
    wake_latency();
    tcp_throughput();
}

fn idle_timer() {
    for run in 1..=5 {
        let cpu0 = cpu_time();
        let wall0 = Instant::now();
        block_on(Timer::after(Duration::from_secs(1)));
        let cpu = cpu_time().saturating_sub(cpu0);
        let wall = wall0.elapsed();
        println!(
            "idle_timer_cpu_pct run={run} value={:.4} cpu_us={} wall_us={}",
            pct(cpu, wall),
            cpu.as_micros(),
            wall.as_micros()
        );
    }
}

fn idle_pool() {
    let value = block_on(blocking::unblock(|| 1u8));
    assert_eq!(value, 1);
    thread::sleep(Duration::from_millis(800));
    for run in 1..=5 {
        let cpu0 = cpu_time();
        let wall0 = Instant::now();
        thread::sleep(Duration::from_secs(1));
        let cpu = cpu_time().saturating_sub(cpu0);
        let wall = wall0.elapsed();
        println!(
            "idle_pool_cpu_pct run={run} value={:.4} cpu_us={} wall_us={}",
            pct(cpu, wall),
            cpu.as_micros(),
            wall.as_micros()
        );
    }
}

fn wake_latency() {
    for run in 1..=5 {
        let (reader, writer) = std::os::unix::net::UnixStream::pair().unwrap();
        let reader = Async::new(reader).unwrap();
        let writer = Async::new(writer).unwrap();
        block_on(async {
            for _ in 0..10 {
                one_wake(&reader, &writer).await.unwrap();
            }
        });
        let mut samples = Vec::with_capacity(100);
        block_on(async {
            for _ in 0..100 {
                samples.push(one_wake(&reader, &writer).await.unwrap());
            }
        });
        samples.sort();
        let median = samples[samples.len() / 2];
        println!(
            "wake_median_us run={run} value={} min_us={} max_us={} samples={}",
            median.as_micros(),
            samples[0].as_micros(),
            samples[samples.len() - 1].as_micros(),
            samples.len()
        );
    }
}

async fn one_wake(reader: &Async<std::os::unix::net::UnixStream>, writer: &Async<std::os::unix::net::UnixStream>) -> io::Result<Duration> {
    let start = Instant::now();
    writer.write_with(|mut io| io.write(b"z")).await?;
    reader.readable().await?;
    let mut buf = [0u8; 1];
    let n = reader.read_with(|mut io| io.read(&mut buf)).await?;
    let elapsed = start.elapsed();
    assert_eq!(n, 1);
    assert_eq!(buf, [b'z']);
    Ok(elapsed)
}

fn tcp_throughput() {
    let payload = vec![0x5a_u8; 64 * 1024];
    let rounds = 64u64;
    let expected = payload.len() as u64 * rounds;
    for run in 1..=5 {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let mut got = 0u64;
            let mut buf = [0u8; 64 * 1024];
            loop {
                match sock.read(&mut buf) {
                    Ok(0) => break got,
                    Ok(n) => got += n as u64,
                    Err(err) => panic!("server read: {err}"),
                }
            }
        });
        let mut client = block_on(AsyncTcpStream::connect(addr)).unwrap();
        let start = Instant::now();
        block_on(async {
            for _ in 0..rounds {
                client.write_all(&payload).await.unwrap();
            }
            client.shutdown(Shutdown::Write).unwrap();
        });
        let got = server.join().unwrap();
        let elapsed = start.elapsed();
        assert_eq!(got, expected);
        let mib = (expected as f64) / (1024.0 * 1024.0) / elapsed.as_secs_f64();
        println!(
            "tcp_mib_per_s run={run} value={mib:.2} bytes={expected} wall_us={}",
            elapsed.as_micros()
        );
    }
}

fn pct(cpu: Duration, wall: Duration) -> f64 {
    cpu.as_secs_f64() / wall.as_secs_f64() * 100.0
}
