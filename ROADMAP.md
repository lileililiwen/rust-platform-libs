# Rust Platform Libraries Roadmap

## Dependency order

1. `rust-platform-foundation-and-contracts` — workspace, crate boundaries,
   compatibility policy, and language-neutral contract representations.
2. `identity-audit-and-permission-contracts` — identity, sessions, tenant
   context, permissions, and audit primitives.
3. `deterministic-rust-testing-toolkit` — clocks, fakes, stores, and scripted
   failure injection.
4. `axum-web-adapters` — opt-in web adapters after two real consumers agree on
   the contracts.
5. `sqlx-session-and-audit-adapters` — persistence adapters after schema and
   concurrency evidence exists.

## Deferred

Universal login pages, dashboards, product entities, migrations, provider
credentials, billing semantics, and deployment automation remain owned by
consumers.
