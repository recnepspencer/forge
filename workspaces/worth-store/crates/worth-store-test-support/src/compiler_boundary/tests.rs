use std::path::{Path, PathBuf};

use super::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite, DependencyBoundaryPredicate,
    UiProofRunFailure,
};

#[test]
fn fixtures_in_one_environment_share_manifest_and_target_root() {
    let scratch = ScratchUiWorld::new("cache-sharing");
    let fixtures = [
        ("private_a.rs", ["private"].as_slice()),
        ("private_b.rs", ["private"].as_slice()),
    ];
    let evidence = run_cargo_ui_fixture_suite(
        scratch.root(),
        "cache-sharing",
        scratch.dependency_manifest(),
        "production",
        "diagnostic-test",
        scratch.fixture_root(),
        &fixtures,
    )
    .unwrap();

    assert_eq!(evidence.fixtures.len(), 2);
    assert!(evidence.environment_manifest_created);
    assert!(evidence
        .shared_target_root
        .contains(&evidence.environment_identity));
    assert!(evidence
        .fixtures
        .iter()
        .all(|fixture| fixture.fixture.environment_identity == evidence.environment_identity));
    assert!(evidence.fixtures[1].target_artifact_count_before > 0);
}

#[test]
fn unrelated_compile_failure_cannot_satisfy_declared_semantic_denial() {
    let scratch = ScratchUiWorld::new("wrong-reason");
    let denial = run_cargo_ui_fixture_suite(
        scratch.root(),
        "wrong-reason",
        scratch.dependency_manifest(),
        "production",
        "diagnostic-test",
        scratch.fixture_root(),
        &[("unresolved.rs", &["private"])],
    )
    .unwrap_err();

    assert!(matches!(
        denial,
        UiProofRunFailure::WrongCompilerDenial { .. }
    ));
    assert!(denial.to_string().contains("semantic fragment"));
}

#[test]
fn dependency_topology_predicates_are_not_disguised_as_compile_fixtures() {
    let predicate = DependencyBoundaryPredicate::ManifestDependencyDirection {
        source_package: "worth-store-authority".to_owned(),
        forbidden_dependency: "worth-store-certification".to_owned(),
    };
    let encoded = serde_json::to_string(&predicate).unwrap();

    assert!(encoded.contains("manifest_dependency_direction"));
    assert!(!encoded.contains("expected_compiler_denial"));
}

struct ScratchUiWorld {
    root: PathBuf,
    fixture_root: PathBuf,
}

impl ScratchUiWorld {
    fn new(label: &str) -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "worth-store-standard-ui-{label}-{}-{nonce}",
            std::process::id()
        ));
        let fixture_root = root.join("fixtures");
        std::fs::create_dir_all(root.join("dependency/src")).unwrap();
        std::fs::create_dir_all(&fixture_root).unwrap();
        std::fs::write(root.join("Cargo.lock"), "# UI harness identity lock\n").unwrap();
        std::fs::write(
            root.join("dependency/Cargo.toml"),
            "[package]\nname = \"sealed-dependency\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        std::fs::write(
            root.join("dependency/src/lib.rs"),
            "pub struct Sealed { value: u64 }\n",
        )
        .unwrap();
        for fixture in ["private_a.rs", "private_b.rs"] {
            std::fs::write(
                fixture_root.join(fixture),
                "use sealed_dependency::Sealed; fn main() { let _ = Sealed { value: 1 }; }\n",
            )
            .unwrap();
        }
        std::fs::write(
            fixture_root.join("unresolved.rs"),
            "use absent_dependency::Missing; fn main() {}\n",
        )
        .unwrap();
        Self { root, fixture_root }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn fixture_root(&self) -> &Path {
        &self.fixture_root
    }

    fn dependency_manifest(&self) -> String {
        cargo_dependency_manifest(
            &[(
                "sealed-dependency",
                self.root.join("dependency").as_path(),
                &[],
            )],
            &[],
        )
    }
}

impl Drop for ScratchUiWorld {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
