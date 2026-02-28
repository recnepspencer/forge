//! PV Suite P0.4 — Non-Manifold Edge Detection Tests
//!
//! Tests that non-manifold edge detection:
//! - PV-11: Constructed non-manifold edge → validator rejects
//! - PV-12: Valid cube passes manifold check (positive control)

use crate::mesh_builder::make_cube;
use forge_core::{KernelError, TopologyError};
use forge_topo::b_rep::{FaceData, HalfEdgeData, LoopData, VertexData};
use forge_topo::handles::{FaceId, HalfEdgeId};
use forge_topo::transactions::{DraftConfig, TopologyState};
use forge_topo::validate::{validate_topology, ValidationLevel};

/// PV-11: A non-manifold edge (3+ halfedges sharing a canonical key) is detected.
///
/// Strategy: Build a valid cube, then insert an extra halfedge pair
/// that duplicates an existing edge's canonical key (same slot indices).
/// The manifoldness check counts halfedge pairs per canonical edge
/// and flags any edge with more than 2 halfedges.
#[test]
fn pv_11_non_manifold_edge_detected() {
    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let result = make_cube([0.0, 0.0, 0.0], 2.0).unwrap();
    let (topo, _geom) = result.into_parts();
    let mut draft = topo.into_mutation_with(config);
    let arena = draft.arena_mut();

    let (he_id, he_twin, he_face, he_origin) = {
        let (id, data) = arena
            .iter_half_edges()
            .filter(|(id, d)| *id != d.radial_next())
            .next()
            .unwrap();
        (id, data.radial_next(), data.face(), data.origin())
    };

    let twin_data = arena.get_half_edge(he_twin).unwrap();
    let twin_face = twin_data.face();
    let twin_origin = twin_data.origin();

    let extra_a = HalfEdgeData::new(
        he_twin,
        he_id,
        he_id,
        he_face,
        he_origin,
        forge_topo::handles::EdgeId::new(0, 0),
    );
    let extra_b = HalfEdgeData::new(
        he_id,
        he_twin,
        he_twin,
        twin_face,
        twin_origin,
        forge_topo::handles::EdgeId::new(0, 0),
    );
    let (extra_a_id, extra_b_id) = arena.insert_radial_pair(extra_a, extra_b);

    let _ = extra_a_id;
    let _ = extra_b_id;

    let err = validate_topology(arena, ValidationLevel::Full);
    assert!(err.is_err(), "Should detect non-manifold edge");
}

/// PV-12: A valid cube passes manifold check (positive control).
#[test]
fn pv_12_valid_cube_passes_manifold() {
    let result = make_cube([0.0, 0.0, 0.0], 2.0).unwrap();
    let (topo, _geom) = result.into_parts();
    let arena = topo.arena();

    let result = validate_topology(arena, ValidationLevel::Full);
    assert!(
        result.is_ok(),
        "Valid cube should pass manifold check: {:?}",
        result.err()
    );
}
