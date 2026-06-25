use crate::certification::error::TopologyCertificationError;

use super::super::super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileSuiteReport,
};
use super::super::super::{
    milestone_three_rejected_scenarios, milestone_three_required_scenarios,
    MilestoneThreeHostileScenario,
};
use super::accepted_branch_schema_authority_projection::ACCEPTED_BRANCH_SCHEMA_AUTHORITY_PROJECTION_MARKER;

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
            && row.mutation_families == scenario_report.mutation_families()
            && mutation_digest_shape_matches(
                &row.topology_mutation_digest,
                scenario_report.topology_mutation_digest(),
            )
            && row.naming_mutation_continuity_matrix
                == *scenario_report.naming_mutation_continuity_matrix()
            && row.derived_fallback_policy == scenario_report.derived_fallback_policy()
            && row.branch_truth_digest.is_some()
            && row
                .row_digest
                .contains(ACCEPTED_BRANCH_SCHEMA_AUTHORITY_PROJECTION_MARKER)
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

fn mutation_digest_shape_matches(
    left: &crate::topology_operators::TopologyMutationDigest,
    right: &crate::topology_operators::TopologyMutationDigest,
) -> bool {
    left.mutation_record_count == right.mutation_record_count
        && left.family_count == right.family_count
        && left.changed_scope_count == right.changed_scope_count
        && left.naming_scope_count == right.naming_scope_count
        && left.derived_region_count == right.derived_region_count
        && left.fallback_policy_count == right.fallback_policy_count
        && left.fallback_rejection_policy_count == right.fallback_rejection_policy_count
}

#[cfg(all(test, feature = "slow-certification"))]
mod tests {
    use super::ensure_branch_local_mutation_parity_rows;
    use crate::certification::topology_operator_closeout::acceptance_rows::test_support;

    #[test]
    fn accepted_branch_local_gate_rejects_missing_schema_authority_projection_marker() {
        let mut report = certified_report();
        let accepted = report
            .mutation_branch_local_parity_rows
            .iter_mut()
            .find(|row| row.outcome_class == super::MilestoneThreeHostileOutcomeClass::Accepted)
            .expect("accepted branch-local row");

        accepted.row_digest = accepted.row_digest.replace(
            &format!(
                "projection={};",
                super::ACCEPTED_BRANCH_SCHEMA_AUTHORITY_PROJECTION_MARKER
            ),
            "",
        );

        assert!(
            ensure_branch_local_mutation_parity_rows(&report).is_err(),
            "accepted branch-local evidence must remain tied to the schema authority projection path"
        );
    }

    fn certified_report(
    ) -> crate::certification::topology_operator_closeout::MilestoneThreeHostileSuiteReport {
        test_support::cached_hostile_suite_report()
    }
}
