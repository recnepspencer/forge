use super::super::super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeMutationBranchLocalParityRow,
};
use super::accepted_branch_schema_authority_projection::ACCEPTED_BRANCH_SCHEMA_AUTHORITY_PROJECTION_MARKER;
use crate::certification::error::TopologyCertificationError;
use crate::certification::TopologyBranchAuthoringBoundary;

pub(in crate::certification::topology_operator_closeout) fn certify_accepted_branch_local_mutation_parity_rows(
    stem: &str,
    scenario_reports: &[MilestoneThreeHostileScenarioReport],
) -> Result<Vec<MilestoneThreeMutationBranchLocalParityRow>, TopologyCertificationError> {
    scenario_reports
        .iter()
        .filter(|report| report.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted)
        .map(|report| accepted_branch_local_row(stem, report))
        .collect()
}

fn accepted_branch_local_row(
    stem: &str,
    report: &MilestoneThreeHostileScenarioReport,
) -> Result<MilestoneThreeMutationBranchLocalParityRow, TopologyCertificationError> {
    match report.scenario {
        MilestoneThreeHostileScenario::CancellationChainParity
        | MilestoneThreeHostileScenario::SplitCollapseChurn
        | MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity => {}
        scenario => {
            return Err(TopologyCertificationError::Query(format!(
                "scenario `{}` is not an accepted branch-local scenario",
                scenario.as_str()
            )));
        }
    }
    let branch_truth_digest = report
        .mutation_replay_parity_report
        .final_materialized_topology_digest
        .clone()
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "accepted branch-local row for `{}` lacks final materialized topology digest",
                report.scenario.as_str()
            ))
        })?;
    let branch_head_diverged_from_main = report.mutation_replay_parity_report.replay_checked
        && !report.mutation_families().is_empty();
    let derived_fallback_policy = report.derived_fallback_policy().ok_or_else(|| {
        TopologyCertificationError::Query(format!(
            "accepted branch-local row for `{}` lacks derived fallback policy",
            report.scenario.as_str()
        ))
    })?;
    let branch_label = format!("{stem}.branch_local_{}", report.scenario.as_str());

    Ok(MilestoneThreeMutationBranchLocalParityRow {
        scenario: Some(report.scenario),
        branch_label: branch_label.clone(),
        branch_id: branch_label.clone(),
        mutation_origin: "branch_local_application".to_string(),
        branch_authoring_boundary: TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        mutation_families: report.mutation_families().to_vec(),
        topology_mutation_digest: report.topology_mutation_digest().clone(),
        naming_mutation_continuity_matrix: report.naming_mutation_continuity_matrix().clone(),
        derived_fallback_policy: Some(derived_fallback_policy),
        branch_head_diverged_from_main,
        branch_head_unchanged_after_rejection: false,
        branch_truth_digest: Some(branch_truth_digest),
        row_digest: format!(
            "branch={};origin=branch_local_application;boundary={};outcome=accepted;projection={};scenario={};families={};fallback_policy={};diverged_from_main={}",
            branch_label,
            TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring.as_str(),
            ACCEPTED_BRANCH_SCHEMA_AUTHORITY_PROJECTION_MARKER,
            report.scenario.as_str(),
            report.mutation_families().len(),
            derived_fallback_policy.as_str(),
            branch_head_diverged_from_main
        ),
    })
}
