use std::sync::{mpsc, Arc};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::Duration;

use async_broadcast::broadcast;
use futures_lite::future::FutureExt;
use futures_lite::pin;

fn finish_within(body: impl FnOnce() + Send + 'static) {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        body();
        let _ = tx.send(());
    });
    rx.recv_timeout(Duration::from_secs(2))
        .expect("broadcast notify reentered the channel");
}

struct QueryLen {
    sender: async_broadcast::Sender<u8>,
}

impl Wake for QueryLen {
    fn wake(self: Arc<Self>) {
        let _ = self.sender.len();
    }
}

#[test]
fn receive_waker_can_query_the_channel() {
    finish_within(|| {
        let (sender, mut receiver) = broadcast::<u8>(1);
        let waker = Waker::from(Arc::new(QueryLen {
            sender: sender.clone(),
        }));
        let mut cx = Context::from_waker(&waker);
        let waiting = receiver.recv();
        pin!(waiting);
        assert!(matches!(waiting.poll(&mut cx), Poll::Pending));
        sender.try_broadcast(7).unwrap();
    });
}
