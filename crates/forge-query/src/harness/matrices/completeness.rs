use super::super::certification::{
    covered_perturbation_classes, milestone_one_requirements, unmet_required_assertion_classes,
    unmet_required_rows, RequiredAssertionClass,
};
use super::model::{
    CertificationBundleCompletenessReport, CertificationMatrix, CertificationPerturbationClass,
    CertificationRow, RejectionCertificationRow,
};

pub(super) fn bundle_completeness_report(
    matrix: &CertificationMatrix,
) -> CertificationBundleCompletenessReport {
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
        .filter(|lane| lane.counter_snapshot.canonicalization_fallback_count == 0)
        .count();
    let all_lanes_emit_required_outputs = matrix
        .rows
        .iter()
        .all(CertificationRow::has_required_outputs)
        && matrix
            .rejection_rows
            .iter()
            .all(RejectionCertificationRow::has_required_outputs);
    let all_rows_have_hostile_coverage = matrix
        .rows
        .iter()
        .all(CertificationRow::has_hostile_coverage)
        && matrix
            .rejection_rows
            .iter()
            .all(RejectionCertificationRow::has_hostile_coverage);
    let covered_perturbation_classes = covered_perturbation_classes(matrix);
    let covered_assertion_classes =
        covered_assertion_classes(matrix, zero_fallback_lane_count, supported_lane_count);
    let requirements = milestone_one_requirements();
    let unmet_required_rows = unmet_required_rows(
        matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );
    let unmet_required_assertion_classes = unmet_required_assertion_classes(
        &covered_assertion_classes,
        requirements.required_assertion_classes,
    );
    let covers_all_mutation_sensitivity_classes = covered_perturbation_classes
        .contains(&CertificationPerturbationClass::ConstructionPath)
        && covered_perturbation_classes.contains(&CertificationPerturbationClass::MeaningChange)
        && covered_perturbation_classes
            .contains(&CertificationPerturbationClass::UnsupportedAuthoredForm)
        && covered_perturbation_classes
            .contains(&CertificationPerturbationClass::ForbiddenFallback);
    let covers_all_milestone_one_normative_scenarios =
        covers_all_milestone_one_normative_scenarios(matrix);
    let offline_analysis_ready = all_lanes_emit_required_outputs
        && all_rows_have_hostile_coverage
        && zero_fallback_lane_count == supported_lane_count
        && unmet_required_rows.is_empty()
        && unmet_required_assertion_classes.is_empty()
        && covers_all_mutation_sensitivity_classes
        && covers_all_milestone_one_normative_scenarios;

    CertificationBundleCompletenessReport {
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
        covers_all_mutation_sensitivity_classes,
        covers_all_milestone_one_normative_scenarios,
        offline_analysis_ready,
    }
}

fn contains_row(matrix: &CertificationMatrix, row_name: &str) -> bool {
    matrix.rows.iter().any(|row| row.row_name == row_name)
        || matrix
            .rejection_rows
            .iter()
            .any(|row| row.row_name == row_name)
}

fn covers_all_milestone_one_normative_scenarios(matrix: &CertificationMatrix) -> bool {
    contains_row(matrix, "detail-query-parity")
        && contains_row(matrix, "result-shape-helper-composition")
        && contains_row(matrix, "binding-descriptor-parity")
        && contains_row(matrix, "collection-reordered-projection-parity")
        && contains_row(matrix, "duplicate-clause-deduplication")
        && contains_row(matrix, "semantic-distinction-boundary")
        && contains_row(matrix, "unsupported-authored-query-family")
        && contains_row(matrix, "unsupported-authored-result-shape-family")
        && contains_row(matrix, "forbidden-fallback-case")
}

fn covered_assertion_classes(
    matrix: &CertificationMatrix,
    zero_fallback_lane_count: usize,
    supported_lane_count: usize,
) -> Vec<RequiredAssertionClass> {
    let mut covered = Vec::new();

    if matrix.rows.iter().any(|row| {
        row.hostile_expectation == super::model::HostileLaneExpectation::EquivalentToControl
    }) {
        covered.push(RequiredAssertionClass::Equality);
    }

    if matrix.rows.iter().any(|row| {
        row.hostile_expectation == super::model::HostileLaneExpectation::DistinctFromControl
    }) {
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
