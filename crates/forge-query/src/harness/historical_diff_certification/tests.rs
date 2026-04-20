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
                !lane.prediction_drift_outcome.is_empty(),
                "prediction drift outcome must be present"
            );
        }
    }
}
