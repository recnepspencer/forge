use std::path::Path;

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
    vec![(
        "copied_recovery_fields_cannot_admit_entry.rs",
        vec!["PhysicalIsolationEntryRequest"],
    )]
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

fn manifest_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}
