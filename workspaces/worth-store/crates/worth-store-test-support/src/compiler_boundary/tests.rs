use std::path::{Path, PathBuf};

use super::{cargo_dependency_manifest, run_cargo_ui_fixture_suite, UiProofRunFailure};

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

    evidence.validate_integrity().unwrap();
    assert_eq!(evidence.fixtures.len(), 2);
    assert!(evidence.environment_manifest_created);
    assert!(evidence.environment_lock_created);
    assert!(evidence.environment_lock_path.ends_with("/Cargo.lock"));
    assert_eq!(evidence.environment_lock_sha256.len(), 64);
    assert!(evidence
        .shared_target_root
        .contains(&evidence.environment_identity));
    assert!(evidence
        .fixtures
        .iter()
        .all(|fixture| fixture.fixture.environment_identity == evidence.environment_identity));
    assert!(evidence.fixtures[0].dependency_artifacts_compiled > 0);
    assert_eq!(evidence.fixtures[1].dependency_artifacts_compiled, 0);
    assert!(evidence.fixtures[1].dependency_artifacts_reused > 0);
    assert_eq!(
        evidence.fixtures[0].target_artifact_count_after,
        evidence.fixtures[1].target_artifact_count_before
    );
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
fn fixture_source_text_cannot_impersonate_a_semantic_compiler_message() {
    let expected = super::ExpectedCompilerDenial::semantic_fragments(["private"]).unwrap();
    let diagnostics = vec![super::CheckedCompilerDiagnostic {
        level: "error".to_owned(),
        code: Some("E0432".to_owned()),
        message: "unresolved import `absent_dependency`".to_owned(),
        rendered: "use absent_dependency::private;".to_owned(),
    }];

    let denial = super::diagnostics::validate_denial(&expected, &diagnostics, "").unwrap_err();

    assert!(denial.contains("missed semantic fragment"));
}

#[test]
fn evidence_root_parent_traversal_is_rejected_before_publication() {
    let scratch = ScratchUiWorld::new("evidence-root-escape");
    let admitted = scratch.root().join(".store-proof/evidence");
    std::fs::create_dir_all(&admitted).unwrap();
    let admitted = admitted.canonicalize().unwrap();
    let escaped = admitted.join("ui/../../outside");

    let denial = super::artifact_store::admit_declared_root(&admitted, &escaped).unwrap_err();

    assert!(denial.to_string().contains("parent traversal"));
    assert!(!scratch.root().join(".store-proof/outside").exists());
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
