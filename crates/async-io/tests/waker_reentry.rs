use std::net::{Ipv4Addr, UdpSocket};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::task::{Context, Wake, Waker};
use std::thread;
use std::time::Duration;

use async_io::Async;

fn finish_within(body: impl FnOnce() + Send + 'static) {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let result = catch_unwind(AssertUnwindSafe(body));
        let _ = tx.send(result);
    });
    match rx.recv_timeout(Duration::from_secs(2)) {
        Ok(Ok(())) => {}
        Ok(Err(payload)) => std::panic::resume_unwind(payload),
        Err(mpsc::RecvTimeoutError::Timeout) => panic!("reactor waker reentry hung"),
        Err(mpsc::RecvTimeoutError::Disconnected) => panic!("reactor waker reentry thread disconnected"),
    }
}

struct PollWrite {
    socket: Arc<Async<UdpSocket>>,
    polled: Arc<AtomicBool>,
}

impl Wake for PollWrite {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let waker = Waker::from(Arc::new(Idle));
        let mut cx = Context::from_waker(&waker);
        let _ = self.socket.poll_writable(&mut cx);
        self.polled.store(true, Ordering::Release);
    }
}

struct Idle;

impl Wake for Idle {
    fn wake(self: Arc<Self>) {}
}

#[test]
fn displaced_read_waker_polls_write_readiness() {
    finish_within(|| {
        let socket = Arc::new(
            Async::<UdpSocket>::bind((Ipv4Addr::LOCALHOST, 0)).expect("bind a local udp socket"),
        );
        let polled = Arc::new(AtomicBool::new(false));
        let reader = Waker::from(Arc::new(PollWrite {
            socket: Arc::clone(&socket),
            polled: Arc::clone(&polled),
        }));
        let mut first = Context::from_waker(&reader);
        assert!(socket.poll_readable(&mut first).is_pending());

        let replacement = Waker::from(Arc::new(Idle));
        assert!(!replacement.will_wake(&reader));
        let mut second = Context::from_waker(&replacement);
        assert!(socket.poll_readable(&mut second).is_pending());
        assert!(polled.load(Ordering::Acquire));
    });
}
