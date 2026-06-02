use forge_relational::facade::runtime::RelationalRuntime;

use super::super::super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeMutationBranchLocalParityRow,
};
use super::accepted_branch_execution::mutation_digest_shape_matches;
use super::accepted_branch_scenarios::{
    execute_ambiguous_rewire_branch, execute_cancellation_chain_branch,
    execute_split_collapse_branch,
};
use super::accepted_branch_schema_authority_projection::AcceptedBranchSchemaAuthorityProjection;
use crate::certification::error::TopologyCertificationError;
use crate::certification::TopologyBranchAuthoringBoundary;

pub(in crate::certification::topology_operator_closeout) fn certify_accepted_branch_local_mutation_parity_rows<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
    scenario_reports: &[MilestoneThreeHostileScenarioReport],
) -> Result<Vec<MilestoneThreeMutationBranchLocalParityRow>, TopologyCertificationError>
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
) -> Result<MilestoneThreeMutationBranchLocalParityRow, TopologyCertificationError>
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
    let topology_mutation_digest = execution.topology_mutation_digest.clone();
    let naming_mutation_continuity_matrix = execution.naming_mutation_continuity_matrix.clone();
    let mutation_families = execution.mutation_families.clone();
    let derived_fallback_policy = execution.derived_fallback_policy;

    if !mutation_digest_shape_matches(&topology_mutation_digest, report.topology_mutation_digest())
        || naming_mutation_continuity_matrix != *report.naming_mutation_continuity_matrix()
        || mutation_families != report.mutation_families()
        || Some(derived_fallback_policy) != report.derived_fallback_policy()
    {
        return Err(TopologyCertificationError::Query(format!(
            "accepted branch-local row for `{}` drifted from scenario mutation shape",
            report.scenario.as_str()
        )));
    }

    Ok(MilestoneThreeMutationBranchLocalParityRow {
        scenario: Some(report.scenario),
        branch_label: execution.branch_label.clone(),
        branch_id: execution.branch_id,
        mutation_origin: "branch_local_application".to_string(),
        branch_authoring_boundary: TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        mutation_families,
        topology_mutation_digest,
        naming_mutation_continuity_matrix,
        derived_fallback_policy: Some(derived_fallback_policy),
        branch_head_diverged_from_main: execution.branch_head_diverged_from_main,
        branch_head_unchanged_after_rejection: false,
        branch_truth_digest: Some(execution.branch_truth_digest),
        row_digest: format!(
            "branch={};origin=branch_local_application;boundary={};outcome=accepted;projection={};scenario={};families={};fallback_policy={};diverged_from_main={}",
            execution.branch_label,
            TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring.as_str(),
            AcceptedBranchSchemaAuthorityProjection::ROW_PROJECTION_MARKER,
            report.scenario.as_str(),
            report.mutation_families().len(),
            derived_fallback_policy.as_str(),
            execution.branch_head_diverged_from_main
        ),
    })
}
