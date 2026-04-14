use super::MilestoneTwoValidationCertificationAdapter;

#[test]
fn schema_aware_rejection_and_projection_legality_adapter_emits_named_matrix() {
    let matrix =
        MilestoneTwoValidationCertificationAdapter::schema_aware_rejection_and_projection_legality_test();

    assert_eq!(
        matrix.suite_name,
        "Schema-Aware Rejection And Projection Legality Test"
    );
    assert!(matrix.rows.iter().any(|row| row.row_name == "legal-detail-query-parity"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "unknown-aspect-projection"));
    assert!(matrix.rows.iter().any(|row| row.row_name == "ordering-only-authority-boundary"));
    assert!(matrix.rows.iter().any(|row| row.row_name == "integer-greater-than-predicate-parity"));
    assert!(matrix.rows.iter().any(|row| row.row_name == "integer-less-than-predicate-parity"));
    assert!(matrix.rows.iter().any(|row| row.row_name == "redundant-greater-than-normalization"));
    assert!(matrix.rows.iter().any(|row| row.row_name == "bounded-range-normalization"));
    assert!(matrix.rows.iter().any(|row| row.row_name == "text-contains-predicate-parity"));
    assert!(matrix.rows.iter().any(|row| row.row_name == "scalar-membership-predicate-parity"));
    assert!(matrix.rows.iter().any(|row| row.row_name == "membership-intersection-normalization"));
    assert!(matrix.rows.iter().any(|row| row.row_name == "presence-predicate-parity"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "non-orderable-ordering-field"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "predicate-contradiction-rejection"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "empty-range-rejection"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "text-predicate-capability-rejection"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "membership-capability-rejection"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "presence-capability-rejection"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "incompatible-predicate-family"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "invalid-result-shape-binding"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "structured-content-illegality"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "workflow-context-illegality"));
    assert!(matrix.rejection_rows.iter().any(|row| row.row_name == "forbidden-widening-case"));
}

#[test]
fn schema_aware_rejection_and_projection_legality_artifact_is_offline_ready_for_current_scope() {
    let artifact = MilestoneTwoValidationCertificationAdapter::
        schema_aware_rejection_and_projection_legality_certification_artifact();

    assert_eq!(
        artifact.suite_name,
        "Schema-Aware Rejection And Projection Legality Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
    assert!(
        artifact
            .bundle_completeness_report
            .covers_all_currently_implemented_normative_scenarios
    );
    assert!(
        artifact
            .bundle_completeness_report
            .covers_full_milestone_two_spec_matrix
    );
    assert!(artifact.bundle_completeness_report.offline_analysis_ready);
}

#[test]
fn schema_aware_rejection_and_projection_legality_artifact_is_deterministic() {
    let left = MilestoneTwoValidationCertificationAdapter::
        schema_aware_rejection_and_projection_legality_certification_artifact();
    let right = MilestoneTwoValidationCertificationAdapter::
        schema_aware_rejection_and_projection_legality_certification_artifact();

    assert_eq!(left.certification_bundle_digest, right.certification_bundle_digest);
    assert_eq!(left.coverage_matrix_digest, right.coverage_matrix_digest);
    assert_eq!(left.bundle_completeness_report, right.bundle_completeness_report);
    assert_eq!(left.counter_snapshot, right.counter_snapshot);
}
