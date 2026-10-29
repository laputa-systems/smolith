use async_signal::{Signal, Signals};
use futures_lite::{future, prelude::*};

/// Send SIGINT to the current process.
#[cfg(unix)]
fn sigint() {
    unsafe {
        libc::raise(libc::SIGINT);
    }
}

#[test]
fn test_sigint() {
    future::block_on(async {
        let mut signals = Signals::new(Some(Signal::Int)).unwrap();
        let mut next = signals.next();
        assert!(future::poll_once(&mut next).await.is_none());
        sigint();
        assert_eq!(signals.next().await.unwrap().unwrap(), Signal::Int);
    });
}
