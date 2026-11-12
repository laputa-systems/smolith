use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use fastwebsockets::{after_handshake_split, Frame, Role, WebSocketError};
use futures_lite::io::{AsyncRead, AsyncWrite};

struct MockStream {
  write_call_count: Arc<AtomicUsize>,
  bytes_written: Arc<AtomicUsize>,
}

impl MockStream {
  fn new() -> (Self, Arc<AtomicUsize>, Arc<AtomicUsize>) {
    let write_call_count = Arc::new(AtomicUsize::new(0));
    let bytes_written = Arc::new(AtomicUsize::new(0));
    (
      Self {
        write_call_count: write_call_count.clone(),
        bytes_written: bytes_written.clone(),
      },
      write_call_count,
      bytes_written,
    )
  }
}

impl AsyncRead for MockStream {
  fn poll_read(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
    _buf: &mut [u8],
  ) -> Poll<std::io::Result<usize>> {
    Poll::Pending
  }
}

impl AsyncWrite for MockStream {
  fn poll_write(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<std::io::Result<usize>> {
    self.write_call_count.fetch_add(1, Ordering::SeqCst);
    self.bytes_written.fetch_add(buf.len(), Ordering::SeqCst);
    Poll::Ready(Ok(buf.len()))
  }

  fn poll_flush(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
  ) -> Poll<std::io::Result<()>> {
    Poll::Ready(Ok(()))
  }

  fn poll_close(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
  ) -> Poll<std::io::Result<()>> {
    Poll::Ready(Ok(()))
  }
}

#[test]
fn double_close_sends_only_one_frame() {
  let (mock, write_count, bytes_written) = MockStream::new();
  let (_read, mut write) =
    after_handshake_split(futures_lite::io::empty(), mock, Role::Server);
  write.set_writev(false);

  futures_lite::future::block_on(async {
    write
      .write_frame(Frame::close(1000, b"goodbye"))
      .await
      .unwrap();

    let writes_after_first = write_count.load(Ordering::SeqCst);
    let bytes_after_first = bytes_written.load(Ordering::SeqCst);

    assert!(writes_after_first > 0, "First close should write data");
    assert!(bytes_after_first > 0, "First close should write bytes");

    write
      .write_frame(Frame::close(1000, b"goodbye again"))
      .await
      .unwrap();

    let writes_after_second = write_count.load(Ordering::SeqCst);
    let bytes_after_second = bytes_written.load(Ordering::SeqCst);

    assert_eq!(
      writes_after_first,
      writes_after_second,
      "Second close should NOT write any data (it wrote {} more bytes)",
      bytes_after_second - bytes_after_first
    );
  });
}

#[test]
fn is_closed_after_close_frame() {
  let (mock, _, _) = MockStream::new();
  let (_read, mut write) =
    after_handshake_split(futures_lite::io::empty(), mock, Role::Server);
  write.set_writev(false);

  futures_lite::future::block_on(async {
    assert!(!write.is_closed(), "Should not be closed initially");
    write.write_frame(Frame::close(1000, b"")).await.unwrap();
    assert!(
      write.is_closed(),
      "Should be closed after sending close frame"
    );
  });
}

#[test]
fn non_close_frame_after_close_fails() {
  let (mock, _, _) = MockStream::new();
  let (_read, mut write) =
    after_handshake_split(futures_lite::io::empty(), mock, Role::Server);
  write.set_writev(false);

  futures_lite::future::block_on(async {
    write.write_frame(Frame::close(1000, b"")).await.unwrap();

    let result = write
      .write_frame(Frame::text(b"hello".to_vec().into()))
      .await;

    assert!(
      result.is_err(),
      "Sending non-close frame after close should fail"
    );
    assert!(
      matches!(result, Err(WebSocketError::ConnectionClosed)),
      "Should get ConnectionClosed error"
    );
  });
}

struct PartialWrite {
  budget: usize,
  given: usize,
  stalled: bool,
  written: Arc<Mutex<Vec<u8>>>,
}

impl PartialWrite {
  fn new(budget: usize) -> (Self, Arc<Mutex<Vec<u8>>>) {
    let written = Arc::new(Mutex::new(Vec::new()));
    (
      Self {
        budget,
        given: 0,
        stalled: false,
        written: written.clone(),
      },
      written,
    )
  }
}

impl AsyncRead for PartialWrite {
  fn poll_read(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
    _buf: &mut [u8],
  ) -> Poll<std::io::Result<usize>> {
    Poll::Pending
  }
}

impl AsyncWrite for PartialWrite {
  fn poll_write(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<std::io::Result<usize>> {
    let this = self.get_mut();
    if this.stalled {
      this.stalled = false;
      this.budget = usize::MAX;
    }
    if this.given >= this.budget {
      this.stalled = true;
      return Poll::Pending;
    }
    let n = buf.len().min(this.budget - this.given);
    this.written.lock().unwrap().extend_from_slice(&buf[..n]);
    this.given += n;
    Poll::Ready(Ok(n))
  }

  fn poll_flush(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
  ) -> Poll<std::io::Result<()>> {
    Poll::Ready(Ok(()))
  }

  fn poll_close(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
  ) -> Poll<std::io::Result<()>> {
    Poll::Ready(Ok(()))
  }
}

fn close_bytes() -> Vec<u8> {
  let mut frame = Frame::close(1000, b"goodbye");
  let mut buffer = Vec::new();
  frame.write(&mut buffer).to_vec()
}

fn assert_cancelled_close_resumes(budget: usize) {
  futures_lite::future::block_on(async move {
    let expected = close_bytes();
    let (stream, written) = PartialWrite::new(budget);
    let (_read, mut write) =
      after_handshake_split(futures_lite::io::empty(), stream, Role::Server);
    write.set_writev(false);

    let mut pending = Box::pin(write.write_frame(Frame::close(1000, b"goodbye")));
    assert!(futures_lite::future::poll_once(&mut pending).await.is_none());
    drop(pending);

    assert!(!write.is_closed());
    assert_eq!(&written.lock().unwrap()[..], &expected[..budget.min(expected.len())]);

    let data = write
      .write_frame(Frame::text(b"hello".to_vec().into()))
      .await;
    assert!(matches!(data, Err(WebSocketError::ConnectionClosed)));

    write
      .write_frame(Frame::close(1000, b"other"))
      .await
      .unwrap();
    assert!(write.is_closed());
    assert_eq!(written.lock().unwrap().as_slice(), expected);
  });
}

#[test]
fn cancelled_close_with_zero_bytes_is_not_sent() {
  assert_cancelled_close_resumes(0);
}

#[test]
fn cancelled_close_keeps_a_partial_write() {
  assert_cancelled_close_resumes(1);
}
