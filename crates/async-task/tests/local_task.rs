use std::future::poll_fn;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::Poll;

use async_task::Runnable;

struct Probe(Arc<AtomicUsize>);

impl Drop for Probe {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Release);
    }
}

impl Probe {
    fn touch(&self) {
        let _ = self.0.load(Ordering::Relaxed);
    }
}

#[test]
fn local_task_wakes_itself() {
    let (sender, receiver) = flume::unbounded::<Runnable>();
    let schedule = move |runnable| sender.send(runnable).unwrap();

    let mut polled = false;
    let future = poll_fn(move |cx| {
        if !polled {
            polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(7)
        }
    });

    let (runnable, task) = async_task::spawn_local(future, schedule);
    runnable.run();
    while let Ok(runnable) = receiver.try_recv() {
        runnable.run();
    }
    assert_eq!(smol::future::block_on(task), 7);
}

#[test]
fn local_task_keeps_a_nested_borrow_across_await() {
    let (sender, receiver) = flume::unbounded::<Runnable>();
    let schedule = move |runnable| sender.send(runnable).unwrap();

    let future = async {
        let value = 7u8;
        let nested = async {
            let reference = &value;
            futures_lite::future::yield_now().await;
            assert_eq!(*reference, 7);
            7
        };
        nested.await
    };

    let (runnable, task) = async_task::spawn_local(future, schedule);
    runnable.run();
    while let Ok(runnable) = receiver.try_recv() {
        runnable.run();
    }
    assert_eq!(smol::future::block_on(task), 7);
}

#[test]
fn local_task_drops_a_suspended_nested_borrow() {
    let (sender, receiver) = flume::unbounded::<Runnable>();
    let schedule = move |runnable| sender.send(runnable).unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let probe = Probe(Arc::clone(&drops));

    let future = async move {
        let nested = async {
            let reference = &probe;
            futures_lite::future::yield_now().await;
            reference.touch();
        };
        nested.await;
    };

    let (runnable, task) = async_task::spawn_local(future, schedule);
    runnable.run();
    drop(task);
    while let Ok(runnable) = receiver.try_recv() {
        drop(runnable);
    }
    assert_eq!(drops.load(Ordering::Acquire), 1);
}
