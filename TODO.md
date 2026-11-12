Prefix: SMOL
Next: 14

### SMOL-13 · P1 · The stuff mirror index lacks locked doc-comment 0.3.4

- **Area:** Build
- **Where:** `.cargo/config.toml:1`, `Cargo.lock:122`
- **Problem:** The committed source replacement resolves crates.io through the stuff sparse index. That index lists only `doc-comment` 0.3.3, while `Cargo.lock` requires 0.3.4 with checksum `780955b8b195a21ab8e4ac6b60dd1dbdcec1dc6c51c0617964b08c81785e12c9`. `cargo test -p laputa-blocking --locked` exits 101 with `failed to select a version` after updating the stuff index. `GET https://crates.17166969.xyz/current/crates-io/do/c-/doc-comment` returns a body whose only version is 0.3.3. The crate file is already in the local crates.io cache. A build with the source replacement removed compiles the locked graph.
- **Recommendation:** Sync `doc-comment` 0.3.4 into the stuff mirror so the committed lockfile resolves. Leave the lockfile and the source replacement unchanged.
- **Fix with:** `stuff-registries`
- **Effort:** S

### SMOL-11 · P2 · Floating-point range helpers return excluded endpoints

- **Area:** Types
- **Where:** `crates/fastrand-contrib/src/float_range.rs:124`, `crates/fastrand-contrib/src/float_range.rs:143`, `crates/fastrand-contrib/src/float_range.rs:160`, `crates/fastrand-contrib/src/lib.rs:156`
- **Problem:** Multiplying a sample by the interval width and adding its lower bound can round up to an excluded upper endpoint. This violates the public range contract for valid nonempty ranges. With `Rng::with_seed(42)` and an upper bound one representable float above `1.0`, both `f32_range(1.0..upper)` and `f64_range(1.0..upper)` returned the excluded endpoint in 60 of the first 100 samples. Both current library sources were compiled unchanged for the probe. Existing tests sample broad ranges and miss adjacent representable values.
- **Recommendation:** Make endpoint inclusion hold after floating-point rounding for both widths and both scaling paths. Add adjacent-value, signed-zero, subnormal, and wide-range cases that assert the actual RangeBounds contract. Define rejection of empty or non-finite input separately from correcting valid finite ranges; avoid an unbounded rejection loop when no representable sample exists.
- **Fix with:** `typing-audit`
- **Effort:** S

### SMOL-12 · P3 · Ordered timer storage shifts the whole queue during cancellation

- **Area:** Performance
- **Where:** `crates/async-io/src/reactor.rs:199`, `crates/async-io/src/reactor.rs:369`, `crates/async-io/src/reactor.rs:387`, `crates/async-io/src/reactor.rs:396`, `crates/async-io/src/reactor.rs:410`, `crates/async-io/benches/timer.rs:13`
- **Problem:** Commit `a7a3d7e` replaced the ordered timer map with an FxHashMap plus a sorted Vec. Each interior insertion or removal shifts the Vec while holding the global timer mutex; expiring a prefix shifts the remaining timers. Cancelling N live timers in deadline order therefore moves a quadratic number of keys. This is source-proven scaling cost, not a measured latency regression. The existing registration benchmark uses one-second timers, so a large setup can expire before the timed region and does not establish cancellation cost with a stable live set.
- **Recommendation:** Measure a fixed, still-live timer set through registration, out-of-order insertion, cancellation, and expiry against the previous ordered-map implementation. Restore logarithmic ordered updates or choose an equivalent structure if representative workloads confirm the cost. Keep equal-deadline ordering and cancellation semantics, and report live counts and completed work alongside timings.
- **Fix with:** `make-it-fast`
- **Effort:** M
