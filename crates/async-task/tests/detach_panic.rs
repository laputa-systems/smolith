use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_task::Runnable;

struct PendingNoop;

impl Future for PendingNoop {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        Poll::Pending
    }
}

#[test]
fn detach_survives_a_scheduler_that_drops_then_panics() {
    let (runnable, task) = async_task::spawn(PendingNoop, |runnable: Runnable| {
        drop(runnable);
        panic!("scheduler panicked");
    });

    assert!(!runnable.run());

    let result =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| task.detach()));
    assert!(result.is_err());
}

#[test]
fn detach_releases_task_when_begin_hook_panics() {
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

    struct DropFlag(Arc<AtomicBool>);
    impl Drop for DropFlag {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    let dropped = Arc::new(AtomicBool::new(false));
    let flag = DropFlag(dropped.clone());
    let first = AtomicBool::new(true);
    let schedule = async_task::ScheduleParts::new(
        move || {
            if first.swap(false, Ordering::SeqCst) {
                panic!("begin panicked");
            }
        },
        || {},
        |runnable: Runnable| drop(runnable),
        || {},
    );
    let (runnable, task) = async_task::spawn(async move {
        let _flag = flag;
        std::future::pending::<()>().await;
    }, schedule);
    assert!(!runnable.run());
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| task.detach())).is_err());
    assert!(dropped.load(Ordering::SeqCst));
}
