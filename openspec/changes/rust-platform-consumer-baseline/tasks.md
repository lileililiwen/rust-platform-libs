# Tasks: A consumer baseline for the Rust platform

## 0. Caveats and confirmations

- [ ] **The MSRV is a policy decision, not a measurement.** Existing
  `rust-version` values range from 1.74 to 1.85 and four repositories declare
  none. Confirm the chosen MSRV with the maintainer; raising it later needs a
  separate change and a migration note.
- [ ] **Record environment-blocked checks.** If `cargo-deny` is unavailable, the
  task is `BLOCKED` with the exact command and next action — never reported as
  passing and never silently skipped.
- [ ] **The 4→9 contract-family parity is out of scope.** It belongs to
  `contracts-consumer-parity` in `platform-contracts`; do not expand this change
  into it.
- [ ] **Do not edit consumers.** `trailCrew`'s hand-written path dependency is the
  only current consumer; migration is documented, not performed.
- [ ] **Keep `publish = false`.** Registry publication is a separate decision.
- [ ] **No silent lint allowances.** Any allow needed to make the baseline pass
  must be recorded with a justification rather than added quietly.
- [ ] **Preserve provider neutrality.** `platform-arch-tests` must keep passing;
  no Axum, SQLx, OIDC/JWT or product dependency may enter a production crate.
- [ ] **Stage only this change directory.** `.project.json` is already modified and
  `.github/` is untracked; keep them out of the change's commits.

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
