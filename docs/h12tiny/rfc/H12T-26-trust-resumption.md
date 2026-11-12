# H12T-26: System trust and session resumption

Status: implemented

## Decision

Managed system-trust configurations disable Rustls session resumption for TLS
1.2 and TLS 1.3. This includes eager and lazy construction and configurations
with a client identity. Every new connection therefore invokes certificate
verification against current trust.

Explicit-root configurations retain Rustls's default resumption policy.
Caller-supplied configurations remain under caller control. Existing pooled
connections may finish under their original trust decision.

Keeping resumption would require trust-aware session lookup and invalidation
before an offer is made, including native platform trust changes. Disabling
resumption provides the strict freshness contract without another cache owner.
No reconnect performance claim is made.

## Verification

The in-memory regression reuses each managed configuration for consecutive
connections. Before the fix, the second handshake resumed. After the fix,
handshakes remain full for TLS 1.2 and TLS 1.3, with and without a configured
client identity. Replacing the trusted root rejects the peer, and an invalid
store fails closed. The same fixture confirms explicit-root configurations
still resume.
