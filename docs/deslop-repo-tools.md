# Repository tools, benchmarks and CI audit

Base `31e6041519af7a3736cfab82b2babc1bd86ba8f9`; Darwin arm64. Scope is repository tools, scripts, Python tests, consumer and qualify fixtures, and twelve-dollar; Makefile and CI are read-only. No production or test code changed. Set `export PATH="/opt/homebrew/bin:$PATH" TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6`: the system Python is 3.9.6 and lacks `tomllib`; Homebrew Python is 3.15.1.

## DSLP-17 · Types

`uvx ty check tools tests --output-format concise` (ty 0.0.107) exited 1 with 10 diagnostics: seven dynamic test imports, one subprocess overload, one callback signature and one optional stream access. The tests add `tools` to the import path; staging opens the stream with a pipe. These are leads, not demonstrated runtime failures. No type rule adopted. `uvx ruff check --select ANN,ANN401,PGH003,TC,B,RET,F401,F841 tools tests --output-format concise` (ruff 0.17.6) exited 1, 323 diagnostics:

| Candidate | Count | Disposition |
| --- | ---: | --- |
| ANN | 318 (182 ANN001, 1 ANN002, 118 ANN201, 15 ANN202, 2 ANN204) | Reject: missing annotations alone do not witness a false contract. |
| ANN401, PGH003, TC | 0 each | Reject: no witness. |
| B | 2 | Reject: `zip(order, order[1:])` intentionally pairs adjacent entries; the other comparison needs separate semantic proof. |
| RET | 2 | Reject: implicit returns in the handler are not a demonstrated defect. |
| F401 / F841 | 0 / 1 | Reject as profile: unused `config_arg` at `tools/stage.py:523` is a dead local, not a type witness. |

Adopted 0 of 8 rule groups; rejected 8. `rg --files tools scripts tests benchmarks/twelve-dollar fixtures -g '*.sh' | xargs shellcheck -f gcc` (0.11.0) exited 1: 61 diagnostics, including 38 SC2016, 8 SC1007 and 5 SC1091; the latter reflects not following sourced files. No shell setting adopted: no correctness witness established. This is not a clean strict profile.

## DSLP-29 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Release versions, graph and exact published version | `tests/test_release_graph.py:48`, `:59`, `:135`, `:249`; `tests/test_release_preflight.py:493` | Retain: ordering, version drift and exact version are independently checked. |
| Release preflight and source isolation | `tests/test_release_preflight.py:31`, `:360`, `:420` | Retain: subprocess fixtures test preflight, dry-run isolation and inventory. |
| Staging owned packages cannot fall back to public registry | `tests/test_stage.py:14` | Retain: HTTP fixture checks both index and download paths. |
| Graph gate rejects Tokio on all dependency kinds | `tests/test_graph_gate.py:11`, `:21` | Retain: positive and negative graph cases; fault below. |
| External consumer shared trait identity | `tests/consumer/tests/interop.rs:4`, `:26`; `tests/consumer/src/lib.rs:5` | Retain: compile-time identity and loopback behavior. |
| Benchmark harness reporting and cancellation | `benchmarks/twelve-dollar/tests/summarize-latency.sh:14`, `tests/ceiling.sh:127`, `tests/cancel-smoke.sh:53` | Retain: shell-level report and process boundaries; external benchmark environment not exercised. |
| Runtime boundary qualification | `tests/qualify/src/boundaries.rs:30`, `:121`, `:356` | Retain: cancellation, readiness and shutdown behavior. |

Test tiers: Python unit and subprocess fixtures; consumer integration and qualify loopback/runtime tests; twelve-dollar shell fixtures and separate box workload. Workspace test and lint costs are recorded in `AGENTS.md`; no warm before/after or flake-rate comparison was measured. No tests were altered.

Fault: copy `tools/graph_gate.py` to `.work/tmp/graph-gate-original.py`, replace its single `if "tokio" in package["name"]:` with `if False:`, then run `python3 -m unittest discover -s tests -p 'test_graph_gate.py' -v`: exit 1, `test_rejects_tokio_even_when_only_a_dev_or_target_dependency` failed for both `tokio` and `tokio-util` (ValueError not raised); the clean-graph test passed. Restore using `cp .work/tmp/graph-gate-original.py tools/graph_gate.py`; `cmp .work/tmp/graph-gate-original.py tools/graph_gate.py` exited 0. The first attempt with system Python exited 1 on missing `tomllib` before testing the fault; the documented PATH fixed it.

Finding: `.github/workflows/ci.yml:23` defines the verify job and `:36` runs the whole test suite without a job timeout. Pinned static reproduction: at the base revision, `git show 31e6041519af7a3736cfab82b2babc1bd86ba8f9:.github/workflows/ci.yml | rg -n 'verify:|timeout-minutes|run: make test'` prints the job and test line but no timeout. The supplied incident reports a hung test exceeding an hour; this audit did not reproduce the hang. Workflow change is outside scope.

Needs environment: `command -v cargo-mutants`, `cargo-llvm-cov`, `mutmut`, and `coverage` exited 1; `python3 -m coverage --version` also failed. No automated mutation survivors or branch coverage are claimed. Coverage and mutation recipes and CI lanes are outside scope; the single injected fault is not a mutation sweep.

## DSLP-41 · Dead code

Workspace-wide tracked-entry search: `git ls-files tools scripts tests benchmarks/twelve-dollar fixtures`, then for every `.sh` and `.py` basename `rg --hidden -l -F "$basename" . --glob '!.git' --glob '!.work'`; inspect dynamic imports and direct execution separately. `tools/identity.py` is imported as `identity` by release tools; Python `test_*.py` files are discovered by unittest. The unreferenced-by-name shell tests `benchmarks/twelve-dollar/tests/cancel-macos.sh`, `cancel-smoke.sh`, and `summarize-latency.sh` are executable test entry points: owner question whether manual invocation is still wanted, not proof of dead code. `fixtures/h12tiny.sh` is documented at `docs/h12tiny/verification.md:52`; `scripts/h2-975-soak.sh` at `docs/h12tiny/fork-test-parity.md:148`; benchmark summarizer is called at `benchmarks/twelve-dollar/run.sh:560`. No scripts or fixtures removed. The unused local `tools/stage.py:523` is a possible private cut; no behavior change was needed for this audit.

Needs environment: `command -v vulture`, `deptry`, `cargo-machete`, `cargo-udeps`, `cargo-bloat`, and `debloat` exited 1. Ruff F401/F841 results above cover Python only; dependency liveness, external manual callers, other platforms and feature variants remain unverified. No clean-or-allowlisted claim is made.
