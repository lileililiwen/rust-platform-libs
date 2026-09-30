# Tasks: crates.io publication

## 0. Caveats and confirmations

- [ ] **Release execution is owner-gated.** No tag push, no real
  publication, until the approval marker + `CARGO_REGISTRY_TOKEN` exist
  (design P6). Tasks 3.x stay unchecked with that reason recorded.
- [ ] **Do not edit consumers.** Migration to registry deps is each
  consumer's own change.
- [ ] **`platform-arch-tests` stays unpublishable.**
- [ ] **Re-verify name availability at publish time** (2026-09-30 evidence
  says all seven names free).
- [ ] **`--allow-dirty` banned**; `--no-verify` banned.

## 1. BFS — Baseline and impact coverage

- [ ] Read `[workspace.package]`, every crate manifest, the DAG used by
  `platform-arch-tests`, and existing `ci.yml` trigger patterns.
- [ ] Map PUB-001..004 scenarios to the workflow matrix and dry-run checks
  in design §7; confirm proposal/design/spec agree.
- [ ] Record the precondition state: no git remote (verified 2026-09-30);
  note the workspace-governance GitHub flow as the exact next action for
  the owner.

## 2. DFS — Requirement-by-requirement implementation

- [ ] 2.1 PUB-001: workspace metadata keys (`repository` left as the
  workflow-input template value until the URL exists — record the chosen
  placeholder mechanism and replace in 3.x), per-crate `keywords`/
  `categories`/`exclude`; flip `publish = true`; explicit `publish = false`
  for `platform-arch-tests`.
- [ ] 2.2 PUB-001: inter-crate deps to `{ path = …, version = … }` form per
  the DAG; `cargo publish --dry-run` all six green; `cargo package --list`
  excludes openspec/.github.
- [ ] 2.3 PUB-002/004: `.github/workflows/publish.yml` — tag trigger;
  marker + secret + tag-vs-workspace version prechecks; topological order
  list with a unit test comparing it to the manifest DAG; per-crate 404
  pre-flight, publish, propagation poll, skip-existing idempotence;
  failure table from design §6 reflected in step names.
- [ ] 2.4 PUB-003: `docs/publication.md` (lockstep rule, pre-1.0 policy,
  yank procedure, token scope/rotation, changelog section); README
  publication line; ROADMAP entry marked.

## 3. BFS — Cross-surface regression and completeness

- [ ] `ci.yml` unaffected and green; `cargo test --workspace` green with
  the version-inheriting dep table; arch-tests still pass (dev-only ban
  unchanged).
- [ ] Workflow negative matrix runs (version mismatch, missing marker):
  each fails with the named reason, zero publishes.
- [ ] Confirm no placeholder remains except the two tracked external
  preconditions (GitHub URL value, approval marker), both named in HANDOFF.

## 4. Verification

- [ ] `cargo fmt --all -- --check`; `cargo test --workspace`;
  `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `node scripts/check-openspec-change-names.mjs`;
  `openspec validate --changes --strict --no-interactive`
- [ ] Record state in `HANDOFF.md`: implementation of the pipeline is
  DONE; the release itself is BLOCKED pending owner approval with the
  exact commands (`git remote add`, `github_metadata.py … --publish`,
  tag `v0.1.0`). Two-commit cadence; do not push.
