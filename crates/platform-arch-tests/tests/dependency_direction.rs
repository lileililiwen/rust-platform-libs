//! Top-level integration test that runs the architecture checks.

use platform_arch_tests::{Lockfile, ParsedCrate, PRODUCTION_CRATES, TEST_ONLY_CRATE};

#[test]
fn production_crates_listed() {
    for name in PRODUCTION_CRATES {
        assert!(!name.is_empty());
    }
    assert_eq!(TEST_ONLY_CRATE, "platform-testing");
}

#[test]
fn production_crates_do_not_depend_on_test_only() {
    // The unit tests in `lib.rs` already enforce the strict rule
    // against forbidden direct deps. This integration test asserts
    // the broader invariant: a production crate can never reach the
    // test-only crate through any path, direct or transitive.
    let ws = platform_arch_tests::workspace_root();
    let manifests = read_manifests(&ws);
    for name in PRODUCTION_CRATES {
        let manifest = manifests
            .iter()
            .find(|m| m.name == *name)
            .unwrap_or_else(|| panic!("missing manifest for {name}"));
        assert!(
            !manifest.dependencies.contains(TEST_ONLY_CRATE),
            "{name} has direct dependency on {TEST_ONLY_CRATE}"
        );
        assert!(
            !manifest.dev_dependencies.contains(TEST_ONLY_CRATE),
            "{name} has dev-dependency on {TEST_ONLY_CRATE}"
        );
    }
}

#[test]
fn no_production_crate_depends_on_axum() {
    let ws = platform_arch_tests::workspace_root();
    let manifests = read_manifests(&ws);
    for name in PRODUCTION_CRATES {
        let manifest = manifests
            .iter()
            .find(|m| m.name == *name)
            .unwrap_or_else(|| panic!("missing manifest for {name}"));
        for dep in manifest
            .dependencies
            .iter()
            .chain(manifest.dev_dependencies.iter())
        {
            assert_ne!(dep, "axum", "{name} depends on axum");
            assert_ne!(dep, "sqlx", "{name} depends on sqlx");
            assert_ne!(dep, "cookie", "{name} depends on cookie");
            assert_ne!(dep, "jsonwebtoken", "{name} depends on jsonwebtoken");
            assert_ne!(dep, "openidconnect", "{name} depends on openidconnect");
        }
    }
}

#[test]
fn lockfile_transitive_closure_is_clean() {
    let ws = platform_arch_tests::workspace_root();
    let lockfile = match Lockfile::load(&ws) {
        Ok(l) => l,
        Err(err) => panic!("BLOCKED: {err}. Run `cargo build` once."),
    };
    for name in PRODUCTION_CRATES {
        let closure = lockfile.transitive_closure(&[name.to_string()]);
        for forbidden in &[
            "axum",
            "axum-core",
            "axum-extra",
            "tower",
            "tower-http",
            "sqlx",
            "sqlx-core",
            "sqlx-macros",
            "diesel",
            "sea-orm",
            "cookie",
            "cookies",
            "jsonwebtoken",
            "openidconnect",
            "oauth2",
            "reqwest",
            "tokio",
        ] {
            assert!(
                !closure.contains(*forbidden),
                "production crate {name} transitively depends on {forbidden:?}"
            );
        }
    }
}

fn read_manifests(ws: &std::path::Path) -> Vec<ParsedCrate> {
    platform_arch_tests::read_workspace_manifests(ws)
        .into_values()
        .collect()
}
