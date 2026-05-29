use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{created_ref, seed_milestone_one_primitive};
use serde_json::Value;

use super::super::super::scenario_programs::successor_relocation_batch;
use super::super::super::shared::first_source_identity_for_relation_kind;
use super::accepted_branch_authority_projection::{
    authority_expanded_collapse_mutations, authority_expanded_rewire_mutations,
    authority_expanded_split_mutations, collapse_split_wire_batch, entity_id_by_label,
    first_entity_id, owned_half_edge_relation_ids, owned_half_edges, read_snapshot,
    relation_id_by_shape, seeded_wire_and_half_edges, split_wire_batch,
};
use super::accepted_branch_execution::{
    apply_branch_batch, apply_branch_mutations, create_branch, execution_from_verified,
    AcceptedBranchExecution,
};
use crate::certification::error::TopologyCertificationError;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::TopologyDomainQuery;
use crate::topology_operators::{BoundaryMembershipKind, TopologyEditBatch, TopologyEditContract};

pub(super) fn execute_cancellation_chain_branch<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<AcceptedBranchExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive =
        schema::facade::topology_authoring::MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 };
    let mut runtime = runtime_factory();
    let seeded = seed_milestone_one_primitive(&mut runtime, stem, &primitive)?;
    let face_id = first_entity_id(
        &runtime,
        &seeded.read_basis(),
        TopologyEntityKind::Face,
        "cancellation-chain seeded face",
    )?;
    let branch = create_branch(&mut runtime, stem, "accepted.cancellation")?;
    let loop_key = CreateKey::new(format!("{stem}.cancellation.inner_loop"));
    let batch_one = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(loop_key.as_str(), TopologyEntityKind::Loop),
        TopologyEditContract::attach_boundary_membership(
            format!("{stem}.cancellation.face-inner-loop"),
            BoundaryMembershipKind::FaceInnerLoop,
            face_id,
            created_ref(loop_key.as_str()),
        ),
    ])
    .expect("cancellation-chain branch batch should be non-empty");
    let verified_one = apply_branch_batch(&mut runtime, &branch.branch_id, batch_one.clone())?;
    let after_attach = read_snapshot(&runtime, &verified_one.read_basis)?;
    let loop_id = entity_id_by_label(&after_attach, loop_key.as_str(), TopologyEntityKind::Loop)?;
    let inner_loop_relation_id = relation_id_by_shape(
        &after_attach,
        TopologyRelationKind::FaceInnerLoop,
        face_id,
        loop_id,
    )?;
    let batch_two = TopologyEditBatch::new(vec![TopologyEditContract::detach_boundary_membership(
        inner_loop_relation_id,
        BoundaryMembershipKind::FaceInnerLoop,
    )])
    .expect("cancellation-chain detach batch should be non-empty");
    let verified_two = apply_branch_batch(&mut runtime, &branch.branch_id, batch_two.clone())?;
    let batch_three = TopologyEditBatch::new(vec![TopologyEditContract::retire_topology_entity(
        loop_id,
        TopologyEntityKind::Loop,
    )])
    .expect("cancellation-chain retire batch should be non-empty");
    let verified_three = apply_branch_batch(&mut runtime, &branch.branch_id, batch_three.clone())?;
    execution_from_verified(
        &runtime,
        branch,
        vec![batch_one, batch_two, batch_three],
        vec![verified_one, verified_two, verified_three],
    )
}

pub(super) fn execute_split_collapse_branch<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<AcceptedBranchExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = schema::facade::topology_authoring::MilestoneOnePrimitiveCase::WireOpen {
        half_edge_count: 4,
    };
    let seed_stem = format!("{stem}.split_collapse_churn");
    let mut runtime = runtime_factory();
    let seeded = seed_milestone_one_primitive(&mut runtime, &seed_stem, &primitive)?;
    let (wire_id, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &seeded.read_basis())?;
    let seeded_read = read_snapshot(&runtime, &seeded.read_basis())?;
    let branch = create_branch(&mut runtime, stem, "accepted.split_collapse")?;
    let split_wire_key = CreateKey::new(format!("{stem}.split_collapse_churn.split_wire"));
    let split_batch = split_wire_batch(split_wire_key.as_str(), &half_edge_ids[2..])?;
    let moved_relation_ids =
        owned_half_edge_relation_ids(&seeded_read, wire_id, &half_edge_ids[2..])?;
    let verified_split = apply_branch_mutations(
        &mut runtime,
        &branch.branch_id,
        authority_expanded_split_mutations(&split_batch, &moved_relation_ids),
    )?;
    let after_split = read_snapshot(&runtime, &verified_split.read_basis)?;
    let split_wire_id = entity_id_by_label(
        &after_split,
        split_wire_key.as_str(),
        TopologyEntityKind::Wire,
    )?;
    let split_half_edge_ids = owned_half_edges(&after_split, split_wire_id)?;
    let split_relation_ids =
        owned_half_edge_relation_ids(&after_split, split_wire_id, &split_half_edge_ids)?;
    let collapse_wire_key = CreateKey::new(format!("{stem}.split_collapse_churn.collapse_wire"));
    let collapse_batch = collapse_split_wire_batch(
        collapse_wire_key.as_str(),
        split_wire_id,
        &split_half_edge_ids,
    )?;
    let verified_collapse = apply_branch_mutations(
        &mut runtime,
        &branch.branch_id,
        authority_expanded_collapse_mutations(&collapse_batch, &split_relation_ids),
    )?;
    execution_from_verified(
        &runtime,
        branch,
        vec![split_batch, collapse_batch],
        vec![verified_split, verified_collapse],
    )
}

pub(super) fn execute_ambiguous_rewire_branch<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<AcceptedBranchExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive =
        schema::facade::topology_authoring::MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 };
    let seed_stem = format!("{stem}.ambiguous_local_rewire.3");
    let mut runtime = runtime_factory();
    let _seeded = seed_milestone_one_primitive(&mut runtime, &seed_stem, &primitive)?;
    let batch = ambiguous_rewire_batch(runtime_factory, stem)?;
    let branch = create_branch(&mut runtime, stem, "accepted.ambiguous_rewire")?;
    let verified = apply_branch_mutations(
        &mut runtime,
        &branch.branch_id,
        authority_expanded_rewire_mutations(&batch, &format!("{stem}.branch_local.ambiguous"))?,
    )?;
    execution_from_verified(&runtime, branch, vec![batch], vec![verified])
}

fn ambiguous_rewire_batch<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<TopologyEditBatch, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive =
        schema::facade::topology_authoring::MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 };
    let mut runtime = runtime_factory();
    let _seeded = seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.ambiguous_local_rewire.3"),
        &primitive,
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, &format!("{stem}.branch_local.plan.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let relation_rows = workspace.read::<Value>(assembly.relations());
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let domain_query = TopologyDomainQuery::load();
    let neighborhood = domain_query
        .local_rewire_neighborhood(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let chosen_successor_identity =
        neighborhood
            .cycle_identities
            .get(3)
            .cloned()
            .ok_or_else(|| {
                TopologyCertificationError::Query("branch-local ambiguous successor missing".into())
            })?;
    successor_relocation_batch(&neighborhood, &chosen_successor_identity)
}
