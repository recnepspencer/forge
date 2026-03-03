//! Group 2: Lineage DAG wiring tests [P3.3].
//!
//! Proves: Every entity created during primitive generation has a lineage
//! record linking back to its origin operation.
//!
//! NOTE: These tests are `#[ignore]` until lineage wiring is implemented
//! in `build_halfedge_mesh`. The `LineageStore` infrastructure exists in
//! `forge-topo` but primitive generation doesn't populate it yet.

use crate::integration_tests::harness::shapes::unit_cube;

/// Every face born during make_cube must have a lineage entry.
#[test]
#[ignore = "Phase 1.2: LineageStore not yet populated during primitive generation"]
fn test_make_block_lineage_covers_all_entities() {
    let result = unit_cube().expect("unit cube should succeed");
    let topo = result.get_value().topology();
    let _arena = topo.arena();

    // After Phase 1.2 wiring, the draft's lineage_store will be committed
    // into TopologyState. For now we verify the plumbing exists.
    //
    // When implemented:
    // let lineage = topo.lineage_store();
    // for (face_id, _) in arena.iter_faces() {
    //     let entity_ref = EntityRef::new(EntityKind::Face, face_id.index());
    //     assert!(
    //         lineage.get_lineage(&entity_ref).is_some(),
    //         "Face {} has no lineage entry", face_id.index()
    //     );
    // }
    //
    // assert_eq!(
    //     lineage.active_count(),
    //     arena.face_count() + arena.vertex_count() + arena.edge_count() + arena.half_edge_count(),
    //     "LineageStore entity count doesn't match arena total"
    // );
    todo!("Implement lineage wiring in build_halfedge_mesh");
}

/// Lineage events must be chronologically ordered with all EntityCreated.
#[test]
#[ignore = "Phase 1.2: LineageStore not yet populated during primitive generation"]
fn test_lineage_event_log_chronological() {
    let _result = unit_cube().expect("unit cube should succeed");

    // When implemented:
    // let lineage = result.topology.lineage_store();
    // let events = lineage.events();
    // assert!(!events.is_empty(), "Event log should be non-empty");
    //
    // // Fresh primitive = all events should be EntityCreated.
    // for event in events {
    //     assert!(
    //         matches!(event, LineageEvent::EntityCreated { .. }),
    //         "Fresh primitive should only have creation events, got: {:?}", event
    //     );
    // }
    todo!("Implement lineage wiring in build_halfedge_mesh");
}

/// Lineage root must contain the operation attribution (OpSignature).
#[test]
#[ignore = "Phase 1.2: LineageStore not yet populated during primitive generation"]
fn test_lineage_root_contains_operation_attribution() {
    let _result = unit_cube().expect("unit cube should succeed");

    // When implemented:
    // let lineage = result.topology.lineage_store();
    // let face_id = result.handles.faces[0];
    // let entity_ref = EntityRef::new(EntityKind::Face, face_id.index());
    // let face_lineage = lineage.get_lineage(&entity_ref).unwrap();
    //
    // assert_eq!(
    //     face_lineage.get_creation_op().get_name(),
    //     "build_halfedge_mesh"
    // );
    todo!("Implement lineage wiring in build_halfedge_mesh");
}

/// Euler ops (split_edge) must produce Lineage::Derived, not Root.
#[test]
#[ignore = "Phase 1.2: Euler operators don't emit lineage yet"]
fn test_euler_operator_produces_derived_lineage() {
    // Step 1: Build a block (all entities get Lineage::Root).
    // Step 2: split_edge on one edge.
    // Assert: New vertex and new edges have Lineage::Derived.
    // Assert: Derived lineage references the original EdgeId as ancestor.
    // Assert: lineage.creation_op().name == "split_edge".
    todo!("Implement lineage wiring in Euler operators");
}
