use super::MilestoneFourCollectionCertificationAdapter;

#[test]
fn collection_certification_adapter_emits_named_matrix() {
    let matrix =
        MilestoneFourCollectionCertificationAdapter::collection_cursor_rollup_and_cdc_shape_test();

    assert_eq!(
        matrix.suite_name,
        "Collection, Cursor, Rollup, And CDC Shape Parity Test"
    );
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "ordered-collection-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "cursor-advance-repeatability"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "bounded-traversal-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "aggregate-rollup-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "derived-field-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "cdc-shaped-result-parity"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "unsupported-ordering-family"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "unstable-cursor-shape"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "unsupported-traversal-bound"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "unsupported-aggregate-family"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "unsupported-cdc-result-family"));
}

#[test]
fn collection_certification_artifact_is_honest_about_current_gaps() {
    let artifact = MilestoneFourCollectionCertificationAdapter::
        collection_cursor_rollup_and_cdc_shape_certification_artifact();

    assert_eq!(
        artifact.suite_name,
        "Collection, Cursor, Rollup, And CDC Shape Parity Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
    assert!(artifact.bundle_completeness_report.unmet_required_rows.is_empty());
    assert!(artifact
        .bundle_completeness_report
        .unmet_required_assertion_classes
        .is_empty());
    assert!(artifact.bundle_completeness_report.covers_full_milestone_four_spec_matrix);
    assert!(artifact.bundle_completeness_report.offline_analysis_ready);
}

#[test]
fn collection_certification_artifact_is_deterministic() {
    let left = MilestoneFourCollectionCertificationAdapter::
        collection_cursor_rollup_and_cdc_shape_certification_artifact();
    let right = MilestoneFourCollectionCertificationAdapter::
        collection_cursor_rollup_and_cdc_shape_certification_artifact();

    assert_eq!(left.certification_bundle_digest, right.certification_bundle_digest);
    assert_eq!(left.coverage_matrix_digest, right.coverage_matrix_digest);
    assert_eq!(left.bundle_completeness_report, right.bundle_completeness_report);
    assert_eq!(left.counter_snapshot, right.counter_snapshot);
}
