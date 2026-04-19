use super::{
    model::CorrespondenceHistoryFailureClass,
    row_catalog::{
        CORRESPONDENCE_HISTORY_CANONICAL_ROW_SPECS, CORRESPONDENCE_HISTORY_REJECTION_ROW_SPECS,
        CORRESPONDENCE_HISTORY_REQUIRED_CANONICAL_ROW_NAMES,
        CORRESPONDENCE_HISTORY_REQUIRED_REJECTION_ROW_NAMES,
    },
    MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter,
};
use crate::harness::certification::{
    milestone_five_point_four_requirements, unmet_required_rows, HostileExpectation,
};
use std::collections::BTreeSet;

#[test]
fn correspondence_history_certification_adapter_emits_named_matrix() {
    let matrix = MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter::
        structural_correspondence_and_historical_materialization_path_test();

    assert_eq!(
        matrix.suite_name,
        "Structural Correspondence And Historical Materialization Path Test"
    );
    assert_eq!(
        matrix.rows.len(),
        CORRESPONDENCE_HISTORY_CANONICAL_ROW_SPECS.len()
    );
    assert_eq!(
        matrix.rejection_rows.len(),
        CORRESPONDENCE_HISTORY_REJECTION_ROW_SPECS.len()
    );
    let unique_rows = matrix
        .rows
        .iter()
        .map(|row| row.row_name)
        .collect::<BTreeSet<_>>();
    let unique_rejections = matrix
        .rejection_rows
        .iter()
        .map(|row| row.row_name)
        .collect::<BTreeSet<_>>();
    assert_eq!(unique_rows.len(), matrix.rows.len());
    assert_eq!(unique_rejections.len(), matrix.rejection_rows.len());
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
    assert_eq!(
        left.bundle_completeness_report.canonical_row_count,
        CORRESPONDENCE_HISTORY_CANONICAL_ROW_SPECS.len()
    );
    assert_eq!(
        left.bundle_completeness_report.rejection_row_count,
        CORRESPONDENCE_HISTORY_REJECTION_ROW_SPECS.len()
    );
    assert!(
        left.bundle_completeness_report
            .all_lanes_emit_required_outputs
    );
    assert_eq!(
        left.bundle_completeness_report.zero_rediscovery_lane_count,
        left.matrix.rows.len() * 3
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

    let correspondence_cost = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "correspondence-cost-posture-parity")
        .expect("correspondence cost row should exist");
    assert_ne!(
        correspondence_cost
            .control_lane
            .parity_bundle
            .correspondence_cost_posture_digest()
            .as_str(),
        correspondence_cost
            .hostile_lane
            .parity_bundle
            .correspondence_cost_posture_digest()
            .as_str()
    );

    let historical_cost = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "historical-cost-posture-parity")
        .expect("historical cost row should exist");
    assert_ne!(
        historical_cost
            .control_lane
            .parity_bundle
            .historical_cost_posture_digest()
            .expect("control posture")
            .as_str(),
        historical_cost
            .hostile_lane
            .parity_bundle
            .historical_cost_posture_digest()
            .expect("hostile posture")
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
    assert_eq!(
        compile_fail.hostile_lane.failure_digest,
        "compile_fail:structural-as-authoritative-forbidden"
    );
    assert_eq!(
        compile_fail.hostile_lane.compile_fail_case,
        Some("tests/ui/advisory_structural_unique_is_not_lineage_continuity.rs")
    );
    assert!(compile_fail.hostile_lane.counter_snapshot_digest.is_none());

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
    assert!(substitution.hostile_lane.counter_snapshot_digest.is_some());
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
    assert!(unsupported.hostile_lane.counter_snapshot_digest.is_some());
    assert!(unsupported.hostile_lane.compile_fail_case.is_none());

    let host_cache = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "host-cache-history-authority-forbidden")
        .expect("host cache authority row should exist");
    assert_eq!(
        host_cache.hostile_lane.failure_class,
        CorrespondenceHistoryFailureClass::HistoricalPathDenied
    );
    assert!(host_cache.hostile_lane.counter_snapshot_digest.is_some());
    assert!(host_cache.hostile_lane.compile_fail_case.is_none());

    let raw_ambiguity = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "raw-ambiguity-bool-forbidden")
        .expect("raw ambiguity row should exist");
    assert_eq!(
        raw_ambiguity.hostile_lane.failure_class,
        CorrespondenceHistoryFailureClass::CompileFail
    );
    assert_eq!(
        raw_ambiguity.hostile_lane.failure_digest,
        "compile_fail:raw-ambiguity-bool-forbidden"
    );
    assert_eq!(
        raw_ambiguity.hostile_lane.compile_fail_case,
        Some("tests/ui/raw_ambiguity_bool_forbidden.rs")
    );
    assert!(raw_ambiguity.hostile_lane.counter_snapshot_digest.is_none());

    let naked_historical = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "naked-historical-payload-forbidden")
        .expect("naked historical payload row should exist");
    assert_eq!(
        naked_historical.hostile_lane.failure_class,
        CorrespondenceHistoryFailureClass::CompileFail
    );
    assert_eq!(
        naked_historical.hostile_lane.failure_digest,
        "compile_fail:naked-historical-payload-forbidden"
    );
    assert_eq!(
        naked_historical.hostile_lane.compile_fail_case,
        Some("tests/ui/naked_historical_payload_forbidden.rs")
    );
    assert!(naked_historical
        .hostile_lane
        .counter_snapshot_digest
        .is_none());
}

#[test]
fn correspondence_history_artifact_digest_changes_when_required_rows_are_tampered() {
    let artifact = MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter::
        structural_correspondence_and_historical_materialization_path_artifact();
    let completeness = artifact.bundle_completeness_report.clone();
    let mut matrix = artifact.matrix.clone();
    let original_coverage_digest = artifact.coverage_matrix_digest;
    let original_bundle_digest = artifact.certification_bundle_digest;

    matrix.rows[0].row_name = "tampered-required-row";

    let mutated = matrix
        .clone()
        .into_milestone_five_point_four_artifact(completeness);
    let missing = unmet_required_rows(
        &matrix,
        CORRESPONDENCE_HISTORY_REQUIRED_CANONICAL_ROW_NAMES,
        CORRESPONDENCE_HISTORY_REQUIRED_REJECTION_ROW_NAMES,
    );

    assert!(missing.contains(&"lineage-correspondence-authoritative"));
    assert_ne!(mutated.coverage_matrix_digest, original_coverage_digest);
    assert_ne!(mutated.certification_bundle_digest, original_bundle_digest);
}
