use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use worth_schema::facade::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};

use super::relation_update_support::{
    find_entity_id_by_identity, half_edge_identities_for_different_loops, next_target_half_edge_id,
    prev_target_half_edge_id, query_entity_id_from_row, relation_id_for_source_kind,
};
use crate::edit::{
    WorthLoopSuccessorKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyQueryEditExecutionError,
    WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_half_edge_relocation_successor_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-rewire-successor")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let moved_identity = relation_rows
        .iter()
        .find_map(|row| {
            (row.payload["topology"]["kind"].as_str()
                == Some(worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext.kind_name()))
            .then(|| row.payload["topology"]["source_identity"].as_str())
            .flatten()
        })
        .expect("sheet disk should expose halfedge successor wiring");
    let moved_half_edge_id = find_entity_id_by_identity(&entity_rows, moved_identity);
    let old_successor_id = next_target_half_edge_id(&entity_rows, &relation_rows, moved_identity);
    let old_predecessor_id = prev_target_half_edge_id(&entity_rows, &relation_rows, moved_identity);
    let new_successor_id = next_target_half_edge_id(
        &entity_rows,
        &relation_rows,
        &entity_rows
            .iter()
            .find(|row| query_entity_id_from_row(row) == old_successor_id)
            .expect("old successor should remain visible")
            .identity,
    );
    let new_successor_id = next_target_half_edge_id(
        &entity_rows,
        &relation_rows,
        &entity_rows
            .iter()
            .find(|row| query_entity_id_from_row(row) == new_successor_id)
            .expect("intermediate successor should remain visible")
            .identity,
    );
    let new_successor_identity = entity_rows
        .iter()
        .find(|row| query_entity_id_from_row(row) == new_successor_id)
        .expect("new successor should remain visible")
        .identity
        .clone();
    let new_predecessor_id =
        prev_target_half_edge_id(&entity_rows, &relation_rows, &new_successor_identity);
    let batch = successor_relocation_batch(
        &entity_rows,
        &relation_rows,
        moved_identity,
        &new_successor_identity,
    );

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("halfedge relocation workflow should execute through admitted successor lane");

    assert_eq!(execution.families.len(), 6);
    assert!(execution
        .families
        .iter()
        .all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor));
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        6
    );
    assert_eq!(execution.inspection.component_operations().len(), 6);
    assert!(execution
        .inspection
        .component_operations()
        .iter()
        .all(|operation| {
            operation.family() == "update"
                && operation.target_collection() == Some("WorthTopologyRelation")
                && operation
                    .existing_truth_assertion_evidence()
                    .is_some_and(|evidence| {
                        evidence.mode()
                            == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                    })
        }));
    let moved_half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_half_edge_id)
        .expect("moved halfedge should remain present");
    assert_eq!(moved_half_edge.prev_half_edge_id, Some(new_predecessor_id));
    assert_eq!(moved_half_edge.next_half_edge_id, Some(new_successor_id));
    let old_predecessor = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == old_predecessor_id)
        .expect("old predecessor should remain present");
    assert_eq!(old_predecessor.next_half_edge_id, Some(old_successor_id));
}

#[test]
fn current_head_runtime_denies_cross_loop_successor_relocation_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor-cross-loop",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-rewire-successor-cross-loop",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let (moved_identity, new_successor_identity) =
        half_edge_identities_for_different_loops(&entity_rows, &relation_rows);
    let batch = successor_relocation_batch(
        &entity_rows,
        &relation_rows,
        &moved_identity,
        &new_successor_identity,
    );

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("cross-loop successor relocation must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor)
    ));
}

fn successor_relocation_batch(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    moved_identity: &str,
    new_successor_identity: &str,
) -> WorthTopologyEditBatch {
    let moved_half_edge_id = find_entity_id_by_identity(entity_rows, moved_identity);
    let old_successor_id = next_target_half_edge_id(entity_rows, relation_rows, moved_identity);
    let old_predecessor_id = prev_target_half_edge_id(entity_rows, relation_rows, moved_identity);
    let new_successor_id = find_entity_id_by_identity(entity_rows, new_successor_identity);
    let new_predecessor_id =
        prev_target_half_edge_id(entity_rows, relation_rows, new_successor_identity);
    let old_successor_identity = entity_rows
        .iter()
        .find(|row| query_entity_id_from_row(row) == old_successor_id)
        .expect("old successor should remain visible")
        .identity
        .clone();
    let old_predecessor_identity = entity_rows
        .iter()
        .find(|row| query_entity_id_from_row(row) == old_predecessor_id)
        .expect("old predecessor should remain visible")
        .identity
        .clone();
    let new_predecessor_identity = entity_rows
        .iter()
        .find(|row| query_entity_id_from_row(row) == new_predecessor_id)
        .expect("new predecessor should remain visible")
        .identity
        .clone();

    WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                moved_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            moved_half_edge_id,
            new_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                moved_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            moved_half_edge_id,
            new_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                &old_predecessor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                &old_successor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                &new_predecessor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            new_predecessor_id,
            moved_half_edge_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                new_successor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            new_successor_id,
            moved_half_edge_id,
        ),
    ])
    .expect("non-empty successor relocation batch")
}
