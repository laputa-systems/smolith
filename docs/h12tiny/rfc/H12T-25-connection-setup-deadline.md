# H12T-25: Connection setup deadlines

Status: implemented

## Decision

`Serve::setup_timeout` and `TlsServe::setup_timeout` configure an optional
nonzero setup budget. Both production presets use 10 seconds. The public
`ProductionBounds` struct keeps its existing fields.

The timer starts when the listener accepts the stream, before executor dispatch.
Cleartext setup ends when protocol detection selects HTTP/1 or HTTP/2. TLS setup
ends when the handshake supplies ALPN for protocol selection. Established
response streams and shutdown draining keep their separate lifetimes.

Expiry drops setup I/O, reports `io::ErrorKind::TimedOut` through the connection
error hook, and completes the tracked task to release its admission slot.
Shutdown still cancels pending setup.

A whole-connection deadline would terminate valid long-lived responses.
Reusing the HTTP/1 header deadline alone would leave negotiation unbounded.

## Verification

The detector unit test controls deadline completion directly for silent input,
a partial H2 preface, and an established HTTP/1 request. Listener integration
tests observe timeout errors and slot reuse for cleartext and TLS, then keep
an established response alive past the setup budget. Shutdown and ALPN tests
continue to pass.
