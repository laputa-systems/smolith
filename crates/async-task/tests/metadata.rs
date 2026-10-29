use async_task::{Builder, Runnable};
use flume::unbounded;
use smol::future;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

struct DropFlag(Arc<AtomicBool>);

impl Drop for DropFlag {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

#[test]
fn metadata_drop_waits_for_the_last_waker() {
    let dropped = Arc::new(AtomicBool::new(false));
    let (runnable, task) = Builder::new()
        .metadata(DropFlag(dropped.clone()))
        .spawn(|_| async {}, |_| {});
    let waker = runnable.waker();
    drop(runnable);
    drop(task);
    assert!(!dropped.load(Ordering::SeqCst));
    drop(waker);
    assert!(dropped.load(Ordering::SeqCst));
}

#[test]
fn metadata_use_case() {
    // Each future has a counter that is incremented every time it is scheduled.
    let (sender, receiver) = unbounded::<Runnable<AtomicUsize>>();
    let schedule = move |runnable: Runnable<AtomicUsize>| {
        runnable.metadata().fetch_add(1, Ordering::SeqCst);
        sender.send(runnable).ok();
    };

    async fn my_future(counter: &AtomicUsize) {
        loop {
            // Loop until we've been scheduled five times.
            let count = counter.load(Ordering::SeqCst);
            if count < 5 {
                // Make sure that we are immediately scheduled again.
                future::yield_now().await;
                continue;
            }

            // We've been scheduled five times, so we're done.
            break;
        }
    }

    let make_task = || {
        // SAFETY: We are spawning a non-'static future, so we need to use the unsafe API.
        // The borrowed variables, in this case the metadata, are guaranteed to outlive the runnable.
        let (runnable, task) = unsafe {
            Builder::new()
                .metadata(AtomicUsize::new(0))
                .spawn_unchecked(my_future, schedule.clone())
        };

        runnable.schedule();
        task
    };

    // Make tasks.
    let t1 = make_task();
    let t2 = make_task();

    // Run the tasks.
    while let Ok(runnable) = receiver.try_recv() {
        runnable.run();
    }

    // Unwrap the tasks.
    smol::future::block_on(async move {
        t1.await;
        t2.await;
    });
}
