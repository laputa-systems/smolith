use atomic_waker::AtomicWaker;
use rustybench::{black_box, Bencher};

use std::task::Waker;

fn main() {
    rustybench::main();
}

#[derive(Clone, Copy, Debug)]
enum Contention {
    Low,
    High,
}

const CONTENTIONS: &[Contention] = &[Contention::Low, Contention::High];

impl Contention {
    fn run(self, bencher: Bencher, f: impl Fn(&AtomicWaker) + Send + Sync) {
        let waker = AtomicWaker::new();

        match self {
            Self::Low => bencher.bench_local(|| {
                for _ in 0..500 {
                    f(&waker);
                }
            }),
            Self::High => bencher.bench_local(|| {
                use rayon::prelude::*;
                (0..100_000).into_par_iter().for_each(|_| f(&waker));
            }),
        }
    }
}

#[rustybench::bench(args = CONTENTIONS)]
fn store_and_wake(bencher: Bencher, contention: &Contention) {
    contention.run(bencher, |waker| {
        waker.register(Waker::noop());
        waker.wake();
    });
}

#[rustybench::bench(args = CONTENTIONS)]
fn store_and_take(bencher: Bencher, contention: &Contention) {
    contention.run(bencher, |waker| {
        waker.register(Waker::noop());
        black_box(waker.take());
    });
}

#[rustybench::bench(args = CONTENTIONS)]
fn wake_without_store(bencher: Bencher, contention: &Contention) {
    contention.run(bencher, |waker| {
        waker.wake();
    });
}

#[rustybench::bench(args = CONTENTIONS)]
fn take_without_store(bencher: Bencher, contention: &Contention) {
    contention.run(bencher, |waker| {
        black_box(waker.take());
    });
}
