use std::collections::BTreeSet;

use crate::certification::error::TopologyCertificationError;

use super::super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenarioReport,
    MilestoneThreeHostileSuiteReport, MilestoneThreeValidatorFamilyCoverageRow,
};
use super::super::validation_breadth_row::MilestoneThreeValidationBreadthRow;
use super::super::{milestone_three_rejected_scenarios, milestone_three_required_scenarios};

pub(in crate::certification::topology_operator_closeout) fn build_validation_breadth_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
    validator_rows: &[MilestoneThreeValidatorFamilyCoverageRow],
) -> Vec<MilestoneThreeValidationBreadthRow> {
    reports
        .iter()
        .map(|report| validation_breadth_row(report, validator_rows))
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_validation_breadth_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    ensure_validation_breadth_row_count(report)?;
    for scenario in milestone_three_required_scenarios() {
        let row = report
            .validation_breadth_rows
            .iter()
            .find(|row| row.scenario == *scenario)
            .ok_or_else(|| validation_breadth_error(&missing_row_reason(scenario.as_str())))?;
        ensure_validation_breadth_row_matches_source_evidence(report, row)?;
        ensure_validation_breadth_row_is_proof_bearing(row)?;
    }
    Ok(())
}

fn ensure_validation_breadth_row_count(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    let required_count = milestone_three_required_scenarios().len();
    if report.validation_breadth_rows.len() != required_count {
        return Err(validation_breadth_error(&format!(
            "expected {required_count} validation breadth rows, found {}",
            report.validation_breadth_rows.len()
        )));
    }
    Ok(())
}

fn ensure_validation_breadth_row_matches_source_evidence(
    report: &MilestoneThreeHostileSuiteReport,
    row: &MilestoneThreeValidationBreadthRow,
) -> Result<(), TopologyCertificationError> {
    let scenario_report = report
        .scenario_reports
        .iter()
        .find(|scenario_report| scenario_report.scenario == row.scenario)
        .ok_or_else(|| {
            validation_breadth_error(&format!(
                "missing source scenario report for {}",
                row.scenario.as_str()
            ))
        })?;
    let expected_row =
        validation_breadth_row(scenario_report, &report.validator_family_coverage_rows);
    if row != &expected_row {
        return Err(validation_breadth_error(&format!(
            "validation breadth row drifted from source evidence for {}",
            row.scenario.as_str()
        )));
    }
    Ok(())
}

fn validation_breadth_row(
    report: &MilestoneThreeHostileScenarioReport,
    validator_rows: &[MilestoneThreeValidatorFamilyCoverageRow],
) -> MilestoneThreeValidationBreadthRow {
    let scenario_validator_rows = validator_rows
        .iter()
        .filter(|row| row.scenario == report.scenario)
        .collect::<Vec<_>>();
    let validator_names = validator_names(&scenario_validator_rows);
    let localized_rejection_boundary_count = scenario_validator_rows
        .iter()
        .filter(|row| row.localized_rejection_boundary)
        .count();
    let derived_validation_row_count = scenario_validator_rows
        .iter()
        .map(|row| row.derived_validation_row_count)
        .max()
        .unwrap_or_default();

    MilestoneThreeValidationBreadthRow {
        scenario: report.scenario,
        outcome_class: report.outcome_class,
        validator_family_count: scenario_validator_rows.len(),
        validator_name_count: validator_names.len(),
        mutation_family_count: report.mutation_families.len(),
        changed_scope_count: report.topology_mutation_digest.changed_scope_count,
        naming_scope_count: report.topology_mutation_digest.naming_scope_count,
        derived_region_count: report.topology_mutation_digest.derived_region_count,
        derived_validation_row_count,
        localized_rejection_boundary_count,
        replay_checked: report.mutation_replay_parity_report.replay_checked,
        row_digest: validation_breadth_digest(
            report,
            scenario_validator_rows.len(),
            validator_names.len(),
            derived_validation_row_count,
            localized_rejection_boundary_count,
        ),
    }
}

fn validator_names(rows: &[&MilestoneThreeValidatorFamilyCoverageRow]) -> BTreeSet<String> {
    rows.iter()
        .flat_map(|row| row.validator_names.iter().cloned())
        .collect()
}

fn validation_breadth_digest(
    report: &MilestoneThreeHostileScenarioReport,
    validator_family_count: usize,
    validator_name_count: usize,
    derived_validation_row_count: usize,
    localized_rejection_boundary_count: usize,
) -> String {
    format!(
        "scenario={};outcome={:?};validator_families={};validator_names={};mutation_families={};changed_scopes={};naming_scopes={};derived_regions={};derived_validation_rows={};localized_rejection_boundaries={};replay_checked={}",
        report.scenario.as_str(),
        report.outcome_class,
        validator_family_count,
        validator_name_count,
        report.mutation_families.len(),
        report.topology_mutation_digest.changed_scope_count,
        report.topology_mutation_digest.naming_scope_count,
        report.topology_mutation_digest.derived_region_count,
        derived_validation_row_count,
        localized_rejection_boundary_count,
        report.mutation_replay_parity_report.replay_checked
    )
}

fn ensure_validation_breadth_row_is_proof_bearing(
    row: &MilestoneThreeValidationBreadthRow,
) -> Result<(), TopologyCertificationError> {
    let common_breadth_is_present = row.validator_family_count >= 3
        && row.validator_name_count >= 3
        && row.mutation_family_count > 0
        && row.changed_scope_count > 0
        && row.naming_scope_count > 0
        && row.derived_region_count > 0
        && row.replay_checked
        && row.row_digest.contains("validator_families=")
        && row.row_digest.contains("derived_validation_rows=")
        && row.row_digest.contains("localized_rejection_boundaries=");
    if !common_breadth_is_present {
        return Err(validation_breadth_error(&format!(
            "validation breadth row is not proof-bearing for {}",
            row.scenario.as_str()
        )));
    }
    match row.outcome_class {
        MilestoneThreeHostileOutcomeClass::Accepted => ensure_accepted_validation_breadth(row),
        MilestoneThreeHostileOutcomeClass::Rejected => ensure_rejected_validation_breadth(row),
    }
}

fn ensure_accepted_validation_breadth(
    row: &MilestoneThreeValidationBreadthRow,
) -> Result<(), TopologyCertificationError> {
    if row.derived_validation_row_count == 0 {
        return Err(validation_breadth_error(&format!(
            "accepted scenario lacks derived validation inspection rows for {}",
            row.scenario.as_str()
        )));
    }
    Ok(())
}

fn ensure_rejected_validation_breadth(
    row: &MilestoneThreeValidationBreadthRow,
) -> Result<(), TopologyCertificationError> {
    if !milestone_three_rejected_scenarios().contains(&row.scenario)
        || row.localized_rejection_boundary_count == 0
    {
        return Err(validation_breadth_error(&format!(
            "rejected scenario lacks localized validation boundary proof for {}",
            row.scenario.as_str()
        )));
    }
    Ok(())
}

fn missing_row_reason(scenario: &str) -> String {
    format!("missing validation breadth row for {scenario}")
}

fn validation_breadth_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three validation breadth failed: {reason}"
    ))
}

#[cfg(test)]
mod tests {
    use crate::facade::certify_milestone_three_hostile_suite;
    use crate::validation::reference_integrity::build_milestone_one_runtime;

    use super::ensure_validation_breadth_rows;

    #[test]
    fn validation_breadth_gate_rejects_weak_validator_name_counts() {
        let mut report = certify_milestone_three_hostile_suite(
            || build_milestone_one_runtime().expect("milestone one runtime builder"),
            "m3.validation_breadth.weak_validator_names",
        )
        .expect("hostile suite should certify before tampering");

        let row = report
            .validation_breadth_rows
            .iter_mut()
            .next()
            .expect("hostile suite should include validation breadth rows");
        row.validator_name_count = 0;

        assert!(
            ensure_validation_breadth_rows(&report).is_err(),
            "validation breadth closeout must reject rows with no validator names"
        );
    }

    #[test]
    fn validation_breadth_gate_rejects_unlocalized_rejected_scenarios() {
        let mut report = certify_milestone_three_hostile_suite(
            || build_milestone_one_runtime().expect("milestone one runtime builder"),
            "m3.validation_breadth.unlocalized_rejection",
        )
        .expect("hostile suite should certify before tampering");

        let row = report
            .validation_breadth_rows
            .iter_mut()
            .find(|row| row.outcome_class == super::MilestoneThreeHostileOutcomeClass::Rejected)
            .expect("hostile suite should include a rejected validation breadth row");
        row.localized_rejection_boundary_count = 0;

        assert!(
            ensure_validation_breadth_rows(&report).is_err(),
            "validation breadth closeout must reject unlocalized rejected scenarios"
        );
    }

    #[test]
    fn validation_breadth_gate_rejects_source_evidence_drift() {
        let mut report = certify_milestone_three_hostile_suite(
            || build_milestone_one_runtime().expect("milestone one runtime builder"),
            "m3.validation_breadth.source_drift",
        )
        .expect("hostile suite should certify before tampering");

        let row = report
            .validation_breadth_rows
            .iter_mut()
            .next()
            .expect("hostile suite should include validation breadth rows");
        row.validator_name_count += 1;
        row.row_digest = row
            .row_digest
            .replace("validator_names=", "tampered_validator_names=");

        assert!(
            ensure_validation_breadth_rows(&report).is_err(),
            "validation breadth closeout must reject rows that drift from source evidence"
        );
    }

    #[test]
    fn validation_breadth_gate_rejects_duplicate_rows() {
        let mut report = certify_milestone_three_hostile_suite(
            || build_milestone_one_runtime().expect("milestone one runtime builder"),
            "m3.validation_breadth.duplicate_row",
        )
        .expect("hostile suite should certify before tampering");

        let duplicate = report
            .validation_breadth_rows
            .first()
            .expect("hostile suite should include validation breadth rows")
            .clone();
        report.validation_breadth_rows.push(duplicate);

        assert!(
            ensure_validation_breadth_rows(&report).is_err(),
            "validation breadth closeout must reject duplicate or extra rows"
        );
    }
}
