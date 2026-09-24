//! Workspace-level architecture tests.
//!
//! These tests are the single source of truth for the
//! dependency-direction rule documented in the OpenSpec change:
//!
//! * Foundation **production** crates MUST NOT depend on Axum, SQLx,
//!   database drivers, cookie/session implementations, OIDC/JWT
//!   providers, or any product crate.
//! * Foundation **production** crates MUST NOT depend on the
//!   test-only `platform-testing` crate.
//!
//! The test approach is intentionally lightweight: we read the
//! workspace's own `Cargo.toml` files, the `Cargo.lock` for the
//! transitive check, and assert the forbidden-dep set is empty. We
//! do not call `cargo metadata` so the tests are deterministic and
//! run offline.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Crate name of the test-only support crate. Production crates MUST
/// NOT list this as a direct dependency.
pub const TEST_ONLY_CRATE: &str = "platform-testing";

/// Workspace member names that are considered production crates.
pub const PRODUCTION_CRATES: &[&str] = &[
    "platform-core",
    "platform-contracts",
    "platform-identity",
    "platform-audit",
];

/// Forbidden dependency crate names. A direct or transitive
/// dependency on any of these in a production crate is a violation
/// of the foundation's provider-neutrality rule.
pub const FORBIDDEN_DEPS: &[&str] = &[
    "axum",
    "axum-extra",
    "tower",
    "tower-http",
    "sqlx",
    "sqlx-core",
    "sqlx-macros",
    "sqlx-postgres",
    "sqlx-sqlite",
    "sqlx-mysql",
    "diesel",
    "sea-orm",
    "cookie",
    "cookies",
    "jsonwebtoken",
    "openidconnect",
    "oauth2",
    "reqwest",
    "tokio", // foundation is sync-only; async adapters live outside
];

/// Direct forbidden deps that are checked by name in each
/// production crate's `Cargo.toml` `dependencies` and
/// `dev-dependencies` tables. These are listed separately from
/// [`FORBIDDEN_DEPS`] so the failure message can be specific.
pub const DIRECT_FORBIDDEN_DEPS: &[&str] = &[
    "axum",
    "sqlx",
    "cookie",
    "cookies",
    "jsonwebtoken",
    "openidconnect",
    "oauth2",
    "reqwest",
    "tokio",
];

/// Locate the workspace root by walking up from `CARGO_MANIFEST_DIR`.
pub fn workspace_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .expect("arch-tests crate lives at crates/platform-arch-tests")
        .to_path_buf()
}

/// Parse every `Cargo.toml` under `crates/` and return a map of
/// crate name -> parsed manifest.
pub fn read_workspace_manifests(workspace: &Path) -> BTreeMap<String, ParsedCrate> {
    let crates_dir = workspace.join("crates");
    let mut out = BTreeMap::new();
    for entry in fs::read_dir(&crates_dir).expect("crates/ is readable") {
        let entry = entry.expect("dir entry");
        let crate_dir = entry.path();
        if !crate_dir.is_dir() {
            continue;
        }
        let manifest_path = crate_dir.join("Cargo.toml");
        if !manifest_path.is_file() {
            continue;
        }
        let raw = fs::read_to_string(&manifest_path).expect("Cargo.toml is readable");
        let value: toml::Value = toml::from_str(&raw).expect("Cargo.toml is valid TOML");
        let name = value
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .expect("Cargo.toml has package.name")
            .to_string();
        let parsed = ParsedCrate::from_manifest(value);
        out.insert(name, parsed);
    }
    out
}

/// A minimal projection of a `Cargo.toml` manifest, just enough to
/// assert dependency direction.
#[derive(Debug, Clone, Default)]
pub struct ParsedCrate {
    /// Crate name.
    pub name: String,
    /// Direct dependencies (crate name -> path or version).
    pub dependencies: BTreeSet<String>,
    /// Dev dependencies.
    pub dev_dependencies: BTreeSet<String>,
    /// Workspace-relative directory.
    pub dir: PathBuf,
}

impl ParsedCrate {
    /// Parse a `Cargo.toml` value into a [`ParsedCrate`].
    #[must_use]
    pub fn from_manifest(value: toml::Value) -> Self {
        let name = value
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();
        let mut dependencies = BTreeSet::new();
        let mut dev_dependencies = BTreeSet::new();
        let direct_keys = ["dependencies", "dev-dependencies", "build-dependencies"];
        for key in direct_keys {
            if let Some(table) = value.get(key).and_then(|v| v.as_table()) {
                for dep_name in table.keys() {
                    if key == "dev-dependencies" {
                        dev_dependencies.insert(dep_name.clone());
                    } else {
                        dependencies.insert(dep_name.clone());
                    }
                }
            }
        }
        Self {
            name,
            dependencies,
            dev_dependencies,
            dir: PathBuf::new(),
        }
    }
}

/// Parse the workspace `Cargo.lock` to enumerate resolved packages.
#[derive(Debug)]
pub struct Lockfile {
    /// Crate name -> list of dependency crate names.
    pub packages: BTreeMap<String, Vec<String>>,
}

impl Lockfile {
    /// Load and parse `Cargo.lock` from `workspace`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the file is missing or malformed.
    pub fn load(workspace: &Path) -> Result<Self, String> {
        let path = workspace.join("Cargo.lock");
        let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let value: toml::Value =
            toml::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))?;
        let mut packages = BTreeMap::new();
        let pkgs = value
            .get("package")
            .and_then(|v| v.as_array())
            .ok_or_else(|| "Cargo.lock has no [[package]] array".to_string())?;
        for pkg in pkgs {
            let name = pkg
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "package entry missing name".to_string())?
                .to_string();
            let deps = pkg
                .get("dependencies")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|d| d.get("name").and_then(|n| n.as_str()).map(str::to_string))
                        .collect()
                })
                .unwrap_or_default();
            packages.insert(name, deps);
        }
        Ok(Self { packages })
    }

    /// Returns the transitive closure of dependencies starting from
    /// `roots`.
    pub fn transitive_closure(&self, roots: &[String]) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        let mut stack: Vec<String> = roots.to_vec();
        while let Some(next) = stack.pop() {
            if !out.insert(next.clone()) {
                continue;
            }
            if let Some(deps) = self.packages.get(&next) {
                for dep in deps {
                    if !out.contains(dep) {
                        stack.push(dep.clone());
                    }
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_workspace_manifests() {
        let ws = workspace_root();
        let manifests = read_workspace_manifests(&ws);
        for name in PRODUCTION_CRATES {
            assert!(
                manifests.contains_key(*name),
                "production crate {name} missing from crates/"
            );
        }
        assert!(
            manifests.contains_key(TEST_ONLY_CRATE),
            "test-only crate {TEST_ONLY_CRATE} missing from crates/"
        );
    }

    #[test]
    fn production_crates_have_no_direct_forbidden_deps() {
        let ws = workspace_root();
        let manifests = read_workspace_manifests(&ws);
        let mut failures: Vec<String> = Vec::new();
        for name in PRODUCTION_CRATES {
            let Some(crate_manifest) = manifests.get(*name) else {
                failures.push(format!("{name}: missing manifest"));
                continue;
            };
            for dep in &crate_manifest.dependencies {
                if DIRECT_FORBIDDEN_DEPS.contains(&dep.as_str()) {
                    failures.push(format!("{name} has forbidden direct dependency {dep:?}"));
                }
            }
            for dep in &crate_manifest.dev_dependencies {
                if DIRECT_FORBIDDEN_DEPS.contains(&dep.as_str()) {
                    failures.push(format!(
                        "{name} has forbidden direct dev-dependency {dep:?}"
                    ));
                }
                if dep == TEST_ONLY_CRATE {
                    failures.push(format!(
                        "{name} has dev-dependency on test-only crate {dep:?}"
                    ));
                }
            }
            if crate_manifest.dependencies.contains(TEST_ONLY_CRATE) {
                failures.push(format!(
                    "{name} has direct dependency on test-only crate {TEST_ONLY_CRATE:?}"
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "dependency-direction violations:\n{}",
            failures.join("\n")
        );
    }

    #[test]
    fn lockfile_resolves_and_does_not_contain_forbidden() {
        // The lockfile may be missing if the workspace has never been
        // built. In that case the test cannot enforce a transitive
        // check and we treat that as a separate, recoverable failure.
        let ws = workspace_root();
        let lockfile = match Lockfile::load(&ws) {
            Ok(l) => l,
            Err(err) => panic!("BLOCKED: {err}. Run `cargo build` once to generate Cargo.lock."),
        };
        for name in PRODUCTION_CRATES {
            let transitive = lockfile.transitive_closure(&[(*name).to_string()]);
            for forbidden in FORBIDDEN_DEPS {
                if transitive.contains(*forbidden) {
                    panic!(
                        "production crate {name} transitively depends on forbidden crate {forbidden:?}"
                    );
                }
            }
            if transitive.contains(TEST_ONLY_CRATE) && *name != TEST_ONLY_CRATE {
                panic!(
                    "production crate {name} transitively depends on test-only crate {TEST_ONLY_CRATE:?}"
                );
            }
        }
    }
}
