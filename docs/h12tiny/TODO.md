Prefix: H12T
Next: 27

### H12T-18 · P1 · H2 read-ahead can reenter occupied control-reply state

- **Area:** Concurrency
- **Where:** `crates/laputa-h2-futures-lite/src/proto/connection.rs:223`, `:386`, `:419`, `:637`; `crates/laputa-h2-futures-lite/src/proto/ping_pong.rs:97`, `:147`; `crates/laputa-h2-futures-lite/src/proto/settings.rs:81`, `:123`; `crates/laputa-h2-futures-lite/src/codec/framed_write.rs:236`, `:305`, `:352`.
- **Problem:** The control-output backpressure path continues reading frames, but the PING and SETTINGS handlers still assume their previous acknowledgment has been buffered. When the codec has no send capacity, a pending PONG or remote SETTINGS remains occupied. Another control frame reaches an unconditional assertion in the corresponding handler. A blocked socket alone is insufficient: the codec must also lack buffering capacity, as it does with a pending chained DATA frame. The read side does not wait for that write capacity. This can panic a connection driver and terminate an application using abort-on-panic. Two independent source reviews traced the dispatch and codec predicates. **Runtime reachability is unproven by execution; no wire trigger was run.** Existing multiple-PING and bidirectional-progress tests cover the two conditions separately.
- **Recommendation:** Give pending control responses one bounded owner that preserves acknowledgment and SETTINGS ordering while allowing the progress required by the bidirectional-deadlock regression. Do not restore the old unconditional stop-reading behavior or merely remove the assertions. Verify multiple control obligations under constrained output together with the existing progress regression.
- **Fix with:** `lifecycle-concurrency`
- **Effort:** M (days)
- **Design:** [Bounded H2 control responses](rfc/H12T-18-h2-control-responses.md).

### H12T-19 · P1 · The benchmark workspace no longer resolves the local facade

- **Area:** Build
- **Where:** `benchmarks/twelve-dollar/Cargo.toml`, `benchmarks/twelve-dollar/Cargo.lock`.
- **Problem:** The benchmark manifest and lock now resolve `h12tiny` 0.9.0 from `crates/h12tiny`, and the nested runtime crates resolve as path packages. `cargo update -p h12tiny` completed that lock update. A full benchmark binary was not rebuilt; an earlier offline run was blocked by uncached `hmac`, which is separate from the version mismatch.
- **Recommendation:** Update the benchmark dependency and its lockfile. Give this non-published local consumer a dependency policy that survives facade releases, or include its requirement and lockfile in release planning. Verify the benchmark's locked build after dependency resolution is available.
- **Fix with:** `single-source-of-truth`
- **Effort:** S (hours)

### H12T-25 · P2 · Production listener deadlines exclude connection setup

- **Area:** Concurrency
- **Where:** `crates/h12tiny-server/src/lifecycle.rs:94`, `:332`, `:603`, `:724`; `crates/h12tiny-server/src/conn/auto.rs:447`, `:705`; `tests/server_bounds.rs:30`.
- **Problem:** The production preset's 10 s header deadline is installed on the eventual Hyper H1 driver. Cleartext protocol detection runs before that driver exists, and TLS acceptance races only against server shutdown. These pending setup states consume admission slots with no elapsed-time bound. The existing header-timeout test supplies a recognizable request prefix, so it starts after the uncovered boundary. **Source-traced; runtime expiry behavior is unproven by execution.** No connection flood or external-service probe was run. The earlier production-bounds work remains implemented; this is a narrower setup-stage gap.
- **Recommendation:** Bound TLS acceptance and protocol detection in the production lifecycle, release the admission slot on expiry, and report a typed timeout. Keep setup separate from established-stream lifetime and shutdown draining. Verify with a controlled pending-I/O fixture and a positive completed-setup case.
- **Fix with:** `lifecycle-concurrency`
- **Effort:** M (days)
- **Design:** [Connection setup deadlines](rfc/H12T-25-connection-setup-deadline.md).

### H12T-26 · P2 · TLS session resumption can retain approval across system-trust changes

- **Area:** Security
- **Where:** `crates/h12tiny-tls-system/src/lib.rs:202`, `:351`, `:438`, `:487`; pinned `rustls 0.23.46` sources `src/client/builder.rs:172`, `src/client/tls13.rs:573`, `src/msgs/persist.rs:273`.
- **Problem:** Managed system-trust configurations retain Rustls's default resumption cache. Store refresh runs when the certificate verifier is called, but a resumed TLS 1.3 handshake reuses the session's previous verification result. Rustls compares verifier identity, and the outer `SystemVerifier` identity stays the same when its internal roots change. A client reusing its configuration with a resumable peer can therefore establish another connection without checking a changed or unusable store. This contradicts the earlier promise that every new handshake observes current trust; the TLS guide now states the implementation's limit. **Source-traced through the pinned dependency; the end-to-end trust-change case is unproven and was not exercised.** The affected asset is the client's current trust policy after an administrator changes it, not established connections already allowed to finish.
- **Recommendation:** Preserve strict trust freshness by disabling resumption for managed system trust, or invalidate cached sessions before any resume attempt when that policy changes. Keep explicit-root and caller-supplied configuration policy separate. Settle the performance/freshness choice in the RFC before implementing it.
- **Fix with:** `security`
- **Effort:** M (days)
- **Design:** [System trust and session resumption](rfc/H12T-26-trust-resumption.md).
