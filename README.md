# Rust Platform Libraries

Small, opt-in Rust crates for shared cross-cutting contracts used by multiple
web, operator, and control-plane products. This is not an application
framework and does not own product databases, users, migrations, or UI.

## Planned crates

- `platform-core`: bounded results, errors, IDs, clocks, redaction, and
  correlation contracts.
- `platform-contracts`: Rust representations of the language-neutral schemas.
- `platform-identity`: identity, session, permission, tenant, and lifecycle
  contracts without a prescribed store or token technology.
- `platform-audit`: normalized security and operational audit events.
- `platform-testing`: deterministic clocks, fakes, stores, and failure tools.

Axum, SQLx, cookie, OIDC, and provider adapters are deferred until multiple
consumers demonstrate a compatible contract.

The project is planning-only. Verification command: `cargo test --workspace`.

Use OpenSpec for every non-trivial change and follow BFS → DFS → BFS.
