#![cfg(all(unix, feature = "server", feature = "http1"))]

mod support;

use std::convert::Infallible;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use async_net::TcpListener;
use h12tiny::runtime::BoxExecutor;
use h12tiny::server::serve;
use http::Response;
use hyper::service::service_fn;
use support::{FullBody, SmolExecutor};

const CHILD_ENV: &str = "H12TINY_DESCRIPTOR_EXHAUSTION_CHILD";

#[test]
fn descriptor_exhaustion_child() {
    if std::env::var_os(CHILD_ENV).is_none() {
        return;
    }
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        println!("PORT {}", listener.local_addr().unwrap().port());
        let result = serve(
            listener,
            service_fn(|_| async {
                Ok::<_, Infallible>(Response::new(FullBody::from_static(b"ok")))
            }),
            BoxExecutor::new(SmolExecutor),
        )
        .max_connections(1024)
        .shutdown_on(std::future::pending::<()>())
        .await;
        println!("SERVE_EXITED {result:?}");
    });
}

#[test]
fn listener_survives_descriptor_exhaustion_and_accepts_after_release() {
    let exe = std::env::current_exe().unwrap();
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("ulimit -n 48; exec \"$0\" --exact descriptor_exhaustion_child --nocapture --test-threads=1")
        .arg(&exe)
        .env(CHILD_ENV, "1")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let port: u16 = loop {
        let line = lines.next().unwrap().unwrap();
        if let Some(index) = line.find("PORT ") {
            break line[index + 5..].trim().parse().unwrap();
        }
    };
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for line in lines.map_while(Result::ok) {
            let _ = tx.send(line);
        }
    });

    let mut held = Vec::new();
    for _ in 0..200 {
        if let Ok(stream) = std::net::TcpStream::connect(("127.0.0.1", port)) {
            held.push(stream);
        }
    }
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut exited = None;
    while Instant::now() < deadline {
        if let Ok(line) = rx.recv_timeout(Duration::from_millis(100))
            && line.contains("SERVE_EXITED")
        {
            exited = Some(line);
            break;
        }
    }
    drop(held);

    let response = exited.is_none().then(|| {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let attempt = (|| -> std::io::Result<Vec<u8>> {
                let mut stream = std::net::TcpStream::connect(("127.0.0.1", port))?;
                stream.set_read_timeout(Some(Duration::from_secs(2)))?;
                stream.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")?;
                let mut response = Vec::new();
                stream.read_to_end(&mut response)?;
                Ok(response)
            })();
            match attempt {
                Ok(response) if response.starts_with(b"HTTP/1.1 200") => break Some(response),
                _ if Instant::now() >= deadline => break None,
                _ => std::thread::sleep(Duration::from_millis(50)),
            }
        }
    });
    let _ = child.kill();
    let _ = child.wait();

    assert!(exited.is_none(), "listener stopped on descriptor exhaustion: {exited:?}");
    assert!(
        matches!(response, Some(Some(_))),
        "listener did not serve a request after descriptors were released"
    );
}
