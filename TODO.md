Prefix: SMOL
Next: 14

### SMOL-13 · P1 · The stuff mirror index lacks locked doc-comment 0.3.4

- **Area:** Build
- **Where:** `.cargo/config.toml:1`, `Cargo.lock:122`
- **Problem:** The committed source replacement resolves crates.io through the stuff sparse index. That index lists only `doc-comment` 0.3.3, while `Cargo.lock` requires 0.3.4 with checksum `780955b8b195a21ab8e4ac6b60dd1dbdcec1dc6c51c0617964b08c81785e12c9`. `cargo test -p laputa-blocking --locked` exits 101 with `failed to select a version` after updating the stuff index. `GET https://crates.17166969.xyz/current/crates-io/do/c-/doc-comment` returns a body whose only version is 0.3.3. The crate file is already in the local crates.io cache. A build with the source replacement removed compiles the locked graph.
- **Recommendation:** Sync `doc-comment` 0.3.4 into the stuff mirror so the committed lockfile resolves. Leave the lockfile and the source replacement unchanged.
- **Fix with:** `stuff-registries`
- **Effort:** S

HTTP findings use the H12T prefix and stay in [docs/h12tiny/TODO.md](docs/h12tiny/TODO.md). Their numbers are unchanged.

