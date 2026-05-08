use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive, verify_topology_intent_on_branch, MilestoneOnePrimitiveCase,
};
use schema::facade::{MutationOrigin, TopologyEntityKind};

use super::report::{
    MilestoneThreeEditBranchLocalParityRow, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenarioReport,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::digest_rows;
use crate::topology_operators::{
    TopologyEditApplicationMode, TopologyEditBatch, TopologyEditContract,
};

pub(super) fn certify_milestone_three_branch_local_edit_parity_impl<F>(
    mut runtime_factory: F,
    stem: &str,
    scenario_reports: &[MilestoneThreeHostileScenarioReport],
) -> Result<Vec<MilestoneThreeEditBranchLocalParityRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut rows = vec![certify_accepted_branch_local_edit_parity(
        &mut runtime_factory,
        stem,
    )?];
    rows.extend(certify_rejected_branch_local_diagnostic_parity(
        &mut runtime_factory,
        stem,
        scenario_reports,
    )?);
    Ok(rows)
}

fn certify_accepted_branch_local_edit_parity<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeEditBranchLocalParityRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut runtime = runtime_factory();
    let primitive = MilestoneOnePrimitiveCase::SheetDisk { edge_count: 3 };
    let _seeded = seed_milestone_one_primitive(&mut runtime, stem, &primitive)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;

    let branch_id = BranchId(format!("{stem}.branch_local"));
    runtime
        .history_authority()
        .create_branch(branch_id.clone(), &BranchId("main".to_string()))
        .map_err(|error| TopologyCertificationError::Query(format!("{error:?}")))?;
    let main_head_before_edit = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .ok_or_else(|| TopologyCertificationError::Query("main branch head missing".into()))?
        .commit_id;

    let batch = TopologyEditBatch::new(vec![TopologyEditContract::create_topology_entity(
        format!("{stem}.branch_local.vertex"),
        TopologyEntityKind::Vertex,
    )])
    .expect("branch-local parity batch should be non-empty");
    let mode = TopologyEditApplicationMode::BranchLocal(branch_id.clone());
    let topology_edit_digest = batch.topology_edit_digest();
    let naming_edit_continuity_matrix = batch.naming_edit_continuity_matrix();
    let edit_families = batch.families();
    let raw_intent = batch.into_raw_intent(&mode);

    let verified = verify_topology_intent_on_branch(&mut runtime, raw_intent, branch_id.clone())
        .map_err(|failure| {
            TopologyCertificationError::Query(format!("{:?}", failure.into_error()))
        })?;
    let branch_head_after_edit = runtime
        .history()
        .branch_head(&branch_id)
        .ok_or_else(|| TopologyCertificationError::Query("branch-local head missing".into()))?
        .commit_id;
    let main_head_after_edit = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .ok_or_else(|| TopologyCertificationError::Query("main branch head missing".into()))?
        .commit_id;

    let branch_truth_digest = digest_rows(
        verified
            .canonical_batch
            .batch
            .mutations
            .iter()
            .map(|mutation| serde_json::to_string(mutation).expect("mutation serializes")),
    );

    Ok(MilestoneThreeEditBranchLocalParityRow {
        scenario: None,
        branch_label: branch_id.0.clone(),
        branch_id: verified.branch_id.0,
        mutation_origin: mutation_origin_label(verified.persisted_truth.mutation_origin)
            .to_string(),
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        edit_families,
        topology_edit_digest,
        naming_edit_continuity_matrix,
        branch_head_diverged_from_main: branch_head_after_edit != main_head_after_edit
            && main_head_before_edit == main_head_after_edit,
        branch_head_unchanged_after_rejection: false,
        branch_truth_digest: Some(branch_truth_digest),
        row_digest: format!(
            "branch={};origin={};outcome=accepted;families={};diverged_from_main={}",
            branch_id.0,
            mutation_origin_label(verified.persisted_truth.mutation_origin),
            verified.canonical_batch.batch.mutations.len(),
            branch_head_after_edit != main_head_after_edit
        ),
    })
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

fn mutation_origin_label(origin: MutationOrigin) -> &'static str {
    match origin {
        MutationOrigin::Seed => "seed",
        MutationOrigin::LocalEdit => "local_edit",
        MutationOrigin::Replay => "replay",
        MutationOrigin::BranchLocalApplication => "branch_local_application",
    }
}
