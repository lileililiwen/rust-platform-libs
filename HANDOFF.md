# Rust Platform Libraries Handoff

current_spec: rust-platform-foundation

## Current state

`rust-platform-foundation-and-contracts` is **implemented, archived,
and spec-synced** in this repository. The OpenSpec change directory
was moved to
`openspec/changes/archive/2026-09-24-rust-platform-foundation-and-contracts/`
on 2026-09-24; the corresponding spec delta now lives at
`openspec/specs/rust-platform-foundation/spec.md`.

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
    `IdentityOutcome`, and the `IdentityStore` / `IdentityProvider` /
    `CurrentSubject` traits. No persistence, no provider config, no
    HTTP.
  * `platform-audit` — `AuditEvent`, `AuditEventBuilder`,
    `RedactedContext` (rejects secret-named keys at construction), and
    the `AuditSink` trait with `NoopAuditSink` and `InMemoryAuditSink`.
  * `platform-testing` — test-only `FakeClock`, `TestSubject`
    builder, `assert_redacted!` helper. Production crates MUST NOT
    depend on this crate; `platform-arch-tests` enforces it.
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

## Evidence (recorded 2026-09-24)

* `node scripts/check-openspec-change-names.mjs` → ok.
* `cargo fmt --all -- --check` → clean.
* `cargo test --workspace` → **69 passed, 0 failed**.
* `cargo clippy --workspace --all-targets -- -D warnings` → clean.
* `openspec validate --changes --strict --no-interactive` → 1/1
  passed (`rust-platform-foundation-and-contracts`).
* `openspec validate --specs --strict --no-interactive` → 1/1 passed
  (`spec/rust-platform-foundation`, 3 requirements, 5 scenarios).
* Cross-repository fixture parity: **48 cases × 4 contract families
  pass** (`identity-subject`, `permission-decision`,
  `tenant-context`, `audit-event`). Every valid, invalid, boundary,
  unknown-field, redaction, and compatibility outcome in the
  language-neutral `platform-contracts` matrix is matched by the
  Rust validator.

The complete commit is `30818c5` on `main` ("Implement
rust-platform-foundation-and-contracts"). The change archive is
`openspec/changes/archive/2026-09-24-rust-platform-foundation-and-contracts/`
and the synced spec is
`openspec/specs/rust-platform-foundation/spec.md`.

## Evidence boundary

The 69 unit and integration tests, the 48-case cross-repository
fixture parity walk, the dependency-direction lockfile check, the
`cargo clippy` clean run, and the strict `openspec validate` prove
that the **wire-format and crate-shape** of the foundation is
consistent, machine-checkable, and provider-neutral. They do not
prove that any consumer (`openaccounting`, `rust-ecommerce`, `hookit`,
`openpanel`, `vowra`, or a future Rust service) has adopted these
crates in production; that evidence will come from each consumer's
own OpenSpec change.

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

## Next action

The next OpenSpec change is `identity-audit-and-permission-contracts`
(see `ROADMAP.md` step 2). It will add concrete permission-decision
constructors, audit-event producer helpers, and tenant-context
resolution that build on the trait surface implemented here. Until
then, the foundation is the working state and the
`current_spec` pointer above is the source of truth.
