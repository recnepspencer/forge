use std::path::{Path, PathBuf};

use worth_store_test_support::compiler_boundary::{
    run_ui_proof_suite, ExpectedCompilerDenial, UiFixtureDeclaration, UiProofEnvironment,
    UiProofSuiteDeclaration,
};

#[test]
fn physical_isolation_physical_isolation_entry_authority_cannot_be_forged_at_compile_time() {
    let root = store_workspace_root();
    let evidence = run_ui_proof_suite(root, &suite(root)).unwrap();
    assert_eq!(evidence.fixtures.len(), fixture_denials().len());
    assert!(evidence
        .fixtures
        .iter()
        .all(|fixture| fixture.semantic_denial_matched));
}

fn suite(root: &Path) -> UiProofSuiteDeclaration {
    let source_root = root.join(
        "crates/worth-store-certification/tests/compile_fail/physical_isolation/physical_isolation_entry",
    );
    let fixtures = fixture_denials()
        .into_iter()
        .map(|(name, fragments)| {
            UiFixtureDeclaration::new(
                name.trim_end_matches(".rs"),
                source_root.join(name),
                ExpectedCompilerDenial::semantic_fragments(fragments).unwrap(),
            )
            .unwrap()
        })
        .collect();
    UiProofSuiteDeclaration::new(
        "physical-isolation-entry-authority",
        UiProofEnvironment::cargo(dependency_manifest(root), "production", "diagnostic-test")
            .unwrap(),
        fixtures,
    )
    .unwrap()
}

fn fixture_denials() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        (
            "entry_request_cannot_be_struct_literal.rs",
            vec!["PhysicalIsolationEntryRequest", "private"],
        ),
        (
            "entry_admission_cannot_be_struct_literal.rs",
            vec!["PhysicalIsolationEntryAdmission", "private"],
        ),
        (
            "entry_identity_cannot_be_struct_literal.rs",
            vec!["PhysicalIsolationEntryIdentity", "private"],
        ),
        (
            "root_epoch_basis_cannot_be_struct_literal.rs",
            vec!["RootEpoch", "private"],
        ),
        (
            "copied_recovery_fields_cannot_admit_entry.rs",
            vec!["PhysicalIsolationEntryRequest"],
        ),
        (
            "semantic_snapshot_cannot_admit_entry.rs",
            vec!["PhysicalIsolationEntryRequest"],
        ),
        (
            "foundational_evidence_cannot_admit_entry.rs",
            vec!["PhysicalIsolationEntryRequest"],
        ),
        (
            "proof_progression_cannot_admit_entry.rs",
            vec!["PhysicalIsolationEntryRequest"],
        ),
        (
            "readiness_alone_cannot_register_lane.rs",
            vec!["PhysicalIsolationHarnessReadinessReceipt"],
        ),
        (
            "copied_rows_cannot_register_lane.rs",
            vec!["PhysicalIsolationHarnessReadinessReceipt"],
        ),
        (
            "entry_evidence_cannot_register_lane.rs",
            vec!["PhysicalIsolationHarnessReadinessReceipt"],
        ),
        (
            "lane_registration_cannot_be_struct_literal.rs",
            vec!["PhysicalIsolationCertificationLaneRegistration", "private"],
        ),
    ]
}

fn dependency_manifest(root: &Path) -> String {
    format!(
        "[dependencies]\nworth-store-physical-certification = {{ path = \"{}\" }}\nworth-store-physical-isolation = {{ path = \"{}\" }}\nworth-store-readiness = {{ path = \"{}\" }}\n",
        manifest_path(&root.join("crates/worth-store-physical-certification")),
        manifest_path(&root.join("crates/worth-store-physical-isolation")),
        manifest_path(&root.join("crates/worth-store-readiness")),
    )
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}

fn manifest_path(path: &PathBuf) -> String {
    path.display().to_string().replace('\\', "/")
}
