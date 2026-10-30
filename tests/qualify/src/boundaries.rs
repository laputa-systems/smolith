use std::io::{self, Read as _, Write as _};
use std::net::{Shutdown, TcpListener as StdTcpListener, TcpStream as StdTcpStream};
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::task::{Context, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant};

use async_channel::{bounded, TrySendError};
use async_executor::Executor;
use async_io::{Async, Timer};
use async_lock::Mutex as AsyncMutex;
use async_net::{TcpListener, TcpStream};
use concurrent_queue::ConcurrentQueue;
use event_listener::Event;
use futures_lite::future;
use futures_lite::prelude::*;
use polling::{Event as PollEvent, Events, Poller};

use crate::support::within;

fn serial() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|err| err.into_inner())
}

#[test]
fn cancel_parked_task() {
    let ex = Executor::new();
    let (sender, receiver) = bounded::<u8>(1);
    let leftover = receiver.clone();
    let task = ex.spawn(async move { receiver.recv().await.ok() });
    assert!(ex.try_tick(), "parked recv did not run");
    let outcome = within(Duration::from_secs(2), ex.run(task.cancel()));
    assert_eq!(outcome, None);
    sender.try_send(1).unwrap();
    assert_eq!(leftover.try_recv().unwrap(), 1);
}

struct AcceptWake(Arc<AtomicBool>);

impl Wake for AcceptWake {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::Release);
    }
}

#[test]
fn cancel_accept_then_connect() {
    within(Duration::from_secs(2), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let woke = Arc::new(AtomicBool::new(false));
        let mut accept = Box::pin(listener.accept());
        let waker = Waker::from(Arc::new(AcceptWake(Arc::clone(&woke))));
        let mut context = Context::from_waker(&waker);
        assert!(accept.as_mut().poll(&mut context).is_pending());

        let (release_tx, release_rx) = mpsc::channel();
        let connector = thread::spawn(move || {
            let stream = StdTcpStream::connect(addr).unwrap();
            let _ = release_rx.recv();
            drop(stream);
        });
        let started = Instant::now();
        while !woke.load(Ordering::Acquire) {
            if started.elapsed() > Duration::from_secs(1) {
                panic!("polled accept was not registered");
            }
            thread::yield_now();
        }
        drop(accept);

        let (accepted, connected) =
            future::zip(listener.accept(), TcpStream::connect(addr)).await;
        accepted.unwrap();
        connected.unwrap();
        let _ = release_tx.send(());
        connector.join().unwrap();
    });
}

#[test]
fn wake_during_executor_drop() {
    for _ in 0..50 {
        let slot = Arc::new(Mutex::new(None));
        let captured = Arc::clone(&slot);
        let ex = Executor::new();
        ex.spawn(future::poll_fn(move |cx| {
            *captured.lock().unwrap() = Some(cx.waker().clone());
            std::task::Poll::<()>::Pending
        }))
        .detach();
        assert!(ex.try_tick());
        let waker = slot.lock().unwrap().take().expect("waker stored");
        let barrier = Arc::new(std::sync::Barrier::new(3));
        let thread_barrier = Arc::clone(&barrier);
        let waker_handle = thread::spawn(move || {
            thread_barrier.wait();
            waker.wake();
        });
        let (done_tx, done_rx) = mpsc::channel();
        let drop_barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            drop_barrier.wait();
            drop(ex);
            waker_handle.join().unwrap();
            let _ = done_tx.send(());
        });
        barrier.wait();
        assert!(
            done_rx.recv_timeout(Duration::from_secs(10)).is_ok(),
            "executor drop hung despite racing wake during drop"
        );
    }
}

#[test]
fn readiness_rearms_after_read() {
    within(Duration::from_secs(2), async {
        let (reader, writer) = std::os::unix::net::UnixStream::pair().unwrap();
        let reader = Async::new(reader).unwrap();
        let writer = Async::new(writer).unwrap();
        let mut buf = [0u8; 1];
        writer.write_with(|mut io| io.write(b"a")).await.unwrap();
        assert_eq!(reader.read_with(|mut io| io.read(&mut buf)).await.unwrap(), 1);
        assert_eq!(buf, [b'a']);
        let early = future::or(
            async { reader.readable().await.map(|_| true) },
            async {
                Timer::after(Duration::from_millis(80)).await;
                Ok(false)
            },
        )
        .await
        .unwrap();
        assert!(!early, "readiness stayed signaled after the byte was read");
        writer.write_with(|mut io| io.write(b"b")).await.unwrap();
        reader.readable().await.unwrap();
        assert_eq!(reader.read_with(|mut io| io.read(&mut buf)).await.unwrap(), 1);
        assert_eq!(buf, [b'b']);
    });
}

#[test]
fn oneshot_modify_rearms() {
    let (reader, mut writer) = std::os::unix::net::UnixStream::pair().unwrap();
    reader.set_nonblocking(true).unwrap();
    writer.set_nonblocking(true).unwrap();
    let poller = Poller::new().unwrap();
    // SAFETY: `reader` is deleted before this test drops it.
    unsafe { poller.add(&reader, PollEvent::readable(7)).unwrap() };
    let _delete = DeleteOnDrop {
        poller: &poller,
        source: &reader,
    };
    writer.write_all(b"a").unwrap();
    let mut events = Events::new();
    let n = poller
        .wait(&mut events, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(n, 1);
    let mut buf = [0u8; 1];
    assert_eq!((&reader).read(&mut buf).unwrap(), 1);
    writer.write_all(b"b").unwrap();
    events.clear();
    let n = poller
        .wait(&mut events, Some(Duration::from_millis(50)))
        .unwrap();
    assert_eq!(n, 0, "oneshot interest delivered a second event before modify");
    poller.modify(&reader, PollEvent::readable(7)).unwrap();
    events.clear();
    let n = poller
        .wait(&mut events, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(n, 1);
    assert_eq!((&reader).read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [b'b']);
}

#[test]
fn timer_fires_once_inside_bounds() {
    let _guard = serial();
    let start = Instant::now();
    within(Duration::from_secs(2), Timer::after(Duration::from_millis(80)));
    let elapsed = start.elapsed();
    assert!(
        elapsed >= Duration::from_millis(50),
        "timer returned in {elapsed:?}"
    );
    assert!(
        elapsed < Duration::from_millis(500),
        "timer returned in {elapsed:?}"
    );
}

#[test]
fn far_timer_does_not_win() {
    let _guard = serial();
    let far_won = within(Duration::from_secs(2), async {
        future::or(
            async {
                Timer::after(Duration::from_secs(30)).await;
                true
            },
            async {
                Timer::after(Duration::from_millis(40)).await;
                false
            },
        )
        .await
    });
    assert!(!far_won);
}

#[test]
fn notify_stays_blocked_until_timeout() {
    let _guard = serial();
    let poller = Poller::new().unwrap();
    let mut events = Events::new();
    poller.notify().unwrap();
    poller.wait(&mut events, None).unwrap();
    events.clear();
    let timeout = Duration::from_millis(100);
    let start = Instant::now();
    poller.wait(&mut events, Some(timeout)).unwrap();
    let elapsed = start.elapsed();
    assert!(
        elapsed >= timeout.saturating_sub(Duration::from_millis(30)),
        "second notify wait returned in {elapsed:?}"
    );
}

#[cfg(unix)]
#[test]
fn sigusr2_wakes_poller() {
    let _restore = install_noop_sigusr2();
    for iteration in 0..10 {
        let poller = Arc::new(Poller::new().unwrap());
        let waiting = Arc::clone(&poller);
        let (ready_tx, ready_rx) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            let mut events = Events::new();
            ready_tx.send(()).unwrap();
            let start = Instant::now();
            let result = waiting.wait(&mut events, Some(Duration::from_secs(2)));
            (start.elapsed(), result)
        });
        ready_rx.recv().unwrap();
        thread::sleep(Duration::from_millis(10));
        // SAFETY: SIGUSR2 has a no-op handler until `RestoreSig` drops.
        let rc = unsafe { libc::kill(libc::getpid(), libc::SIGUSR2) };
        assert_eq!(rc, 0, "kill: {}", io::Error::last_os_error());
        poller.notify().unwrap();
        let (elapsed, result) = handle.join().unwrap();
        result.unwrap();
        assert!(
            elapsed < Duration::from_millis(1_500),
            "iteration {iteration} blocked for {elapsed:?}"
        );
    }
}

#[test]
fn reused_descriptor_is_not_watched() {
    let poller = Poller::new().unwrap();
    let listener = StdTcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let fd = std::os::unix::io::AsRawFd::as_raw_fd(&listener);
    // SAFETY: `listener` is deleted before this test drops it.
    unsafe { poller.add(&listener, PollEvent::readable(1)).unwrap() };
    poller.delete(&listener).unwrap();
    drop(listener);

    let mut held = Vec::new();
    let mut reused = None;
    for _ in 0..128 {
        let next = StdTcpListener::bind("127.0.0.1:0").unwrap();
        next.set_nonblocking(true).unwrap();
        if std::os::unix::io::AsRawFd::as_raw_fd(&next) == fd {
            reused = Some(next);
            break;
        }
        held.push(next);
    }
    let reused = reused.expect("descriptor number was not reused");
    let mut events = Events::new();
    let n = poller
        .wait(&mut events, Some(Duration::from_millis(50)))
        .unwrap();
    assert_eq!(n, 0, "a deleted descriptor delivered an event after reuse");
    let addr = reused.local_addr().unwrap();
    let _client = StdTcpStream::connect(addr).unwrap();
    events.clear();
    let n = poller
        .wait(&mut events, Some(Duration::from_millis(50)))
        .unwrap();
    assert_eq!(
        n, 0,
        "interest followed a descriptor number onto a new socket"
    );
    drop(held);
}

#[test]
fn reused_async_listener_is_not_readable() {
    within(Duration::from_secs(2), async {
        let listener = Async::<StdTcpListener>::bind(([127, 0, 0, 1], 0)).unwrap();
        let fd = std::os::unix::io::AsRawFd::as_raw_fd(&listener);
        drop(listener);
        let mut held = Vec::new();
        let mut reused = None;
        for _ in 0..128 {
            let next = Async::<StdTcpListener>::bind(([127, 0, 0, 1], 0)).unwrap();
            if std::os::unix::io::AsRawFd::as_raw_fd(&next) == fd {
                reused = Some(next);
                break;
            }
            held.push(next);
        }
        let reused = reused.expect("descriptor number was not reused");
        let early = future::or(
            async { reused.readable().await.map(|_| true) },
            async {
                Timer::after(Duration::from_millis(80)).await;
                Ok(false)
            },
        )
        .await
        .unwrap();
        assert!(!early, "reused listener was readable with nobody connected");
        let addr = reused.get_ref().local_addr().unwrap();
        let _client = Async::<StdTcpStream>::connect(addr).await.unwrap();
        reused.readable().await.unwrap();
        drop(held);
    });
}

#[test]
fn tcp_shutdown_returns_eof() {
    within(Duration::from_secs(2), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (accepted, connected) =
            future::zip(listener.accept(), TcpStream::connect(addr)).await;
        let (mut server, _) = accepted.unwrap();
        let mut client = connected.unwrap();
        client.write_all(b"hi").await.unwrap();
        client.shutdown(Shutdown::Write).unwrap();
        let mut buf = [0u8; 8];
        let n = server.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"hi");
        let n = server.read(&mut buf).await.unwrap();
        assert_eq!(n, 0);
    });
}

#[test]
fn channel_close_wakes_receiver() {
    within(Duration::from_secs(2), async {
        let (sender, receiver) = bounded::<u8>(1);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(30));
            sender.close();
        });
        assert!(receiver.recv().await.is_err());
    });
}

#[test]
fn drop_socket_leaves_reactor_usable() {
    within(Duration::from_secs(2), async {
        let socket = async_net::UdpSocket::bind("127.0.0.1:0").await.unwrap();
        drop(socket);
        Timer::after(Duration::from_millis(20)).await;
    });
}

#[test]
fn fs_remove_then_read_fails() {
    within(Duration::from_secs(2), async {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.work/tmp/qualify-close");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        async_fs::write(&path, b"hi").await.unwrap();
        let text = async_fs::read_to_string(&path).await.unwrap();
        assert_eq!(text, "hi");
        async_fs::remove_file(&path).await.unwrap();
        assert!(async_fs::read(&path).await.is_err());
    });
}

#[test]
fn bounded_send_waits_until_recv() {
    within(Duration::from_secs(2), async {
        let (sender, receiver) = bounded(1);
        sender.send(1u8).await.unwrap();
        assert!(matches!(sender.try_send(9), Err(TrySendError::Full(9))));
        let (sent, got) = future::zip(sender.send(2u8), receiver.recv()).await;
        assert_eq!(sent, Ok(()));
        assert_eq!(got, Ok(1));
        assert_eq!(receiver.recv().await, Ok(2));
    });
}

#[test]
fn blocking_pool_queues_past_the_thread_limit() {
    let _restore = RestorePool;
    blocking::set_max_blocking_threads(NonZeroUsize::new(1).unwrap());
    within(Duration::from_secs(2), async {
        let mut tasks = Vec::new();
        for i in 0..4u8 {
            tasks.push(blocking::unblock(move || {
                thread::sleep(Duration::from_millis(30));
                i
            }));
        }
        let mut out = Vec::new();
        for task in tasks {
            out.push(task.await);
        }
        out.sort_unstable();
        assert_eq!(out, vec![0, 1, 2, 3]);
    });
}

#[test]
fn second_mutex_waiter_runs() {
    let ex = Executor::new();
    let mutex = Arc::new(AsyncMutex::new(0u8));
    let hold = mutex.try_lock().expect("mutex was free");
    let first_mutex = Arc::clone(&mutex);
    let second_mutex = Arc::clone(&mutex);
    let first = ex.spawn(async move {
        *first_mutex.lock().await = 1;
    });
    let second = ex.spawn(async move {
        *second_mutex.lock().await = 2;
    });
    assert!(ex.try_tick(), "first waiter did not run");
    assert!(ex.try_tick(), "second waiter did not run");
    drop(hold);
    let ex = Arc::new(ex);
    let driver = Arc::clone(&ex);
    within(Duration::from_secs(2), driver.run(async move {
        first.await;
        second.await;
    }));
    assert_eq!(*mutex.try_lock().expect("mutex still held"), 2);
}

#[test]
fn close_survives_force_push() {
    const TRIALS: usize = 1_000;
    let mut reopens = 0;
    for trial in 0..TRIALS {
        let queue = ConcurrentQueue::bounded(2);
        let stop = AtomicBool::new(false);
        thread::scope(|scope| {
            scope.spawn(|| {
                let mut n = 0u32;
                while !stop.load(Ordering::Relaxed) && queue.force_push(n).is_ok() {
                    n = n.wrapping_add(1);
                }
            });
            while queue.pop().is_err() {}
            for _ in 0..(trial % 50) * 10 {
                std::hint::spin_loop();
            }
            queue.close();
            stop.store(true, Ordering::Relaxed);
        });
        if !queue.is_closed() {
            reopens += 1;
        }
    }
    assert_eq!(reopens, 0, "reopens in {TRIALS} trials");
}

#[test]
fn dropped_listener_does_not_consume_notify() {
    within(Duration::from_secs(2), async {
        let event = Arc::new(Event::new());
        drop(event.listen());
        let waiter = event.listen();
        let notify = Arc::clone(&event);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(30));
            notify.notify(1);
        });
        waiter.await;
    });
}

#[test]
fn facade_spawn_completes() {
    let task = smol::spawn(async { 7u8 });
    let value = within(Duration::from_secs(2), task);
    assert_eq!(value, 7);
}

#[test]
fn concurrent_boundaries_overlap() {
    const TRIALS: usize = 50;
    const LIMIT: Duration = Duration::from_secs(20);
    let start = Instant::now();
    let gate = Arc::new(std::sync::Barrier::new(4));
    thread::scope(|scope| {
        let lane = Arc::clone(&gate);
        scope.spawn(move || {
            lane.wait();
            for _ in 0..TRIALS {
                let ex = Executor::new();
                let (sender, receiver) = bounded::<u8>(1);
                let leftover = receiver.clone();
                let task = ex.spawn(async move { receiver.recv().await.ok() });
                assert!(ex.try_tick(), "overlap spawn/cancel: parked recv did not run");
                let outcome = within(Duration::from_secs(2), ex.run(task.cancel()));
                assert_eq!(outcome, None, "overlap spawn/cancel: cancel lost the parked task");
                sender.try_send(1).unwrap();
                assert_eq!(
                    leftover.try_recv().unwrap(),
                    1,
                    "overlap spawn/cancel: channel unusable after cancel"
                );
            }
        });
        let lane = Arc::clone(&gate);
        scope.spawn(move || {
            lane.wait();
            for _ in 0..TRIALS {
                let ex = Executor::new();
                let mutex = Arc::new(AsyncMutex::new(0u8));
                let hold = mutex.try_lock().expect("overlap mutex: mutex was free");
                let first_mutex = Arc::clone(&mutex);
                let second_mutex = Arc::clone(&mutex);
                let first = ex.spawn(async move {
                    *first_mutex.lock().await = 1;
                });
                let second = ex.spawn(async move {
                    *second_mutex.lock().await = 2;
                });
                assert!(ex.try_tick(), "overlap mutex: first waiter did not run");
                assert!(ex.try_tick(), "overlap mutex: second waiter did not run");
                drop(hold);
                let ex = Arc::new(ex);
                let driver = Arc::clone(&ex);
                within(Duration::from_secs(2), driver.run(async move {
                    first.await;
                    second.await;
                }));
                assert_eq!(
                    *mutex.try_lock().expect("overlap mutex: mutex still held"),
                    2,
                    "overlap mutex: second waiter did not run last"
                );
            }
        });
        let lane = Arc::clone(&gate);
        scope.spawn(move || {
            lane.wait();
            let mut reopens = 0;
            for _ in 0..TRIALS {
                let queue = ConcurrentQueue::bounded(2);
                let stop = AtomicBool::new(false);
                thread::scope(|inner| {
                    inner.spawn(|| {
                        let mut n = 0u32;
                        while !stop.load(Ordering::Relaxed) && queue.force_push(n).is_ok() {
                            n = n.wrapping_add(1);
                        }
                    });
                    while queue.pop().is_err() {}
                    queue.close();
                    stop.store(true, Ordering::Relaxed);
                });
                if !queue.is_closed() {
                    reopens += 1;
                }
            }
            assert_eq!(reopens, 0, "overlap queue: close reopened under force_push");
        });
        let lane = Arc::clone(&gate);
        scope.spawn(move || {
            lane.wait();
            for _ in 0..TRIALS {
                let ex = Executor::new();
                let (sender, receiver) = bounded::<u8>(1);
                let task = ex.spawn(async move { receiver.recv().await });
                assert!(ex.try_tick(), "overlap channel: recv did not park");
                sender.close();
                let outcome = within(Duration::from_secs(2), ex.run(task));
                assert!(
                    outcome.is_err(),
                    "overlap channel: closed channel returned a value"
                );
            }
        });
    });
    assert!(
        start.elapsed() < LIMIT,
        "overlap lanes took {:?}, expected under {LIMIT:?}",
        start.elapsed()
    );
}

struct RestorePool;

impl Drop for RestorePool {
    fn drop(&mut self) {
        blocking::set_max_blocking_threads(NonZeroUsize::new(500).unwrap());
    }
}

struct DeleteOnDrop<'a, T: polling::AsSource> {
    poller: &'a Poller,
    source: &'a T,
}

impl<T> Drop for DeleteOnDrop<'_, T>
where
    T: polling::AsSource,
{
    fn drop(&mut self) {
        let _ = self.poller.delete(self.source);
    }
}

#[cfg(unix)]
struct RestoreSig(libc::sigaction);

#[cfg(unix)]
impl Drop for RestoreSig {
    fn drop(&mut self) {
        // SAFETY: restores the action captured before this test installed its handler.
        unsafe {
            libc::sigaction(libc::SIGUSR2, &self.0, std::ptr::null_mut());
        }
    }
}

#[cfg(unix)]
fn install_noop_sigusr2() -> RestoreSig {
    // SAFETY: the handler only returns. `RestoreSig` puts the previous action back.
    unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        let mut previous: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = nop_sigusr2 as *const () as libc::sighandler_t;
        assert_eq!(libc::sigemptyset(&mut action.sa_mask), 0);
        action.sa_flags = 0;
        let rc = libc::sigaction(libc::SIGUSR2, &action, &mut previous);
        assert_eq!(rc, 0, "sigaction: {}", io::Error::last_os_error());
        RestoreSig(previous)
    }
}

#[cfg(unix)]
extern "C" fn nop_sigusr2(_signal: libc::c_int) {}
