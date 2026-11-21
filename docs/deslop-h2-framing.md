# HTTP/2 framing and compression audit

Base `5f2694c09359747290c83f2399658d5e7f2b2f0d`; Darwin aarch64, rustc `1.100.0-nightly (574ff7d98 2026-09-14)`. Fork provenance is in `UPSTREAM.toml`. No API or runtime code change is proposed.

## DSLP-8 · Types

With `export TMPDIR="$PWD/.work/tmp" CARGO_BUILD_JOBS=6`, `cargo clippy -p laputa-h2-futures-lite --lib -- -W unsafe_code -W clippy::wildcard_enum_match_arm -W clippy::match_wildcard_for_single_variants -W clippy::cast_possible_truncation -W clippy::cast_sign_loss -W clippy::unwrap_used -W clippy::expect_used -W clippy::redundant_clone -W clippy::needless_pass_by_value -W dead_code` exited 0. Counts are diagnostics with primary location in the four scoped directories, default features, library target; not dependency or test diagnostics.

| Candidate | Diagnostics | Disposition |
| --- | ---: | --- |
| `unsafe_code` | 3 | Reject: initialized read buffer and HPACK representation need their invariants; no safe equivalent established. |
| `wildcard_enum_match_arm` | 9 | Reject: header matching groups cases; no defect witnessed. |
| `match_wildcard_for_single_variants` | 0 | Reject: no witness. |
| `cast_possible_truncation` | 23 | Reject: wire encodings and buffer positions need individual range proofs; no defect witnessed. |
| `cast_sign_loss` | 4 | Reject: signed length adjustment branches need individual range proofs; no defect witnessed. |
| `unwrap_used` / `expect_used` | 4 / 0 | Reject: table and decoder invariants need individual proof; no removable escape hatch demonstrated. |
| `redundant_clone` / `needless_pass_by_value` | 0 / 2 | Reject: no type defect or removed trust demonstrated. |

Adopted 0 of 9 candidates; rejected 9. These diagnostics are leads, not a claim that every cast or assertion is safe. No escape hatches were added.

## DSLP-20 · Tests

| Contract | Guarding tests | Disposition |
| --- | --- | --- |
| Receive frame size limit | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/codec_read.rs:206` | Retain: a 16,385-byte frame is rejected at 16,384. The empty ignored `read_frame_too_big` at line 14 provides no evidence. |
| DATA payload and padding | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/codec_read.rs:19`, `:74` | Retain: exact payload and flags with and without padding; malformed padding boundary not established by these tests. |
| Continuation headers | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/codec_read.rs:145`; `crates/laputa-h2-futures-lite/tests/h2-tests/tests/codec_write.rs:9` | Retain: reconstructed headers and outbound framing. |
| Header table size and compression | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/codec_write.rs:62`, `:107`; `crates/laputa-h2-futures-lite/src/hpack/encoder.rs:619` | Retain: negotiated settings and table update combinations. |
| Invalid Huffman encoding | `crates/laputa-h2-futures-lite/src/hpack/huffman/decode.rs:289` | Retain: EOS and invalid padding rejected. |
| Transport read window after DSLP-44 | `crates/laputa-h2-futures-lite/tests/read_window.rs:46` | Retain: trickled maximum-length frame never presents a read buffer over 8 KiB. |
| Write zero | `crates/laputa-h2-futures-lite/tests/h2-tests/tests/codec_write.rs:190` | Retain: zero write becomes an error. |

Fault injection: `cp crates/laputa-h2-futures-lite/src/futures_codec/length_delimited.rs .work/tmp/h2-length-original`; replace the single `if n > self.builder.max_frame_len as u64 {` with `if false {`; `cargo test -p h2-tests --test codec_read update_max_frame_len_at_rest -- --exact` exited 101. `update_max_frame_len_at_rest` failed at `codec_read.rs:228` because `unwrap_err()` received `Ok(Data)`. `cp .work/tmp/h2-length-original crates/laputa-h2-futures-lite/src/futures_codec/length_delimited.rs`; `cmp .work/tmp/h2-length-original crates/laputa-h2-futures-lite/src/futures_codec/length_delimited.rs` exited 0. This proves that guard detects the disabled receive-size check.

Needs environment: `command -v cargo-mutants`, `command -v cargo-llvm-cov` each exited 1. No branch coverage or automated survivor counts are claimed; one injected fault is not a mutation sweep. Test tiers and observed suite costs are in `AGENTS.md`; warm before/after cost and flake rate were not measured. No tests were changed. Coverage and mutation recipes and CI are outside this task.

## DSLP-32 · Dead code

The Types command enabled rustc `dead_code`; default-feature scoped library diagnostics: 0. `rg -n 'allow\(dead_code|allow\(unused' crates/laputa-h2-futures-lite/src/{codec,frame,futures_codec,hpack}` found `futures_codec/mod.rs:52` and `hpack/decoder.rs:166`. `rg -n 'BytesCodec|LinesCodec|FramedParts|queue_size_update|set_recv_header_table_size|strip_padding' --glob '!TODO-DESLOP.md' --glob '!*.lock'` searched workspace callers: `strip_padding` is used by `frame/data.rs:129`; `queue_size_update` is used by `codec/framed_read.rs:150`, reached from `src/proto/settings.rs:62`. Retain both. `BytesCodec`, `LinesCodec`, and `FramedParts` have no production callers in that search beyond their definitions and reexports; removal has not been established across build configurations and upstream compatibility, so retain pending proof.

`rg -n 'Codec::|frame::|h2::(client|server)' crates/laputa-h2-futures-lite/src/{client.rs,server.rs,proto} crates/laputa-hyper-futures-lite/src/proto/h2 crates/h12tiny crates/h12tiny-server/src` checks workspace entry points: the h2 client and server use the codec (`src/client.rs:136`, `src/server.rs:107`), Hyper uses h2 (`crates/laputa-hyper-futures-lite/src/proto/h2/client.rs:15`), and h12tiny exposes its server (`crates/h12tiny/src/lib.rs:38`), whose HTTP/2 builder uses Hyper (`crates/h12tiny-server/src/conn/auto.rs:307`). The `unstable` feature exposes `frame` (`src/lib.rs:231`); public consumers and feature variants cannot be ruled out by default-feature warnings. No code was removed.

Needs environment: `command -v cargo-machete`, `command -v cargo-udeps`, `command -v cargo-bloat`, and `command -v debloat` each exited 1. Dependency liveness, non-default features and other platforms remain unverified; no clean-or-allowlisted claim is made.
