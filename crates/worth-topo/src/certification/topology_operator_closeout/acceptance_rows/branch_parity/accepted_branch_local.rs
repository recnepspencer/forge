use forge_relational::facade::runtime::RelationalRuntime;

use super::super::super::report::{
    MilestoneThreeEditBranchLocalParityRow, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
};
use super::super::super::shared::{
    aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
};
use super::accepted_branch_execution::edit_digest_shape_matches;
use super::accepted_branch_scenarios::{
    execute_ambiguous_rewire_branch, execute_cancellation_chain_branch,
    execute_split_collapse_branch,
};
use crate::certification::error::TopologyCertificationError;

pub(in crate::certification::topology_operator_closeout) fn certify_accepted_branch_local_edit_parity_rows<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
    scenario_reports: &[MilestoneThreeHostileScenarioReport],
) -> Result<Vec<MilestoneThreeEditBranchLocalParityRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    scenario_reports
        .iter()
        .filter(|report| report.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted)
        .map(|report| accepted_branch_local_row(runtime_factory, stem, report))
        .collect()
}

fn accepted_branch_local_row<F>(
    runtime_factory: &mut F,
    stem: &str,
    report: &MilestoneThreeHostileScenarioReport,
) -> Result<MilestoneThreeEditBranchLocalParityRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let execution = match report.scenario {
        MilestoneThreeHostileScenario::CancellationChainParity => {
            execute_cancellation_chain_branch(runtime_factory, stem)?
        }
        MilestoneThreeHostileScenario::SplitCollapseChurn => {
            execute_split_collapse_branch(runtime_factory, stem)?
        }
        MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity => {
            execute_ambiguous_rewire_branch(runtime_factory, stem)?
        }
        scenario => {
            return Err(TopologyCertificationError::Query(format!(
                "scenario `{}` is not an accepted branch-local scenario",
                scenario.as_str()
            )));
        }
    };
    let topology_edit_digest = aggregate_topology_edit_digest(&execution.batches);
    let naming_edit_continuity_matrix = aggregate_naming_edit_continuity_matrix(&execution.batches);
    let edit_families = execution
        .batches
        .iter()
        .flat_map(|batch| batch.families())
        .collect::<Vec<_>>();

    if !edit_digest_shape_matches(&topology_edit_digest, &report.topology_edit_digest)
        || naming_edit_continuity_matrix != report.naming_edit_continuity_matrix
        || edit_families != report.edit_families
    {
        return Err(TopologyCertificationError::Query(format!(
            "accepted branch-local row for `{}` drifted from scenario edit shape",
            report.scenario.as_str()
        )));
    }

    Ok(MilestoneThreeEditBranchLocalParityRow {
        scenario: Some(report.scenario),
        branch_label: execution.branch_label.clone(),
        branch_id: execution.branch_id,
        mutation_origin: "branch_local_application".to_string(),
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        edit_families,
        topology_edit_digest,
        naming_edit_continuity_matrix,
        branch_head_diverged_from_main: execution.branch_head_diverged_from_main,
        branch_head_unchanged_after_rejection: false,
        branch_truth_digest: Some(execution.branch_truth_digest),
        row_digest: format!(
            "branch={};origin=branch_local_application;outcome=accepted;projection=authority_branch_projection;scenario={};families={};diverged_from_main={}",
            execution.branch_label,
            report.scenario.as_str(),
            report.edit_families.len(),
            execution.branch_head_diverged_from_main
        ),
    })
}




