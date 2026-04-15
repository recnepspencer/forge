use super::super::certification::{
    RequiredAssertionClass, covered_perturbation_classes, milestone_two_requirements,
    unmet_required_assertion_classes, unmet_required_rows,
};
use super::model::{
    ValidationBundleCompletenessReport, ValidationCertificationMatrix, ValidationCertificationRow,
    ValidationRejectionCertificationRow,
};

pub(crate) fn bundle_completeness_report(
    matrix: &ValidationCertificationMatrix,
) -> ValidationBundleCompletenessReport {
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
        .filter(|lane| lane.counter_snapshot.validation_fallback_count() == 0)
        .count();
    let all_lanes_emit_required_outputs = matrix
        .rows
        .iter()
        .all(ValidationCertificationRow::has_required_outputs)
        && matrix
            .rejection_rows
            .iter()
            .all(ValidationRejectionCertificationRow::has_required_outputs);
    let all_rows_have_hostile_coverage = matrix
        .rows
        .iter()
        .all(ValidationCertificationRow::has_hostile_coverage)
        && matrix
            .rejection_rows
            .iter()
            .all(ValidationRejectionCertificationRow::has_hostile_coverage);
    let covered_perturbation_classes = covered_perturbation_classes(matrix);
    let requirements = milestone_two_requirements();
    let unmet_required_rows = unmet_required_rows(
        matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );
    let covered_assertion_classes =
        covered_assertion_classes(matrix, zero_fallback_lane_count, supported_lane_count);
    let unmet_required_assertion_classes = unmet_required_assertion_classes(
        &covered_assertion_classes,
        requirements.required_assertion_classes,
    );
    let covers_all_currently_implemented_normative_scenarios =
        covers_all_currently_implemented_normative_scenarios(matrix);
    let covers_full_milestone_two_spec_matrix =
        covers_all_currently_implemented_normative_scenarios
            && unmet_required_rows.is_empty()
            && unmet_required_assertion_classes.is_empty();
    let offline_analysis_ready = all_lanes_emit_required_outputs
        && all_rows_have_hostile_coverage
        && zero_fallback_lane_count == supported_lane_count
        && covers_all_currently_implemented_normative_scenarios
        && unmet_required_assertion_classes.is_empty();

    ValidationBundleCompletenessReport {
        canonical_row_count: matrix.rows.len(),
        rejection_row_count: matrix.rejection_rows.len(),
        supported_lane_count,
        successful_lane_count,
        zero_fallback_lane_count,
        covered_perturbation_classes,
        all_lanes_emit_required_outputs,
        all_rows_have_hostile_coverage,
        unmet_required_rows,
        unmet_required_assertion_classes,
        covers_all_currently_implemented_normative_scenarios,
        covers_full_milestone_two_spec_matrix,
        offline_analysis_ready,
    }
}

fn contains_row(matrix: &ValidationCertificationMatrix, row_name: &str) -> bool {
    matrix.rows.iter().any(|row| row.row_name == row_name)
        || matrix
            .rejection_rows
            .iter()
            .any(|row| row.row_name == row_name)
}

fn covers_all_currently_implemented_normative_scenarios(
    matrix: &ValidationCertificationMatrix,
) -> bool {
    contains_row(matrix, "legal-detail-query-parity")
        && contains_row(matrix, "equivalent-builder-composed-legal-query")
        && contains_row(matrix, "unknown-aspect-projection")
        && contains_row(matrix, "ordering-only-authority-boundary")
        && contains_row(matrix, "non-orderable-ordering-field")
        && contains_row(matrix, "integer-greater-than-predicate-parity")
        && contains_row(matrix, "integer-less-than-predicate-parity")
        && contains_row(matrix, "scalar-membership-predicate-parity")
        && contains_row(matrix, "membership-intersection-normalization")
        && contains_row(matrix, "presence-predicate-parity")
        && contains_row(matrix, "bounded-range-normalization")
        && contains_row(matrix, "text-contains-predicate-parity")
        && contains_row(matrix, "predicate-contradiction-rejection")
        && contains_row(matrix, "membership-capability-rejection")
        && contains_row(matrix, "presence-capability-rejection")
        && contains_row(matrix, "empty-range-rejection")
        && contains_row(matrix, "text-predicate-capability-rejection")
        && contains_row(matrix, "incompatible-predicate-family")
        && contains_row(matrix, "illegal-traversal-edge-or-depth")
        && contains_row(matrix, "invalid-result-shape-binding")
        && contains_row(matrix, "structured-content-illegality")
        && contains_row(matrix, "workflow-context-illegality")
        && contains_row(matrix, "forbidden-widening-case")
}

fn covered_assertion_classes(
    matrix: &ValidationCertificationMatrix,
    zero_fallback_lane_count: usize,
    supported_lane_count: usize,
) -> Vec<RequiredAssertionClass> {
    let mut covered = Vec::new();

    if matrix
        .rows
        .iter()
        .any(|row| row.hostile_expectation == super::model::ValidationHostileExpectation::EquivalentToControl)
    {
        covered.push(RequiredAssertionClass::Equality);
    }

    if matrix
        .rows
        .iter()
        .any(|row| row.hostile_expectation == super::model::ValidationHostileExpectation::DistinctFromControl)
    {
        covered.push(RequiredAssertionClass::Inequality);
    }

    if !matrix.rejection_rows.is_empty() {
        covered.push(RequiredAssertionClass::TypedFailure);
    }

    if zero_fallback_lane_count == supported_lane_count {
        covered.push(RequiredAssertionClass::ZeroResidue);
    }

    covered
}
