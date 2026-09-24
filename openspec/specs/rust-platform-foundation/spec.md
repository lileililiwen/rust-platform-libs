# rust-platform-foundation Specification

## Purpose

The Rust platform foundation provides small, opt-in crates that carry
language-neutral contract envelopes, identity / tenant / lifecycle
contracts, audit event types, and deterministic test support. Every
production crate is provider-neutral: it does not depend on Axum, SQLx,
any database driver, cookie/session implementations, OIDC/JWT providers,
or any product crate. The application owns persistence, session storage,
provider configuration, credential material, and migrations. The
foundation only exposes types, traits, and a JSON-Schema-driven
compatibility validator that the `platform-contracts` repository
certifies.
## Requirements
### Requirement: Keep foundation crates provider-neutral

Foundation production crates MUST NOT depend on Axum, SQLx, database drivers,
cookie/session implementations, OIDC/JWT providers, or product crates.

#### Scenario: Dependency-direction check

- **WHEN** the architecture test inspects foundation dependencies
- **THEN** forbidden runtime/provider dependencies fail the check

### Requirement: Expose safe identity contracts

The identity crate MUST expose provider-neutral subject, session, permission,
tenant, and lifecycle outcomes without owning persistence or credential values.

#### Scenario: Application-owned store

- **WHEN** an application supplies its own identity store and provider
- **THEN** it can implement the platform traits without adopting a platform
  schema or database

#### Scenario: Safe failure

- **WHEN** authentication fails
- **THEN** the normalized outcome contains a stable failure category and no
  password, token, hash, or provider response detail

### Requirement: Match language-neutral contracts

Serialized Rust contract values MUST validate against the corresponding
language-neutral schema and compatibility fixtures.

#### Scenario: Valid fixture

- **WHEN** a Rust value is serialized for a supported contract version
- **THEN** the resulting document passes the canonical fixture validator

#### Scenario: Unsupported version

- **WHEN** a Rust consumer receives an unsupported major version
- **THEN** deserialization returns an explicit compatibility error

