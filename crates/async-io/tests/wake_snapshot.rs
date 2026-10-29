use std::future::{poll_fn, Future};
use std::net::{Ipv4Addr, UdpSocket};
use std::pin::pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::Duration;

use async_io::Async;
use parking::Parker;

const ITERATIONS: usize = 200;
const WAIT_TIMEOUT: Duration = Duration::from_millis(500);
const ENTER_TIMEOUT: Duration = Duration::from_millis(5);
const BLOCK_GRACE: Duration = Duration::from_millis(5);

#[test]
fn wake_snapshot() {
    replace_waker_wakes_previous();

    let sender = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let driver = Driver::start();

    for i in 0..ITERATIONS {
        expect_delivery(&sender, Wait::PollReadable, i, &driver);
    }
    for i in 0..ITERATIONS {
        expect_delivery(&sender, Wait::Readable, i, &driver);
    }

    driver.stop();
}

#[derive(Clone, Copy)]
enum Wait {
    Readable,
    PollReadable,
}

struct Driver {
    stop: Arc<AtomicBool>,
    slot: Arc<Mutex<Option<Waker>>>,
    entered: mpsc::Receiver<()>,
}

impl Driver {
    fn start() -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let slot = Arc::new(Mutex::new(None));
        let (entered_tx, entered_rx) = mpsc::channel();
        let stop_thread = Arc::clone(&stop);
        let slot_thread = Arc::clone(&slot);
        thread::spawn(move || {
            async_io::block_on(poll_fn(|cx| {
                if stop_thread.load(Ordering::SeqCst) {
                    return Poll::Ready(());
                }
                *slot_thread.lock().unwrap() = Some(cx.waker().clone());
                let _ = entered_tx.send(());
                Poll::Pending
            }));
        });
        let driver = Self {
            stop,
            slot,
            entered: entered_rx,
        };
        driver.settle();
        driver
    }

    fn settle(&self) {
        if let Some(waker) = self.slot.lock().unwrap().clone() {
            waker.wake_by_ref();
        }
        let _ = self.entered.recv_timeout(ENTER_TIMEOUT);
        while self.entered.try_recv().is_ok() {}
        thread::sleep(BLOCK_GRACE);
    }

    fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(waker) = self.slot.lock().unwrap().clone() {
            waker.wake_by_ref();
        }
    }
}

impl Drop for Driver {
    fn drop(&mut self) {
        self.stop();
    }
}

fn expect_delivery(sender: &UdpSocket, kind: Wait, iteration: usize, driver: &Driver) {
    let socket = udp();
    let addr = socket.get_ref().local_addr().unwrap();
    let hold = Parker::new();
    let hold_waker = Waker::from(hold.unparker());
    let mut hold_cx = Context::from_waker(&hold_waker);
    match socket.poll_readable(&mut hold_cx) {
        Poll::Pending => {}
        poll => panic!("iteration {iteration} was readable before it registered: {poll:?}"),
    }
    driver.settle();

    let parker = Parker::new();
    let waker = Waker::from(parker.unparker());
    let mut cx = Context::from_waker(&waker);
    match kind {
        Wait::PollReadable => {
            assert!(
                matches!(socket.poll_readable(&mut cx), Poll::Pending),
                "iteration {iteration} was readable before the send"
            );
            send_byte(sender, addr);
            assert!(
                parker.park_timeout(WAIT_TIMEOUT),
                "iteration {iteration} lost the wakeup"
            );
            match socket.poll_readable(&mut cx) {
                Poll::Ready(Ok(())) => {}
                poll => panic!("iteration {iteration} dropped the readable event: {poll:?}"),
            }
        }
        Wait::Readable => {
            let mut fut = pin!(socket.readable());
            assert!(
                matches!(fut.as_mut().poll(&mut cx), Poll::Pending),
                "iteration {iteration} was readable before the send"
            );
            send_byte(sender, addr);
            assert!(
                parker.park_timeout(WAIT_TIMEOUT),
                "iteration {iteration} lost the wakeup"
            );
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(Ok(())) => {}
                poll => panic!("iteration {iteration} dropped the readable event: {poll:?}"),
            }
        }
    }

    let mut buf = [0u8; 1];
    let (n, _) = socket.get_ref().recv_from(&mut buf).unwrap();
    assert_eq!(n, 1);
}

fn send_byte(sender: &UdpSocket, addr: std::net::SocketAddr) {
    thread::scope(|scope| {
        scope.spawn(|| {
            sender.send_to(&[1], addr).unwrap();
        });
    });
}

fn replace_waker_wakes_previous() {
    let socket = udp();
    let seen = Arc::new(AtomicBool::new(false));
    let first = Waker::from(Arc::new(Flag(Arc::clone(&seen))));
    let second = Waker::from(Arc::new(Flag(Arc::new(AtomicBool::new(false)))));

    let mut cx = Context::from_waker(&first);
    assert!(matches!(socket.poll_readable(&mut cx), Poll::Pending));
    let mut cx = Context::from_waker(&second);
    assert!(matches!(socket.poll_readable(&mut cx), Poll::Pending));
    assert!(seen.load(Ordering::SeqCst), "replaced waker was not woken");
}

struct Flag(Arc<AtomicBool>);

impl Wake for Flag {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::SeqCst);
    }
}

fn udp() -> Async<UdpSocket> {
    Async::<UdpSocket>::bind((Ipv4Addr::LOCALHOST, 0)).unwrap()
}
