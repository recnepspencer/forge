use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use worth_schema::facade::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};

use super::relation_update_support::{query_entity_id_from_row, query_relation_id_from_row};
use crate::edit::{
    WorthLoopEndpointKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyQueryEditRunner,
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
fn query_native_edit_runner_executes_rewire_loop_endpoint_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.rewire-endpoint",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.query-native-edit.rewire-endpoint")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let relation_rows = workspace.read(assembly.relations());
    let entity_rows = workspace.read(assembly.entities());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name
                        == worth_schema::facade::WorthTopologyRelationKind::HalfEdgeEndsAtVertex
                            .kind_name()
                })
        })
        .expect("seeded topology should contain an endpoint relation");
    let current_target_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
        .expect("endpoint relation should expose topology.target_identity");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("endpoint relation should expose topology.source_identity");
    let target_vertex_id = entity_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == "worth.vertex")
                && row.identity != current_target_identity
        })
        .map(query_entity_id_from_row)
        .expect("seeded sheet disk should provide an alternate vertex");
    let half_edge_id = entity_rows
        .iter()
        .find(|row| row.identity == source_identity)
        .map(query_entity_id_from_row)
        .expect("relation source identity should resolve to a halfedge");
    let batch = WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::rewire_loop_endpoint(
        query_relation_id_from_row(relation),
        WorthLoopEndpointKind::End,
        half_edge_id,
        target_vertex_id,
    )])
    .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("endpoint rewire should execute through the admitted runtime family");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::RewireLoopEndpoint]
    );
    let half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == half_edge_id)
        .expect("rewired halfedge should remain present");
    assert_eq!(half_edge.target_vertex_id, Some(target_vertex_id));
}
