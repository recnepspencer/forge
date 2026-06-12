use super::artifact_fixture_support::{
    duplicate_import_identity_seeded_input, import_only_identity_seeded_input,
};

#[test]
fn canonical_artifact_discards_pre_artifact_provenance() {
    let left = crate::source::WorthUiCanonicalArtifactAssembler::assemble(
        &import_only_identity_seeded_input(0),
    )
    .expect("phase 8 artifact assembly should succeed");
    let right = crate::source::WorthUiCanonicalArtifactAssembler::assemble(
        &import_only_identity_seeded_input(99),
    )
    .expect("phase 8 artifact assembly should succeed");

    assert!(left.equivalent_shape(&right));
    assert_eq!(left, right);
}

#[test]
fn duplicate_canonical_node_keys_report_deterministically() {
    let report = crate::source::WorthUiCanonicalArtifactAssembler::assemble_with_metrics(
        &duplicate_import_identity_seeded_input(),
    )
    .expect_err("duplicate canonical import keys should fail assembly");

    assert_eq!(report.metrics().modules_assembled(), 1);
    assert_eq!(report.metrics().nodes_assembled(), 2);
    assert_eq!(report.metrics().modules_with_reordered_nodes(), 0);
    assert_eq!(report.diagnostics().len(), 1);

    let diagnostic = &report.diagnostics()[0];
    assert_eq!(
        diagnostic.code(),
        crate::source::WorthUiArtifactAssemblyDiagnosticCode::DuplicateCanonicalArtifactNodeKey
    );
    assert_eq!(diagnostic.module_id().as_str(), "app/main.wui");
    assert_eq!(diagnostic.semantic_locus(), "import:app/shared.wui");
    assert_eq!(
        diagnostic.key_text(),
        "import:app/shared.wui:module:app/main.wui|import:app/shared.wui"
    );
}
