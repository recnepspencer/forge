use crate::certification::error::TopologyCertificationError;

use super::report::{MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileSuiteReport};
use super::{milestone_three_rejected_scenarios, MilestoneThreeHostileScenario};

pub(super) fn ensure_branch_local_edit_parity_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    ensure_accepted_branch_local_edit_parity_row(report)?;
    ensure_rejected_branch_local_edit_parity_rows(report)
}

fn ensure_accepted_branch_local_edit_parity_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    let accepted_verified = report.edit_branch_local_parity_rows.iter().any(|row| {
        row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
            && row.rejection_class.is_none()
            && row.branch_head_diverged_from_main
            && row.mutation_origin == "branch_local_application"
            && !row.edit_families.is_empty()
            && row.topology_edit_digest.contract_count > 0
            && !row.naming_edit_continuity_matrix.rows.is_empty()
            && row.branch_truth_digest.is_some()
    });
    if accepted_verified {
        Ok(())
    } else {
        Err(closeout_requirement_error(
            "missing accepted branch-local topology edit parity row",
        ))
    }
}

fn ensure_rejected_branch_local_edit_parity_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_rejected_scenarios() {
        ensure_rejected_branch_local_edit_parity_row(report, *scenario)?;
    }
    Ok(())
}

fn ensure_rejected_branch_local_edit_parity_row(
    report: &MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) -> Result<(), TopologyCertificationError> {
    let rejected_verified = report.edit_branch_local_parity_rows.iter().any(|row| {
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
            "missing rejected branch-local topology edit parity row for {}",
            scenario.as_str()
        )))
    }
}

fn closeout_requirement_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three closeout requirement failed: {reason}"
    ))
}
