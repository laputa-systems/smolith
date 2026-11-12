use rustybench::black_box;
use rustybench::counter::BytesCount;

fn main() {
    rustybench::main();
}

const STREAM_SIZE: usize = 64 << 20;

#[rustybench::bench]
fn unmask_64_mib(bencher: rustybench::Bencher) {
    let mut data: Vec<u8> = Vec::with_capacity(STREAM_SIZE);
    data.resize_with(STREAM_SIZE, || fastrand::u8(..));

    bencher
        .counter(BytesCount::new(STREAM_SIZE as u64))
        .bench_local(|| {
            fastwebsockets::unmask(black_box(&mut data), [1, 2, 3, 4]);
        });
}
