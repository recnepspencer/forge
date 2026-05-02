use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use worth_schema::facade::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};

use super::relation_update_successor_span_support::successor_span_relocation_batch;
use super::relation_update_support::{find_entity_id_by_identity, successor_cycle_identities};
use crate::edit::{
    WorthTopologyEditApplicationMode, WorthTopologyEditFamily, WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn query_native_edit_runner_executes_four_half_edge_span_relocation_on_larger_loop() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.rewire-successor-four-span",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 7 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.rewire-successor-four-span",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let moved_start_identity = relation_rows
        .iter()
        .find_map(|row| {
            (row.payload["topology"]["kind"].as_str()
                == Some(worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext.kind_name()))
            .then(|| row.payload["topology"]["source_identity"].as_str())
            .flatten()
        })
        .expect("sheet disk should expose halfedge successor wiring");
    let cycle = successor_cycle_identities(&entity_rows, &relation_rows, moved_start_identity, 7);
    let moved_start_id = find_entity_id_by_identity(&entity_rows, moved_start_identity);
    let moved_second_id = find_entity_id_by_identity(&entity_rows, cycle[1].as_str());
    let moved_third_id = find_entity_id_by_identity(&entity_rows, cycle[2].as_str());
    let moved_end_id = find_entity_id_by_identity(&entity_rows, cycle[3].as_str());
    let new_successor_id = find_entity_id_by_identity(&entity_rows, cycle[6].as_str());
    let batch = successor_span_relocation_batch(
        &entity_rows,
        &relation_rows,
        moved_start_identity,
        cycle[6].as_str(),
        4,
    );

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("four-halfedge span relocation should execute through the contiguous span lane");

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
    let moved_start = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_start_id)
        .expect("moved start halfedge should remain present");
    assert_eq!(moved_start.next_half_edge_id, Some(moved_second_id));
    let moved_second = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_second_id)
        .expect("moved second halfedge should remain present");
    assert_eq!(moved_second.next_half_edge_id, Some(moved_third_id));
    let moved_third = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_third_id)
        .expect("moved third halfedge should remain present");
    assert_eq!(moved_third.next_half_edge_id, Some(moved_end_id));
    let moved_end = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_end_id)
        .expect("moved end halfedge should remain present");
    assert_eq!(moved_end.next_half_edge_id, Some(new_successor_id));
}
