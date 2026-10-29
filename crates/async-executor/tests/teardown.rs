use std::future::poll_fn;
use std::sync::{Arc, Barrier, Mutex};
use std::task::Poll;
use std::thread;

use async_executor::Executor;

#[test]
fn wake_during_drop() {
    for _ in 0..200 {
        let slot = Arc::new(Mutex::new(None));
        let captured = Arc::clone(&slot);
        let ex = Executor::new();
        ex.spawn(poll_fn(move |cx| {
            *captured.lock().unwrap() = Some(cx.waker().clone());
            Poll::<()>::Pending
        }))
        .detach();
        assert!(ex.try_tick());

        let waker = slot.lock().unwrap().take().expect("waker stored");
        let barrier = Arc::new(Barrier::new(2));
        let thread_barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            thread_barrier.wait();
            waker.wake();
        });
        barrier.wait();
        drop(ex);
        handle.join().unwrap();
    }
}
