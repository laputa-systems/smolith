# Features

The [crate manifests](../crates/) own feature definitions. `cargo metadata --no-deps --format-version 1` reports the effective feature names, including those created by optional dependencies. The table lists crates with features; other workspace crates have none.

| Crate | Default | Available features other than `default` |
|---|---|---|
| `laputa-async-channel` | `std` | `std`, `portable-atomic` |
| `laputa-async-executor` | none | `static` |
| `laputa-async-io`, `laputa-async-process`, `laputa-blocking`, `laputa-polling` | none | `tracing` |
| `laputa-async-lock` | `std` | `std`, `loom` |
| `laputa-async-task` | `std` | `std`, `portable-atomic` |
| `laputa-atomic-waker` | none | `portable-atomic` |
| `laputa-concurrent-queue` | `std` | `std`, `portable-atomic`, `loom` |
| `laputa-event-listener` | `std` | `std`, `critical-section`, `loom`, `parking`, `portable-atomic`, `portable-atomic-util`, `portable_atomic_crate` |
| `laputa-event-listener-strategy` | `std` | `std`, `portable-atomic`, `loom` |
| `laputa-fastrand` | `std` | `std`, `alloc`, `js`, `getrandom` |
| `laputa-fastrand-contrib` | `std` | `std`, `alloc`, `libm`, `libm_dep` |
| `laputa-futures-lite` | `race`, `std` | `race`, `std`, `alloc`, `fastrand`, `futures-io`, `memchr`, `parking` |
| `laputa-parking` | none | `loom` |
| `laputa-piper` | `std` | `std`, `futures-io`, `portable-atomic`, `portable-atomic-util`, `portable_atomic_crate` |
| `laputa-waker-fn` | none | `portable-atomic`, `portable-atomic-util` |

Optional-dependency features can be target-specific. The Loom implementations require both the Cargo feature and `cfg(loom)`. The event-listener implementation recommends `std` or `critical-section` for production; without either, its internal lock is a spinlock.

Set `default-features = false` on the workspace dependency when a member must disable inherited defaults. A workspace all-features build unifies features and does not prove an isolated no-default-features build.

`tools/graph-gate.sh` resolves default and all-feature graphs for the host. It also checks `aarch64-unknown-linux-musl` and `x86_64-unknown-linux-musl` when those targets are installed.
