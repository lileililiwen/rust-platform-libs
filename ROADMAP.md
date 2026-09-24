# Rust Platform Libraries Roadmap

The roadmap is the sequence of OpenSpec changes that build out the
foundation plus the application-level helpers every consumer needs.
Two changes are done and archived; the next change brings the
in-tree fixture parity walker from 4 of 9 contract families toward
the full set.

## Dependency order

1. `rust-platform-foundation-and-contracts` — **done** (commit
   `30818c5`; archive
   `openspec/changes/archive/2026-09-24-rust-platform-foundation-and-contracts/`;
   spec `openspec/specs/rust-platform-foundation/spec.md`). Workspace,
   crate boundaries, compatibility policy, and language-neutral
   contract representations.
2. `identity-audit-and-permission-contracts` — **done** (commit
   `33917c2`; archive
   `openspec/changes/archive/2026-09-24-identity-audit-and-permission-contracts/`;
   spec `openspec/specs/rust-identity-audit-permission/spec.md`).
   `PermissionEvaluator` (with `DenyByDefaultEvaluator` and
   `AllowListEvaluator`), `TenantResolver` (with
   `NoTenantResolver`), `AuthenticationAuditor` /
   `PermissionAuditor` / `SessionAuditor`, and the
   `ScriptedPermissionEvaluator` test double.
3. `deterministic-rust-testing-toolkit` — **rolled into steps 1
   and 2** rather than shipped as a separate change. The
   `platform-testing` crate now holds `FakeClock`, `TestSubject`,
   `assert_redacted!`, and `ScriptedPermissionEvaluator`. There is
   no separate archive for this step; the test-only surface is
   covered by the two spec-synced changes above.
4. `consumer-conformance-matrix` — **next** (per `HANDOFF.md`).
   Adds the `readiness`, `gate-result`, `job-outcome`,
   `release-evidence`, and `capability` fixture walkers so the
   in-tree parity coverage moves from 4 of 9 contract families
   toward the full set. New external dependencies require
   two-consumer evidence.
5. `axum-web-adapters` — deferred. Opt-in web adapters land only
   after at least two real consumers agree on the contracts and
   an explicit OpenSpec change records the decision.
6. `sqlx-session-and-audit-adapters` — deferred. Persistence
   adapters land only after schema and concurrency evidence
   exists and an explicit OpenSpec change records the decision.

## Deferred

Universal login pages, dashboards, product entities, migrations,
provider credentials, billing semantics, and deployment automation
remain owned by consumers. Provider adapters (OIDC, JWT, cookie,
HTTP client) and persistence adapters (SQLx, Diesel, SeaORM) are
not part of the foundation; they ship in the deferred steps above
or in consumer-specific OpenSpec changes.
