//! PV Suite P0.3 — Orientation Canonicalization Tests
//!
//! Tests that orientation consistency validation:
//! - PV-09: Valid cube passes orientation check (positive control)
//! - PV-10: Cube with forced twin-same-face → detected

use forge_core::{KernelError, TopologyError};
use forge_topo::validate::{validate_geometric_invariants, validate_topology, ValidationLevel};
use forge_topo::handles::VertexId;
use forge_topo::state::{TopologyState, DraftConfig};
use crate::mesh_builder::make_cube;
use crate::geometry_store::GeometryStore;

/// Build a position lookup closure from a GeometryStore.
fn position_lookup(store: &GeometryStore) -> impl Fn(VertexId) -> Option<[f64; 3]> + '_ {
    |vertex_id| store.get_vertex_position(vertex_id).copied()
}

/// PV-09: A valid cube passes all orientation checks.
///
/// Positive control: make_cube produces a correctly oriented solid.
#[test]
fn pv_09_valid_cube_passes_orientation() {
    let result = make_cube([0.0, 0.0, 0.0], 2.0).unwrap();
    let (topo, geom) = result.into_parts();
    let arena = topo.arena();

    let lookup = position_lookup(&geom);
    let result = validate_geometric_invariants(arena, &lookup, 1e-10, 1e-12);
    assert!(result.is_ok(), "Valid cube should pass orientation: {:?}", result.err());
}

/// PV-10: A cube with a twin pair forced to share the same face is detected.
///
/// Strategy: Build a valid cube, then patch one twin's face pointer
/// to point to its sibling's face. This simulates an inverted face
/// where adjacent faces have incompatible winding.
#[test]
fn pv_10_twin_same_face_detected() {
    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let result = make_cube([0.0, 0.0, 0.0], 2.0).unwrap();
    let (topo, geom) = result.into_parts();

    let mut draft = topo.into_mutation_with(config);
    let arena = draft.arena_mut();

    let (he_id, twin_id, he_face) = {
        let (id, data) = arena.iter_half_edges()
            .filter(|(id, d)| *id != d.twin())
            .next()
            .unwrap();
        (id, data.twin(), data.face())
    };

    arena.get_half_edge_mut(twin_id).unwrap().set_face(he_face);

    let lookup = position_lookup(&geom);
    let err = validate_geometric_invariants(arena, &lookup, 1e-10, 1e-12);

    assert!(err.is_err(), "Should detect same-face twin pair");
    match err.unwrap_err() {
        KernelError::TopologyViolation { err: TopologyError::OrientationInconsistency { .. }, .. } => {}
        other => panic!("Expected OrientationInconsistency, got: {:?}", other),
    }
}
