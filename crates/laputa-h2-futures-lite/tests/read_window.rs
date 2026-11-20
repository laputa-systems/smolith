use futures_io::{AsyncRead, AsyncWrite};
use laputa_protocol_test_support::executor::block_on_with_limit;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};
use std::time::Duration;

const PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
const MAX_FRAME_SIZE: u32 = 16_777_215;

struct Trickle {
    script: Vec<u8>,
    position: usize,
    largest_read_buffer: Arc<AtomicUsize>,
}

impl AsyncRead for Trickle {
    fn poll_read(mut self: Pin<&mut Self>, _: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        self.largest_read_buffer.fetch_max(buf.len(), Ordering::Relaxed);
        if self.position == self.script.len() {
            return Poll::Ready(Ok(0));
        }
        buf[0] = self.script[self.position];
        self.position += 1;
        Poll::Ready(Ok(1))
    }
}

impl AsyncWrite for Trickle {
    fn poll_write(self: Pin<&mut Self>, _: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[test]
fn read_buffer_handed_to_the_transport_does_not_grow_with_the_declared_frame_length() {
    let mut script = PREFACE.to_vec();
    script.extend_from_slice(&[0, 0, 0, 4, 0, 0, 0, 0, 0]);
    script.extend_from_slice(&MAX_FRAME_SIZE.to_be_bytes()[1..]);
    script.extend_from_slice(&[0, 0, 0, 0, 0, 1]);
    script.extend(std::iter::repeat_n(0u8, 64));

    let largest_read_buffer = Arc::new(AtomicUsize::new(0));
    let observed = largest_read_buffer.clone();
    block_on_with_limit(
        async move {
            let io = Trickle {
                script,
                position: 0,
                largest_read_buffer,
            };
            let mut connection = h2::server::Builder::new()
                .max_frame_size(MAX_FRAME_SIZE)
                .handshake::<_, bytes::Bytes>(io)
                .await
                .unwrap();
            while let Some(result) = connection.accept().await {
                if result.is_err() {
                    break;
                }
            }
        },
        Duration::from_secs(60),
    )
    .unwrap();

    let largest = observed.load(Ordering::Relaxed);
    assert!(
        largest <= 8 * 1024,
        "transport read buffer grew to {largest} bytes for a {MAX_FRAME_SIZE}-byte frame"
    );
}
