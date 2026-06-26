use forge_relational::facade::runtime::RelationalRuntime;

use super::super::super::mutation_sequence_support::closeout_mutation_plan_for_declaration;
use super::super::super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenarioReport,
    MilestoneThreeMutationBranchLocalParityRow,
};
use super::super::super::scenario_programs::{
    rejected_branch_local_bowtie_adjacent_declaration,
    rejected_branch_local_broken_radial_declaration,
};
use super::accepted_branch_local::certify_accepted_branch_local_mutation_parity_rows;
use crate::certification::error::TopologyCertificationError;
use crate::certification::{MilestoneThreeHostileScenario, TopologyBranchAuthoringBoundary};
use crate::test_support::schema_topology_authoring_boundary::{
    seed_milestone_one_primitive_through_schema_execution,
    witness_rejected_branch_local_intent_through_schema_execution,
};

pub(in crate::certification::topology_operator_closeout) fn certify_milestone_three_branch_local_mutation_parity_impl<
    F,
>(
    mut runtime_factory: F,
    stem: &str,
    scenario_reports: &[MilestoneThreeHostileScenarioReport],
) -> Result<Vec<MilestoneThreeMutationBranchLocalParityRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut rows = certify_accepted_branch_local_mutation_parity_rows(stem, scenario_reports)?;
    rows.extend(certify_rejected_branch_local_diagnostic_parity(
        &mut runtime_factory,
        stem,
        scenario_reports,
    )?);
    Ok(rows)
}

fn certify_rejected_branch_local_diagnostic_parity<F>(
    runtime_factory: &mut F,
    stem: &str,
    scenario_reports: &[MilestoneThreeHostileScenarioReport],
) -> Result<Vec<MilestoneThreeMutationBranchLocalParityRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    scenario_reports
        .iter()
        .filter(|report| report.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected)
        .map(|report| rejected_branch_local_row(runtime_factory, stem, report))
        .collect()
}

fn rejected_branch_local_row<F>(
    runtime_factory: &mut F,
    stem: &str,
    report: &MilestoneThreeHostileScenarioReport,
) -> Result<MilestoneThreeMutationBranchLocalParityRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let declaration = match report.scenario {
        MilestoneThreeHostileScenario::BowtieAdjacentRewire => {
            rejected_branch_local_bowtie_adjacent_declaration(
                runtime_factory,
                &format!("{stem}.branch_local_rejected.{}", report.scenario.as_str()),
            )?
        }
        MilestoneThreeHostileScenario::BrokenRadialLocalization => {
            rejected_branch_local_broken_radial_declaration(
                runtime_factory,
                &format!("{stem}.branch_local_rejected.{}", report.scenario.as_str()),
            )?
        }
        scenario => {
            return Err(TopologyCertificationError::Query(format!(
                "scenario `{}` is not a rejected branch-local scenario",
                scenario.as_str()
            )));
        }
    };
    let rejected_plan = closeout_mutation_plan_for_declaration(declaration);
    let mut runtime = runtime_factory();
    let _seeded = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        &format!("{stem}.branch_local_rejected.{}", report.scenario.as_str()),
        &report.primitive,
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let rejected_branch = witness_rejected_branch_local_intent_through_schema_execution(
        &mut runtime,
        format!("{stem}.branch_local_rejected.{}", report.scenario.as_str()),
        rejected_plan.raw_intent,
    )
    .map_err(TopologyCertificationError::Query)?;
    let branch_label = rejected_branch.branch_label().to_string();
    let branch_id = rejected_branch.branch_id().0.clone();
    let rejection_class = report.rejection_class.ok_or_else(|| {
        TopologyCertificationError::Query(format!(
            "branch-local rejected parity expected rejection class for {}",
            report.scenario.as_str()
        ))
    })?;
    if rejected_branch.rejection_detail().trim().is_empty() {
        return Err(TopologyCertificationError::Query(format!(
            "rejected branch-local witness for `{}` did not retain an honest rejection detail",
            report.scenario.as_str()
        )));
    }
    Ok(MilestoneThreeMutationBranchLocalParityRow {
        scenario: Some(report.scenario),
        branch_label: branch_label.clone(),
        branch_id,
        mutation_origin: "branch_local_application".to_string(),
        branch_authoring_boundary: TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring,
        outcome_class: MilestoneThreeHostileOutcomeClass::Rejected,
        rejection_class: Some(rejection_class),
        mutation_families: report.mutation_families().to_vec(),
        topology_mutation_digest: report.topology_mutation_digest().clone(),
        naming_mutation_continuity_matrix: report.naming_mutation_continuity_matrix().clone(),
        derived_fallback_policy: None,
        branch_head_diverged_from_main: false,
        branch_head_unchanged_after_rejection: rejected_branch
            .branch_head_unchanged_after_rejection(),
        branch_truth_digest: None,
        row_digest: format!(
            "branch={};origin=branch_local_application;boundary={};outcome=rejected;scenario={};rejection_class={rejection_class:?};attempted_rejected_execution=true;head_unchanged={}",
            branch_label,
            TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring.as_str(),
            report.scenario.as_str(),
            rejected_branch.branch_head_unchanged_after_rejection()
        ),
    })
}
