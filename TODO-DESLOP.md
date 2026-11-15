Prefix: DSLP
Next: 6

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier D). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After DSLP-5, work the open P1 and P2 findings, then fill in `## Report` below.

## Report

Not started.

## Tickets

### DSLP-1 · P2 · Ground: baseline, boundaries, and CI test entry point

- **Area:** deslop · ground
- **Where:** AGENTS.md, Makefile, .github/workflows/
- **Problem:** Base 347b1d2; code lines Rust 115.2k, Python 2.9k (fleet/loc/v1, vendored included); timings not yet measured. Open fleet lint findings: mdtool 82, gha-syntax 10, linux-musl-toolchain 5, laputa-rustix 4, no-formatters 4, rust-collections 3, audit-upgrades 1, make-clean-target 1, and 2 more. Over 50k lines, so Types, Tests, and Dead code run per component; largest first-party groups: crates/laputa-h2-futures-lite (39k), crates/laputa-hyper-futures-lite (32k), crates/futures-lite (8k), crates/h12tiny (7k), crates/async-task (6k), crates/async-lock (5k), crates/polling (5k), crates/h12tiny-client (5k), and 2 more. Consumers in the fleet: bctl, ctrs, laputa, soulseek-rs, stats, stuff, tansu, xsh; their tests are the consumer check for API changes. Perf gate: decide in Ground whether a user-visible hot path needs a workload.
- **Recommendation:** Run the deslop Ground step. Accept: CI runs make test and is green, and AGENTS.md has a Verification section.
- **Fix with:** deslop
- **Effort:** M

### DSLP-2 · P2 · Diagnose: ultrareview into this backlog

- **Area:** deslop · diagnose
- **Where:** whole repository
- **Problem:** No review has ranked this repository's risks for the deslop pass.
- **Recommendation:** Run the deslop Diagnose step. Accept: the diff touches only docs, comments, and this file, and every finding names its Fix with.
- **Fix with:** deslop
- **Effort:** M

### DSLP-3 · P2 · Types: strict profile and typing findings

- **Area:** deslop · types
- **Where:** type checker and lint configuration; Rust/Python sources
- **Problem:** The strict-profile candidates in the deslop skill's tools reference have not been measured here.
- **Recommendation:** Run the deslop Types step. Accept: clean at the adopted profile, escape hatches not increased, witnesses recorded.
- **Fix with:** deslop
- **Effort:** M

### DSLP-4 · P2 · Tests: coverage, boundary gaps, mutation

- **Area:** deslop · tests
- **Where:** test suites and CI
- **Problem:** Coverage and mutation results are unmeasured; no deslop mutation lane exists.
- **Recommendation:** Run the deslop Tests step. Accept: no undispositioned survivor in critical code, the test cost budget holds, and the lane is green where the tools are mirrored.
- **Fix with:** deslop
- **Effort:** L

### DSLP-5 · P2 · Dead code: tools and debloat

- **Area:** deslop · dead code
- **Where:** whole repository
- **Problem:** No dead-code scan has run.
- **Recommendation:** Run the deslop Dead code step. Accept: tools clean or allowlisted with reasons; walk-backs filed as Needs owner tickets.
- **Fix with:** deslop
- **Effort:** M

