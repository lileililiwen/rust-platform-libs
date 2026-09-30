//! Cross-repository fixture parity tests.
//!
//! These tests load the **live** `platform-contracts` repository and
//! walk the canonical fixture matrix to prove the Rust validator
//! agrees with the language-neutral one. The path is resolved in this
//! order:
//!
//! 1. `PLATFORM_CONTRACTS_DIR` environment variable (exact path).
//! 2. `../platform-contracts` relative to the workspace root (the
//!    common monorepo layout).
//!
//! If neither is reachable, the test **fails** with a single, clear
//! next-action message — the foundation does not silently pass on
//! missing cross-repository evidence, per task 4 of the OpenSpec
//! change.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use platform_contracts::validate::{validate_document, ValidationResult};
use platform_contracts::ContractRegistry;
use serde_json::Value;

/// Resolve the live `platform-contracts` repository path.
fn platform_contracts_dir() -> PathBuf {
    if let Ok(env_path) = env::var("PLATFORM_CONTRACTS_DIR") {
        return PathBuf::from(env_path);
    }
    // `CARGO_MANIFEST_DIR` for an integration test in
    // `crates/platform-contracts/tests/` is
    // `.../rust-platform-libs/crates/platform-contracts`. Two levels up
    // is the workspace root; one more level up is the monorepo root.
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest
        .parent()
        .and_then(Path::parent)
        .expect("crate path is at least two levels deep");
    let monorepo = workspace.parent().expect("workspace path has a parent");
    monorepo.join("platform-contracts")
}

fn load_registry_or_fail(dir: &Path) -> ContractRegistry {
    ContractRegistry::load(dir).unwrap_or_else(|err| {
        panic!(
            "BLOCKED: cannot load platform-contracts registry from {}: {err}. \
             Set PLATFORM_CONTRACTS_DIR to the language-neutral repository root \
             (the directory containing `schemas/registry.json`).",
            dir.display()
        )
    })
}

#[derive(Debug)]
struct FixtureCase {
    file: String,
    #[allow(dead_code)]
    category: String,
    description: String,
    expected: String,
    expected_error_codes: Vec<String>,
    expected_warning_codes: Vec<String>,
}

fn load_manifest(family_dir: &Path) -> Vec<FixtureCase> {
    let manifest_path = family_dir.join("manifest.json");
    let raw = fs::read_to_string(&manifest_path).unwrap_or_else(|err| {
        panic!(
            "BLOCKED: missing manifest at {}: {err}",
            manifest_path.display()
        )
    });
    let value: Value = serde_json::from_str(&raw).unwrap_or_else(|err| {
        panic!(
            "manifest at {} is not valid JSON: {err}",
            manifest_path.display()
        )
    });
    let cases = value
        .get("fixtures")
        .and_then(Value::as_array)
        .expect("manifest must contain a 'fixtures' array");
    cases
        .iter()
        .map(|case| FixtureCase {
            file: case
                .get("file")
                .and_then(Value::as_str)
                .unwrap()
                .to_string(),
            category: case
                .get("category")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            description: case
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            expected: case
                .get("expected")
                .and_then(Value::as_str)
                .unwrap_or("valid")
                .to_string(),
            expected_error_codes: case
                .get("expected_error_codes")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default(),
            expected_warning_codes: case
                .get("expected_warning_codes")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default(),
        })
        .collect()
}

fn check_one(
    family: &str,
    family_dir: &Path,
    registry: &ContractRegistry,
    case: &FixtureCase,
) -> Result<(), String> {
    let path = family_dir.join(&case.file);
    let raw =
        fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let value: Value = serde_json::from_str(&raw)
        .map_err(|e| format!("{} is not valid JSON: {e}", path.display()))?;
    let result: ValidationResult = validate_document(&value, registry)
        .map_err(|e| format!("validator failure on {}: {e}", path.display()))?;

    let ok = result.valid;
    let want_valid = case.expected == "valid";

    if ok != want_valid {
        return Err(format!(
            "{}/{}: expected={} got valid={} errors={:?} warnings={:?} description={}",
            family,
            case.file,
            case.expected,
            ok,
            result
                .errors
                .iter()
                .map(|e| (&e.code, &e.path))
                .collect::<Vec<_>>(),
            result
                .warnings
                .iter()
                .map(|w| (&w.code, &w.path))
                .collect::<Vec<_>>(),
            case.description,
        ));
    }

    for code in &case.expected_error_codes {
        if !result.errors.iter().any(|e| &e.code == code) {
            return Err(format!(
                "{}/{}: expected error code {code:?}, got {:?}",
                family,
                case.file,
                result
                    .errors
                    .iter()
                    .map(|e| e.code.clone())
                    .collect::<Vec<_>>(),
            ));
        }
    }
    for code in &case.expected_warning_codes {
        if !result.warnings.iter().any(|w| &w.code == code) {
            return Err(format!(
                "{}/{}: expected warning code {code:?}, got {:?}",
                family,
                case.file,
                result
                    .warnings
                    .iter()
                    .map(|w| w.code.clone())
                    .collect::<Vec<_>>(),
            ));
        }
    }
    Ok(())
}

fn walk_family(family: &str, dir: &Path, registry: &ContractRegistry) -> Vec<String> {
    let family_dir = dir.join("fixtures").join(family);
    if !family_dir.is_dir() {
        return vec![format!(
            "BLOCKED: missing fixtures directory for {family} at {}",
            family_dir.display()
        )];
    }
    let cases = load_manifest(&family_dir);
    let mut failures = Vec::new();
    for case in &cases {
        if let Err(err) = check_one(family, &family_dir, registry, case) {
            failures.push(err);
        }
    }
    failures
}

#[test]
fn registry_loads_from_live_repo() {
    let dir = platform_contracts_dir();
    assert!(
        dir.join("schemas").join("registry.json").is_file(),
        "BLOCKED: {} does not contain schemas/registry.json; \
         set PLATFORM_CONTRACTS_DIR to the platform-contracts repository root",
        dir.display()
    );
    let _registry = load_registry_or_fail(&dir);
}

#[test]
fn identity_subject_fixtures_match() {
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let failures = walk_family("identity-subject", &dir, &registry);
    assert!(
        failures.is_empty(),
        "identity-subject fixture mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn permission_decision_fixtures_match() {
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let failures = walk_family("permission-decision", &dir, &registry);
    assert!(
        failures.is_empty(),
        "permission-decision fixture mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn tenant_context_fixtures_match() {
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let failures = walk_family("tenant-context", &dir, &registry);
    assert!(
        failures.is_empty(),
        "tenant-context fixture mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn audit_event_fixtures_match() {
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let failures = walk_family("audit-event", &dir, &registry);
    assert!(
        failures.is_empty(),
        "audit-event fixture mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn readiness_fixtures_match() {
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let failures = walk_family("readiness", &dir, &registry);
    assert!(
        failures.is_empty(),
        "readiness fixture mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn gate_result_fixtures_match() {
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let failures = walk_family("gate-result", &dir, &registry);
    assert!(
        failures.is_empty(),
        "gate-result fixture mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn job_outcome_fixtures_match() {
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let failures = walk_family("job-outcome", &dir, &registry);
    assert!(
        failures.is_empty(),
        "job-outcome fixture mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn release_evidence_fixtures_match() {
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let failures = walk_family("release-evidence", &dir, &registry);
    assert!(
        failures.is_empty(),
        "release-evidence fixture mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn capability_fixtures_match() {
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let failures = walk_family("capability", &dir, &registry);
    assert!(
        failures.is_empty(),
        "capability fixture mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn readme_documented_families_all_covered() {
    // Smoke test: every family in the registry that has a fixtures
    // directory must be reachable. This catches registry drift.
    let dir = platform_contracts_dir();
    let registry = load_registry_or_fail(&dir);
    let fixtures_root = dir.join("fixtures");
    for family in registry.contract_ids() {
        let short = family.strip_prefix("platform.").unwrap_or(family);
        let family_dir = fixtures_root.join(short);
        assert!(
            family_dir.is_dir(),
            "BLOCKED: contract family {family} is registered but {} is missing",
            family_dir.display()
        );
    }
}
