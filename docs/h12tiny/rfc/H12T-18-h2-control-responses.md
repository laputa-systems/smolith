# H12T-18: Bounded H2 control responses

Status: implemented

## Decision

The connection owns one FIFO queue of at most 32 pending PING and SETTINGS
acknowledgments. Encoder capacity drains the queue before local control work.
A peer that exceeds this budget receives `ENHANCE_YOUR_CALM` and the connection
closes. The budget is internal.

Remote SETTINGS apply in receive order before the next frame is processed.
Their acknowledgments enter the same queue as PONGs. Settings state no longer
retains a single unacknowledged remote frame. PING state owns outbound pings
and incoming ping acknowledgments, while the connection owns PONG output.

Stopping reads during blocked output would restore the bidirectional deadlock.
An unbounded reply queue would let a peer grow connection memory without limit.
The bounded queue preserves progress until the peer exhausts its control budget.

## Verification

The regression in `tests/h2-tests/tests/h2_975.rs` reproduced the occupied-PONG
assertion before the fix. With constrained output, it now delivers inbound DATA
and verifies ordered PONG and SETTINGS acknowledgments after writes resume.
A second test floods PING or SETTINGS under blocked output and observes
`ENHANCE_YOUR_CALM` on the wire and from the driver. The inherited H2 suite passes.
