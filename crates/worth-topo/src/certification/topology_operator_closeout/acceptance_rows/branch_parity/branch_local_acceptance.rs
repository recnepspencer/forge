use crate::certification::error::TopologyCertificationError;

use super::super::super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileSuiteReport,
};
use super::super::super::{
    milestone_three_rejected_scenarios, milestone_three_required_scenarios,
    MilestoneThreeHostileScenario,
};
use super::accepted_branch_execution::mutation_digest_shape_matches;

pub(in crate::certification::topology_operator_closeout) fn ensure_branch_local_mutation_parity_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    ensure_accepted_branch_local_mutation_parity_row(report)?;
    ensure_rejected_branch_local_mutation_parity_rows(report)
}

fn ensure_accepted_branch_local_mutation_parity_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_required_scenarios()
        .iter()
        .filter(|scenario| !milestone_three_rejected_scenarios().contains(scenario))
    {
        ensure_accepted_branch_local_mutation_parity_scenario(report, *scenario)?;
    }
    Ok(())
}

fn ensure_accepted_branch_local_mutation_parity_scenario(
    report: &MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) -> Result<(), TopologyCertificationError> {
    let scenario_report = report
        .scenario_reports
        .iter()
        .find(|scenario_report| scenario_report.scenario == scenario)
        .ok_or_else(|| closeout_requirement_error("missing accepted scenario report"))?;
    let accepted_verified = report.mutation_branch_local_parity_rows.iter().any(|row| {
        row.scenario == Some(scenario)
            && row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
            && row.rejection_class.is_none()
            && row.branch_head_diverged_from_main
            && row.mutation_origin == "branch_local_application"
            && row.mutation_families == scenario_report.mutation_families
            && mutation_digest_shape_matches(
                &row.topology_mutation_digest,
                &scenario_report.topology_mutation_digest,
            )
            && row.naming_mutation_continuity_matrix
                == scenario_report.naming_mutation_continuity_matrix
            && row.branch_truth_digest.is_some()
            && row
                .row_digest
                .contains("projection=authority_branch_projection")
    });
    if accepted_verified {
        Ok(())
    } else {
        Err(closeout_requirement_error(&format!(
            "missing accepted branch-local topology mutation parity row for {}",
            scenario.as_str()
        )))
    }
}

fn ensure_rejected_branch_local_mutation_parity_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_rejected_scenarios() {
        ensure_rejected_branch_local_mutation_parity_row(report, *scenario)?;
    }
    Ok(())
}

fn ensure_rejected_branch_local_mutation_parity_row(
    report: &MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) -> Result<(), TopologyCertificationError> {
    let rejected_verified = report.mutation_branch_local_parity_rows.iter().any(|row| {
        row.scenario == Some(scenario)
            && row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
            && row.rejection_class.is_some()
            && row.branch_head_unchanged_after_rejection
            && row.mutation_origin == "branch_local_application"
            && row.branch_truth_digest.is_none()
    });
    if rejected_verified {
        Ok(())
    } else {
        Err(closeout_requirement_error(&format!(
            "missing rejected branch-local topology mutation parity row for {}",
            scenario.as_str()
        )))
    }
}

fn closeout_requirement_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three closeout requirement failed: {reason}"
    ))
}

#[cfg(test)]
mod tests {
    use crate::facade::certify_milestone_three_hostile_suite;
    use crate::validation::reference_integrity::build_milestone_one_runtime;

    use super::ensure_branch_local_mutation_parity_rows;

    #[test]
    fn accepted_branch_local_gate_rejects_missing_authority_projection_marker() {
        let mut report = certify_milestone_three_hostile_suite(
            || build_milestone_one_runtime().expect("milestone one runtime builder"),
            "m3.branch_local_acceptance.projection_marker",
        )
        .expect("hostile suite should certify before tampering");
        let accepted = report
            .mutation_branch_local_parity_rows
            .iter_mut()
            .find(|row| row.outcome_class == super::MilestoneThreeHostileOutcomeClass::Accepted)
            .expect("accepted branch-local row");

        accepted.row_digest = accepted
            .row_digest
            .replace("projection=authority_branch_projection;", "");

        assert!(
            ensure_branch_local_mutation_parity_rows(&report).is_err(),
            "accepted branch-local evidence must remain tied to the authority projection path"
        );
    }
}
