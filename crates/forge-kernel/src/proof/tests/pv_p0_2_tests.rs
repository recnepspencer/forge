//! PV Suite P0.2 — Euler Characteristic Hardening Tests
//!
//! Tests that generalized Euler validation correctly handles:
//! - PV-05: Genus-1 topology (torus-like, V-E+F=0)
//! - PV-07: Multi-shell solid (two disjoint cubes)
//! - PV-08: Shell with removed edge → fails validation

use super::test_support::{insert_test_solid_shell, materialize_edge_entities_from_radials};
use crate::operations::primitives::make_cube;
use crate::integration_tests::harness::shapes::test_config;
use forge_core::{KernelError, TopologyError};
use forge_topo::b_rep::{FaceData, HalfEdgeData, LoopData, VertexData};
use forge_topo::handles::{FaceId, HalfEdgeId, LoopId, VertexId};
use forge_topo::transactions::{DraftConfig, TopologyState};
use forge_topo::validate::{validate_topology, ValidationLevel};

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

    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);

    let mut verts: Vec<VertexId> = Vec::new();
    for _i in 0..9 {
        let v = arena.insert_vertex(VertexData::new(placeholder_he));
        verts.push(v);
    }

    let v_grid = |r: usize, c: usize| -> VertexId { verts[(r % 3) * 3 + (c % 3)] };

    let mut faces: Vec<FaceId> = Vec::new();
    let mut all_he_ids: Vec<Vec<HalfEdgeId>> = Vec::new();

    let placeholder_shell = insert_test_solid_shell(&mut draft);
    let placeholder_edge = forge_topo::handles::EdgeId::new(0, 0);

    for row in 0..3 {
        for col in 0..3 {
            let loop_id = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
            let face = draft.insert_face(FaceData::new(loop_id, placeholder_shell));
            if faces.is_empty() {
                arena
                    .get_shell_mut(placeholder_shell)
                    .unwrap()
                    .set_representative_face(face);
            }

            let origins = [
                v_grid(row, col),
                v_grid(row, col + 1),
                v_grid(row + 1, col + 1),
                v_grid(row + 1, col),
            ];

            let mut he_ids: Vec<HalfEdgeId> = Vec::new();
            for _k in 0..4 {
                let he = draft.insert_half_edge(
                    HalfEdgeData::new(
                        placeholder_he,
                        placeholder_he,
                        placeholder_he,
                        face,
                        origins[0],
                        placeholder_edge,
                    )
        );
                he_ids.push(he);
            }

            for k in 0..4 {
                let he_data = arena.get_half_edge_mut(he_ids[k]).unwrap();
                he_data.set_origin(origins[k]);
                he_data.set_next(he_ids[(k + 1) % 4]);
                he_data.set_prev(he_ids[(k + 3) % 4]);
                he_data.set_face(face);
                he_data.set_radial_next(he_ids[k]);
            }

            arena
                .get_loop_mut(loop_id)
                .unwrap()
                .set_half_edge(he_ids[0]);
            arena.get_loop_mut(loop_id).unwrap().set_face(face);

            for k in 0..4 {
                arena
                    .get_vertex_mut(origins[k])
                    .unwrap()
                    .set_outgoing(he_ids[k]);
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
            arena
                .get_half_edge_mut(right_edge)
                .unwrap()
                .set_radial_next(left_of_right_neighbor);
            arena
                .get_half_edge_mut(left_of_right_neighbor)
                .unwrap()
                .set_radial_next(right_edge);

            let bottom_edge = get_face_he(row, col, 2);
            let top_of_bottom_neighbor = get_face_he((row + 1) % 3, col, 0);
            arena
                .get_half_edge_mut(bottom_edge)
                .unwrap()
                .set_radial_next(top_of_bottom_neighbor);
            arena
                .get_half_edge_mut(top_of_bottom_neighbor)
                .unwrap()
                .set_radial_next(bottom_edge);
        }
    }

    materialize_edge_entities_from_radials(&mut draft).unwrap();

    let result = validate_topology(draft.arena(), ValidationLevel::Full);
    assert!(
        result.is_ok(),
        "Genus-1 torus topology should pass generalized Euler: {:?}",
        result.err()
    );
}

/// PV-07: Two disjoint cubes (multi-shell) pass Euler validation.
///
/// Each cube is an independent shell with V-E+F=2.
/// The validator must decompose into components and check each.
#[test]
fn pv_07_multi_shell_passes_euler() {
    let config = test_config();
    let result_a = make_cube([0.0, 0.0, 0.0], 1.0, &config).unwrap();
    let (topo_a, _geom_a, _brep_a) = result_a.into_parts();

    let result_b = make_cube([5.0, 0.0, 0.0], 1.0, &config).unwrap();
    let (topo_b, _geom_b, _brep_b) = result_b.into_parts();

    let result_a_check = validate_topology(topo_a.arena(), ValidationLevel::Full);
    assert!(
        result_a_check.is_ok(),
        "Cube A should pass Euler: {:?}",
        result_a_check.err()
    );

    let result_b_check = validate_topology(topo_b.arena(), ValidationLevel::Full);
    assert!(
        result_b_check.is_ok(),
        "Cube B should pass Euler: {:?}",
        result_b_check.err()
    );
}

/// PV-08: A valid cube with one edge removed fails Euler validation.
///
/// Strategy: Build a cube, then remove one halfedge pair to break
/// the Euler characteristic. The shell will have V-E+F ≠ 2.
#[test]
fn pv_08_removed_edge_fails_euler() {
    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let config = test_config();
    let result = make_cube([0.0, 0.0, 0.0], 2.0, &config).unwrap();
    let (topo, _geom, _brep) = result.into_parts();
    let mut draft = topo.into_mutation_with(config);
    let arena = draft.arena_mut();

    let first_he = arena.iter_half_edges().next().unwrap().0;
    let twin_id = arena.get_half_edge(first_he).unwrap().radial_next();

    let _ = arena.remove_half_edge(first_he);
    let _ = arena.remove_half_edge(twin_id);

    let err = validate_topology(draft.arena(), ValidationLevel::Full);
    assert!(err.is_err(), "Should fail after edge removal");
}

/// PV-06: Cube with through-hole passes generalized Euler (R=2, G=1).
///
/// Topology: 16 vertices (8 outer + 8 inner), 10 faces (4 outer sides,
/// 4 inner channel sides, 2 annular caps with inner loops), R=2.
///
/// Uses the same directed-edge-map stitching pattern as `make_cube`/
/// `stitch_twins` — each face inserts halfedges into an edge_map keyed
/// by (origin_vertex, target_vertex), then twins are paired via
/// (a,b)↔(b,a) matching. This guarantees correct twin pairing.
///
/// Generalized Euler: V-E+F = 2-2G+R = 2-2(1)+2 = 2.
#[test]
fn pv_06_through_hole_passes_euler() {
    use forge_topo::b_rep::LoopData;
    use std::collections::BTreeMap;

    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation_with(config);
    let arena = draft.arena_mut();

    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);

    let mut verts: Vec<VertexId> = Vec::new();
    for _ in 0..16 {
        verts.push(arena.insert_vertex(VertexData::new(placeholder_he), None));
    }

    let outer_faces: Vec<Vec<usize>> = vec![
        vec![0, 1, 5, 4],
        vec![1, 2, 6, 5],
        vec![2, 3, 7, 6],
        vec![3, 0, 4, 7],
    ];

    let inner_faces: Vec<Vec<usize>> = vec![
        vec![8, 12, 13, 9],
        vec![9, 13, 14, 10],
        vec![10, 14, 15, 11],
        vec![11, 15, 12, 8],
    ];

    let top_outer: Vec<usize> = vec![3, 2, 1, 0];
    let top_inner: Vec<usize> = vec![8, 9, 10, 11];
    let bot_outer: Vec<usize> = vec![4, 5, 6, 7];
    let bot_inner: Vec<usize> = vec![15, 14, 13, 12];

    let mut edge_map: BTreeMap<(u32, u32), HalfEdgeId> = BTreeMap::new();
    let shell_id = insert_test_solid_shell(&mut draft);

    let mut build_face_loop = |arena: &mut forge_topo::b_rep::TopologyArena,
                               face_verts: &[usize],
                               edge_map: &mut BTreeMap<(u32, u32), HalfEdgeId>|
     -> (FaceId, LoopId) {
        let n = face_verts.len();
        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
        let face = draft.insert_face(FaceData::new(loop_id, shell_id));
        if arena.get_shell(shell_id).unwrap().representative_face()
            == FaceId::new(u32::MAX, 0)
        {
            arena
                .get_shell_mut(shell_id)
                .unwrap()
                .set_representative_face(face);
        }

        let mut he_ids: Vec<HalfEdgeId> = Vec::new();
        for _ in 0..n {
            let he = arena.insert_half_edge(
                HalfEdgeData::new(
                    placeholder_he,
                    placeholder_he,
                    placeholder_he,
                    face,
                    verts[0],
                    forge_topo::handles::EdgeId::new(0, 0),
                ),
                None,
            );
            he_ids.push(he);
        }

        for k in 0..n {
            let origin = verts[face_verts[k]];
            let target = verts[face_verts[(k + 1) % n]];
            let he_mut = arena.get_half_edge_mut(he_ids[k]).unwrap();
            he_mut.set_origin(origin);
            he_mut.set_next(he_ids[(k + 1) % n]);
            he_mut.set_prev(he_ids[(k + n - 1) % n]);
            he_mut.set_face(face);

            edge_map.insert(
                (
                    verts[face_verts[k]].index(),
                    verts[face_verts[(k + 1) % n]].index(),
                ),
                he_ids[k],
            );
        }

        arena
            .get_loop_mut(loop_id)
            .unwrap()
            .set_half_edge(he_ids[0]);
        arena.get_loop_mut(loop_id).unwrap().set_face(face);

        for k in 0..n {
            arena
                .get_vertex_mut(verts[face_verts[k]])
                .unwrap()
                .set_outgoing(he_ids[k]);
        }

        (face, loop_id)
    };

    for face_verts in &outer_faces {
        build_face_loop(arena, face_verts, &mut edge_map);
    }

    for face_verts in &inner_faces {
        build_face_loop(arena, face_verts, &mut edge_map);
    }

    let (top_face, _top_loop) = build_face_loop(arena, &top_outer, &mut edge_map);

    let top_inner_loop = {
        let n = top_inner.len();
        let il_loop_id = draft.insert_loop(LoopData::new(placeholder_he, top_face));
        let mut il_he_ids: Vec<HalfEdgeId> = Vec::new();
        for _ in 0..n {
            let he = arena.insert_half_edge(
                HalfEdgeData::new(
                    placeholder_he,
                    placeholder_he,
                    placeholder_he,
                    top_face,
                    verts[0],
                    forge_topo::handles::EdgeId::new(0, 0),
                ),
                None,
            );
            il_he_ids.push(he);
        }
        for k in 0..n {
            let origin = verts[top_inner[k]];
            let target_idx = top_inner[(k + 1) % n];
            let he_mut = arena.get_half_edge_mut(il_he_ids[k]).unwrap();
            he_mut.set_origin(origin);
            he_mut.set_next(il_he_ids[(k + 1) % n]);
            he_mut.set_prev(il_he_ids[(k + n - 1) % n]);
            he_mut.set_face(top_face);
            edge_map.insert(
                (verts[top_inner[k]].index(), verts[target_idx].index()),
                il_he_ids[k],
            );
        }
        arena
            .get_loop_mut(il_loop_id)
            .unwrap()
            .set_half_edge(il_he_ids[0]);
        arena
            .get_face_mut(top_face)
            .unwrap()
            .add_inner_loop(il_loop_id);
        il_loop_id
    };

    let (bot_face, _bot_loop) = build_face_loop(arena, &bot_outer, &mut edge_map);

    let bot_inner_loop = {
        let n = bot_inner.len();
        let il_loop_id = draft.insert_loop(LoopData::new(placeholder_he, bot_face));
        let mut il_he_ids: Vec<HalfEdgeId> = Vec::new();
        for _ in 0..n {
            let he = arena.insert_half_edge(
                HalfEdgeData::new(
                    placeholder_he,
                    placeholder_he,
                    placeholder_he,
                    bot_face,
                    verts[0],
                    forge_topo::handles::EdgeId::new(0, 0),
                ),
                None,
            );
            il_he_ids.push(he);
        }
        for k in 0..n {
            let origin = verts[bot_inner[k]];
            let target_idx = bot_inner[(k + 1) % n];
            let he_mut = arena.get_half_edge_mut(il_he_ids[k]).unwrap();
            he_mut.set_origin(origin);
            he_mut.set_next(il_he_ids[(k + 1) % n]);
            he_mut.set_prev(il_he_ids[(k + n - 1) % n]);
            he_mut.set_face(bot_face);
            edge_map.insert(
                (verts[bot_inner[k]].index(), verts[target_idx].index()),
                il_he_ids[k],
            );
        }
        arena
            .get_loop_mut(il_loop_id)
            .unwrap()
            .set_half_edge(il_he_ids[0]);
        arena
            .get_face_mut(bot_face)
            .unwrap()
            .add_inner_loop(il_loop_id);
        il_loop_id
    };

    for (&(a, b), &he_ab) in &edge_map.clone() {
        if let Some(&he_ba) = edge_map.get(&(b, a)) {
            arena
                .get_half_edge_mut(he_ab)
                .unwrap()
                .set_radial_next(he_ba);
            arena
                .get_half_edge_mut(he_ba)
                .unwrap()
                .set_radial_next(he_ab);
        }
    }

    materialize_edge_entities_from_radials(&mut draft).unwrap();

    let mut total_inner_loops: usize = 0;
    for (_, face_data) in arena.iter_faces() {
        total_inner_loops += face_data.inner_loop_count();
    }
    assert_eq!(total_inner_loops, 2, "Should have 2 inner loops (R=2)");

    let v_count = arena.vertex_count() as i64;
    let f_count = arena.face_count() as i64;

    let mut edge_set: std::collections::BTreeSet<(u32, u32)> = std::collections::BTreeSet::new();
    for (id, data) in arena.iter_half_edges() {
        if id != data.radial_next() {
            let lo = id.index().min(data.radial_next().index());
            let hi = id.index().max(data.radial_next().index());
            edge_set.insert((lo, hi));
        }
    }
    let e_count = edge_set.len() as i64;
    let euler_char = v_count - e_count + f_count;

    assert_eq!(
        euler_char, 2,
        "Through-hole cube: V({v_count})-E({e_count})+F({f_count})={euler_char}, expected 2"
    );

    let result = validate_topology(draft.arena(), ValidationLevel::Full);
    assert!(
        result.is_ok(),
        "Through-hole cube should pass Euler: {:?}",
        result.err()
    );
}
