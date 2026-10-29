use std::io::{self, Error};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

use polling::{Events, Poller};

fn serial() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|err| err.into_inner())
}

#[test]
fn notify_wait_blocks_until_the_timeout() -> io::Result<()> {
    let _guard = serial();
    let poller = Poller::new()?;
    let mut events = Events::new();

    poller.notify()?;
    poller.wait(&mut events, None)?;

    let timeout = Duration::from_millis(100);
    let start = Instant::now();
    poller.wait(&mut events, Some(timeout))?;
    let elapsed = start.elapsed();
    assert!(
        elapsed >= timeout.saturating_sub(Duration::from_millis(30)),
        "second wait returned in {elapsed:?}"
    );
    Ok(())
}

#[cfg(unix)]
#[test]
fn notify_wakes_after_sigusr1() -> io::Result<()> {
    let _guard = serial();
    let restore = install_noop_sigusr1();
    let _restore = Restore(restore);

    for iteration in 0..50 {
        let poller = Arc::new(Poller::new()?);
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
        // SAFETY: SIGUSR1 has a no-op handler for the rest of this test.
        let rc = unsafe { libc::kill(libc::getpid(), libc::SIGUSR1) };
        assert_eq!(rc, 0, "kill: {}", Error::last_os_error());
        poller.notify()?;
        let (elapsed, result) = handle.join().expect("waiter thread panicked");
        result?;
        assert!(
            elapsed < Duration::from_millis(1_500),
            "iteration {iteration} blocked for {elapsed:?}"
        );
    }
    Ok(())
}

#[cfg(unix)]
struct Restore(libc::sigaction);

#[cfg(unix)]
impl Drop for Restore {
    fn drop(&mut self) {
        // SAFETY: restores the action captured before this test installed its handler.
        unsafe {
            libc::sigaction(libc::SIGUSR1, &self.0, std::ptr::null_mut());
        }
    }
}

#[cfg(unix)]
fn install_noop_sigusr1() -> libc::sigaction {
    // SAFETY: the handler only returns. `Restore` puts the previous action back.
    unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        let mut previous: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = nop_sigusr1 as *const () as libc::sighandler_t;
        assert_eq!(libc::sigemptyset(&mut action.sa_mask), 0);
        action.sa_flags = 0;
        let rc = libc::sigaction(libc::SIGUSR1, &action, &mut previous);
        assert_eq!(rc, 0, "sigaction: {}", Error::last_os_error());
        previous
    }
}

#[cfg(unix)]
extern "C" fn nop_sigusr1(_signal: libc::c_int) {}
