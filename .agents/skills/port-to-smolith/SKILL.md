---
name: port-to-smolith
description: "Port a Rust application from Tokio or reqwest to laputa-smol and h12tiny while preserving observable behavior. Use when the owner requests that migration, not for ordinary async changes."
---

# Port an application to smolith

Use this skill when the owner asks to migrate an application from Tokio, reqwest, or both to the smolith runtime and h12tiny. The request to write or revise this skill does not authorize a migration. Preserve the application's existing behavior and its unrelated changes. Check the target repository's instructions and dependency sources before editing.

When a standalone package's worktree is nested under an actively edited checkout, Cargo may parse the parent manifest while searching for a workspace. If that manifest is temporarily invalid, a temporary `[workspace]` in the worktree manifest isolates the build. Remove the temporary section before committing and recheck workspace discovery.

## Inspect the application

1. Inventory direct and transitive Tokio and reqwest dependencies, including dev dependencies, target-specific dependencies, enabled features, and generated code. Use `cargo tree` with the target's supported feature sets and inspect reverse dependencies. A source search alone cannot prove the dependency graph is clear.
2. Read the runtime entry points, task ownership, shutdown paths, async I/O, synchronization, timers, tests, and HTTP call sites. Identify runtime-bound third-party dependencies and replace or adapt them only within the requested scope. Record unresolved blockers rather than claiming a complete port.
3. Identify observable contracts before choosing replacements: task cancellation and panic handling, concurrency limits, ordering, notification permits, channel closure and backpressure, timer deadlines, and behavior when a selected future is dropped. Preserve each contract, including tests and platform support.
4. Inventory HTTP behavior actually used: methods and bodies, URL and query encoding, headers and authentication, JSON, error bodies, redirects, proxies, TLS trust, protocol negotiation, compression, retries, pooling, deadlines, and streaming. Check defaults as well as explicit configuration. Preserve security boundaries, especially credentials across redirects.

## Choose replacements

Read the current smolith sources and docs before choosing APIs or versions; installed and published versions may differ from this checkout. Start with [the h12tiny client guide](../../../docs/h12tiny/client.md), [feature selection](../../../docs/h12tiny/features.md), [TLS configuration](../../../docs/h12tiny/tls.md), and [the runtime facade](../../../crates/smol/src/lib.rs). Inspect examples and tests for the selected release. Confirm that the target's registry can resolve every selected package and that its Rust version and supported platforms remain compatible.

- The runtime package is `laputa-smol`, with library name `smol`. Other runtime packages use `laputa-*` package names. Select only the crates and features the application needs, and preserve shared futures-I/O trait identities.
- smolith declares Windows unsupported. If the application supports Windows, check the selected release against that target before migration and resolve the platform tradeoff with the owner before merging.
- h12tiny's facade has no default roles or protocols. Select the client role and required HTTP and TLS features explicitly. Its async client accepts absolute HTTP URIs and requires an executor for connection drivers. Use the current examples and tests to construct requests and consume response bodies.
- Check reqwest re-exports independently of HTTP calls. For example, `reqwest::Url` callers may still need the `url` crate for parsing and query encoding after reqwest is removed.
- Check configured URLs for userinfo: reqwest converts it to Basic authorization and removes it from the request URI. Compare header names case-insensitively in wire tests; HTTP does not require a particular spelling.
- Do not assume a Tokio primitive has an equivalent with identical semantics. In particular, dropping a smol task cancels it unless detached; choose explicit ownership and shutdown behavior. Check notification permits and races, `select!` cancellation, task-set completion and errors, and blocking work independently.
- Do not assume reqwest convenience behavior exists in h12tiny. Its client has separate connection and response-header deadlines, which do not cover pool waits or response-body streaming. Preserve an application's whole-request deadline explicitly when required. Check the selected release's redirect, proxy, decompression, URL, and JSON facilities before implementing or selecting replacements. Preserve retry limits and avoid replaying unsafe requests.
- Inspect the resolved reqwest features and its actual proxy matcher. Disabling default features does not by itself disable proxy environment variables; test proxy and bypass behavior when the old client honors them.
- h12tiny's default TLS client uses system trust. Compare that policy with the existing client and preserve required custom roots and verification. Do not weaken certificate or hostname verification to make a migration pass.

## Migrate and verify

Port one coherent boundary at a time. Preserve existing tests and add focused behavioral tests for contracts that could otherwise change, using local servers and fixtures where appropriate. Exercise cancellation, shutdown, request construction, status and error handling, retries, deadlines including slow bodies, and any required redirect, compression, TLS, or protocol behavior. Cover only behaviors the application uses. Do not contact live services or require credentials when local fixtures can prove the contract.

Run the target's relevant tests and checks for each supported feature set and platform that can be exercised. Inspect the resolved dependency graph for Tokio and reqwest across normal, build, dev, and target-specific edges within the requested scope. Explain any remaining edges and their owners. Verify the final application workflows affected by the port, and report untested platforms, unresolved behavior differences, dependency changes, and any remaining migration blockers. Do not call the migration complete while a requested dependency or required behavior remains unresolved.
