# HTTP/1 protocol audit

Base `49afc6a66a517742e09f9a4c315b7b2b0806ff93`; Darwin aarch64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. Fork provenance is in `UPSTREAM.toml`. No API or runtime code change is proposed. The requested `origin/deslop/h2-framing` ref is absent; `git show deslop/h2-framing:docs/deslop-h2-framing.md` provided the local reference.

## DSLP-11 · Types

With `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6`, run `cargo clippy -p laputa-hyper-futures-lite --lib --features full --message-format=json -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code`. Exit 0. Counts include only primary locations in `src/proto/h1`, not dependency or test diagnostics. The inherited restriction baseline is tracked by DSLP-46.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 7 | Reject: existing buffer and parsing invariants require individual proof; no safe replacement established. |
| `wildcard_enum_match_arm` | 10 | Reject: grouped states do not establish a type defect. |
| `match_wildcard_for_single_variants` | 0 | Reject: no witness. |
| `cast_possible_truncation` / `cast_sign_loss` | 0 / 0 | Reject: no witness. |
| `unwrap_used` / `expect_used` | 0 / 14 | Reject: no removable escape hatch demonstrated; e.g. dispatch retains a result until close completes (`dispatch.rs:133`, `:158`). |
| `redundant_clone` / `needless_pass_by_value` | 0 / 2 | Reject: no type defect witnessed. |

Adopted 0 of 9 candidates; rejected 9. These diagnostics are leads, not proofs of safety.

## DSLP-23 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Request and response parsing; ambiguous framing | `crates/laputa-hyper-futures-lite/src/proto/h1/role.rs:1695`, `:1727`, `:1797`, `:1813` | Retain: parse both directions and reject conflicting or duplicate framing. |
| Chunked payload, malformed input and trailer limits | `crates/laputa-hyper-futures-lite/src/proto/h1/decode.rs:883`, `:907`, `:923`, `:1183`, `:1215`, `:1247` | Retain: EOF, payload and limit decisions have distinct assertions. |
| Connection close and outbound framing | `crates/laputa-hyper-futures-lite/src/proto/h1/conn.rs:1228`, `:1439` | Retain: keep-alive decision and chunked output. |
| Buffered response and shutdown | `crates/laputa-hyper-futures-lite/tests/h1_flush_before_yield.rs:65`, `crates/laputa-hyper-futures-lite/tests/h1_shutdown_while_buffered.rs:141` | Retain: flushing and shutdown regressions exercise the connection boundary. |
| Stream readiness | `crates/laputa-hyper-futures-lite/tests/ready_on_poll_stream.rs:139`, `crates/laputa-hyper-futures-lite/tests/unbuffered_stream.rs:121` | Retain: observable body delivery under different readiness. |

The previous attempt witnessed default-feature `cargo test -p laputa-hyper-futures-lite` fail to compile `h1_flush_before_yield` (exit 101). Base commit `49afc6a` gates that test on `full`; the default build then exited 0. This was a test-wiring defect.

Fault injection: `cp crates/laputa-hyper-futures-lite/src/proto/h1/decode.rs .work/tmp/h1-decode-original`; replace the single `if *trailers_cnt >= h1_max_headers {` with `if false {`; `cargo test -p laputa-hyper-futures-lite --features full --lib test_trailer_max_headers_enforced` exited 101: `proto::h1::decode::tests::test_trailer_max_headers_enforced` failed at `decode.rs:1209`, receiving eleven trailers instead of an error. Restore with `cp .work/tmp/h1-decode-original crates/laputa-hyper-futures-lite/src/proto/h1/decode.rs`; `cmp .work/tmp/h1-decode-original crates/laputa-hyper-futures-lite/src/proto/h1/decode.rs` exited 0.

Needs environment: `command -v cargo-mutants` and `command -v cargo-llvm-cov` each exited 1. No branch coverage or automated survivor counts are claimed; one injected fault is not a mutation sweep. Test tiers and prior suite costs are in `AGENTS.md`; warm before/after cost and flake rate were not measured. No tests were changed. Coverage and mutation recipes and CI are outside this task.

## DSLP-35 · Dead code

The Types command enabled rustc `dead_code`; full-feature scoped library diagnostics: 0. `rg -n 'allow\(dead_code|allow\(unused' crates/laputa-hyper-futures-lite/src/proto/h1` exited 1. `rg -n 'pending_result|poll_close|set_http1_max_headers|set_http1_max_header_size|poll_without_shutdown|disable_keep_alive' crates --glob '*.rs'` searched workspace callers: both client and server builders set limits (`src/client/conn/http1.rs:606`, `src/server/conn/http1.rs:494`); both expose polling without shutdown (`src/client/conn/http1.rs:94`, `src/server/conn/http1.rs:168`). `pending_result` carries completion through asynchronous close (`src/proto/h1/dispatch.rs:133`, `:158`). Retain these paths.

`rg -n 'http1::Builder|http1::handshake|http1_max_headers|max_headers\(|max_buf_size\(|http1_max_buf_size' crates/h12tiny* --glob '*.rs'` finds h12tiny client and server builders (`crates/h12tiny-client/src/lib.rs:1306`, `crates/h12tiny-server/src/conn/auto.rs:327`) and websocket client handshake (`crates/h12tiny-web/src/websocket_client.rs:97`). These are entry points to the component; external consumers and feature variants cannot be ruled out from local searches. No code was removed.

Needs environment: `command -v cargo-machete`, `command -v cargo-udeps`, `command -v cargo-bloat`, and `command -v debloat` each exited 1. Dependency liveness, other feature combinations and other platforms remain unverified; no clean-or-allowlisted claim is made.
