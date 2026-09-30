# Publishing the platform crates to crates.io

This document defines the version, yank, and token policy for the
`platform-*` crates, plus the pre-publish checklist the owner follows
when releasing. The `registry-publication` OpenSpec change ships the
pipeline; this file is the policy that pipeline enforces and the
reference humans read when they cut a release.

## Lockstep versioning

The workspace ships one version across all publishable crates. The
version lives in `[workspace.package] version` in the root
`Cargo.toml`; every crate manifest inherits it through
`version.workspace = true`. The publish order
([`.publish-order`](../.publish-order)) is the single source of truth
for the topological sequence in which the crates are uploaded.

A version bump MUST land in one commit together with:

1. The `version` line in the root `Cargo.toml`.
2. The corresponding `vX.Y.Z` tag (created at release time, not in
   the version-bump commit).
3. A new entry in [Changelog](#changelog) below.

Crates are not versioned independently. The contract families
(`identity-subject`, `permission-decision`, `tenant-context`,
`audit-event`, `readiness`, `gate-result`, `job-outcome`,
`release-evidence`, `capability`) are co-released because the
language-neutral schema and the Rust types evolve together; an
out-of-band version of any one crate would break the
`platform-contracts` parity walker.

## Pre-1.0 semver rule

The workspace is at `0.y.z`. Under cargo's pre-1.0 convention:

* A patch bump (`0.y.(z+1)`) is reserved for backward-compatible
  changes and bug fixes that do not alter any public API.
* A minor bump (`0.(y+1).0`) is the only way to ship a breaking
  change while the workspace is pre-1.0. The release MUST be
  accompanied by a `Breaking:` line in the changelog naming the
  affected crate(s) and the new API.
* A major bump (`(X+1).0.0`) is a future event when the foundation
  commits to API stability. Until then, the version stays below
  `1.0.0`.

`0.1.0` is the reserved first release; subsequent `0.1.x` patches
keep the API surface unchanged. Any intentional break is a
`0.2.0` (or later) and MUST carry the changelog notice.

## Yank procedure

crates.io versions are immutable: once a version is published, the
source it points at cannot be edited. A bad release is corrected
by yanking, not by re-publishing.

Yank rules:

* Yank is a manual `cargo yank` per crate, executed by the owner.
* Every yank MUST add a `Yanked:` line to the changelog below,
  naming the yanked version, the reason, and the replacement
  version (typically the next patch that fixes the defect).
* A yanked version stays on the registry (the `Cargo.toml` line
  is preserved) but new `cargo add` calls cannot pick it up by
  default; consumers who already depend on it keep building.
* A yanked version is NOT deleted from the registry; the
  registry does not support deletion. Do not promise deletion
  in any release note.

If a published version has a defect that consumers can mitigate
by upgrading, the patch is the right tool, not yank: bump the
patch version, fix the defect, and tell consumers to upgrade.
Yank is reserved for versions that are unsafe or misleading to
use even as a transitive dependency.

## Token handling

The `CARGO_REGISTRY_TOKEN` repository secret gates every release.
The token MUST be crate-scoped to `platform-*` (not user-wide),
time-limited, and rotated whenever a maintainer with the token
leaves the maintainer group. The token-creation recipe is:

1. Visit <https://crates.io/settings/tokens> and create a token
   with the scope `publish` for the `platform-*` crates.
2. Store the token in the repository's
   `CARGO_REGISTRY_TOKEN` secret (Settings → Secrets and
   variables → Actions → New repository secret).
3. Never paste the token into a commit, an issue, or a PR
   comment. If the token is exposed, revoke it immediately and
   create a replacement.

The publish workflow refuses to run without the secret. A leaked
secret is a release-stopping event: rotate before retrying.

## Pre-publish checklist

The release is BLOCKED on two named preconditions. Both are
checked by `.github/workflows/publish.yml` before any registry
call is made, so a missing precondition fails the job with a
named cause and zero registry side effects.

1. **GitHub publication.** The repository must have a `git
   remote` pointing at the public GitHub URL the owner chose
   (the `[workspace.package] repository` value). At the time
   of this change the workspace has no remote
   (`git remote -v` is empty), so the `repository` value in the
   root `Cargo.toml` is a documented placeholder
   (`https://github.com/rust-platform-libs/rust-platform-libs`)
   that MUST be replaced with the real URL before the first
   `v0.1.0` tag. The replacement is performed through the
   workspace-governance `scripts/github_metadata.py` flow
   (`--publish` mode), which both updates the metadata and
   pushes the repository.

2. **Owner approval marker.** The `PUBLICATION-APPROVED` file
   at the repository root is created and tracked only when the
   owner explicitly approves the release. The publish workflow
   refuses to start without it. Removing the file in a later
   commit re-blocks the release; the file is the durable,
   auditable sign-off.

When both preconditions are met, the release steps are:

```sh
# One-time per release: bump the workspace version in Cargo.toml,
# commit, and tag.
$EDITOR Cargo.toml   # bump `version = "0.1.0"` to `0.1.1` (or `0.2.0` for a break)
git commit -am "Bump workspace version to 0.1.1"
git tag v0.1.1
git push origin main --tags
```

The tag push triggers `.github/workflows/publish.yml`, which runs
the prechecks, the name pre-flight, the topological publish, and
the propagation polls. The release is complete when the workflow
ends green and every crate in `.publish-order` shows the new
version on <https://crates.io>.

## Changelog

This section records every published version, every yank, and
every breaking change. Entries are appended, never edited, so
the history is auditable.

| Version | Date | Notes |
|---|---|---|
| _pending_ | _pending_ | The first `v0.1.0` release is BLOCKED on the two preconditions in the pre-publish checklist above. |
