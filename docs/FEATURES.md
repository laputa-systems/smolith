# Feature matrix

Reference for the `[features]` table of each workspace crate at version 0.1.0. Cargo also turns an optional dependency into a feature when the `[features]` table does not name it. Those implicit features are listed under the crate.

Crates with no `[features]` table and no optional dependency: `laputa-async-broadcast`, `laputa-async-dup`, `laputa-async-fs`, `laputa-async-io`, `laputa-async-net`, `laputa-async-process`, `laputa-async-signal`, `laputa-atomic-waker`, `laputa-blocking`, `laputa-easy-parallel`, `laputa-parking`, `laputa-polling`, `laputa-smol`, `laputa-smol-macros`.

| Crate | Default | Other features |
|---|---|---|
| `laputa-async-channel` | `std` | `std` enables `concurrent-queue/std` and `event-listener-strategy/std`. `portable-atomic` enables the same features on those crates and the local `portable-atomic` dependencies. |
| `laputa-async-executor` | none | `static` is empty. |
| `laputa-async-lock` | `std` | `std` enables `event-listener/std` and `event-listener-strategy/std`. `loom` enables `event-listener/loom` and the `loom` dependency. |
| `laputa-async-task` | `std` | `std` is empty. Implicit feature `portable-atomic` from the optional dependency. |
| `laputa-concurrent-queue` | `std` | `std` is empty. Implicit feature `portable-atomic` from the optional dependency. The `loom` dependency is optional and only linked under `cfg(loom)`. |
| `laputa-event-listener` | `std` | `std` enables `parking`. `portable-atomic` enables `portable-atomic-util` and `portable_atomic_crate`. `loom` enables `parking?/loom` and the `loom` dependency. |
| `laputa-event-listener-strategy` | `std` | `std` enables `event-listener/std`. `portable-atomic` enables `event-listener/portable-atomic`. `loom` enables `event-listener/loom`. |
| `laputa-fastrand` | `std` | `alloc` is empty. `std` enables `alloc`. `js` enables `std` and `getrandom`. |
| `laputa-fastrand-contrib` | `std` | `std` enables `alloc` and `fastrand/std`. `alloc` enables `fastrand/alloc`. `libm` enables `libm_dep`. |
| `laputa-futures-lite` | `race`, `std` | `std` enables `alloc`, `fastrand/std`, `futures-io`, and `parking`. `alloc` is empty. `race` enables `fastrand`. |
| `laputa-piper` | `std` | `std` enables `fastrand/std` and `futures-io`. `portable-atomic` enables `atomic-waker/portable-atomic`, `portable_atomic_crate`, and `portable-atomic-util`. |
| `laputa-waker-fn` | none | `portable-atomic` enables `portable-atomic-util`. There is no `default` feature. |

`tools/graph-gate.sh` resolves the workspace at default features and at `--all-features` for the host, `aarch64-unknown-linux-musl`, and `x86_64-unknown-linux-musl`.
