# HTTP facade and clients audit

Base `58f1233ee70eb44af306750eab59f9c8c130992b`; Darwin arm64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. The sibling report is available locally as `git show deslop/hyper-rt:docs/deslop-hyper-rt.md`; the requested remote-tracking ref is absent. No API or runtime change. Set `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6` for commands below.

## DSLP-15 · Types

Run `cargo clippy -p h12tiny -p h12tiny-core -p h12tiny-client -p h12tiny-client-normalize -p h12tiny-client-sync -p h12tiny-tls-system --lib --all-features --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code`. Exit 0. Counts are compiler messages with a primary span in these six crates, on Darwin, excluding tests. The facade's all-features selection also compiles server and web dependencies; their diagnostics are excluded.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 11 | Reject: Hyper I/O cursor and pin adapters, e.g. `crates/h12tiny-core/src/io.rs:90`; no safe replacement established. |
| `wildcard_enum_match_arm` | 1 | Reject: `crates/h12tiny-client-sync/src/lib.rs:626` catches states invalid for this reader; no defect established. |
| `match_wildcard_for_single_variants` | 0 | Reject: no witness. |
| `cast_possible_truncation` / `cast_sign_loss` | 0 / 0 | Reject: no witness. |
| `unwrap_used` / `expect_used` | 0 / 26 | Reject: invariants include loaded verifier state at `crates/h12tiny-tls-system/src/lib.rs:329`; no removable invariant established. |
| `redundant_clone` / `needless_pass_by_value` | 0 / 7 | Reject: no typing defect or removed escape hatch established. |

Adopted 0 of 9 candidates; rejected 9. No escape hatch removed or added. Absolute URI and explicit port validation precede normalization (`crates/h12tiny-client-normalize/src/lib.rs:74`, `:96`); the public `is_connect` and `set_host` arguments remain consumer contracts.

## DSLP-27 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Direct URI, Host and CONNECT wire form | `crates/h12tiny/tests/raw_h1.rs:50`, `:269`; `crates/h12tiny-client-normalize/src/lib.rs:226`, `:277` | Retain: literal wire and explicit Host assertions. |
| Pool reuse and stale connection replacement | `crates/h12tiny/tests/raw_h1.rs:142`, `:229`; `crates/h12tiny/tests/lifecycle_regressions.rs:342` | Retain: connection reuse, reconnection and per-origin cap. |
| Async DNS, dial and header deadlines | `crates/h12tiny-client/src/connect.rs:1189`, `:1293`, `:1344`; `crates/h12tiny/tests/raw_h1.rs:101` | Retain: cancellation and deadline scope. |
| Blocking request and body deadlines | `crates/h12tiny-client-sync/src/lib.rs:1748`, `:1795`, `:1878` | Retain: shared dispatch deadline and separate body reads. |
| TLS trust and hostname | `crates/h12tiny/tests/tls_system_sync.rs:173`, `:195`, `:221`; `crates/h12tiny/tests/dogfood_tls.rs:365` | Retain: trusted and untrusted roots and hostname rejection. |
| Streaming and bounded response | `crates/h12tiny/tests/client_compat.rs:230`, `:292`, `:343`; `crates/h12tiny-client-sync/src/lib.rs:1339` | Retain: upload, download, framing and pool recovery. |

Test tiers and observed workspace costs are in `AGENTS.md`: unit, loopback protocol integration and external consumer tests. Default facade features are empty (`crates/h12tiny/Cargo.toml:13`), so integration tests require the all-features run. Network fixtures inspected use loopback; no internet-reaching test was identified by the URI and connection search, but a runtime network trace was not collected. No test changes; warm before/after cost and flake rate were not measured. Redirect and proxy policy belongs to applications (`docs/h12tiny/client.md:71`, `:94`).

Fault injection: `cp crates/h12tiny-client-normalize/src/lib.rs .work/tmp/deslop-normalize-original`; replace the `return;` inside the `request.headers().contains_key(HOST)` guard with an empty body. `cargo test -p h12tiny-client-normalize explicit_host_is_preserved_and_default_ports_are_hidden` exited 101: `tests::explicit_host_is_preserved_and_default_ports_are_hidden` failed, observed `example.test` versus `chosen.test`. Restore with `cp .work/tmp/deslop-normalize-original crates/h12tiny-client-normalize/src/lib.rs`; `cmp .work/tmp/deslop-normalize-original crates/h12tiny-client-normalize/src/lib.rs` exited 0. This is one fault, not a mutation sweep.

Needs environment: `command -v cargo-mutants` and `command -v cargo-llvm-cov` each exited 1. Branch coverage and automated mutation survivor counts are unmeasured; adding recipes and CI is outside scope.

## DSLP-39 · Dead code

The Types command enabled `dead_code`: 0 scoped primary diagnostics in the all-features library profile. `rg -n 'SystemVerifier|system_client_config_lazy|system_roots_snapshot|system_client_config_with_auth\b|authority_form|synthesize_host' --glob '*.rs' --glob 'Cargo.toml' .` found the normalizer helpers at `crates/h12tiny-client-normalize/src/lib.rs:125` and `:128`, and default TLS configuration callers at `crates/h12tiny-client/src/connect.rs:1012` and `crates/h12tiny-client-sync/src/lib.rs:1209`. `rg -n 'h12tiny[_-]client[_-]normalize|h12tiny[_-]tls[_-]system|system_client_config_with_auth\b|\b(authority_form|synthesize_host|origin_form)\b' --glob '!Cargo.lock' .` also found workspace dependencies and staging consumer `tools/stage.py:493`. `system_client_config_with_auth` at `crates/h12tiny-tls-system/src/lib.rs:477` has no other workspace text match: do published consumers use it? Retain this public API pending consumer evidence. No private code or dependencies removed. This search cannot establish liveness in published consumers, other platforms, or all feature combinations.

Needs environment: `command -v cargo-machete`, `command -v cargo-udeps`, `command -v cargo-bloat`, and `command -v debloat` each exited 1. Dependency liveness and size attribution remain unverified; no clean-or-allowlisted claim is made.
