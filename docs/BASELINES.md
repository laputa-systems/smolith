# Baselines

Reference measurements taken after S06, before any later performance change. No earlier numeric baseline is in the tree. These figures are the record. They are not a comparison and they do not name a faster implementation.

Host: macOS aarch64, 18 cores, load 2.49, 88% memory free. Toolchain: the pinned nightly. Binary: `cargo run --release -p smolith-qualify --bin baseline`. Five runs. Every run checked its output: the timer returned, the pool job returned `1`, each wake read the byte `z`, and the TCP peer read `4194304` bytes.

Idle CPU is process user plus system time from `getrusage` divided by wall time. The timer sample is one `Timer::after` of 1 second. The pool sample is 1 second of sleep after one `unblock` and an 800 ms pause, so the idle thread can exit. Wake time is a Unix socket write, `readable`, and a one-byte read, 100 samples per run after 10 discarded warmups. The reported wake figure is the median of that run. TCP is 4 MiB written to `127.0.0.1` in 64 KiB chunks, timed through peer shutdown and the peer's final read.

| Probe | Statistic | Value | Spread across 5 runs |
|---|---|---|---|
| Reactor idle | median CPU | 0.0049% | 0.0034% to 0.0130% |
| Blocking pool idle | median CPU | 0.0010% | 0.0006% to 0.0025% |
| Wake | median of run medians | 15 µs | run medians 6, 15, 15, 15, 15 µs |
| TCP loopback | median | 4359 MiB/s | 3012 to 6686 MiB/s |

Idle stays under 1% CPU, so there is no spin to fix. The TCP loopback number moves by about 2x. Treat it as a coarse record. The limiter was not profiled. A later run is a regression only when it falls outside this spread for the same probe, build, and host.
