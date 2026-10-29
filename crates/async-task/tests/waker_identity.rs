use std::future::poll_fn;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::Poll;

use async_task::Runnable;

#[test]
fn context_waker_will_wake_its_clone() {
    let schedules = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&schedules);
    let (sender, receiver) = flume::unbounded::<Runnable>();
    let schedule = move |runnable| {
        counted.fetch_add(1, Ordering::SeqCst);
        sender.send(runnable).unwrap();
    };

    let mut polls = 0usize;
    let future = poll_fn(move |cx| {
        let cloned = cx.waker().clone();
        assert!(cloned.will_wake(cx.waker()));
        assert!(cx.waker().will_wake(&cloned));
        polls += 1;
        if polls == 1 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(polls)
        }
    });

    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    let mut runs = 0usize;
    while let Ok(runnable) = receiver.try_recv() {
        runs += 1;
        runnable.run();
    }
    assert_eq!(runs, 2);
    assert_eq!(schedules.load(Ordering::SeqCst), 2);
    assert_eq!(smol::future::block_on(task), 2);
}
