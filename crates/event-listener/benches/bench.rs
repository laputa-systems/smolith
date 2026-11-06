use std::iter;

use event_listener::{Event, Listener};
use rustybench::Bencher;

fn main() {
    rustybench::main();
}

const COUNT: usize = 8000;

#[rustybench::bench]
fn notify_and_wait(bencher: Bencher) {
    let ev = Event::new();
    let mut handles = Vec::with_capacity(COUNT);

    bencher.bench_local(|| {
        handles.extend(iter::repeat_with(|| ev.listen()).take(COUNT));

        ev.notify(COUNT);

        for handle in handles.drain(..) {
            handle.wait();
        }
    });
}
