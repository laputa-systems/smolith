# Changelog

## 0.1.0 - 2026-10-28

The workspace version is 0.1.0. Package names are `laputa-<upstream-name>`. The facade package is `laputa-smol` and the library name is `smol`. These crates are not on crates.io. Windows is unsupported.

Safe task builders require `'static` metadata. An empty `Unblock` read or write returns without waiting on the pipe, and `flush` pushes buffered bytes through to the inner writer. After `Lines` reports a malformed line, it discards that line before reading the next one. Executor shutdown collects a runnable whose wake or cancellation began before the queue closed. That bracket starts before the task is marked scheduled, ends before the last waker frees the task, and notifies runners only after the publication count reaches zero. A schedule callback that drops its runnable still runs the end hook on a live task. An `Unblock` write after a successful flush requires another flush. Reactor registration invokes a displaced waker after it releases the source mutex. The advisory scan refreshes its database, counts an opt-in feature as a production dependency, and orders prerelease identifiers by SemVer.
