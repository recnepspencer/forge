//! Group 4: Serialization round-trip tests [P3.5-lite].
//!
//! Proves: TopologyState, GeometryStore, and DecisionLog survive
//! serialization round-trips with structural identity preserved.

use crate::integration_tests::harness::shapes::unit_cube;

/// TopologyState serialization round-trip preserves hash and entity counts.
#[test]
fn test_topology_state_roundtrip() {
    let result = unit_cube().expect("unit cube should succeed");
    let topo = result.get_value().topology();

    let serialized = serde_json::to_vec(topo).expect("TopologyState should serialize");
    let deserialized: forge_topo::transactions::TopologyState =
        serde_json::from_slice(&serialized).expect("TopologyState should deserialize");

    assert_eq!(
        topo.topology_hash(),
        deserialized.topology_hash(),
        "Topology hash must survive round-trip"
    );

    let original_faces = topo.arena().iter_faces().count();
    let deser_faces = deserialized.arena().iter_faces().count();
    assert_eq!(original_faces, deser_faces, "Face count must match");

    let original_verts = topo.arena().iter_vertices().count();
    let deser_verts = deserialized.arena().iter_vertices().count();
    assert_eq!(original_verts, deser_verts, "Vertex count must match");
}

/// GeometryStore serialization round-trip preserves planes and positions.
#[test]
fn test_geometry_store_roundtrip() {
    let result = unit_cube().expect("unit cube should succeed");
    let envelope = result.get_value();
    let geom = envelope.geometry();

    let serialized = serde_json::to_vec(geom).expect("GeometryStore should serialize");
    let deserialized: crate::geometry::facade::GeometryStore =
        serde_json::from_slice(&serialized).expect("GeometryStore should deserialize");

    // Verify plane count matches.
    assert_eq!(
        geom.planes.len(),
        deserialized.planes.len(),
        "Plane count must survive round-trip"
    );

    // Verify position count matches.
    assert_eq!(
        geom.positions.len(),
        deserialized.positions.len(),
        "Position count must survive round-trip"
    );
}

/// DecisionLog serialization round-trip preserves decision count and kinds.
#[test]
fn test_decision_log_roundtrip() {
    let result = unit_cube().expect("unit cube should succeed");
    let log = result.get_decision_log();

    let serialized = serde_json::to_vec(log).expect("DecisionLog should serialize");
    let deserialized: forge_core::DecisionLog =
        serde_json::from_slice(&serialized).expect("DecisionLog should deserialize");

    assert_eq!(
        log.len(),
        deserialized.len(),
        "Decision count must survive round-trip"
    );

    // Verify decision kinds match.
    let original_kinds: Vec<_> = log.decisions()
        .map(|d: &forge_core::TracedDecision| std::mem::discriminant(d.get_kind()))
        .collect();
    let deser_kinds: Vec<_> = deserialized.decisions()
        .map(|d: &forge_core::TracedDecision| std::mem::discriminant(d.get_kind()))
        .collect();
    assert_eq!(
        original_kinds, deser_kinds,
        "Decision kinds must be preserved through round-trip"
    );
}

/// Same inputs produce identical topology — deterministic replay.
#[test]
fn test_replay_same_inputs_same_topology() {
    fn build_cube() -> (u128, usize) {
        let result = unit_cube().expect("unit cube should succeed");
        let hash = result.get_value().topology().topology_hash();
        let decision_count = result.get_decision_log().len();
        (hash, decision_count)
    }

    let (hash_a, decisions_a) = build_cube();
    let (hash_b, decisions_b) = build_cube();

    assert_eq!(
        hash_a, hash_b,
        "Identical inputs must produce identical topology hash"
    );
    assert_eq!(
        decisions_a, decisions_b,
        "Identical inputs must produce identical decision count"
    );
}

/// Lineage events must survive TopologyState serialization round-trip.
#[test]
fn test_lineage_events_survive_serialization() {
    use forge_topo::provenance::{LineageEvent, LineageStore};

    let result = unit_cube().expect("unit cube should succeed");
    let topo = result.get_value().topology();
    let original_events = topo.lineage_events();

    assert!(!original_events.is_empty(), "Lineage events should be non-empty");

    // Serialize and deserialize the entire TopologyState.
    let serialized = serde_json::to_vec(topo).expect("TopologyState should serialize");
    let deserialized: forge_topo::transactions::TopologyState =
        serde_json::from_slice(&serialized).expect("TopologyState should deserialize");

    let deser_events = deserialized.lineage_events();

    // Same event count.
    assert_eq!(
        original_events.len(), deser_events.len(),
        "Lineage event count must survive round-trip"
    );

    // Rebuild stores from both and compare active counts.
    let orig_store = LineageStore::from_prior_events(original_events);
    let deser_store = LineageStore::from_prior_events(deser_events);

    assert_eq!(
        orig_store.active_count(), deser_store.active_count(),
        "Lineage active count must survive round-trip"
    );

    // All events should be EntityCreated for a fresh primitive.
    for (i, event) in deser_events.iter().enumerate() {
        assert!(
            matches!(event, LineageEvent::EntityCreated { .. }),
            "Deserialized event {} should be EntityCreated, got {:?}", i, event
        );
    }

    // Op attribution preserved.
    for eref in orig_store.tracked_entities() {
        let orig_lineage = orig_store.get_lineage(eref).unwrap();
        let deser_lineage = deser_store.get_lineage(eref)
            .expect("Deserialized store should track same entity");
        assert_eq!(
            orig_lineage.get_creation_op().get_name(),
            deser_lineage.get_creation_op().get_name(),
            "Op name must survive round-trip for {:?}", eref
        );
    }
}
