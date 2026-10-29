use std::future::Future;
use std::mem::MaybeUninit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use async_io::{block_on, Timer};
use futures_lite::future;

pub fn within<T>(limit: Duration, fut: impl Future<Output = T>) -> T {
    block_on(async move {
        let result = future::or(
            async move { Ok(fut.await) },
            async move {
                Timer::after(limit).await;
                Err(limit)
            },
        )
        .await;
        match result {
            Ok(value) => value,
            Err(limit) => panic!("watchdog expired after {limit:?}"),
        }
    })
}

pub struct AbortAfter {
    tx: Option<mpsc::Sender<()>>,
}

impl Drop for AbortAfter {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(());
        }
    }
}

pub fn abort_after(limit: Duration) -> AbortAfter {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        if rx.recv_timeout(limit).is_err() {
            eprintln!("qualify watchdog expired after {limit:?}");
            std::process::exit(2);
        }
    });
    AbortAfter { tx: Some(tx) }
}

pub fn cpu_time() -> Duration {
    let mut usage = MaybeUninit::<libc::rusage>::zeroed();
    let rc = unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) };
    assert_eq!(rc, 0, "getrusage: {}", std::io::Error::last_os_error());
    let usage = unsafe { usage.assume_init() };
    timeval(usage.ru_utime) + timeval(usage.ru_stime)
}

fn timeval(value: libc::timeval) -> Duration {
    Duration::from_secs(value.tv_sec as u64) + Duration::from_micros(value.tv_usec as u64)
}
