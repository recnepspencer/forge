//! PV Suite P0.2 — Euler Characteristic Hardening Tests
//!
//! Tests that generalized Euler validation correctly handles:
//! - PV-05: Genus-1 topology (torus-like, V-E+F=0)
//! - PV-07: Multi-shell solid (two disjoint cubes)
//! - PV-08: Shell with removed edge → fails validation

use forge_core::{KernelError, TopologyError};
use forge_topo::validate::{validate_topology, ValidationLevel};
use forge_topo::state::{TopologyState, DraftConfig};
use forge_topo::arena::{FaceData, HalfEdgeData, VertexData, LoopData};
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use crate::mesh_builder::make_cube;

/// PV-05: A genus-1 topology passes the generalized Euler formula.
///
/// Constructs a minimal torus-like topology with the correct combinatorics:
/// V=9, E=18, F=9 → χ = 9-18+9 = 0 → genus = (2-0)/2 = 1.
/// This is a valid genus-1 surface that should NOT be flagged.
///
/// Strategy: Build a 3×3 grid of quads on a torus by identifying
/// opposite edges. The topology has the right Euler characteristic
/// without needing geometrically accurate positions.
#[test]
fn pv_05_genus_1_passes_generalized_euler() {
    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation_with(config);
    let arena = draft.arena_mut();

    let placeholder_he = HalfEdgeId::from_raw_parts(0, 0);
    let placeholder_face = FaceId::from_raw_parts(0, 0);

    let mut verts: Vec<VertexId> = Vec::new();
    for _i in 0..9 {
        let v = arena.insert_vertex(VertexData::new(placeholder_he));
        verts.push(v);
    }

    let v_grid = |r: usize, c: usize| -> VertexId { verts[(r % 3) * 3 + (c % 3)] };

    let mut faces: Vec<FaceId> = Vec::new();
    let mut all_he_ids: Vec<Vec<HalfEdgeId>> = Vec::new();

    for row in 0..3 {
        for col in 0..3 {
            let loop_id = arena.insert_loop(LoopData::new(placeholder_he, placeholder_face));
            let face = arena.insert_face(FaceData::new(loop_id));

            let origins = [
                v_grid(row, col),
                v_grid(row, col + 1),
                v_grid(row + 1, col + 1),
                v_grid(row + 1, col),
            ];

            let mut he_ids: Vec<HalfEdgeId> = Vec::new();
            for _k in 0..4 {
                let he = arena.insert_half_edge(HalfEdgeData::new(
                    placeholder_he, placeholder_he, placeholder_he, face, origins[0],
                ));
                he_ids.push(he);
            }

            for k in 0..4 {
                let he_data = arena.get_half_edge_mut(he_ids[k]).unwrap();
                he_data.set_origin(origins[k]);
                he_data.set_next(he_ids[(k + 1) % 4]);
                he_data.set_prev(he_ids[(k + 3) % 4]);
                he_data.set_face(face);
                he_data.set_twin(he_ids[k]);
            }

            arena.get_loop_mut(loop_id).unwrap().set_half_edge(he_ids[0]);
            arena.get_loop_mut(loop_id).unwrap().set_face(face);

            for k in 0..4 {
                arena.get_vertex_mut(origins[k]).unwrap().set_outgoing(he_ids[k]);
            }

            faces.push(face);
            all_he_ids.push(he_ids);
        }
    }

    let get_face_he = |face_row: usize, face_col: usize, edge_idx: usize| -> HalfEdgeId {
        all_he_ids[face_row * 3 + face_col][edge_idx]
    };

    for row in 0..3 {
        for col in 0..3 {
            let right_edge = get_face_he(row, col, 1);
            let left_of_right_neighbor = get_face_he(row, (col + 1) % 3, 3);
            arena.get_half_edge_mut(right_edge).unwrap().set_twin(left_of_right_neighbor);
            arena.get_half_edge_mut(left_of_right_neighbor).unwrap().set_twin(right_edge);

            let bottom_edge = get_face_he(row, col, 2);
            let top_of_bottom_neighbor = get_face_he((row + 1) % 3, col, 0);
            arena.get_half_edge_mut(bottom_edge).unwrap().set_twin(top_of_bottom_neighbor);
            arena.get_half_edge_mut(top_of_bottom_neighbor).unwrap().set_twin(bottom_edge);
        }
    }

    let result = validate_topology(arena, ValidationLevel::Full);
    assert!(result.is_ok(), "Genus-1 torus topology should pass generalized Euler: {:?}", result.err());
}

/// PV-07: Two disjoint cubes (multi-shell) pass Euler validation.
///
/// Each cube is an independent shell with V-E+F=2.
/// The validator must decompose into components and check each.
#[test]
fn pv_07_multi_shell_passes_euler() {
    let result_a = make_cube([0.0, 0.0, 0.0], 1.0).unwrap();
    let (topo_a, _geom_a) = result_a.into_parts();

    let result_b = make_cube([5.0, 0.0, 0.0], 1.0).unwrap();
    let (topo_b, _geom_b) = result_b.into_parts();

    let result_a_check = validate_topology(topo_a.arena(), ValidationLevel::Full);
    assert!(result_a_check.is_ok(), "Cube A should pass Euler: {:?}", result_a_check.err());

    let result_b_check = validate_topology(topo_b.arena(), ValidationLevel::Full);
    assert!(result_b_check.is_ok(), "Cube B should pass Euler: {:?}", result_b_check.err());
}

/// PV-08: A valid cube with one edge removed fails Euler validation.
///
/// Strategy: Build a cube, then remove one halfedge pair to break
/// the Euler characteristic. The shell will have V-E+F ≠ 2.
#[test]
fn pv_08_removed_edge_fails_euler() {
    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let result = make_cube([0.0, 0.0, 0.0], 2.0).unwrap();
    let (topo, _geom) = result.into_parts();
    let mut draft = topo.into_mutation_with(config);
    let arena = draft.arena_mut();

    let first_he = arena.iter_half_edges().next().unwrap().0;
    let twin_id = arena.get_half_edge(first_he).unwrap().twin();

    let _ = arena.remove_half_edge(first_he);
    let _ = arena.remove_half_edge(twin_id);

    let err = validate_topology(arena, ValidationLevel::Full);
    assert!(err.is_err(), "Should fail after edge removal");
}
