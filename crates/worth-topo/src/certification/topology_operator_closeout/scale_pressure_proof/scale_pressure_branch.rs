use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive, MilestoneOnePrimitiveCase,
};
use schema::facade::platform::entities::TopologyEntityKind;

use super::super::shared::aggregate_topology_edit_digest;
use super::scale_pressure_types::{
    MilestoneThreeScalePressureRow, MilestoneThreeScalePressureSweep,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::{digest_rows, primitive_family_name};
use crate::test_support::topology_commit::commit_topology_intent_on_branch;
use crate::topology_operators::{
    TopologyEditApplicationMode, TopologyEditBatch, TopologyEditContract, TopologyEditDigest,
    TopologyEditFamily,
};

struct BranchHistoryExecution {
    topology_edit_digest: TopologyEditDigest,
    edit_families: Vec<TopologyEditFamily>,
    final_state_digest: String,
}

pub(super) fn certify_large_branch_history_row<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeScalePressureRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::SheetDisk { edge_count: 3 };
    let left = execute_large_branch_history(runtime_factory, stem, &primitive)?;
    let replay = execute_large_branch_history(runtime_factory, stem, &primitive)?;
    let replay_verified = left.topology_edit_digest == replay.topology_edit_digest
        && left.final_state_digest == replay.final_state_digest;
    Ok(MilestoneThreeScalePressureRow {
        sweep: MilestoneThreeScalePressureSweep::LargeBranchLocalHistories,
        primitive_family: primitive_family_name(&primitive).to_string(),
        primitive,
        workload_size: large_branch_history_step_count(),
        edit_step_count: left.topology_edit_digest.contract_count,
        edit_families: left.edit_families,
        branch_local: true,
        topology_edit_digest: left.topology_edit_digest,
        replay_verified,
        final_state_digest: left.final_state_digest.clone(),
        replay_final_state_digest: replay.final_state_digest,
        derived_validation_row_count: 0,
        row_digest: branch_history_row_digest(replay_verified),
    })
}

fn execute_large_branch_history<F>(
    runtime_factory: &mut F,
    stem: &str,
    primitive: &MilestoneOnePrimitiveCase,
) -> Result<BranchHistoryExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut runtime = runtime_factory();
    seed_milestone_one_primitive(&mut runtime, &format!("{stem}.branch_pressure"), primitive)?;
    let branch_id = BranchId(format!("{stem}.branch_pressure.history"));
    runtime
        .history_authority()
        .create_branch(branch_id.clone(), &BranchId("main".to_string()))
        .map_err(|error| TopologyCertificationError::Query(format!("{error:?}")))?;
    execute_branch_local_vertex_history(runtime, branch_id, stem)
}

fn execute_branch_local_vertex_history(
    mut runtime: RelationalRuntime,
    branch_id: BranchId,
    stem: &str,
) -> Result<BranchHistoryExecution, TopologyCertificationError> {
    let mode = TopologyEditApplicationMode::BranchLocal(branch_id.clone());
    let mut batches = Vec::new();
    let mut truth_digest_rows = Vec::new();
    for step in 0..large_branch_history_step_count() {
        let batch = branch_local_vertex_creation_batch(stem, step)?;
        let verified = commit_topology_intent_on_branch(
            &mut runtime,
            batch.clone().into_raw_intent(&mode),
            branch_id.clone(),
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
        truth_digest_rows.extend(
            verified
                .canonical_batch
                .batch
                .mutations
                .iter()
                .map(|mutation| serde_json::to_string(mutation).expect("mutation serializes")),
        );
        batches.push(batch);
    }
    Ok(BranchHistoryExecution {
        topology_edit_digest: aggregate_topology_edit_digest(&batches),
        edit_families: batches.iter().flat_map(|batch| batch.families()).collect(),
        final_state_digest: digest_rows(truth_digest_rows.into_iter()).digest_hex,
    })
}

fn branch_local_vertex_creation_batch(
    stem: &str,
    step: usize,
) -> Result<TopologyEditBatch, TopologyCertificationError> {
    TopologyEditBatch::new(vec![TopologyEditContract::create_topology_entity(
        format!("{stem}.branch_pressure.vertex.{step:02}"),
        TopologyEntityKind::Vertex,
    )])
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

fn large_branch_history_step_count() -> usize {
    12
}

fn branch_history_row_digest(replay_verified: bool) -> String {
    format!(
        "scale_pressure={};replay_verified={replay_verified};workload_size={}",
        MilestoneThreeScalePressureSweep::LargeBranchLocalHistories.as_str(),
        large_branch_history_step_count()
    )
}




