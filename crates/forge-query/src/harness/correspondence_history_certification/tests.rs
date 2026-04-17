use super::{
    row_catalog::{
        CORRESPONDENCE_HISTORY_CANONICAL_ROW_SPECS, CORRESPONDENCE_HISTORY_REJECTION_ROW_SPECS,
        CORRESPONDENCE_HISTORY_REQUIRED_CANONICAL_ROW_NAMES,
        CORRESPONDENCE_HISTORY_REQUIRED_REJECTION_ROW_NAMES,
    },
    CorrespondenceHistoryFailureClass,
    MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter,
};
use crate::harness::certification::{
    milestone_five_point_four_requirements, unmet_required_rows, HostileExpectation,
};

#[test]
fn correspondence_history_certification_adapter_emits_named_matrix() {
    let matrix = MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter::
        structural_correspondence_and_historical_materialization_path_test();

    assert_eq!(
        matrix.suite_name,
        "Structural Correspondence And Historical Materialization Path Test"
    );
    for spec in CORRESPONDENCE_HISTORY_CANONICAL_ROW_SPECS {
        assert!(matrix.rows.iter().any(|row| row.row_name == spec.row_name));
    }
    for spec in CORRESPONDENCE_HISTORY_REJECTION_ROW_SPECS {
        assert!(matrix
            .rejection_rows
            .iter()
            .any(|row| row.row_name == spec.row_name));
    }
}

#[test]
fn correspondence_history_certification_matrix_meets_milestone_requirements() {
    let matrix = MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter::
        structural_correspondence_and_historical_materialization_path_test();
    let requirements = milestone_five_point_four_requirements();

    let missing = unmet_required_rows(
        &matrix,
        CORRESPONDENCE_HISTORY_REQUIRED_CANONICAL_ROW_NAMES,
        CORRESPONDENCE_HISTORY_REQUIRED_REJECTION_ROW_NAMES,
    );
    assert!(missing.is_empty(), "missing 5.4 rows: {missing:?}");

    let spec_missing = unmet_required_rows(
        &matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );
    assert!(
        spec_missing.is_empty(),
        "missing spec 5.4 rows: {spec_missing:?}"
    );

    assert!(matrix
        .rows
        .iter()
        .all(|row| row.control_lane.has_required_outputs()));
    assert!(matrix
        .rows
        .iter()
        .all(|row| row.hostile_lane.has_required_outputs()));
    assert!(matrix
        .rows
        .iter()
        .all(|row| row.parity_lane.has_required_outputs()));
    assert!(matrix
        .rejection_rows
        .iter()
        .all(|row| row.hostile_lane.has_required_outputs()));
}

#[test]
fn correspondence_history_artifact_is_deterministic_and_offline_ready() {
    let left = MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter::
        structural_correspondence_and_historical_materialization_path_artifact();
    let right = MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter::
        structural_correspondence_and_historical_materialization_path_artifact();

    assert_eq!(
        left.certification_bundle_digest,
        right.certification_bundle_digest
    );
    assert_eq!(left.coverage_matrix_digest, right.coverage_matrix_digest);
    assert!(
        left.bundle_completeness_report
            .all_lanes_emit_required_outputs
    );
    assert!(left
        .bundle_completeness_report
        .unmet_required_rows
        .is_empty());
    assert!(left
        .bundle_completeness_report
        .unmet_required_assertion_classes
        .is_empty());
    assert!(left.bundle_completeness_report.offline_analysis_ready);
}

#[test]
fn correspondence_history_rows_preserve_expected_boundaries() {
    let matrix = MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter::
        structural_correspondence_and_historical_materialization_path_test();

    let structural = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "structural-correspondence-advisory")
        .expect("structural advisory row should exist");
    assert_eq!(
        structural.hostile_expectation,
        HostileExpectation::DistinctFromControl
    );
    assert_eq!(
        structural
            .control_lane
            .parity_bundle
            .parity_variant()
            .as_str(),
        "success"
    );
    assert_ne!(
        structural
            .control_lane
            .parity_bundle
            .correspondence_outcome_digest()
            .as_str(),
        structural
            .hostile_lane
            .parity_bundle
            .correspondence_outcome_digest()
            .as_str()
    );

    let replay = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "historical-delta-replay-path")
        .expect("replay row should exist");
    assert_ne!(
        replay
            .control_lane
            .parity_bundle
            .resolved_path_digest()
            .expect("control path")
            .as_str(),
        replay
            .hostile_lane
            .parity_bundle
            .resolved_path_digest()
            .expect("hostile path")
            .as_str()
    );

    let drift = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "prediction-drift-explicit")
        .expect("drift row should exist");
    assert_ne!(
        drift
            .control_lane
            .parity_bundle
            .performance_prediction_drift_outcome()
            .as_str(),
        "within_budget"
    );
}

#[test]
fn correspondence_history_rejections_bind_typed_failures() {
    let matrix = MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter::
        structural_correspondence_and_historical_materialization_path_test();

    let compile_fail = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "structural-as-authoritative-forbidden")
        .expect("compile fail row should exist");
    assert_eq!(
        compile_fail.hostile_lane.failure_class,
        CorrespondenceHistoryFailureClass::CompileFail
    );
    assert!(compile_fail.hostile_lane.compile_fail_case.is_some());

    let substitution = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "hidden-materialization-path-substitution-forbidden")
        .expect("substitution row should exist");
    assert_eq!(
        substitution.hostile_lane.failure_class,
        CorrespondenceHistoryFailureClass::HistoricalPathDenied
    );
    assert!(!substitution.hostile_lane.failure_digest.is_empty());
    assert!(substitution.hostile_lane.compile_fail_case.is_none());

    let unsupported = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "unsupported-correspondence-family")
        .expect("unsupported correspondence row should exist");
    assert_eq!(
        unsupported.hostile_lane.failure_class,
        CorrespondenceHistoryFailureClass::CorrespondenceDenied
    );

    let host_cache = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "host-cache-history-authority-forbidden")
        .expect("host cache authority row should exist");
    assert_eq!(
        host_cache.hostile_lane.failure_class,
        CorrespondenceHistoryFailureClass::HistoricalPathDenied
    );
}
