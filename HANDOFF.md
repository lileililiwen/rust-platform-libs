# Rust Platform Libraries Handoff

current_spec: rust-platform-foundation-and-contracts

## Current state

The repository is a newly initialized, non-deployable planning project. Git,
OpenSpec, governance metadata, and the crate roadmap exist; no Rust crates or
runtime adapters have been implemented.

## Next action

Implement only `rust-platform-foundation-and-contracts` after selection. Begin
with pure contracts and compatibility fixtures; do not introduce Axum, SQLx,
OIDC, cookie, or application persistence dependencies in the foundation.

## Evidence boundary

Documentation and strict OpenSpec validation prove planning artifacts only.
They do not prove that any product can adopt these crates or that an identity
flow is secure in production.

Each completed OpenSpec spec/change requires exactly two commits: first the
implementation/tests/archive commit, then a HANDOFF.md-only pointer/evidence
commit.
