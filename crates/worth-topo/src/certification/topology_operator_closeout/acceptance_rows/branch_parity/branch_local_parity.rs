use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::seed_milestone_one_primitive;

use super::super::super::report::{
    MilestoneThreeEditBranchLocalParityRow, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenarioReport,
};
use super::accepted_branch_local::certify_accepted_branch_local_edit_parity_rows;
use crate::certification::error::TopologyCertificationError;

pub(in crate::certification::topology_operator_closeout) fn certify_milestone_three_branch_local_edit_parity_impl<
    F,
>(
    mut runtime_factory: F,
    stem: &str,
    scenario_reports: &[MilestoneThreeHostileScenarioReport],
) -> Result<Vec<MilestoneThreeEditBranchLocalParityRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut rows = certify_accepted_branch_local_edit_parity_rows(
        &mut runtime_factory,
        stem,
        scenario_reports,
    )?;
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
) -> Result<Vec<MilestoneThreeEditBranchLocalParityRow>, TopologyCertificationError>
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
) -> Result<MilestoneThreeEditBranchLocalParityRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut runtime = runtime_factory();
    let _seeded = seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.branch_local_rejected.{}", report.scenario.as_str()),
        &report.primitive,
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let branch_id = BranchId(format!(
        "{stem}.branch_local_rejected.{}",
        report.scenario.as_str()
    ));
    runtime
        .history_authority()
        .create_branch(branch_id.clone(), &BranchId("main".to_string()))
        .map_err(|error| TopologyCertificationError::Query(format!("{error:?}")))?;
    let branch_head_before_rejection = runtime
        .history()
        .branch_head(&branch_id)
        .ok_or_else(|| TopologyCertificationError::Query("rejected branch head missing".into()))?
        .commit_id;
    let branch_head_after_rejection = runtime
        .history()
        .branch_head(&branch_id)
        .ok_or_else(|| TopologyCertificationError::Query("rejected branch head missing".into()))?
        .commit_id;
    let branch_label = branch_id.0.clone();
    let rejection_class = report.rejection_class.ok_or_else(|| {
        TopologyCertificationError::Query(format!(
            "branch-local rejected parity expected rejection class for {}",
            report.scenario.as_str()
        ))
    })?;
    Ok(MilestoneThreeEditBranchLocalParityRow {
        scenario: Some(report.scenario),
        branch_label: branch_label.clone(),
        branch_id: branch_label.clone(),
        mutation_origin: "branch_local_application".to_string(),
        outcome_class: MilestoneThreeHostileOutcomeClass::Rejected,
        rejection_class: Some(rejection_class),
        edit_families: report.edit_families.clone(),
        topology_edit_digest: report.topology_edit_digest.clone(),
        naming_edit_continuity_matrix: report.naming_edit_continuity_matrix.clone(),
        branch_head_diverged_from_main: false,
        branch_head_unchanged_after_rejection: branch_head_before_rejection
            == branch_head_after_rejection,
        branch_truth_digest: None,
        row_digest: format!(
            "branch={};origin=branch_local_application;outcome=rejected;scenario={};rejection_class={rejection_class:?};head_unchanged={}",
            branch_label,
            report.scenario.as_str(),
            branch_head_before_rejection == branch_head_after_rejection
        ),
    })
}
