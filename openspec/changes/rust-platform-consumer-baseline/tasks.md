# Tasks: A consumer baseline for the Rust platform

## 1. BFS — Baseline and impact coverage

- [ ] Inventory the workspace `Cargo.toml`, the existing `[workspace.lints]`,
  `platform-arch-tests`, and the current CI workflow.
- [ ] Record the chosen MSRV and confirm every crate builds under the pinned
  toolchain before changing CI.
- [ ] Add conformance fixtures: an in-workspace consumer, an out-of-workspace
  consumer, a lint-breaking case, an MSRV-breaking case, and a denied-dependency
  case.
- [ ] Confirm `publish = false` and the provider-neutral boundary are untouched.

## 2. DFS — Requirement-by-requirement implementation

- [ ] Add `rustfmt.toml`, `clippy.toml`, `deny.toml` and the `[workspace.lints]`
  baseline.
- [ ] Add `rust-toolchain.toml` and the MSRV policy in
  `[workspace.package].rust-version`.
- [ ] Extend CI to formatting, clippy `-D warnings`, `cargo deny check`, and
  tests.
- [ ] Write `docs/consumer-baseline.md` with the adoption recipe (in-workspace
  path and out-of-workspace versioned dependency).
- [ ] Implement `scripts/conformance.sh` and the consumer fixture.

## 3. BFS — Cross-surface regression and completeness

- [ ] Re-run the full crate test suite and `platform-arch-tests` under the pinned
  toolchain.
- [ ] Verify the lint baseline does not require allow-listing existing code
  silently; record any justified allow.
- [ ] Verify a consumer following the documented recipe resolves the platform
  without any hard-coded sibling path.
- [ ] Update `ROADMAP.md` and `HANDOFF.md`.

## 4. Verification

- [ ] `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D
  warnings && cargo deny check && cargo test --workspace`.
- [ ] `./scripts/conformance.sh`.
- [ ] `node scripts/check-openspec-change-names.mjs` (if present) and
  `openspec validate --changes --strict --no-interactive`.
- [ ] Record actual outputs; report any environment-blocked check (for example a
  missing `cargo-deny`) with its exact next action rather than as passing.
