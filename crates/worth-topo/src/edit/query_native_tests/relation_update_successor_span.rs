use worth_schema::facade::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};

use super::relation_update_successor_span_support::{
    successor_span_relocation_batch, two_half_edge_span_relocation_batch,
};
use super::relation_update_support::{
    find_entity_id_by_identity, half_edge_identities_for_different_loops, prev_target_half_edge_id,
    successor_cycle_identities,
};
use crate::edit::{
    WorthTopologyEditApplicationMode, WorthTopologyEditFamily,
    WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn query_native_edit_runner_executes_two_half_edge_span_relocation_successor_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.rewire-successor-span",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.rewire-successor-span")
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
    let cycle = successor_cycle_identities(&entity_rows, &relation_rows, moved_start_identity, 6);
    let moved_end_identity = cycle[1].as_str();
    let old_successor_identity = cycle[2].as_str();
    let new_predecessor_identity = cycle[3].as_str();
    let new_successor_identity = cycle[4].as_str();
    let moved_start_id = find_entity_id_by_identity(&entity_rows, moved_start_identity);
    let moved_end_id = find_entity_id_by_identity(&entity_rows, moved_end_identity);
    let old_predecessor_id =
        prev_target_half_edge_id(&entity_rows, &relation_rows, moved_start_identity);
    let old_successor_id = find_entity_id_by_identity(&entity_rows, old_successor_identity);
    let new_predecessor_id = find_entity_id_by_identity(&entity_rows, new_predecessor_identity);
    let new_successor_id = find_entity_id_by_identity(&entity_rows, new_successor_identity);
    let batch = two_half_edge_span_relocation_batch(
        &entity_rows,
        &relation_rows,
        moved_start_identity,
        new_successor_identity,
    );

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect(
            "two-halfedge span relocation workflow should execute through admitted successor lane",
        );

    assert_eq!(execution.families.len(), 6);
    assert!(execution
        .families
        .iter()
        .all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor));
    let moved_start = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_start_id)
        .expect("moved start halfedge should remain present");
    assert_eq!(moved_start.prev_half_edge_id, Some(new_predecessor_id));
    assert_eq!(moved_start.next_half_edge_id, Some(moved_end_id));
    let moved_end = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_end_id)
        .expect("moved end halfedge should remain present");
    assert_eq!(moved_end.prev_half_edge_id, Some(moved_start_id));
    assert_eq!(moved_end.next_half_edge_id, Some(new_successor_id));
    let old_predecessor = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == old_predecessor_id)
        .expect("old predecessor should remain present");
    assert_eq!(old_predecessor.next_half_edge_id, Some(old_successor_id));
    let old_successor = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == old_successor_id)
        .expect("old successor should remain present");
    assert_eq!(old_successor.prev_half_edge_id, Some(old_predecessor_id));
}

#[test]
fn query_native_edit_runner_executes_three_half_edge_span_relocation_successor_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.rewire-successor-three-span",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.rewire-successor-three-span",
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
    let cycle = successor_cycle_identities(&entity_rows, &relation_rows, moved_start_identity, 6);
    let moved_mid_identity = cycle[1].as_str();
    let moved_end_identity = cycle[2].as_str();
    let old_successor_identity = cycle[3].as_str();
    let new_predecessor_identity = cycle[4].as_str();
    let new_successor_identity = cycle[5].as_str();
    let moved_start_id = find_entity_id_by_identity(&entity_rows, moved_start_identity);
    let moved_mid_id = find_entity_id_by_identity(&entity_rows, moved_mid_identity);
    let moved_end_id = find_entity_id_by_identity(&entity_rows, moved_end_identity);
    let old_predecessor_id =
        prev_target_half_edge_id(&entity_rows, &relation_rows, moved_start_identity);
    let old_successor_id = find_entity_id_by_identity(&entity_rows, old_successor_identity);
    let new_predecessor_id = find_entity_id_by_identity(&entity_rows, new_predecessor_identity);
    let new_successor_id = find_entity_id_by_identity(&entity_rows, new_successor_identity);
    let batch = successor_span_relocation_batch(
        &entity_rows,
        &relation_rows,
        moved_start_identity,
        new_successor_identity,
        3,
    );

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("three-halfedge span relocation workflow should execute through admitted successor lane");

    assert_eq!(execution.families.len(), 6);
    assert!(execution
        .families
        .iter()
        .all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor));
    let moved_start = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_start_id)
        .expect("moved start halfedge should remain present");
    assert_eq!(moved_start.prev_half_edge_id, Some(new_predecessor_id));
    assert_eq!(moved_start.next_half_edge_id, Some(moved_mid_id));
    let moved_mid = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_mid_id)
        .expect("moved middle halfedge should remain present");
    assert_eq!(moved_mid.prev_half_edge_id, Some(moved_start_id));
    assert_eq!(moved_mid.next_half_edge_id, Some(moved_end_id));
    let moved_end = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_end_id)
        .expect("moved end halfedge should remain present");
    assert_eq!(moved_end.prev_half_edge_id, Some(moved_mid_id));
    assert_eq!(moved_end.next_half_edge_id, Some(new_successor_id));
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
fn query_native_edit_runner_denies_cross_loop_two_half_edge_span_relocation_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.rewire-successor-span-cross-loop",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.rewire-successor-span-cross-loop",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let (moved_start_identity, new_successor_identity) =
        half_edge_identities_for_different_loops(&entity_rows, &relation_rows);
    let batch = two_half_edge_span_relocation_batch(
        &entity_rows,
        &relation_rows,
        &moved_start_identity,
        &new_successor_identity,
    );

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("cross-loop two-halfedge span relocation must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor)
    ));
}

#[test]
fn query_native_edit_runner_denies_degenerate_two_half_edge_span_relocation_before_current_successor(
) {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.rewire-successor-span-degenerate",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.rewire-successor-span-degenerate",
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
    let cycle = successor_cycle_identities(&entity_rows, &relation_rows, moved_start_identity, 3);
    let old_successor_identity = cycle[2].as_str();
    let batch = two_half_edge_span_relocation_batch(
        &entity_rows,
        &relation_rows,
        moved_start_identity,
        old_successor_identity,
    );

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("degenerate same-loop two-halfedge span relocation must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor)
    ));
}

#[test]
fn query_native_edit_runner_denies_three_half_edge_span_relocation_before_internal_member() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.rewire-successor-three-span-internal",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.rewire-successor-three-span-internal",
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
    let cycle = successor_cycle_identities(&entity_rows, &relation_rows, moved_start_identity, 3);
    let internal_successor_identity = cycle[1].as_str();
    let batch = successor_span_relocation_batch(
        &entity_rows,
        &relation_rows,
        moved_start_identity,
        internal_successor_identity,
        3,
    );

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("span relocation before an internal member must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor)
    ));
}
