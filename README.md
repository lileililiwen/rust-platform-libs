# Rust Platform Libraries

Small, opt-in Rust crates for shared cross-cutting contracts used by multiple
web, operator, and control-plane products. This is not an application
framework and does not own product databases, users, migrations, or UI.

The foundation and the second-layer identity/audit/permission surface are
implemented and spec-synced. Axum, SQLx, cookie, OIDC, and provider adapters
remain deferred until two real consumers demonstrate a compatible contract.
See `HANDOFF.md` for the current state and the next OpenSpec change.

## Crate matrix

Every crate inherits `version`, `license`, `publish`, `rust-version`,
and `edition` from the workspace `Cargo.toml`; the matrix restates
them so a consumer never has to open a manifest.

| Crate | Purpose | Posture | Version | License | Publish | MSRV | Edition |
|---|---|---|---|---|---|---|---|
| `platform-core` | Bounded strings, opaque IDs, clocks, correlation IDs, redaction, safe errors | production | `0.1.0` | `Apache-2.0` (workspace) | `false` | `1.75` | `2021` |
| `platform-contracts` | Rust representations of the language-neutral schemas and the JSON-Schema-driven compatibility validator | production | `0.1.0` | `Apache-2.0` (workspace) | `false` | `1.75` | `2021` |
| `platform-identity` | Provider-neutral subjects, sessions, permissions, tenants, lifecycle, and the evaluator/resolver trait surface | production | `0.1.0` | `Apache-2.0` (workspace) | `false` | `1.75` | `2021` |
| `platform-audit` | Normalized audit events, redaction policy, sink trait, and event producers | production | `0.1.0` | `Apache-2.0` (workspace) | `false` | `1.75` | `2021` |
| `platform-testing` | Deterministic fakes, builders, and assertions for consumer test suites | test-only | `0.1.0` | `Apache-2.0` (workspace) | `false` | `1.75` | `2021` |
| `platform-arch-tests` | Workspace-level dependency-direction tests | test-only | `0.1.0` | `Apache-2.0` (workspace) | `false` | `1.75` | `2021` |

All crates are version `0.1.0` and `publish = false`: nothing here is
published to a registry. Depending on a crate means a path or git
dependency (see Adoption below), never a version from crates.io.

## Usage

`docs/usage.md` gives one minimal, compile-checked example per
production crate, each backed by a file under `crates/*/examples/`
that `cargo test --workspace` builds. Test-only crates are excluded.

## License

The workspace declares `license = "Apache-2.0"` and every crate
inherits it (`license.workspace = true`). The license text lives at
the repository root in `LICENSE`.

## Adoption

The only proven adoption route is a path dependency, as used by the
in-workspace `tests/consumer-fixture` crate:

```toml
[dependencies]
platform-core = { path = "../rust-platform-libs/crates/platform-core" }
```

The shared lint, MSRV, CI, and conformance recipe for adopting the
platform is owned by the `rust-platform-consumer-baseline` change
(spec `openspec/specs/rust-platform-consumer-baseline/spec.md`,
recipe `docs/consumer-baseline.md`). This README does not re-declare
that recipe; follow it verbatim instead of forking it.

## Implemented crates

- `platform-core` — bounded results, errors, IDs, clocks, redaction, and
  correlation contracts.
- `platform-contracts` — Rust representations of the language-neutral
  schemas and the JSON-Schema-driven compatibility validator.
- `platform-identity` — provider-neutral `Subject`, `Session`, `Permission`,
  `Tenant`, `Lifecycle`, `IdentityError`, `IdentityOutcome`, and the
  `IdentityStore` / `IdentityProvider` / `CurrentSubject` traits. The
  second-layer surface adds `PermissionEvaluator` (with
  `DenyByDefaultEvaluator` and `AllowListEvaluator` reference impls) and
  `TenantResolver` (with `NoTenantResolver` placeholder). No persistence,
  no provider config, no HTTP.
- `platform-audit` — normalized `AuditEvent`, `AuditEventBuilder`,
  `RedactedContext` (rejects secret-named keys at construction), the
  `AuditSink` trait with `NoopAuditSink` and `InMemoryAuditSink`, and the
  `AuthenticationAuditor`, `PermissionAuditor`, and `SessionAuditor`
  event producers. No storage, no transport, no retention policy.
- `platform-testing` — deterministic `FakeClock`, the `TestSubject`
  builder, `assert_redacted!` helper, and the
  `ScriptedPermissionEvaluator` test double. Production crates MUST NOT
  depend on this crate; `platform-arch-tests` enforces it.
- `platform-arch-tests` — workspace-level dependency-direction tests that
  read the `Cargo.lock` and assert no production crate (directly or
  transitively) depends on `axum`, `sqlx`, `cookie`, `jsonwebtoken`,
  `openidconnect`, `oauth2`, `reqwest`, `tokio`, or `platform-testing`.

## Deferred

Axum, SQLx, cookie, OIDC, universal login pages, dashboards, product
entities, migrations, provider credentials, billing semantics, and
deployment automation remain owned by consumers. Provider adapters
land after at least two consumers demonstrate a compatible contract;
persistence adapters land after schema and concurrency evidence exists.

## Verification

The verification command is `cargo test --workspace`. The full
verification surface (recorded in `HANDOFF.md`) also includes
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets
-- -D warnings`, `openspec validate --specs --strict --no-interactive`,
and the in-tree 108-case cross-repository fixture parity walker in
`crates/platform-contracts/tests/fixture_parity.rs` (9 of 9 roadmap
contract families × 12 canonical cases).

Build/test/skeleton success is not runtime or release evidence. Each
completed OpenSpec spec/change requires exactly two commits: first the
implementation/tests/archive commit, then a `HANDOFF.md`-only
pointer/evidence commit.

Use OpenSpec for every non-trivial change and follow BFS → DFS → BFS.
