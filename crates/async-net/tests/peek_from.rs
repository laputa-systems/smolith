use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::Duration;

use async_net::UdpSocket;
use futures_lite::future;

#[test]
fn peek_from_keeps_the_datagram() {
    future::block_on(async {
        exercise(SocketAddr::from((Ipv4Addr::LOCALHOST, 0)))
            .await
            .unwrap();
        match UdpSocket::bind(SocketAddr::from((Ipv6Addr::LOCALHOST, 0))).await {
            Ok(socket) => exercise_socket(socket).await.unwrap(),
            Err(err) if is_address_family(&err) => {
                eprintln!("skipping IPv6 peek_from: {err}");
            }
            Err(err) => panic!("IPv6 bind failed: {err}"),
        }
    });
}

async fn exercise(bind: SocketAddr) -> io::Result<()> {
    exercise_socket(UdpSocket::bind(bind).await?).await
}

async fn exercise_socket(socket: UdpSocket) -> io::Result<()> {
    let addr = socket.local_addr()?;
    let msg = b"peek-me";
    assert_eq!(socket.send_to(msg, addr).await?, msg.len());

    let mut buf = [0u8; 16];
    let (n1, from1) = socket.peek_from(&mut buf).await?;
    assert_eq!(&buf[..n1], msg);
    let (n2, from2) = socket.peek_from(&mut buf).await?;
    assert_eq!((n2, from2), (n1, from1));
    assert_eq!(&buf[..n2], msg);

    buf.fill(0);
    let (n3, from3) = socket.recv_from(&mut buf).await?;
    assert_eq!((n3, from3), (n1, from1));
    assert_eq!(&buf[..n3], msg);

    let consumed = future::or(
        async {
            socket.peek_from(&mut [0u8; 16]).await.expect("peek_from");
            false
        },
        async {
            async_io::Timer::after(Duration::from_millis(80)).await;
            true
        },
    )
    .await;
    assert!(consumed, "recv_from left the datagram queued");
    Ok(())
}

fn is_address_family(err: &io::Error) -> bool {
    // EAFNOSUPPORT on Unix, WSAEAFNOSUPPORT on Windows.
    matches!(err.raw_os_error(), Some(47 | 10047))
}
