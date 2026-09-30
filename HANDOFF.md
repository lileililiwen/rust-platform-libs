# Rust Platform Libraries Handoff

current_spec: rust-platform-foundation, rust-identity-audit-permission, rust-platform-consumer-baseline

## Current state

Two OpenSpec changes are **implemented, archived, and
spec-synced** in this repository.

1. `rust-platform-foundation-and-contracts` (archived on 2026-09-24
   to `openspec/changes/archive/2026-09-24-rust-platform-foundation-and-contracts/`,
   spec at `openspec/specs/rust-platform-foundation/spec.md`).
2. `identity-audit-and-permission-contracts` (archived on
   2026-09-24 to
   `openspec/changes/archive/2026-09-24-identity-audit-and-permission-contracts/`,
   spec at `openspec/specs/rust-identity-audit-permission/spec.md`).

What is in the tree:

* `Cargo.toml` — workspace with six crates:
  * `platform-core` — bounded strings, opaque `Id<T>`, `Clock` trait
    plus `SystemClock` / `FixedClock`, `CorrelationId`, `Secret<T>`
    redaction wrapper, and the `ErrorCategory` / `SafeError` model.
  * `platform-contracts` — `Envelope`, `ContractRegistry` loader,
    JSON-Schema-driven compatibility validator (Draft 2020-12 via the
    `jsonschema` crate), and Rust payload types for the nine
    language-neutral contract families.
  * `platform-identity` — provider-neutral `Subject`, `Session`,
    `Permission`, `Tenant`, `Lifecycle`, `IdentityError`,
    `IdentityOutcome`, `IdentityStore` / `IdentityProvider` /
    `CurrentSubject` traits, plus the second-layer `PermissionEvaluator`
    (with `DenyByDefaultEvaluator` and `AllowListEvaluator` reference
    impls) and `TenantResolver` (with `NoTenantResolver` placeholder).
    No persistence, no provider config, no HTTP.
  * `platform-audit` — `AuditEvent`, `AuditEventBuilder`,
    `RedactedContext` (rejects secret-named keys at construction), the
    `AuditSink` trait with `NoopAuditSink` and `InMemoryAuditSink`, and
    the `AuthenticationAuditor`, `PermissionAuditor`, and
    `SessionAuditor` event producers.
  * `platform-testing` — test-only `FakeClock`, `TestSubject`
    builder, `assert_redacted!` helper, and the
    `ScriptedPermissionEvaluator` test double. Production crates
    MUST NOT depend on this crate; `platform-arch-tests` enforces
    it.
  * `platform-arch-tests` — workspace-level dependency-direction
    tests that read the `Cargo.lock` and assert no production crate
    (directly or transitively) depends on `axum`, `sqlx`, `cookie`,
    `jsonwebtoken`, `openidconnect`, `oauth2`, `reqwest`, `tokio`, or
    `platform-testing`.
* `crates/platform-contracts/tests/fixture_parity.rs` — the
  cross-repository fixture walker. The test resolves the live
  `platform-contracts` repository via `PLATFORM_CONTRACTS_DIR` (or
  `../platform-contracts` relative to the workspace root) and walks
  the canonical fixture matrix.
* `crates/platform-testing/tests/identity_permission_audit_e2e.rs` —
  the end-to-end identity → permission → audit smoke test that
  asserts the new producer surface and the new evaluator trait
  surface compose into the same canonical audit event that the
  `platform-contracts` schema expects.

## Evidence (recorded 2026-09-24)

* `node scripts/check-openspec-change-names.mjs` → ok.
* `cargo fmt --all -- --check` → clean.
* `cargo test --workspace` → **90 passed, 0 failed** (69 foundation
  tests + 21 new tests across the permission evaluator, tenant
  resolver, audit producers, scripted test double, and the
  end-to-end identity → permission → audit smoke test).
* `cargo clippy --workspace --all-targets -- -D warnings` → clean.
* `openspec validate --changes --strict --no-interactive` → 1/1
  passed (`identity-audit-and-permission-contracts`).
* `openspec validate --specs --strict --no-interactive` → 2/2 passed
  (`spec/rust-platform-foundation` 3 requirements / 5 scenarios;
  `spec/rust-identity-audit-permission` 4 requirements / 4
  scenarios).
* Cross-repository fixture parity: **48 cases × 4 contract families
  pass** (`identity-subject`, `permission-decision`,
  `tenant-context`, `audit-event`). Every valid, invalid, boundary,
  unknown-field, redaction, and compatibility outcome in the
  language-neutral `platform-contracts` matrix is matched by the
  Rust validator.

The complete implementation commit is `33917c2` on `main`
("Implement identity-audit-and-permission-contracts"). The
foundation implementation commit is `30818c5`; the foundation
HANDOFF pointer is `73a8f88`. The change archives are
`openspec/changes/archive/2026-09-24-rust-platform-foundation-and-contracts/`
and
`openspec/changes/archive/2026-09-24-identity-audit-and-permission-contracts/`,
and the synced specs are `openspec/specs/rust-platform-foundation/spec.md`
and `openspec/specs/rust-identity-audit-permission/spec.md`.

## Evidence — rust-platform-consumer-baseline (recorded 2026-09-30)

The change is implemented and verified; the archive step is commit 1
of the two-commit cadence below.

What was added (no crate public API, spec, or consumer change):

* `rustfmt.toml` — stable-only settings; `cargo fmt --check` is
  warning-free on the pinned toolchain (nightly-only
  `imports_granularity` / `group_imports` were tried and removed for
  exactly this reason).
* `clippy.toml` — `msrv = "1.75"` matching the declared MSRV.
* `rust-toolchain.toml` — pin `channel = "1.95.0"`. The first choice,
  `1.85.0`, could not build the workspace (`time 0.3.55` requires
  rustc >= 1.88); the declared MSRV stays `1.75` per maintainer
  decision and the effective 1.88 floor is recorded in
  `docs/consumer-baseline.md` §2. Raising the declared MSRV is a
  separate change with a migration note.
* `deny.toml` — `cargo deny` baseline (permissive-license allowlist,
  RustSec advisories, wildcard versions denied with
  `allow-wildcard-paths = true` so intra-workspace path deps pass,
  crates.io only). Passes with one standing warning (duplicate
  `getrandom` 0.3.4/0.4.3 via `jsonschema` vs `uuid`); duplicates are
  `warn`, not `deny`.
* `.github/workflows/ci.yml` — extended from tests-only to
  formatting, clippy `-D warnings`, `cargo deny check`, tests, and
  `./scripts/conformance.sh`, all on the pinned toolchain (no
  toolchain input; `rust-toolchain.toml` governs).
* `docs/consumer-baseline.md` — the adoption recipe (in-workspace
  path form, out-of-workspace versioned git form, MSRV policy, CI
  table, harness proofs, non-goals).
* `scripts/conformance.sh` — six proofs, all holding: pinned
  toolchain identity, in-workspace fixture green with
  workspace-relative paths only, out-of-workspace scratch crate
  resolving/building/testing/formatting/linting clean under the
  verbatim-copied baseline, and three negative fixtures failing
  loudly with named causes (`unused-variables`, `requires rustc
  1.99`, banned `serde`).
* `tests/consumer-fixture` — workspace-member consumer proving the
  path recipe end to end (`TestSubject` → `DenyByDefaultEvaluator` →
  `PermissionAuditor` → `InMemoryAuditSink`, asserting
  `Permission`/`Denied`).

Verification outputs (pinned toolchain 1.95.0, all fresh this session):

* `cargo test --workspace` → **91 passed, 0 failed** (90 prior + 1
  fixture).
* `cargo fmt --all -- --check` → clean.
* `cargo clippy --workspace --all-targets -- -D warnings` → clean.
* `cargo deny check` → advisories ok, bans ok, licenses ok, sources ok.
* `./scripts/conformance.sh` → all proofs hold.
* `node scripts/check-openspec-change-names.mjs` → ok.
* `openspec validate --changes --strict --no-interactive` → 3/3 passed.
* `openspec validate --specs --strict --no-interactive` → 2/2 passed.
* `platform-arch-tests` passes unchanged (run inside `cargo test
  --workspace`); `[workspace.lints]` untouched; `publish = false`
  unchanged; no sibling repository touched.
* Commit note: `.github/workflows/ci.yml` existed only as untracked
  tests-only scaffolding; this change extends it to the enforced
  baseline above and commits it. The pre-existing `.project.json`
  modification stays out of this change's commits per the task
  caveats.

Evidence boundary for this change: the harness proves the recipe
resolves, builds, tests, formats, lints, and fails loudly with named
causes. It does not prove any consumer has adopted the baseline in
production, and the out-of-workspace probe uses an absolute path
dependency as an explicitly documented offline stand-in for the
versioned git form (no registry or git remote exists in this
environment).

## Evidence boundary

The 90 unit and integration tests, the 48-case cross-repository
fixture parity walk, the dependency-direction lockfile check, the
`cargo clippy` clean run, and the strict `openspec validate` prove
that the **wire-format and crate-shape** of the foundation plus
the second-layer permission / audit / tenant helper surface is
consistent, machine-checkable, and provider-neutral. They do not
prove that any consumer (`openaccounting`, `rust-ecommerce`,
`hookit`, `openpanel`, `vowra`, or a future Rust service) has
adopted these crates in production; that evidence will come from
each consumer's own OpenSpec change.

In particular:

* Only 4 of the 9 language-neutral contract families are exercised
  in the in-tree parity walker. The remaining 5 (`readiness`,
  `gate-result`, `job-outcome`, `release-evidence`, `capability`)
  are part of the next consumer-conformance-matrix change; the
  foundation does not claim 9-family parity today.
* The fixture parity proves the Rust validator agrees with the
  language-neutral one on documents that already conform to the
  schema. It does not prove that the Rust types are semantically
  faithful to every consumer's existing identity, permission, or
  audit logic; that is the application-owned side of the boundary
  described in `crates/platform-identity/src/lib.rs`.
* The dependency-direction test forbids `axum`, `sqlx`, `cookie`,
  `jsonwebtoken`, `openidconnect`, `oauth2`, `reqwest`, `tokio`, and
  any product crate in production crates. It does not assert that
  the application that adopts the foundation is itself provider-
  neutral; applications may pull in any framework they want behind
  the platform's trait surface.
* The `AllowListEvaluator` is the smallest non-trivial reference
  implementation. The platform deliberately does not ship an
  RBAC DSL, role inheritance, or attribute-based policy language;
  applications plug in their real policy behind the
  `PermissionEvaluator` trait. The reference impls are safe
  defaults and tests fixtures, not a recommended production
  policy.

## Next action

The next OpenSpec change is the consumer-conformance-matrix
described in `ROADMAP.md` step 3. It will add the readiness,
gate-result, job-outcome, release-evidence, and capability
fixture walkers and bring the in-tree parity coverage from 4 of
9 contract families to a larger share. Until then, the
foundation plus the second-layer permission / audit / tenant
helpers are the working state and the `current_spec` pointer
above is the source of truth.
