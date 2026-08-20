use crate::tests::support::*;

#[test]
fn publication_rejects_envelope_identity_drift_before_any_effect() {
    let mut runtime = runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "publication-identity");
    let receipt = commit.commit.clone();
    let envelope = commit.publication().envelope.clone();
    let identity = runtime
        .branch_identity(&BranchId("main".to_owned()))
        .expect("main identity");
    let binding = runtime
        .legacy_branch_binding_for_identity(&identity)
        .expect("main binding");
    let before_cells = runtime.history.branch_cells_snapshot();
    let before_catalog = runtime.history.commit_catalog.len();

    let mut mismatched_commit = envelope.as_ref().clone();
    mismatched_commit.commit.commit_id.0 += 100;
    let error = runtime
        .mvcc_publication_authority()
        .validate_versioned_publication(receipt.commit_id, &receipt, &binding, &mismatched_commit)
        .expect_err("envelope commit identity drift must be denied");
    assert!(error.contains("envelope commit identity mismatch"));
    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.commit_catalog.len(), before_catalog);

    let mut mismatched_branch = envelope.as_ref().clone();
    mismatched_branch.branch_context = BranchId("other".to_owned());
    let error = runtime
        .mvcc_publication_authority()
        .validate_versioned_publication(receipt.commit_id, &receipt, &binding, &mismatched_branch)
        .expect_err("envelope branch context drift must be denied");
    assert!(error.contains("envelope branch context mismatch"));
    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.commit_catalog.len(), before_catalog);
}
