use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::super::edit_sequence_support::{
    aggregate_topology_edit_digest_for_declarations,
    branch_local_raw_topology_intent_for_declaration, topology_edit_families_for_declarations,
};
use super::scale_pressure_types::{
    MilestoneThreeScalePressureRow, MilestoneThreeScalePressureSweep,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::{digest_rows, primitive_family_name};
use crate::test_support::topology_commit::commit_topology_intent_on_branch;
use crate::topology_operators::{
    TopologyCreateTopologyEntityDeclaration, TopologyEditDigest, TopologyEditFamily,
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
    let mut declarations = Vec::new();
    let mut truth_digest_rows = Vec::new();
    for step in 0..large_branch_history_step_count() {
        let declaration = branch_local_vertex_creation_declaration(stem, step);
        let verified = commit_topology_intent_on_branch(
            &mut runtime,
            branch_local_raw_topology_intent_for_declaration(declaration.clone()),
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
        declarations.push(declaration);
    }
    Ok(BranchHistoryExecution {
        topology_edit_digest: aggregate_topology_edit_digest_for_declarations(declarations.clone()),
        edit_families: topology_edit_families_for_declarations(declarations),
        final_state_digest: digest_rows(truth_digest_rows.into_iter()).digest_hex,
    })
}

fn branch_local_vertex_creation_declaration(
    stem: &str,
    step: usize,
) -> TopologyCreateTopologyEntityDeclaration {
    TopologyCreateTopologyEntityDeclaration::new(
        format!("{stem}.branch_pressure.vertex.{step:02}"),
        TopologyEntityKind::Vertex,
    )
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
