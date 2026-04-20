use super::matrix::MilestoneSixHistoricalDiffCertificationAdapter;
use crate::harness::certification::{
    covered_perturbation_classes, milestone_six_requirements, unmet_required_assertion_classes,
    unmet_required_rows, RequiredAssertionClass,
};

#[test]
fn historical_diff_matrix_covers_milestone_six_rows() {
    let matrix =
        MilestoneSixHistoricalDiffCertificationAdapter::branch_scoped_historical_and_diff_query_context_test();
    let requirements = milestone_six_requirements();

    let missing_rows = unmet_required_rows(
        &matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );

    assert!(
        missing_rows.is_empty(),
        "missing historical diff certification rows: {missing_rows:?}"
    );
}

#[test]
fn historical_diff_matrix_covers_required_assertion_classes() {
    let covered = [
        RequiredAssertionClass::Equality,
        RequiredAssertionClass::Inequality,
        RequiredAssertionClass::TypedFailure,
        RequiredAssertionClass::ZeroResidue,
    ];
    let missing = unmet_required_assertion_classes(
        &covered,
        milestone_six_requirements().required_assertion_classes,
    );

    assert!(
        missing.is_empty(),
        "missing historical diff assertion classes: {missing:?}"
    );
}

#[test]
fn historical_diff_matrix_covers_multiple_perturbation_classes() {
    let matrix =
        MilestoneSixHistoricalDiffCertificationAdapter::branch_scoped_historical_and_diff_query_context_test();
    let covered = covered_perturbation_classes(&matrix);

    assert!(
        covered.len() >= 4,
        "expected broad perturbation coverage, got {covered:?}"
    );
}

#[test]
fn historical_diff_lanes_emit_required_verification_artifacts() {
    let matrix =
        MilestoneSixHistoricalDiffCertificationAdapter::branch_scoped_historical_and_diff_query_context_test();

    for row in &matrix.rows {
        for lane in [&row.control_lane, &row.hostile_lane, &row.parity_lane] {
            assert!(
                !lane.query_digest.is_empty(),
                "query digest must be present"
            );
            assert!(
                !lane.basis_digest.is_empty(),
                "basis digest must be present"
            );
            assert!(
                !lane.result_digest.is_empty(),
                "result digest must be present"
            );
            assert!(
                !lane.replay_digest.is_empty(),
                "replay digest must be present"
            );
            assert!(
                !lane.counter_snapshot_digest.is_empty(),
                "counter snapshot digest must be present"
            );
            assert!(
                !lane.exact_counter_values.is_empty(),
                "exact counter values must be present"
            );
            assert!(
                !lane.prediction_drift_outcome.is_empty(),
                "prediction drift outcome must be present"
            );
        }
    }
}

#[test]
fn historical_diff_branch_to_branch_lane_emits_exact_phase_three_posture() {
    let matrix =
        MilestoneSixHistoricalDiffCertificationAdapter::branch_scoped_historical_and_diff_query_context_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "branch-to-branch-diff-shaped")
        .expect("branch-to-branch diff row should exist");

    assert_eq!(
        row.hostile_lane.comparison_family, "branch_to_branch",
        "branch-to-branch diff must advertise the exact comparison family"
    );
    assert_eq!(
        row.hostile_lane.cost_class, "diff_comparison_bounded",
        "branch-to-branch diff must advertise diff cost posture"
    );
    assert_eq!(
        row.hostile_lane.budget_class, "comparison_bounded",
        "branch-to-branch diff must advertise diff budget posture"
    );
    assert_eq!(
        row.hostile_lane.prediction_drift_outcome, "within_budget",
        "branch-to-branch diff shaping must report realized prediction posture"
    );
    assert!(row
        .hostile_lane
        .exact_counter_values
        .iter()
        .any(|value| value == "comparison_lookups:1"));
    assert!(row
        .hostile_lane
        .exact_counter_values
        .iter()
        .any(|value| value == "comparison_broadening_denials:0"));
    assert!(row
        .hostile_lane
        .exact_counter_values
        .iter()
        .any(|value| value == "comparison_family_rediscovery:0"));
}

#[test]
fn historical_diff_current_to_historical_lane_emits_exact_comparison_family() {
    let matrix =
        MilestoneSixHistoricalDiffCertificationAdapter::branch_scoped_historical_and_diff_query_context_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "current-to-historical-diff-shaped")
        .expect("current-to-historical diff row should exist");

    assert_eq!(row.hostile_lane.comparison_family, "current_to_historical");
    assert_eq!(row.hostile_lane.prediction_drift_outcome, "within_budget");
    assert!(row
        .hostile_lane
        .exact_counter_values
        .iter()
        .any(|value| value == "comparison_lookups:1"));
}

#[test]
fn historical_diff_rejection_rows_emit_exact_phase_three_failure_counters() {
    let matrix =
        MilestoneSixHistoricalDiffCertificationAdapter::branch_scoped_historical_and_diff_query_context_test();

    let broadening = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "broadening-required-comparison-denial")
        .expect("broadening rejection row should exist");
    assert_eq!(
        broadening.hostile_lane.failure_class,
        crate::harness::historical_diff_certification::HistoricalDiffFailureClass::ComparisonBroadeningRequired
    );
    assert!(broadening
        .hostile_lane
        .exact_counter_values
        .iter()
        .any(|value| value == "comparison_broadening_denials:1"));

    let shape_mismatch = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "declared-result-shape-mismatch")
        .expect("shape mismatch rejection row should exist");
    assert_eq!(
        shape_mismatch.hostile_lane.failure_class,
        crate::harness::historical_diff_certification::HistoricalDiffFailureClass::ComparisonShapeMismatch
    );
    assert!(shape_mismatch
        .hostile_lane
        .exact_counter_values
        .iter()
        .any(|value| value == "comparison_broadening_denials:0"));
}
