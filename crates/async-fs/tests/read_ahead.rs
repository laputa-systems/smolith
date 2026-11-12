use std::io::{SeekFrom, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use async_fs::{File, OpenOptions};
use futures_lite::future;
use futures_lite::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use futures_lite::pin;

struct TempPath(PathBuf);

impl Drop for TempPath {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn fixture(bytes: &[u8]) -> TempPath {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let path = std::env::temp_dir().join(format!(
        "async-fs-read-ahead-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(bytes).unwrap();
    TempPath(path)
}

async fn open_rw(path: &std::path::Path) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .await
        .unwrap()
}

async fn read_one(file: &mut File) -> u8 {
    let mut buf = [0; 1];
    assert_eq!(file.read(&mut buf).await.unwrap(), 1);
    buf[0]
}

async fn read_rest(file: &mut File) -> Vec<u8> {
    let mut rest = Vec::new();
    file.read_to_end(&mut rest).await.unwrap();
    rest
}

#[test]
fn sync_all_keeps_unread_suffix() {
    future::block_on(async {
        let path = fixture(b"abcdef");
        let mut file = open_rw(&path.0).await;
        assert_eq!(read_one(&mut file).await, b'a');
        file.sync_all().await.unwrap();
        assert_eq!(read_rest(&mut file).await, b"bcdef");
    });
}

#[test]
fn sync_data_keeps_unread_suffix() {
    future::block_on(async {
        let path = fixture(b"abcdef");
        let mut file = open_rw(&path.0).await;
        assert_eq!(read_one(&mut file).await, b'a');
        file.sync_data().await.unwrap();
        assert_eq!(read_rest(&mut file).await, b"bcdef");
    });
}

#[test]
fn set_len_same_size_keeps_unread_suffix() {
    future::block_on(async {
        let path = fixture(b"abcdef");
        let mut file = open_rw(&path.0).await;
        assert_eq!(read_one(&mut file).await, b'a');
        file.set_len(6).await.unwrap();
        assert_eq!(read_rest(&mut file).await, b"bcdef");
    });
}

#[test]
fn set_len_truncation_does_not_return_discarded_tail() {
    future::block_on(async {
        let path = fixture(b"abcdef");
        let mut file = open_rw(&path.0).await;
        assert_eq!(read_one(&mut file).await, b'a');
        file.set_len(2).await.unwrap();
        assert_eq!(read_rest(&mut file).await, b"b");
    });
}

#[test]
fn cancelled_sync_after_dirty_read_keeps_unread_suffix() {
    future::block_on(async {
        let path = fixture(b"abcdef");
        let mut file = open_rw(&path.0).await;
        file.seek(SeekFrom::End(0)).await.unwrap();
        file.write_all(b"Z").await.unwrap();
        file.seek(SeekFrom::Start(0)).await.unwrap();
        assert_eq!(read_one(&mut file).await, b'a');
        {
            let sync = file.sync_all();
            pin!(sync);
            future::poll_once(sync.as_mut()).await;
        }
        assert_eq!(read_rest(&mut file).await, b"bcdefZ");
    });
}
