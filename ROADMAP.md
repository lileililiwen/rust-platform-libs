# Rust Platform Libraries Roadmap

The roadmap is the sequence of OpenSpec changes that build out the
foundation plus the application-level helpers every consumer needs.
The foundation, identity/audit/permission, consumer-baseline,
conformance-matrix, readiness, and mailing changes are done and
archived; the specs under `openspec/specs/` are the working state.

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
4. `consumer-conformance-matrix` — **done** (archive
   `openspec/changes/archive/2026-09-30-consumer-conformance-matrix/`).
   Adds the `readiness`, `gate-result`, `job-outcome`,
   `release-evidence`, and `capability` fixture walkers plus typed
   Rust payloads for `gate-result`, `job-outcome`,
   `release-evidence`, and `capability` (`Readiness` already
   shipped), bringing in-tree parity coverage from 4 of 9 contract
   families to 9 of 9 roadmap families. `public-portfolio-manifest`
   (tenth registry family, Forge/Hugo owner, major version 1) stays
   out of scope. New external dependencies require
   two-consumer evidence.
4b. `rust-platform-consumer-baseline` — **done**. Committed
   `rustfmt.toml`, `clippy.toml`, `deny.toml`, and
   `rust-toolchain.toml` (pin `1.95.0`; declared MSRV stays `1.75`
   with an effective floor of 1.88 via `time 0.3.55`, recorded in
   `docs/consumer-baseline.md`), extended CI to formatting, clippy
   `-D warnings`, `cargo deny check`, tests, and
   `scripts/conformance.sh`, plus the `tests/consumer-fixture`
   in-workspace consumer. `publish = false` unchanged.
4c. `mailing-provider-adapters` — **done** (archive
    `openspec/changes/archive/2026-09-30-mailing-provider-adapters/`;
    spec `openspec/specs/platform-mailing/spec.md`). New satellite
    crate `platform-mailing`: provider-neutral send contract
    (MAIL-001), opt-in `smtp-lettre` adapter with rustls only
    (MAIL-002), deterministic in-crate `mock` transport (MAIL-003),
    and the named-satellite dependency-direction allowlist in
    `platform-arch-tests` (MAIL-004). HTTP API provider adapters
    (Resend/Brevo) stay deferred per the two-consumer rule; router,
    quota, templates, DKIM/IMAP, and consumer migrations are each
    their own future change.
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
