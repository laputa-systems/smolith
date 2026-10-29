use std::io::{self, Error};
use std::sync::atomic::Ordering;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

use crate::{Events, Poller, INTERRUPT_RECOVERIES};

struct Tid(libc::pthread_t);

// SAFETY: `pthread_t` is an identifier. The receiving thread only passes it to `pthread_kill`.
unsafe impl Send for Tid {}

#[test]
fn sigusr1_on_the_waiter_runs_interrupt_recovery() -> io::Result<()> {
    let previous = install_noop_sigusr1();
    let _restore = Restore(previous);
    let poller = Arc::new(Poller::new()?);
    let waiting = Arc::clone(&poller);
    let (tid_tx, tid_rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        tid_tx
            .send(Tid(unsafe { libc::pthread_self() }))
            .unwrap();
        let mut events = Events::new();
        waiting.wait(&mut events, Some(Duration::from_secs(2)))
    });
    let Tid(tid) = tid_rx.recv().unwrap();
    let before = INTERRUPT_RECOVERIES.load(Ordering::Relaxed);
    let started = Instant::now();
    while INTERRUPT_RECOVERIES.load(Ordering::Relaxed) == before {
        if started.elapsed() > Duration::from_secs(1) {
            panic!("waiter never entered interrupt recovery");
        }
        let rc = unsafe { libc::pthread_kill(tid, libc::SIGUSR1) };
        assert_eq!(rc, 0, "pthread_kill: {}", Error::last_os_error());
        thread::sleep(Duration::from_millis(5));
    }
    poller.notify()?;
    handle.join().expect("waiter thread panicked")?;
    assert!(INTERRUPT_RECOVERIES.load(Ordering::Relaxed) > before);
    Ok(())
}

struct Restore(libc::sigaction);

impl Drop for Restore {
    fn drop(&mut self) {
        unsafe {
            libc::sigaction(libc::SIGUSR1, &self.0, std::ptr::null_mut());
        }
    }
}

fn install_noop_sigusr1() -> libc::sigaction {
    unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        let mut previous: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = nop as *const () as libc::sighandler_t;
        assert_eq!(libc::sigemptyset(&mut action.sa_mask), 0);
        action.sa_flags = 0;
        let rc = libc::sigaction(libc::SIGUSR1, &action, &mut previous);
        assert_eq!(rc, 0, "sigaction: {}", Error::last_os_error());
        previous
    }
}

extern "C" fn nop(_signal: libc::c_int) {}
