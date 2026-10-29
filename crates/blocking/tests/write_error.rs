use std::io::{self, ErrorKind, Write};
use std::pin::Pin;

use blocking::Unblock;
use futures_lite::future;
use futures_lite::prelude::*;

struct FailWrite {
    accepted: usize,
    fail_after: usize,
}

impl Write for FailWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.accepted >= self.fail_after {
            return Err(io::Error::new(ErrorKind::Other, "background write failed"));
        }
        let n = (self.fail_after - self.accepted).min(buf.len());
        self.accepted += n;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn background_write_error_surfaces_on_poll_write() {
    future::block_on(async {
        let mut writer = Unblock::with_capacity(
            8,
            FailWrite {
                accepted: 0,
                fail_after: 4,
            },
        );
        let buf = [7u8; 64];
        let mut saw = None;
        for _ in 0..10 {
            match future::poll_fn(|cx| Pin::new(&mut writer).poll_write(cx, &buf)).await {
                Ok(_) => {}
                Err(err) => {
                    saw = Some(err);
                    break;
                }
            }
        }
        let err = saw.expect("background write error");
        assert_eq!(err.kind(), ErrorKind::Other);
        let io = writer.into_inner().await;
        assert!(io.accepted <= io.fail_after);
    });
}

#[test]
fn background_write_error_surfaces_on_flush() {
    future::block_on(async {
        let mut writer = Unblock::with_capacity(
            64,
            FailWrite {
                accepted: 0,
                fail_after: 1,
            },
        );
        let wrote =
            future::poll_fn(|cx| Pin::new(&mut writer).poll_write(cx, b"abcdefghijklmnop")).await;
        match wrote {
            Err(err) => assert_eq!(err.kind(), ErrorKind::Other),
            Ok(_) => {
                let err = writer
                    .flush()
                    .await
                    .expect_err("flush reports the background error");
                assert_eq!(err.kind(), ErrorKind::Other);
            }
        }
        let _io = writer.into_inner().await;
    });
}
