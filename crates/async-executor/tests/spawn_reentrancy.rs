use async_executor::Executor;
use std::future::Future;
use std::sync::{mpsc, Arc, Weak};
use std::task::{Context, Poll, Wake, Waker};
use std::time::Duration;

struct Reenter(Weak<Executor<'static>>);

impl Wake for Reenter {
    fn wake(self: Arc<Self>) {
        self.0.upgrade().unwrap().is_empty();
    }
}

fn check_spawn(many: bool) {
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let executor = Arc::new(Executor::new());
        let waker = Waker::from(Arc::new(Reenter(Arc::downgrade(&executor))));
        let mut tick = Box::pin(executor.tick());
        assert!(matches!(tick.as_mut().poll(&mut Context::from_waker(&waker)), Poll::Pending));
        if many {
            let mut tasks = Vec::new();
            executor.spawn_many([async {}], &mut tasks);
        } else {
            drop(executor.spawn(async {}));
        }
        sender.send(()).unwrap();
    });
    receiver.recv_timeout(Duration::from_secs(5)).expect("spawn deadlocked during wake");
}

#[test]
fn spawn_wakes_without_holding_active_lock() {
    check_spawn(false);
}

#[test]
fn spawn_many_wakes_without_holding_active_lock() {
    check_spawn(true);
}
