# h12tiny-tls-system

Operating-system certificate trust for h12tiny HTTPS clients. The default
verifier loads the host trust store and refreshes it when the store changes.
Explicit root stores and caller-provided verifiers stay under caller control and
are never merged with system trust. Managed system-trust configurations disable
session resumption so every new connection checks current trust.

See the [TLS guide](../../docs/tls.md).
