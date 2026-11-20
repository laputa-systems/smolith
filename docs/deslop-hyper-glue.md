# Hyper client, server, body and HTTP/2 glue audit

Base `11f2345a92ff7935cf766316d1f93046726de989`; Darwin aarch64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. Fork provenance is in `UPSTREAM.toml`. No API or runtime change. `origin/deslop/hyper-h1` was absent; `git show deslop/hyper-h1:docs/deslop-hyper-h1.md` supplied the local reference.

## DSLP-12 · Types

With `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6`, run `cargo clippy -p laputa-hyper-futures-lite --lib --features full --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code`. Exit 0. Count diagnostics by code only where a primary span is in the scoped source directories; DSLP-46 tracks the inherited restriction baseline.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 0 | Reject: no witness for this scope. |
| `wildcard_enum_match_arm` | 1 | Reject: the unreachable consumed callback states at `client/dispatch.rs:348` do not establish a defect. |
| `match_wildcard_for_single_variants` | 0 | Reject: no witness. |
| `cast_possible_truncation` / `cast_sign_loss` | 0 / 0 | Reject: no witness. |
| `unwrap_used` / `expect_used` | 0 / 31 | Reject: no removable invariant established; e.g. H2 ping setup obtains the connection's ping handle at `proto/h2/server.rs:217`. |
| `redundant_clone` / `needless_pass_by_value` | 0 / 2 | Reject: no type defect demonstrated. |

Adopted 0 of 9 candidates; rejected 9. Existing escape hatches remain; no new witness or sound replacement was established.

## DSLP-24 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Client body length and incomplete response | `crates/laputa-hyper-futures-lite/tests/client.rs:1753`, `:1799` | Retain: size hint, received bytes and incomplete-body error. |
| HTTP/2 response length | `crates/laputa-hyper-futures-lite/tests/server.rs:336`, `:354` | Retain: known length and explicitly supplied conflicting length. |
| Graceful HTTP/1 shutdown | `crates/laputa-hyper-futures-lite/tests/server.rs:1211`, `:1267`, `:2954` | Retain: shutdown during and after a request and before the first request. |
| HTTP/2 flow control and upgrades | `crates/laputa-hyper-futures-lite/tests/client.rs:3301`, `:3467`; `crates/laputa-hyper-futures-lite/tests/server.rs:2351`, `:2430` | Retain: connection window, backpressure and release. |
| Remaining decoded length | `crates/laputa-hyper-futures-lite/src/body/length.rs:115`, `:123` | Retain: known length decrements and chunked length remains unknown. |

Test tiers and previous suite costs are in `AGENTS.md`: unit state-machine tests, protocol integration tests, and workspace/consumer tests. No test changes; warm before/after cost and flake rate were not measured.

Fault injection: `cp crates/laputa-hyper-futures-lite/src/proto/h2/server.rs .work/tmp/glue-server-original`; replace `headers::set_content_length_if_missing(res.headers_mut(), len);` with `let _ = len;`; `cargo test -p laputa-hyper-futures-lite --features full --test server http2_auto_response_with_known_length` exited 101: `response_body_lengths::http2_auto_response_with_known_length` failed at `tests/server.rs:348` because the content-length header was absent. Restore with `cp .work/tmp/glue-server-original crates/laputa-hyper-futures-lite/src/proto/h2/server.rs`; `cmp .work/tmp/glue-server-original crates/laputa-hyper-futures-lite/src/proto/h2/server.rs` exited 0.

Needs environment: `command -v cargo-mutants` and `command -v cargo-llvm-cov` each exited 1. No automated mutation survivor counts or branch coverage are claimed; one injected fault is not a mutation sweep. Coverage and mutation recipes and CI are outside this task.

## DSLP-36 · Dead code

The Types command enabled rustc `dead_code`: 0 scoped primary diagnostics. `rg -n 'allow\((dead_code|unused)' crates/laputa-hyper-futures-lite/src/{client,server,body,service,proto/h2}` found suppressions including `body/incoming.rs:100` and `body/chan.rs:84`, `:125`. `rg -n 'send_trailers|send_data|Incoming::empty|new_channel|graceful_shutdown|http2::Builder' crates --glob '*.rs'` searched workspace callers, including h12tiny: `proto/h1/dispatch.rs:279`, `:293`, `:345` uses the active channel methods; `h12tiny-client/src/lib.rs:1325` constructs an H2 client builder; `h12tiny-server/src/conn/auto.rs:329`, `:642` constructs a server builder and forwards shutdown. `Incoming::empty` is used in tests (`client/dispatch.rs:567`); the suppressed `send_data` and `send_trailers` wrappers have no demonstrated caller in that search. Retain: feature-specific and complete removal evidence is insufficient, and public entry points have external consumers. No code removed.

Needs environment: `command -v cargo-machete`, `command -v cargo-udeps`, `command -v cargo-bloat`, and `command -v debloat` each exited 1. Dependency liveness, other feature combinations and other platforms remain unverified; no clean-or-allowlisted claim is made.
