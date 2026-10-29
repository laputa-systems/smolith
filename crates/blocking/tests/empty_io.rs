use std::io::{self, BufWriter, ErrorKind, IoSlice, Write};
use std::pin::Pin;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use blocking::Unblock;
use futures_lite::future;
use futures_lite::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};

fn finish_within(body: impl FnOnce() + Send + 'static) {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        body();
        let _ = tx.send(());
    });
    rx.recv_timeout(Duration::from_secs(2))
        .expect("blocking adapter hung");
}

#[test]
fn empty_write_on_a_fresh_adapter_returns_zero() {
    finish_within(|| {
        future::block_on(async {
            let mut writer = Unblock::new(Vec::<u8>::new());
            assert_eq!(writer.write(&[]).await.unwrap(), 0);
            assert_eq!(writer.write(b"ab").await.unwrap(), 2);
            writer.flush().await.unwrap();
            assert_eq!(writer.into_inner().await, b"ab");
        });
    });
}

#[test]
fn empty_write_on_an_active_adapter_returns_zero() {
    finish_within(|| {
        future::block_on(async {
            let mut writer = Unblock::with_capacity(8, Vec::<u8>::new());
            assert_eq!(writer.write(b"hi").await.unwrap(), 2);
            assert_eq!(writer.write(&[]).await.unwrap(), 0);
            assert_eq!(writer.write(b"!").await.unwrap(), 1);
            writer.flush().await.unwrap();
            assert_eq!(writer.into_inner().await, b"hi!");
        });
    });
}

#[test]
fn empty_vectored_write_delegates_to_the_scalar_write() {
    finish_within(|| {
        future::block_on(async {
            let mut writer = Unblock::new(Vec::<u8>::new());
            let n = future::poll_fn(|cx| {
                Pin::new(&mut writer).poll_write_vectored(cx, &[IoSlice::new(&[])])
            })
            .await
            .unwrap();
            assert_eq!(n, 0);
            writer.write_all(b"z").await.unwrap();
            writer.flush().await.unwrap();
            assert_eq!(writer.into_inner().await, b"z");
        });
    });
}

#[test]
fn empty_read_on_a_fresh_and_active_adapter_returns_zero() {
    finish_within(|| {
        future::block_on(async {
            let mut reader = Unblock::new(io::repeat(7));
            assert_eq!(reader.read(&mut []).await.unwrap(), 0);
            let mut buf = [0; 4];
            assert_eq!(reader.read(&mut buf).await.unwrap(), 4);
            assert_eq!(buf, [7, 7, 7, 7]);
            assert_eq!(reader.read(&mut []).await.unwrap(), 0);
            assert_eq!(reader.read(&mut buf).await.unwrap(), 4);
        });
    });
}

struct FailOnce {
    failed: bool,
    buf: Vec<u8>,
}

impl Write for FailOnce {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !self.failed {
            self.failed = true;
            return Err(io::Error::new(ErrorKind::Other, "once"));
        }
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn empty_write_after_a_background_error_does_not_hang() {
    finish_within(|| {
        future::block_on(async {
            let mut writer = Unblock::with_capacity(4, FailOnce {
                failed: false,
                buf: Vec::new(),
            });
            let mut saw = false;
            for _ in 0..8 {
                match writer.write(b"abcdef").await {
                    Ok(_) => {}
                    Err(err) => {
                        assert_eq!(err.kind(), ErrorKind::Other);
                        saw = true;
                        break;
                    }
                }
            }
            assert!(saw, "background write error");
            assert_eq!(writer.write(&[]).await.unwrap(), 0);
            assert_eq!(writer.write(b"ok").await.unwrap(), 2);
            writer.flush().await.unwrap();
            let io = writer.into_inner().await;
            assert_eq!(io.buf, b"ok");
        });
    });
}

struct CountFlush {
    buf: Vec<u8>,
    flushes: usize,
    fail_flushes: usize,
}

impl Write for CountFlush {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flushes += 1;
        if self.fail_flushes > 0 {
            self.fail_flushes -= 1;
            return Err(io::Error::new(ErrorKind::Other, "flush"));
        }
        Ok(())
    }
}

#[test]
fn flush_commits_bytes_already_buffered_in_the_inner_writer() {
    finish_within(|| {
        future::block_on(async {
            let mut inner = BufWriter::new(Vec::<u8>::new());
            inner.write_all(b"payload").unwrap();
            let mut writer = Unblock::new(inner);
            writer.flush().await.unwrap();
            let inner = writer.into_inner().await;
            assert_eq!(inner.get_ref().as_slice(), b"payload");
            assert!(inner.buffer().is_empty());
        });
    });
}

#[test]
fn flush_commits_bytes_written_through_get_mut_and_with_mut() {
    finish_within(|| {
        future::block_on(async {
            let mut writer = Unblock::new(BufWriter::new(Vec::<u8>::new()));
            writer.get_mut().await.write_all(b"mut").unwrap();
            writer.flush().await.unwrap();
            writer
                .with_mut(|inner| inner.write_all(b"-more").unwrap())
                .await;
            writer.flush().await.unwrap();
            let inner = writer.into_inner().await;
            assert_eq!(inner.get_ref().as_slice(), b"mut-more");
            assert!(inner.buffer().is_empty());
        });
    });
}

#[test]
fn flush_retries_after_the_inner_flush_fails() {
    finish_within(|| {
        future::block_on(async {
            let mut writer = Unblock::new(CountFlush {
                buf: Vec::new(),
                flushes: 0,
                fail_flushes: 1,
            });
            writer.write_all(b"payload").await.unwrap();
            let err = writer.flush().await.unwrap_err();
            assert_eq!(err.kind(), ErrorKind::Other);
            writer.flush().await.unwrap();
            let inner = writer.into_inner().await;
            assert_eq!(inner.buf, b"payload");
            assert!(inner.flushes >= 2);
        });
    });
}

struct HoldUntilFlush {
    committed: Vec<u8>,
    pending: Vec<u8>,
    flushes: usize,
}

impl Write for HoldUntilFlush {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.pending.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flushes += 1;
        if self.flushes == 2 {
            return Err(io::Error::new(ErrorKind::Other, "flush"));
        }
        self.committed.extend_from_slice(&self.pending);
        self.pending.clear();
        Ok(())
    }
}

#[test]
fn flush_after_a_later_write_retries_a_failed_flush() {
    finish_within(|| {
        future::block_on(async {
            let mut writer = Unblock::new(HoldUntilFlush {
                committed: Vec::new(),
                pending: Vec::new(),
                flushes: 0,
            });
            writer.write_all(b"A").await.unwrap();
            writer.flush().await.unwrap();
            writer.write_all(b"B").await.unwrap();
            assert!(writer.flush().await.is_err());
            writer.flush().await.unwrap();
            let inner = writer.into_inner().await;
            assert_eq!(inner.committed, b"AB");
            assert!(inner.pending.is_empty());
            assert_eq!(inner.flushes, 3);
        });
    });
}
