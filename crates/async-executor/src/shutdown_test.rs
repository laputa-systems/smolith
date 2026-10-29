use std::future::Future;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread::{self, ThreadId};
use std::time::Duration;

use futures_lite::future;

use crate::{Executor, LocalExecutor};

fn finish_within(body: impl FnOnce() + Send + 'static) {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let result = catch_unwind(AssertUnwindSafe(body));
        let _ = tx.send(result);
    });
    match rx.recv_timeout(Duration::from_secs(2)) {
        Ok(Ok(())) => {}
        Ok(Err(payload)) => std::panic::resume_unwind(payload),
        Err(mpsc::RecvTimeoutError::Timeout) => panic!("executor shutdown hung"),
        Err(mpsc::RecvTimeoutError::Disconnected) => panic!("executor shutdown thread disconnected"),
    }
}

struct Probe {
    drops: Arc<AtomicUsize>,
    thread: Arc<Mutex<Option<ThreadId>>>,
}

impl Drop for Probe {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Release);
        *self.thread.lock().unwrap() = Some(thread::current().id());
    }
}

struct Hold {
    probe: Probe,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Hold {
    fn touch(&self) {
        let _ = self.probe.drops.load(Ordering::Relaxed);
    }
}

impl Future for Hold {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        this.touch();
        *this.waker.lock().unwrap() = Some(cx.waker().clone());
        Poll::Pending
    }
}

struct LocalHold {
    probe: Probe,
    waker: Arc<Mutex<Option<Waker>>>,
    _local: Rc<()>,
}

impl Future for LocalHold {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        let _ = this.probe.drops.load(Ordering::Relaxed);
        *this.waker.lock().unwrap() = Some(cx.waker().clone());
        Poll::Pending
    }
}

struct BorrowedHold<'a> {
    local: &'a u8,
    probe: Probe,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Future for BorrowedHold<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        let _ = *this.local;
        let _ = this.probe.drops.load(Ordering::Relaxed);
        *this.waker.lock().unwrap() = Some(cx.waker().clone());
        Poll::Pending
    }
}

fn wait_until_publishing(publishing: &AtomicUsize) {
    while publishing.load(Ordering::Acquire) == 0 {
        thread::yield_now();
    }
}

#[test]
fn send_executor_drops_a_runnable_published_during_shutdown() {
    finish_within(|| {
        let ex = Executor::new();
        let weak = Arc::downgrade(&ex.state_as_arc());
        let drops = Arc::new(AtomicUsize::new(0));
        let thread = Arc::new(Mutex::new(None));
        let slot = Arc::new(Mutex::new(None));
        ex.spawn(Hold {
            probe: Probe {
                drops: drops.clone(),
                thread: thread.clone(),
            },
            waker: slot.clone(),
        })
        .detach();
        assert!(ex.try_tick());
        let waker = slot.lock().unwrap().take().expect("waker stored");
        ex.state().pause_before_queue.store(true, Ordering::Release);

        let wake = thread::spawn(move || waker.wake());
        wait_until_publishing(&ex.state().publishing);

        drop(ex);
        wake.join().unwrap();
        assert_eq!(drops.load(Ordering::Acquire), 1);
        assert_eq!(weak.strong_count(), 0);
    });
}

#[test]
fn local_executor_drops_a_non_send_future_on_the_origin_thread() {
    finish_within(|| {
        let origin = thread::current().id();
        let ex = LocalExecutor::new();
        let weak = Arc::downgrade(&ex.inner.state_as_arc());
        let drops = Arc::new(AtomicUsize::new(0));
        let thread_id = Arc::new(Mutex::new(None));
        let slot = Arc::new(Mutex::new(None));
        ex.spawn(LocalHold {
            probe: Probe {
                drops: drops.clone(),
                thread: thread_id.clone(),
            },
            waker: slot.clone(),
            _local: Rc::new(()),
        })
        .detach();
        assert!(ex.try_tick());
        let waker = slot.lock().unwrap().take().expect("waker stored");
        ex.inner
            .state()
            .pause_before_queue
            .store(true, Ordering::Release);

        let wake = thread::spawn(move || waker.wake());
        wait_until_publishing(&ex.inner.state().publishing);

        drop(ex);
        wake.join().unwrap();
        assert_eq!(drops.load(Ordering::Acquire), 1);
        assert_eq!(*thread_id.lock().unwrap(), Some(origin));
        assert_eq!(weak.strong_count(), 0);
    });
}

#[test]
fn local_executor_drops_a_borrowed_future_on_the_origin_thread() {
    finish_within(|| {
        let origin = thread::current().id();
        let local = 1u8;
        let drops = Arc::new(AtomicUsize::new(0));
        let thread_id = Arc::new(Mutex::new(None));
        let slot = Arc::new(Mutex::new(None));
        let ex = LocalExecutor::new();
        let weak = Arc::downgrade(&ex.inner.state_as_arc());
        ex.spawn(BorrowedHold {
            local: &local,
            probe: Probe {
                drops: drops.clone(),
                thread: thread_id.clone(),
            },
            waker: slot.clone(),
        })
        .detach();
        assert!(ex.try_tick());
        let waker = slot.lock().unwrap().take().expect("waker stored");
        ex.inner
            .state()
            .pause_before_queue
            .store(true, Ordering::Release);

        let wake = thread::spawn(move || waker.wake());
        wait_until_publishing(&ex.inner.state().publishing);

        drop(ex);
        wake.join().unwrap();
        assert_eq!(drops.load(Ordering::Acquire), 1);
        assert_eq!(*thread_id.lock().unwrap(), Some(origin));
        assert_eq!(weak.strong_count(), 0);
    });
}

#[test]
fn cancellation_completes_when_shutdown_collects_the_late_runnable() {
    finish_within(|| {
        let ex = Executor::new();
        let drops = Arc::new(AtomicUsize::new(0));
        let thread_id = Arc::new(Mutex::new(None));
        let slot = Arc::new(Mutex::new(None));
        let task = ex.spawn(Hold {
            probe: Probe {
                drops: drops.clone(),
                thread: thread_id,
            },
            waker: slot.clone(),
        });
        assert!(ex.try_tick());
        let waker = slot.lock().unwrap().take().expect("waker stored");
        ex.state().pause_before_queue.store(true, Ordering::Release);

        let wake = thread::spawn(move || waker.wake());
        wait_until_publishing(&ex.state().publishing);

        let cancel = thread::spawn(move || future::block_on(task.cancel()));
        drop(ex);
        assert!(cancel.join().unwrap().is_none());
        wake.join().unwrap();
        assert_eq!(drops.load(Ordering::Acquire), 1);
    });
}

#[test]
fn cancellation_pauses_after_commit_and_still_drops_the_future() {
    finish_within(|| {
        let ex = Executor::new();
        let drops = Arc::new(AtomicUsize::new(0));
        let slot = Arc::new(Mutex::new(None));
        let task = ex.spawn(Hold {
            probe: Probe {
                drops: Arc::clone(&drops),
                thread: Arc::new(Mutex::new(None)),
            },
            waker: Arc::clone(&slot),
        });
        assert!(ex.try_tick());
        assert!(slot.lock().unwrap().is_some());
        ex.state().pause_before_queue.store(true, Ordering::Release);

        let cancel = thread::spawn(move || future::block_on(task.cancel()));
        wait_until_publishing(&ex.state().publishing);
        drop(ex);
        assert!(cancel.join().unwrap().is_none());
        assert_eq!(drops.load(Ordering::Acquire), 1);
    });
}

struct DriverSlot(Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>);

fn driver_waker(slot: &Arc<DriverSlot>) -> Waker {
    unsafe fn clone(ptr: *const ()) -> RawWaker {
        let arc = Arc::from_raw(ptr as *const DriverSlot);
        let cloned = Arc::clone(&arc);
        std::mem::forget(arc);
        RawWaker::new(Arc::into_raw(cloned) as *const (), &DRIVER_VTABLE)
    }
    unsafe fn wake(ptr: *const ()) {
        let arc = Arc::from_raw(ptr as *const DriverSlot);
        wake_by_ref(Arc::as_ptr(&arc) as *const ());
    }
    unsafe fn wake_by_ref(ptr: *const ()) {
        let slot = &*(ptr as *const DriverSlot);
        let driver = slot.0.lock().unwrap().take();
        drop(driver);
    }
    unsafe fn drop_waker(ptr: *const ()) {
        drop(Arc::from_raw(ptr as *const DriverSlot));
    }
    static DRIVER_VTABLE: RawWakerVTable =
        RawWakerVTable::new(clone, wake, wake_by_ref, drop_waker);
    let raw = RawWaker::new(Arc::into_raw(Arc::clone(slot)) as *const (), &DRIVER_VTABLE);
    unsafe { Waker::from_raw(raw) }
}

#[test]
fn notifying_the_driver_can_drop_its_executor() {
    finish_within(|| {
        let ex = Arc::new(Executor::new());
        let state = ex.state_as_arc();
        let weak = Arc::downgrade(&state);
        drop(state);
        let drops = Arc::new(AtomicUsize::new(0));
        let slot = Arc::new(Mutex::new(None));
        let task_slot = Arc::clone(&slot);
        let task = ex.spawn(Hold {
            probe: Probe {
                drops: Arc::clone(&drops),
                thread: Arc::new(Mutex::new(None)),
            },
            waker: task_slot,
        });
        assert!(ex.try_tick());
        let task_waker = slot.lock().unwrap().take().expect("waker stored");

        let driver_ex = Arc::clone(&ex);
        let driver = Box::pin(async move {
            driver_ex.run(future::pending::<()>()).await;
        });
        let drivers = Arc::new(DriverSlot(Mutex::new(Some(driver))));
        let waker = driver_waker(&drivers);
        let mut context = Context::from_waker(&waker);
        {
            let mut guard = drivers.0.lock().unwrap();
            assert!(guard
                .as_mut()
                .unwrap()
                .as_mut()
                .poll(&mut context)
                .is_pending());
        }
        drop(ex);
        task_waker.wake();
        drop(task);
        assert_eq!(drops.load(Ordering::Acquire), 1);
        assert_eq!(weak.strong_count(), 0);
    });
}
