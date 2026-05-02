use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use worth_schema::facade::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase, WorthTopologyRelationKind,
};

use super::relation_update_support::{
    alternate_same_edge_half_edge_id, different_edge_half_edge_id, find_entity_id_by_identity,
    query_relation_id_from_row,
};
use crate::edit::{
    WorthTopologyEditApplicationMode, WorthTopologyEditBatch, WorthTopologyEditContract,
    WorthTopologyEditFamily, WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn splice_radial_adjacency_contract_preserves_upsert_relation_lowering() {
    let relation_id = RelationId::new(PartitionId::main(), 17, 1);
    let half_edge_id = EntityId::new(PartitionId::main(), 18, 1);
    let radial_next_half_edge_id = EntityId::new(PartitionId::main(), 19, 1);
    let contract = WorthTopologyEditContract::splice_radial_adjacency(
        relation_id,
        half_edge_id,
        radial_next_half_edge_id,
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
                    WorthTopologyRelationKind::HalfEdgeRadialNext
                )
            );
            assert_eq!(*source, half_edge_id);
            assert_eq!(*target, radial_next_half_edge_id);
        }
        other => panic!("expected upsert relation lowering, got {other:?}"),
    }
}

#[test]
fn query_native_edit_runner_executes_splice_radial_adjacency_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.splice-radial",
        &WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.query-native-edit.splice-radial")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let relation_rows = workspace.read(assembly.relations());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name == WorthTopologyRelationKind::HalfEdgeRadialNext.kind_name()
                })
        })
        .expect("seeded topology should contain a radial relation");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.source_identity");
    let current_target_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.target_identity");
    let entity_rows = workspace.read(assembly.entities());
    let half_edge_id = find_entity_id_by_identity(&entity_rows, source_identity);
    let radial_next_half_edge_id = alternate_same_edge_half_edge_id(
        &entity_rows,
        &relation_rows,
        source_identity,
        current_target_identity,
    );
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::splice_radial_adjacency(
            query_relation_id_from_row(relation),
            half_edge_id,
            radial_next_half_edge_id,
        )])
        .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("radial splice should execute through the admitted runtime family");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::SpliceRadialAdjacency]
    );
    let half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == half_edge_id)
        .expect("rewired halfedge should remain present");
    assert_eq!(
        half_edge.radial_next_half_edge_id,
        Some(radial_next_half_edge_id)
    );
}

#[test]
fn query_native_edit_runner_denies_splice_radial_adjacency_with_mismatched_source_binding() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.splice-radial-source-mismatch",
        &WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.splice-radial-source-mismatch",
    )
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
                    kind_name == WorthTopologyRelationKind::HalfEdgeRadialNext.kind_name()
                })
        })
        .expect("seeded topology should contain a radial relation");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.source_identity");
    let current_target_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.target_identity");
    let wrong_half_edge_id = find_entity_id_by_identity(&entity_rows, current_target_identity);
    let radial_next_half_edge_id = alternate_same_edge_half_edge_id(
        &entity_rows,
        &relation_rows,
        source_identity,
        current_target_identity,
    );
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::splice_radial_adjacency(
            query_relation_id_from_row(relation),
            wrong_half_edge_id,
            radial_next_half_edge_id,
        )])
        .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("radial splice with mismatched source binding must fail typed and early");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
            relation_id,
            expected_source_entity_id,
            ..
        } if relation_id == query_relation_id_from_row(relation)
            && expected_source_entity_id == wrong_half_edge_id
    ));
}

#[test]
fn query_native_edit_runner_denies_splice_radial_adjacency_across_different_edges() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.splice-radial-mismatch",
        &WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.splice-radial-mismatch")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name == WorthTopologyRelationKind::HalfEdgeRadialNext.kind_name()
                })
        })
        .expect("seeded topology should contain a radial relation");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.source_identity");
    let half_edge_id = find_entity_id_by_identity(&entity_rows, source_identity);
    let different_edge_half_edge_id =
        different_edge_half_edge_id(&entity_rows, &relation_rows, source_identity);
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::splice_radial_adjacency(
            query_relation_id_from_row(relation),
            half_edge_id,
            different_edge_half_edge_id,
        )])
        .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("radial splice across different edges must fail typed and early");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::ExistingHalfEdgesNotOnSameEdge {
            source_half_edge_id,
            target_half_edge_id,
            ..
        } if source_half_edge_id == half_edge_id
            && target_half_edge_id == different_edge_half_edge_id
    ));
}
