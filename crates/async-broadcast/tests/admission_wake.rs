use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

use async_broadcast::broadcast;
use futures_lite::future::FutureExt;
use futures_lite::pin;

struct Count(Arc<AtomicUsize>);

impl Wake for Count {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn counter() -> (Waker, Arc<AtomicUsize>) {
    let count = Arc::new(AtomicUsize::new(0));
    let waker = Waker::from(Arc::new(Count(Arc::clone(&count))));
    (waker, count)
}

#[test]
fn admission_changes_wake_a_pending_sender() {
    let (waker, wakes) = counter();
    let mut cx = Context::from_waker(&waker);

    let (sender, receiver) = broadcast(1);
    let _inactive = receiver.deactivate();
    let send = sender.broadcast(9);
    pin!(send);
    assert!(matches!(send.poll(&mut cx), Poll::Pending));
    let _receiver = sender.new_receiver();
    assert!(wakes.swap(0, Ordering::SeqCst) >= 1);
    assert!(matches!(send.poll(&mut cx), Poll::Ready(Ok(_))));

    let (sender, _receiver) = broadcast(1);
    sender.try_broadcast(1).unwrap();
    let mut other = sender.clone();
    let send = sender.broadcast(2);
    pin!(send);
    assert!(matches!(send.poll(&mut cx), Poll::Pending));
    other.set_capacity(2);
    assert!(wakes.swap(0, Ordering::SeqCst) >= 1);
    assert!(matches!(send.poll(&mut cx), Poll::Ready(Ok(_))));

    let (sender, _receiver) = broadcast(1);
    sender.try_broadcast(1).unwrap();
    let mut other = sender.clone();
    let send = sender.broadcast(2);
    pin!(send);
    assert!(matches!(send.poll(&mut cx), Poll::Pending));
    other.set_overflow(true);
    assert!(wakes.swap(0, Ordering::SeqCst) >= 1);
    assert!(matches!(send.poll(&mut cx), Poll::Ready(Ok(_))));

    let (sender, receiver) = broadcast(1);
    let _inactive = receiver.deactivate();
    let mut other = sender.clone();
    let send = sender.broadcast(3);
    pin!(send);
    assert!(matches!(send.poll(&mut cx), Poll::Pending));
    other.set_await_active(false);
    assert!(wakes.swap(0, Ordering::SeqCst) >= 1);
    assert!(matches!(send.poll(&mut cx), Poll::Ready(Err(_))));
}
