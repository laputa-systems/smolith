# h12tiny

Tokio-free Rust HTTP components: a futures-I/O client and server, a separate blocking HTTP/1.1 client, body utilities, and an optional router.

## Commands

- Build: `cargo build --workspace --all-features`
- Test: `cargo test --workspace --all-features`
- Fork checks: see [verification](verification.md)
- Fork test classification and the verification campaign record: [parity ledger](fork-test-parity.md), [campaign](CAMPAIGN-FORK-VERIFICATION.md)
- Feature and dependency isolation: `scripts/check-features.sh`
- Normal dependency policy: `sh scripts/check-normal-dependencies.sh`
- Minimum compiler versions: `sh scripts/check-msrv.sh`
- Unsafe I/O adapter check (nightly Miri): `scripts/miri-io.sh`

## Invariants

- Facade features default to empty; protocol features only forward to selected roles. See [features](features.md).
- Hyper owns async HTTP framing; the blocking client owns direct-origin HTTP/1.1 only. See [architecture](architecture.md) and [clients](client.md).
- The Hyper, H2, and fastwebsockets fork crates live under `crates/` and resolve through the workspace. See [verification](verification.md).
- Pool keys use parsed scheme and authority; H1 is reserved exclusively, H2 is shared, and cross-origin coalescing is absent. See [clients](client.md).
- Production async code is runtime-neutral and does not depend on Tokio. See [architecture](architecture.md).
- Plaintext H2 selection uses the prior-knowledge preface, TLS uses ALPN, and h2c upgrade is unsupported. See [server](server.md).
- Router and body helpers stay protocol-neutral; application policy remains with the application. See [application layer](application.md).

## Topic map

- Crate ownership, dependency direction, upstream boundaries, and feature behavior: [architecture](architecture.md), [features](features.md)
- Async and blocking client behavior: [clients](client.md)
- Server selection, listeners, and upgrades: [server](server.md)
- Bodies, routing, and application conveniences: [application layer](application.md)
- Rustls defaults and custom configuration: [TLS configuration](tls.md)
- Build, tests, feature gates, and coverage: [verification](verification.md)
- External interoperability and performance tools: [scripts guide](../../scripts/README.md)
- Production bounds and lifecycle defaults: [limits](limits.md), [server](server.md)
- Deferred fixes: [TODO.md](TODO.md)
- Draft designs: [H2 control responses](rfc/H12T-18-h2-control-responses.md), [router composition](rfc/H12T-20-router-composition.md), [setup deadlines](rfc/H12T-25-connection-setup-deadline.md), [trust and resumption](rfc/H12T-26-trust-resumption.md)
