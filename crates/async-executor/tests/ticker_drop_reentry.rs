use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::Duration;

use async_executor::Executor;

fn finish_within(body: impl FnOnce() + Send + 'static) {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        body();
        let _ = tx.send(());
    });
    rx.recv_timeout(Duration::from_secs(2))
        .expect("ticker drop reentered the executor");
}

struct SpawnOnDrop {
    executor: Arc<Executor<'static>>,
    spawned: Arc<AtomicBool>,
}

impl Wake for SpawnOnDrop {
    fn wake(self: Arc<Self>) {}
}

impl Drop for SpawnOnDrop {
    fn drop(&mut self) {
        drop(self.executor.spawn(async {}));
        self.spawned.store(true, Ordering::Release);
    }
}

#[test]
fn dropping_a_pending_tick_can_spawn() {
    finish_within(|| {
        let executor = Arc::new(Executor::new());
        let spawned = Arc::new(AtomicBool::new(false));
        let waker = Waker::from(Arc::new(SpawnOnDrop {
            executor: Arc::clone(&executor),
            spawned: Arc::clone(&spawned),
        }));
        let mut tick = Box::pin(executor.tick());
        assert!(matches!(
            tick.as_mut().poll(&mut Context::from_waker(&waker)),
            Poll::Pending
        ));
        drop(waker);
        drop(tick);
        assert!(spawned.load(Ordering::Acquire));
    });
}
