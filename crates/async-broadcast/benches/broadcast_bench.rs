use async_broadcast::broadcast;
use futures_lite::future::block_on;
use rustybench::Bencher;

fn main() {
    rustybench::main();
}

const RECEIVERS: &[usize] = &[1, 2, 4, 8];

#[rustybench::bench(args = RECEIVERS)]
fn broadcast_and_recv(bencher: Bencher, receivers: usize) {
    let (sender, first) = broadcast(1);
    let mut receivers: Vec<_> = (1..receivers).map(|_| first.clone()).collect();
    receivers.push(first);
    let mut n = 0;
    bencher.bench_local(|| {
        block_on(async {
            sender.broadcast(n).await.unwrap();
            for receiver in &mut receivers {
                assert_eq!(receiver.recv().await.unwrap(), n);
            }
            n += 1;
        })
    });
}
