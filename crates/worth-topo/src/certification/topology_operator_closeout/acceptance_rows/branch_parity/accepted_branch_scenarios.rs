use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::seed_milestone_one_primitive;
use serde_json::Value;

use super::super::super::mutation_sequence_support::TopologyCloseoutDeclaration;
use super::super::super::scenario_programs::successor_relocation_declaration;
use super::super::super::shared::first_source_identity_for_relation_kind;
use super::accepted_branch_authority_projection::{
    authority_expanded_collapse_plan, authority_expanded_rewire_plan,
    authority_expanded_split_plan, collapse_split_wire_declaration, entity_id_by_label,
    first_entity_id, owned_half_edge_relation_ids, owned_half_edges, read_snapshot,
    relation_id_by_shape, seeded_wire_and_half_edges,
};
use super::accepted_branch_execution::{
    apply_branch_declaration, apply_branch_plan, create_branch,
    execution_from_verified_declarations, execution_from_verified_plans, AcceptedBranchExecution,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyCreateInnerLoopOnExistingFaceDeclaration,
    TopologyDetachBoundaryMembershipDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration, TopologyWireSplitHalfEdgeMember,
};

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
    let create_inner_loop = TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
        loop_key.as_str(),
        format!("{stem}.cancellation.face-inner-loop"),
        face_id,
    );
    let verified_one =
        apply_branch_declaration(&mut runtime, &branch.branch_id, create_inner_loop.clone())?;
    let after_attach = read_snapshot(&runtime, verified_one.read_basis())?;
    let loop_id = entity_id_by_label(&after_attach, loop_key.as_str(), TopologyEntityKind::Loop)?;
    let inner_loop_relation_id = relation_id_by_shape(
        &after_attach,
        TopologyRelationKind::FaceInnerLoop,
        face_id,
        loop_id,
    )?;
    let detach_inner_loop = TopologyDetachBoundaryMembershipDeclaration::new(
        inner_loop_relation_id,
        BoundaryMembershipKind::FaceInnerLoop,
    );
    let verified_two =
        apply_branch_declaration(&mut runtime, &branch.branch_id, detach_inner_loop.clone())?;
    let retire_loop =
        TopologyRetireTopologyEntityDeclaration::new(loop_id, TopologyEntityKind::Loop);
    let verified_three =
        apply_branch_declaration(&mut runtime, &branch.branch_id, retire_loop.clone())?;
    execution_from_verified_declarations(
        &runtime,
        branch,
        vec![
            TopologyCloseoutDeclaration::CreateInnerLoopOnExistingFace(create_inner_loop),
            TopologyCloseoutDeclaration::DetachBoundaryMembership(detach_inner_loop),
            TopologyCloseoutDeclaration::RetireTopologyEntity(retire_loop),
        ],
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
    let split_declaration = TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(
        split_wire_key.as_str(),
        half_edge_ids[2..]
            .iter()
            .enumerate()
            .map(|(index, half_edge_id)| {
                TopologyWireSplitHalfEdgeMember::new(
                    format!("{}.owns_half_edge_{}", split_wire_key.as_str(), index + 1),
                    *half_edge_id,
                )
            })
            .collect(),
    );
    let moved_relation_ids =
        owned_half_edge_relation_ids(&seeded_read, wire_id, &half_edge_ids[2..])?;
    let split_plan = authority_expanded_split_plan(split_declaration.clone(), &moved_relation_ids);
    let verified_split = apply_branch_plan(&mut runtime, &branch.branch_id, &split_plan)?;
    let after_split = read_snapshot(&runtime, verified_split.read_basis())?;
    let split_wire_id = entity_id_by_label(
        &after_split,
        split_wire_key.as_str(),
        TopologyEntityKind::Wire,
    )?;
    let split_half_edge_ids = owned_half_edges(&after_split, split_wire_id)?;
    let split_relation_ids =
        owned_half_edge_relation_ids(&after_split, split_wire_id, &split_half_edge_ids)?;
    let collapse_wire_key = CreateKey::new(format!("{stem}.split_collapse_churn.collapse_wire"));
    let collapse_plan = authority_expanded_collapse_plan(
        collapse_split_wire_declaration(
            collapse_wire_key.as_str(),
            split_wire_id,
            &split_half_edge_ids,
        ),
        &split_relation_ids,
    );
    let verified_collapse = apply_branch_plan(&mut runtime, &branch.branch_id, &collapse_plan)?;
    execution_from_verified_plans(
        &runtime,
        branch,
        vec![split_plan, collapse_plan],
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
    let declaration = ambiguous_rewire_declaration(runtime_factory, stem)?;
    let branch = create_branch(&mut runtime, stem, "accepted.ambiguous_rewire")?;
    let plan =
        authority_expanded_rewire_plan(declaration, &format!("{stem}.branch_local.ambiguous"))?;
    let verified = apply_branch_plan(&mut runtime, &branch.branch_id, &plan)?;
    execution_from_verified_plans(&runtime, branch, vec![plan], vec![verified])
}

fn ambiguous_rewire_declaration<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<
    crate::topology_operators::TopologyRewireLoopSuccessorProgramDeclaration,
    TopologyCertificationError,
>
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
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let domain_query = TopologyReadProofHarness::new();
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
    successor_relocation_declaration(&neighborhood, &chosen_successor_identity)
}
