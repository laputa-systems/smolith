use std::thread;
use std::time::Duration;

use blocking::unblock;
use futures_lite::future;

#[test]
fn idle_thread_exits() {
    future::block_on(async {
        let first = unblock(|| thread::current().id()).await;
        thread::sleep(Duration::from_millis(800));
        let second = unblock(|| thread::current().id()).await;
        assert_ne!(first, second);
    });
}
