use blocking::Unblock;
use futures_lite::{future, io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt}};
use std::io::{self, Cursor, ErrorKind, Read, Seek, SeekFrom, Write};

struct Interrupted {
    inner: Cursor<Vec<u8>>,
    read: bool,
    write: bool,
    flush: bool,
    seek: bool,
}

fn once(flag: &mut bool) -> io::Result<()> {
    if std::mem::take(flag) {
        Err(ErrorKind::Interrupted.into())
    } else {
        Ok(())
    }
}

impl Read for Interrupted {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        once(&mut self.read)?;
        self.inner.read(buf)
    }
}

impl Write for Interrupted {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        once(&mut self.write)?;
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        once(&mut self.flush)?;
        self.inner.flush()
    }
}

impl Seek for Interrupted {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        once(&mut self.seek)?;
        self.inner.seek(pos)
    }
}

#[test]
fn interruptions_are_retried_without_losing_bytes() {
    future::block_on(async {
        let mut io = Unblock::new(Interrupted {
            inner: Cursor::new(Vec::new()), read: true, write: true, flush: true, seek: true,
        });
        io.write_all(b"hello").await.unwrap();
        io.flush().await.unwrap();
        io.seek(SeekFrom::Start(0)).await.unwrap();
        let mut bytes = Vec::new();
        io.read_to_end(&mut bytes).await.unwrap();
        assert_eq!(bytes, b"hello");
    });
}

#[test]
fn full_writer_reports_write_zero() {
    future::block_on(async {
        let mut io = Unblock::new(Cursor::new([0u8; 1]));
        io.write_all(b"AB").await.unwrap();
        assert_eq!(io.flush().await.unwrap_err().kind(), ErrorKind::WriteZero);
    });
}

#[test]
fn standalone_flush_retries_interruption() {
    future::block_on(async {
        let mut io = Unblock::new(Interrupted {
            inner: Cursor::new(Vec::new()), read: false, write: false, flush: true, seek: false,
        });
        io.flush().await.unwrap();
    });
}
