use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

use crate::edit::{
    WorthLoopEndpointKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyQueryEditExecutionError,
    WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn rewire_loop_endpoint_contract_preserves_upsert_relation_lowering() {
    let relation_id = RelationId::new(PartitionId::main(), 7, 1);
    let half_edge_id = EntityId::new(PartitionId::main(), 8, 1);
    let vertex_id = EntityId::new(PartitionId::main(), 9, 1);
    let contract = WorthTopologyEditContract::rewire_loop_endpoint(
        relation_id,
        WorthLoopEndpointKind::End,
        half_edge_id,
        vertex_id,
    );

    match &contract.lowered_mutations()[0] {
        worth_schema::facade::WorthTopologyMutation::UpsertRelation {
            relation_id: lowered_relation_id,
            kind,
            source,
            target,
        } => {
            assert_eq!(*lowered_relation_id, relation_id);
            assert_eq!(
                *kind,
                worth_schema::facade::WorthRelationKind::Topology(
                    worth_schema::facade::WorthTopologyRelationKind::HalfEdgeEndsAtVertex
                )
            );
            assert_eq!(*source, half_edge_id);
            assert_eq!(*target, vertex_id);
        }
        other => panic!("expected upsert relation lowering, got {other:?}"),
    }
}

#[test]
fn query_native_edit_runner_denies_rewire_loop_endpoint_until_invariant_complete_rewire_workflows_are_admitted(
) {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.query-native-edit.rewire-endpoint")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::rewire_loop_endpoint(
        RelationId::new(PartitionId::main(), 7, 1),
        WorthLoopEndpointKind::End,
        EntityId::new(PartitionId::main(), 8, 1),
        EntityId::new(PartitionId::main(), 9, 1),
    )])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("endpoint rewires must stay fail-closed until invariant-complete rewire workflows are admitted");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::RewireLoopEndpoint]
    ));
}
