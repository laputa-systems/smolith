use std::future::Future;
use std::pin::Pin;
use std::sync::{mpsc, Arc};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::Duration;

use event_listener::Event;

fn finish_within(body: impl FnOnce() + Send + 'static) {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        body();
        let _ = tx.send(());
    });
    rx.recv_timeout(Duration::from_secs(2))
        .expect("event notify reentered its listener list");
}

struct ListenAgain(Arc<Event>);

impl Wake for ListenAgain {
    fn wake(self: Arc<Self>) {
        let _listener = self.0.listen();
    }
}

#[test]
fn notify_can_listen_from_the_waker() {
    finish_within(|| {
        let event = Arc::new(Event::new());
        let waker = Waker::from(Arc::new(ListenAgain(Arc::clone(&event))));
        let mut listener = event.listen();
        let mut cx = Context::from_waker(&waker);
        assert!(matches!(
            Pin::new(&mut listener).poll(&mut cx),
            Poll::Pending
        ));
        event.notify(1);
    });
}
