//! Benchmarks for registering timers.

use async_io::Timer;
use futures_lite::future;
use rustybench::{black_box, Bencher};
use std::time::Duration;

fn main() {
    rustybench::main();
}

/// Create a new `Timer` and poll it once to register it into the timer wheel.
fn make_timer() -> Timer {
    let mut timer = Timer::after(Duration::from_secs(3600));
    future::block_on(future::poll_once(&mut timer));
    timer
}

/// Benchmark the time it takes to register and deregister a timer.
#[rustybench::bench(args = [0, 1_000_000])]
fn register_timer(bencher: Bencher, prev_timer_count: usize) {
    let timers: Vec<_> = (0..prev_timer_count).map(|_| make_timer()).collect();

    bencher.bench_local(|| {
        let timer = make_timer();
        black_box(timer);
    });

    drop(timers);
}

/// Drop a still-live set. The deadline is an hour out, so none of the timers expire during setup.
#[rustybench::bench(args = [1_000, 10_000])]
fn cancel_live_timers(bencher: Bencher, count: usize) {
    bencher.bench_local(|| {
        let timers: Vec<_> = (0..count).map(|_| make_timer()).collect();
        assert_eq!(timers.len(), count);
        drop(timers);
    });
}
