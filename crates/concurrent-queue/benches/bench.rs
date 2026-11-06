use std::fmt::Debug;

use concurrent_queue::{ConcurrentQueue, PopError};
use easy_parallel::Parallel;
use rustybench::{black_box, Bencher};

fn main() {
    rustybench::main();
}

const COUNT: usize = 100_000;
const THREADS: usize = 7;

fn spsc<T: Default + std::fmt::Debug + Send>(recv: &ConcurrentQueue<T>, send: &ConcurrentQueue<T>) {
    Parallel::new()
        .add(|| loop {
            match recv.pop() {
                Ok(_) => (),
                Err(PopError::Empty) => (),
                Err(PopError::Closed) => break,
            }
        })
        .add(|| {
            for _ in 0..COUNT {
                send.push(T::default()).unwrap();
            }
            send.close();
        })
        .run();
}

fn mpsc<T: Default + std::fmt::Debug + Send>(recv: &ConcurrentQueue<T>, send: &ConcurrentQueue<T>) {
    Parallel::new()
        .each(0..THREADS, |_| {
            for _ in 0..COUNT {
                send.push(T::default()).unwrap();
            }
        })
        .add(|| {
            let mut recieved = 0;
            while recieved < THREADS * COUNT {
                match recv.pop() {
                    Ok(_) => recieved += 1,
                    Err(PopError::Empty) => (),
                    Err(PopError::Closed) => unreachable!(),
                }
            }
        })
        .run();
}

fn single_thread<T: Default + std::fmt::Debug>(
    recv: &ConcurrentQueue<T>,
    send: &ConcurrentQueue<T>,
) {
    for _ in 0..COUNT {
        send.push(T::default()).unwrap();
    }
    for _ in 0..COUNT {
        recv.pop().unwrap();
    }
}

#[rustybench::bench(types = [u8, u16, u32, u64, u128])]
fn unbounded_single_thread<T: Default + Debug>(bencher: Bencher) {
    bencher.bench_local(|| {
        let q = ConcurrentQueue::unbounded();
        single_thread::<T>(black_box(&q), black_box(&q));
    });
}

#[rustybench::bench(types = [u8, u16, u32, u64, u128])]
fn bounded_single_thread<T: Default + Debug>(bencher: Bencher) {
    bencher.bench_local(|| {
        let q = ConcurrentQueue::bounded(THREADS * COUNT);
        single_thread::<T>(black_box(&q), black_box(&q));
    });
}

#[rustybench::bench(types = [u8, u16, u32, u64, u128])]
fn unbounded_spsc<T: Default + Debug + Send>(bencher: Bencher) {
    bencher.bench_local(|| {
        let q = ConcurrentQueue::unbounded();
        spsc::<T>(black_box(&q), black_box(&q));
    });
}

#[rustybench::bench(types = [u8, u16, u32, u64, u128])]
fn bounded_spsc<T: Default + Debug + Send>(bencher: Bencher) {
    bencher.bench_local(|| {
        let q = ConcurrentQueue::bounded(THREADS * COUNT);
        spsc::<T>(black_box(&q), black_box(&q));
    });
}

#[rustybench::bench(types = [u8, u16, u32, u64, u128])]
fn unbounded_mpsc<T: Default + Debug + Send>(bencher: Bencher) {
    bencher.bench_local(|| {
        let q = ConcurrentQueue::unbounded();
        mpsc::<T>(black_box(&q), black_box(&q));
    });
}

#[rustybench::bench(types = [u8, u16, u32, u64, u128])]
fn bounded_mpsc<T: Default + Debug + Send>(bencher: Bencher) {
    bencher.bench_local(|| {
        let q = ConcurrentQueue::bounded(THREADS * COUNT);
        mpsc::<T>(black_box(&q), black_box(&q));
    });
}
