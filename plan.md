# Build smolith: unified, independently maintained smol ecosystem

Independent monorepo for the complete active no-Tokio smol ecosystem. Repo `smolith`; published crates `laputa-<upstream-name>`; facade `laputa-smol`.

## Status

- Goal: one clean `smolith` repo with all 26 scoped components, consistent Laputa dependency graph, fixed known soundness bugs, qualified on primary platforms, proven via external consumer plus h12tiny fixture, with unified CI and verified release artifacts.
- Last accepted: S08 (external consumer and h12tiny fixture) on macOS aarch64.
- Accepted slices: S01, S02, S03, S04, S05, S06, S07, S07.5, S08. Red: none. Parked: none.
- Next: S09.
- Resume: S04 through S08 are on `main`. Do not restart them.
- S08: `tests/consumer/check.sh` resolved owned crates to `laputa-*` path sources with shared `futures-io` and `futures-core` and no tokio; interop tests passed. `fixtures/h12tiny.sh` pinned `efeea1cd4fdc028c1f74fa853c96ae4e3bccd17e`. HTTP/1, HTTP/2, TLS, cancellation, reuse, timeout, and shutdown gates passed. Backpressure is a named gap: the only tests are tokio hyper and h2 harnesses outside the fixture workspace.
- S07 platforms: macOS aarch64 execution (`cargo test --workspace` green). Linux glibc aarch64 execution in native Docker on pinned `nightly-2026-09-15` (`cargo test --workspace` green). Linux musl aarch64 execution for `cargo test --workspace --exclude laputa-smol`; the full musl workspace does not build because `openssl-sys` has no musl sysroot, so `laputa-smol` tests did not run there. Linux musl x86_64 is compile-only (no qemu); libs and tests that do not need a cross C compiler checked, and the full check stops on `x86_64-linux-musl-gcc` for criterion's `alloca`. ASan `wake_during_drop` and TSan `close_survives_force_push` passed on Linux glibc aarch64 with `-Zbuild-std`. Darwin ASan/TSan were not run. Windows is removed: `cargo check --workspace --all-targets --target x86_64-pc-windows-msvc` stops at `compile_error!`.
- Linux `async-h1` and `surf` examples are stubs. `rustix` 0.37 does not compile on this nightly, and only those examples use it. They still build on macOS.

## Charter

- Decide and keep going. Record a decision only when a later slice depends on it. Run the parent steering checkpoint after each accepted slice, on resume, and after any slice fails twice; rewrite Status in place.
- One writer by default. Parallel lanes only for the disjoint pairs named in Sequence, in worktrees at `.work/wt/<lane>` following the orchestrate worktree lifecycle.
- Within the execution grant, proceed without waiting. Park anything needing authority outside the grant.
- Hard stops park the item: irreversible or outside-the-repo actions, a forbidden boundary, or a loop with nothing checkable. Pushing is always forbidden.
- Parity before optimization. Fix egregious problems met on the way in their own commit: idle spin above 1% CPU at rest, periodic wakeups without work above 1 Hz, unbounded queue/thread growth, or a hang without a watchdog. Record anything else and leave it.
- Every commit is a Conventional Commit with the workstream as scope and the item ID in the body. Code comments never cite this doc.
- Red work never lands on `main`. A failing item gets at most 3 attempts, then parks on its branch with cause and retry condition.

## Decisions

- Owner 2026-10-29: repo is `smolith` (remote `laputa-systems/smolith` already configured), published crates are `laputa-<upstream-name>` (`laputa-smol` facade).
- Prescribed: fresh Git history without upstream graphs, submodules, or nested `.git`; upstream license texts, notices, attribution, and exact SHA provenance preserved in `UPSTREAM.toml`. Reason: code is the truth; history carries no behavior.
- Prescribed: canonical ledger lives at `docs/LEDGER.md`, not `plan.md`. Reason: ledger of every issue/PR cannot fit the 300-line doc cap; `plan.md` keeps counts and pointers. Supersedes draft section 4.
- Prescribed: never push. Reason: repo rule pushes only when asked; supersedes draft "push when available". Report the exact remaining remote operation at close.
- Prescribed: no root `[patch.crates-io]` as migration substitute; explicit `package` plus `path` plus `version` edges under `[workspace.dependencies]`. Versionless path edges allowed only for member-level dev-dependencies to break publish cycles, or cross-layer tests move to nonpublishable integration packages. Reason: patches do not rename third-party requests and are not inherited by consumers.
- Prescribed: `futures-core` and `futures-io` stay the shared external trait identities; aliases preserve spelling only. No `tokio` or `tokio-util` in canonical normal/build paths. Reason: trait identity is the interop contract.
- Prescribed: archived `async-rustls`, `cache-padded`, `vec-arena`, `nb-connect` stay excluded after disposition check in S01; `async-compat` (Tokio bridge) and `.github` (infra) stay out of the shipped set. Reason: matches current archived flags verified 2026-10-29.
- Prescribed: h12tiny fixture is read-only at a recorded commit under disposable checkout; all campaign changes land in `smolith`. Reason: destructive outside-the-repo edits need explicit authority.
- Delegated: executor picks stable pin at S02 and documents MSRV as the max inherited `rust-version` unless a fix raises it, then verifies with that toolchain. Bound: no historical compiler matrix.
- Delegated: executor samples per-crate feature combinations beyond the default. Bound: default plus `no_std` plus portable-atomic where they exist, plus each fix's affected feature.
- Delegated: registry-name collisions. Bound: if any `laputa-*` name is taken, park S02 with evidence and continue all non-packaging work; do not invent an alternate prefix unasked.
- Owner 2026-10-28: remove Windows support entirely in S07.5, before S08. Reason: no Windows target is in scope. IOCP, `cfg(windows)`, and Windows-only dependencies go. A Windows build must fail loudly.
- Owner 2026-10-28: no formatting tools. S09 keeps linters such as Clippy and drops rustfmt and other formatters. Reason: formatters are out of scope.

## Baseline

- Tree 2026-10-29: no commits, no red, no `.work/`. Remote `origin` is `laputa-systems/smolith`. Toolchain `rustc/cargo 1.99.1`, `gh`, `mdtool`, `jaq`, `rg`, `fd` present. Host `Darwin arm64`.
- Upstream inventory verified 2026-10-29 via `gh repo list smol-rs`: 26 active code repos matching the draft scope plus `.github`, `async-compat`, and 4 archived, with `event-listener-strategy`, `fastrand-contrib`, `piper` on `main` and the rest on `master`. Exact SHAs pinned in S01.
- Loop timings: `cargo --version` 0.03 s; `mdtool check --untracked plan.md` 0 findings. Workspace metadata, graph-gate, per-slice test, and full-suite timings first measured in S01/S02/S07 and recorded here before any expensive run.

## Sequence

- S01 Import and provenance. Owns `crates/<upstream-name>/`, `UPSTREAM.toml`, `.git/info/exclude` (`.work/`). Prereq: none. Accept: `cargo metadata --no-deps --format-version 1 >/dev/null && test -f UPSTREAM.toml && git status --short | rg -v '^\?\? .work/' | head` shows only intended files; no nested `.git`; all 26 present or an evidence-backed count update plus archived dispositions.
- S02 Workspace identity and graph gate. Integrator-owned: root `Cargo.toml`, `Cargo.lock`, `crates/*/Cargo.toml`, `tools/graph-gate.sh`. Prereq: S01. Accept: `tools/graph-gate.sh` passes on supported feature/target combos: owned deps resolve to `laputa-*` path sources, no upstream-owned packages or `tokio` in normal/build paths, one version per owned component, no unpublished paths or workspace-only patches required by consumers.
- S03 Issue ledger. Owns `docs/LEDGER.md`. Prereq: S01. Accept: `test -f docs/LEDGER.md` with snapshot date, pinned SHAs, and every open issue/PR plus merged-since-release fixes, closed bug reports, and RustSec advisories classified with URL, component, severity, disposition, local commit, and regression evidence; mandatory targets cross-linked to S04-S06.
- S04 Executor and task soundness. Owns `crates/async-executor`, `crates/async-task`. Prereq: S02, S03. Accept: `cargo test -p laputa-async-executor -p laputa-async-task` green plus teardown-regression (wake races destruction), metadata Send/Sync compile-fail tests, Miri run for the local-task report, and waker-identity progress/idle checks.
- S05 Reactor, polling, networking. Owns `crates/async-io`, `crates/polling`, `crates/async-net`. Prereq: S02, S03. Accept: `cargo test -p laputa-async-io -p laputa-polling -p laputa-async-net` green plus tick-snapshot/wake-replacement, UDP peek loopback, and preserved polling fixes validation on each reachable backend.
- S06 Blocking, sync, helpers. Owns `crates/blocking`, `crates/concurrent-queue`, `crates/async-channel`, `crates/futures-lite`, `crates/event-listener`, remaining helpers. Prereq: S02, S03. Accept: `cargo test` for the owned set green plus spawn-failure fault injection, idle-retirement checks, write-error propagation, close/overwrite interleaving models, line-reader contracts, and `StackSlot` 5.4.2/RUSTSEC-2026-0221 cross-thread rejection coverage.
- S07 System qualification. Owns `tests/`, feature-matrix docs, baseline records. Prereq: S04-S06. Accept: `cargo test --workspace` on reachable primary targets green with bounded watchdogs; cancellation, destruction, rearm, timer, syscall-interrupt, descriptor-reuse, shutdown, exhaustion, and close/cleanup boundaries exercised; sanitizer plus model/interleaving runs for touched races; idle/wake/latency/throughput measured against S06 baselines with material regressions investigated.
- S07.5 Drop Windows. Owns every `cfg(windows)`, IOCP, and Windows-only dependency under `crates/`. Prereq: S07. Accept: `rg` finds no `cfg(windows)`, `target_os = "windows"`, `windows-sys`, or `winapi` in `*.rs` and `*.toml`; `cargo test --workspace` still green on the host; a Windows target fails at compile time.
- S08 External consumer and h12tiny fixture. Owns `tests/consumer/`, `fixtures/h12tiny.*`. Prereq: S02, S07.5. Accept: clean consumer without workspace patches resolves everything to `laputa-*` with trait interop checked; fixture script pins an immutable h12tiny commit, records its unmodified baseline, redirects edges preserving aliases, regenerates the lockfile with minimal changes, and runs the applicable HTTP/1, HTTP/2, TLS, cancellation, reuse, backpressure, timeout, and shutdown gates with fixture gaps named, never reported as passed when unrun.
- S09 CI and release tooling. Integrator-owned: `.github/workflows/`, packaging scripts, `SECURITY.md` advisory mapping via `UPSTREAM.toml`. Prereq: S07.5-S08. Accept: root workflows cover Clippy and other linters, tests/doctests, feature/target combos, graph gate, advisory scan mapped via `UPSTREAM.toml`, concurrency checks, consumer plus fixture, and packaging with Actions pinned to SHAs. No rustfmt and no other formatter. `cargo package --dry-run` plus staging-registry build and fresh-consumer check without `--no-verify`; license files and dependency names verified on normalized artifacts; a clean scan of renamed packages alone is not evidence, mapping through original names in `UPSTREAM.toml` is required.
- S10 Docs and close. Owns `README.md`, `CHANGELOG.md`, `SECURITY.md`, `AGENTS.md`, final `UPSTREAM.toml`, dead-code deletion. Prereq: all above. Accept: `mdtool check` clean, full suite at close green, `ultrareview` P1/P2 fixed, `SECURITY.md` carries private-reporting guidance, tree clean with work committed, release tag versus artifacts versus publication distinguished.

Parallel lanes allowed only between disjoint ownership sets: S04 with S05, S04 with S06, S05 with S06. All other items are serial. Manifests, lockfile, package identity, and CI need integrator review. Methods may be revised only for ordering, width, and batching; deliverables, contracts, proof, and effects stay as written.

## Acceptance

- All 26 components present with exact SHA provenance: S01 check plus `jaq --from toml` key counts in `UPSTREAM.toml`.
- Consistent Laputa graph with no upstream residue or Tokio: `tools/graph-gate.sh` in S02 and S07.
- License and provenance survive import and packaging: S01 plus normalized-artifact license check in S09.
- Every tracker item dispositioned with evidence, including RustSec advisories: `docs/LEDGER.md` review in S03; confirmed bugs fixed with failing-baseline and regression tests in S04-S06.
- Primary-platform results with gaps labeled execution, emulation, compile-only, or unavailable: S07 plus Needs environment below.
- External consumer and h12tiny fixture demonstrably on Laputa implementations with feature isolation and no-Tokio: S08 gates.
- Unified CI and consumable packaged artifacts verified without hidden patches: S09 packaging flow.
- Clean committed tree with consumption instructions and blocker report: S10.

## Needs environment

- BSD/kqueue specifics beyond reachable macOS runners: owner runs `cargo test -p laputa-polling -p laputa-async-io` on the target BSD and reports.
- Linux musl x86_64 execution, and `laputa-smol` tests on musl aarch64: the first needs qemu or a musl cross C toolchain; the second needs a musl OpenSSL sysroot for `openssl-sys`.
- Full h12tiny TLS plus HTTP/2 matrix on platforms the executor cannot reach: owner runs the S08 fixture script there and reports.

## Execution boundary and non-goals

- Authorized: read and fetch upstream SHAs; disposable h12tiny checkout under `.work/`; edit, build, test, and commit inside `smolith`; add root CI workflows and release scripts; add vetted dependencies staying within build-time and binary-size gates with the reason in the reply.
- Forbidden: pushing; modifying or pushing the real h12tiny repo; public registry publication; `/tmp` or `$TMPDIR` for worktrees, scratch, or state; formatters; history rewrites; scope expansion from a stale note.
- Contract changes: `smol` to `laputa-smol`, all others to `laputa-<upstream-name>`, one workspace `0.1.0` version train with upstream versions in provenance, versionless member-level dev-deps only to break publish cycles, canonical normal/build graph strictly no-Tokio.
- Non-goals: actual crates.io publication; reviving archived repos; historical compiler matrix or broad edition migration; unrelated Hyper/H2/WebSocket protocol-test migration beyond small adapters; new APIs or speculative optimizations except the listed correctness repairs.

## Reference

- Upstream scope verified 2026-10-29: facade `smol`, `smol-macros`; execution/IO `async-executor`, `async-task`, `async-io`, `polling`, `blocking`; OS `async-net`, `async-fs`, `async-process`, `async-signal`; sync `async-channel`, `async-broadcast`, `async-lock`, `event-listener`, `event-listener-strategy`, `concurrent-queue`, `parking`, `atomic-waker`, `piper`; helpers `futures-lite`, `fastrand`, `fastrand-contrib`, `easy-parallel`, `waker-fn`, `async-dup`. Out: `async-compat`, `.github`, archived `async-rustls`, `cache-padded`, `vec-arena`, `nb-connect`.
- Mandatory bug entry points: executor teardown, task metadata, local-task pinning, reactor wakes, waker identity, blocking spawn/retirement/write-errors, queue close race, polling OS contracts, UDP peek, line readers, `StackSlot` bounds. Full per-issue detail belongs in `docs/LEDGER.md`.
- Primary targets: Linux/musl x86_64, Linux/musl aarch64, macOS aarch64; sanitizers on Linux/glibc. Windows is out of scope after S07.5. BSD where available, else Needs environment.
