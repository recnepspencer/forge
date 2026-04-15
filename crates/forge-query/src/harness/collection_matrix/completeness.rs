use super::super::certification::{
    RequiredAssertionClass, covered_perturbation_classes, milestone_four_requirements,
    unmet_required_assertion_classes, unmet_required_rows,
};
use super::model::{
    CollectionBundleCompletenessReport, CollectionCertificationMatrix, CollectionCertificationRow,
    CollectionHostileExpectation, CollectionRejectionRow,
};

pub(super) fn bundle_completeness_report(
    matrix: &CollectionCertificationMatrix,
) -> CollectionBundleCompletenessReport {
    let supported_lane_count = (matrix.rows.len() * 3) + (matrix.rejection_rows.len() * 2);
    let successful_lane_count = supported_lane_count;
    let zero_fallback_lane_count = matrix
        .rows
        .iter()
        .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
        .chain(
            matrix
                .rejection_rows
                .iter()
                .flat_map(|row| [&row.control_lane, &row.parity_lane]),
        )
        .filter(|lane| lane.counter_snapshot.execution_fallback_taken_count() == 0)
        .count();
    let zero_rediscovery_lane_count = matrix
        .rows
        .iter()
        .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
        .chain(
            matrix
                .rejection_rows
                .iter()
                .flat_map(|row| [&row.control_lane, &row.parity_lane]),
        )
        .filter(|lane| lane.counter_snapshot.executor_semantic_rediscovery_count() == 0)
        .count();
    let all_lanes_emit_required_outputs = matrix
        .rows
        .iter()
        .all(CollectionCertificationRow::has_required_outputs)
        && matrix
            .rejection_rows
            .iter()
            .all(CollectionRejectionRow::has_required_outputs);
    let all_rows_have_hostile_coverage = matrix
        .rows
        .iter()
        .all(CollectionCertificationRow::has_hostile_coverage)
        && matrix
            .rejection_rows
            .iter()
            .all(CollectionRejectionRow::has_hostile_coverage);
    let covered_perturbation_classes = covered_perturbation_classes(matrix);
    let requirements = milestone_four_requirements();
    let unmet_required_rows = unmet_required_rows(
        matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );
    let covered_assertion_classes = covered_assertion_classes(
        matrix,
        zero_fallback_lane_count,
        zero_rediscovery_lane_count,
        supported_lane_count,
    );
    let unmet_required_assertion_classes = unmet_required_assertion_classes(
        &covered_assertion_classes,
        requirements.required_assertion_classes,
    );
    let covers_all_currently_implemented_normative_scenarios =
        covers_all_currently_implemented_normative_scenarios(matrix);
    let covers_full_milestone_four_spec_matrix =
        covers_all_currently_implemented_normative_scenarios
            && unmet_required_rows.is_empty()
            && unmet_required_assertion_classes.is_empty();
    let offline_analysis_ready = all_lanes_emit_required_outputs
        && all_rows_have_hostile_coverage
        && zero_fallback_lane_count == supported_lane_count
        && zero_rediscovery_lane_count == supported_lane_count
        && covers_full_milestone_four_spec_matrix;

    CollectionBundleCompletenessReport {
        canonical_row_count: matrix.rows.len(),
        rejection_row_count: matrix.rejection_rows.len(),
        supported_lane_count,
        successful_lane_count,
        zero_fallback_lane_count,
        zero_rediscovery_lane_count,
        covered_perturbation_classes,
        all_lanes_emit_required_outputs,
        all_rows_have_hostile_coverage,
        unmet_required_rows,
        unmet_required_assertion_classes,
        covers_all_currently_implemented_normative_scenarios,
        covers_full_milestone_four_spec_matrix,
        offline_analysis_ready,
    }
}

fn contains_row(matrix: &CollectionCertificationMatrix, row_name: &str) -> bool {
    matrix.rows.iter().any(|row| row.row_name == row_name)
        || matrix
            .rejection_rows
            .iter()
            .any(|row| row.row_name == row_name)
}

fn covers_all_currently_implemented_normative_scenarios(
    matrix: &CollectionCertificationMatrix,
) -> bool {
    contains_row(matrix, "ordered-collection-parity")
        && contains_row(matrix, "cursor-advance-repeatability")
        && contains_row(matrix, "bounded-traversal-parity")
        && contains_row(matrix, "aggregate-rollup-parity")
        && contains_row(matrix, "derived-field-parity")
        && contains_row(matrix, "cdc-shaped-result-parity")
        && contains_row(matrix, "unsupported-ordering-family")
        && contains_row(matrix, "unstable-cursor-shape")
        && contains_row(matrix, "unsupported-traversal-bound")
        && contains_row(matrix, "unsupported-aggregate-family")
        && contains_row(matrix, "unsupported-cdc-result-family")
}

fn covered_assertion_classes(
    matrix: &CollectionCertificationMatrix,
    zero_fallback_lane_count: usize,
    zero_rediscovery_lane_count: usize,
    supported_lane_count: usize,
) -> Vec<RequiredAssertionClass> {
    let mut covered = Vec::new();
    if matrix
        .rows
        .iter()
        .any(|row| row.hostile_expectation == CollectionHostileExpectation::EquivalentToControl)
    {
        covered.push(RequiredAssertionClass::Equality);
    }
    if matrix
        .rows
        .iter()
        .any(|row| row.hostile_expectation == CollectionHostileExpectation::DistinctFromControl)
    {
        covered.push(RequiredAssertionClass::Inequality);
    }
    if !matrix.rejection_rows.is_empty() {
        covered.push(RequiredAssertionClass::TypedFailure);
    }
    if zero_fallback_lane_count == supported_lane_count
        && zero_rediscovery_lane_count == supported_lane_count
    {
        covered.push(RequiredAssertionClass::ZeroResidue);
    }
    covered
}
