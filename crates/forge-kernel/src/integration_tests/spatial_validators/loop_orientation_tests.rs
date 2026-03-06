//! Loop orientation validator poison tests.
//!
//! Validates face loop winding direction: outer loops must be CCW,
//! inner loops must be CW relative to the face Newell normal.
//!
//! IMPORTANT: This validator computes the Newell normal FROM the outer loop
//! positions themselves. Therefore, for a single-loop face, the projected
//! signed area against its own normal is always positive regardless of
//! the actual winding in world space. The validator detects inconsistencies
//! between outer and inner loops, not absolute CW/CCW orientation.

use super::test_support::*;
use forge_core::{FlatToleranceProvider, KernelError, TopologyError};
use forge_spatial::validators::loop_orientation::validate_loop_orientation;
use forge_topo::b_rep::{FaceData, HalfEdgeData, LoopData, VertexData};
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId, VertexId};

// ── Baseline ────────────────────────────────────────────────────────────

#[test]
fn valid_3d_planar_face_passes() {
    let mut draft = empty_test_draft();
    let (_face, v0, v1, v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();

    let result = validate_loop_orientation(
        arena,
        &|v| {
            if v == v0 {
                Some([1.0, 0.0, 0.0])
            } else if v == v1 {
                Some([0.0, 1.0, 0.0])
            } else if v == v2 {
                Some([0.0, 0.0, 1.0])
            } else {
                None
            }
        },
        &|_| true,
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(result.is_ok(), "Valid CCW triangle on 3D plane should pass");
}

#[test]
fn single_face_any_winding_passes() {
    // A single-loop face with CW winding computes a Newell normal pointing
    // in the "CW" direction. The projected signed area against THAT normal
    // is still positive. This is correct validator behavior: absolute orientation
    // is enforced by the *signed volume* validator, not loop orientation.
    let mut draft = empty_test_draft();
    let (_face, v0, v1, v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();

    let result = validate_loop_orientation(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([0.0, 1.0, 0.0])
            } else if v == v2 {
                Some([1.0, 0.0, 0.0])
            } else {
                None
            }
        },
        &|_| true,
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(
        result.is_ok(),
        "Single outer loop with CW winding is self-consistent"
    );
}

// ── Poison ──────────────────────────────────────────────────────────────

#[test]
fn inner_loop_ccw_detected() {
    // Build a face with an outer loop (CCW) and one inner loop (also CCW — WRONG!).
    // Inner loops must be CW (negative projected signed area relative to the face normal).
    let mut draft = empty_test_draft();
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = EdgeId::new(0, 0);

    // 6 vertices: 3 for outer, 3 for inner
    let v0 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v1 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v2 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v3 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v4 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v5 = draft.insert_vertex(VertexData::new(placeholder_he));

    let shell = insert_test_solid_shell(&mut draft);
    let outer_loop = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let inner_loop = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face = draft.insert_face(FaceData::new(outer_loop, shell));

    // Outer loop: v0 → v1 → v2 (CCW)
    let h0 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v0,
        placeholder_edge,
    ));
    let h1 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v1,
        placeholder_edge,
    ));
    let h2 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v2,
        placeholder_edge,
    ));

    // Inner loop: v3 → v4 → v5 (will be CCW — WRONG! should be CW)
    let hi0 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v3,
        placeholder_edge,
    ));
    let hi1 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v4,
        placeholder_edge,
    ));
    let hi2 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v5,
        placeholder_edge,
    ));

    {
        let arena = draft.arena_mut();
        // Wire outer loop
        arena.get_half_edge_mut(h0).unwrap().set_next(h1);
        arena.get_half_edge_mut(h1).unwrap().set_next(h2);
        arena.get_half_edge_mut(h2).unwrap().set_next(h0);
        arena.get_half_edge_mut(h0).unwrap().set_prev(h2);
        arena.get_half_edge_mut(h1).unwrap().set_prev(h0);
        arena.get_half_edge_mut(h2).unwrap().set_prev(h1);

        // Wire inner loop
        arena.get_half_edge_mut(hi0).unwrap().set_next(hi1);
        arena.get_half_edge_mut(hi1).unwrap().set_next(hi2);
        arena.get_half_edge_mut(hi2).unwrap().set_next(hi0);
        arena.get_half_edge_mut(hi0).unwrap().set_prev(hi2);
        arena.get_half_edge_mut(hi1).unwrap().set_prev(hi0);
        arena.get_half_edge_mut(hi2).unwrap().set_prev(hi1);

        arena.get_loop_mut(outer_loop).unwrap().set_half_edge(h0);
        arena.get_loop_mut(outer_loop).unwrap().set_face(face);
        arena.get_loop_mut(inner_loop).unwrap().set_half_edge(hi0);
        arena.get_loop_mut(inner_loop).unwrap().set_face(face);
        arena
            .get_shell_mut(shell)
            .unwrap()
            .set_representative_face(face);

        // Register inner loop on the face
        arena.get_face_mut(face).unwrap().loops.add_inner(inner_loop);
    }

    let arena = draft.arena();
    let result = validate_loop_orientation(
        arena,
        &|v| {
            // Outer: large CCW triangle on XY plane
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([10.0, 0.0, 0.0])
            } else if v == v2 {
                Some([0.0, 10.0, 0.0])
            }
            // Inner: small CCW triangle (WRONG! should be CW)
            else if v == v3 {
                Some([1.0, 1.0, 0.0])
            } else if v == v4 {
                Some([2.0, 1.0, 0.0])
            } else if v == v5 {
                Some([1.0, 2.0, 0.0])
            } else {
                None
            }
        },
        &|_| true,
        &FlatToleranceProvider::new(1e-10),
    );

    assert!(result.is_err(), "Inner loop winding CCW should be caught");
    match result.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::OrientationInconsistency { .. },
            ..
        } => {}
        other => panic!("Expected OrientationInconsistency, got: {:?}", other),
    }
}

#[test]
fn collinear_vertices_skipped() {
    let mut draft = empty_test_draft();
    let (_face, v0, v1, v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();

    let result = validate_loop_orientation(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([5.0, 5.0, 5.0])
            } else if v == v2 {
                Some([10.0, 10.0, 10.0])
            } else {
                None
            }
        },
        &|_| true,
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(
        result.is_ok(),
        "Collinear vertices (zero normal) should be skipped"
    );
}

#[test]
fn numerical_jitter_near_zero() {
    let mut draft = empty_test_draft();
    let (_face, v0, v1, v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();

    let result = validate_loop_orientation(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([1e-10, 0.0, 0.0])
            } else if v == v2 {
                Some([0.0, 1e-10, 0.0])
            } else {
                None
            }
        },
        &|_| true,
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(
        result.is_ok(),
        "Micro-face with tiny positive area should not fail"
    );
}

#[test]
fn non_planar_face_skipped() {
    let mut draft = empty_test_draft();
    let (_face, v0, v1, v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();

    let result = validate_loop_orientation(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([0.0, 1.0, 0.0])
            } else if v == v2 {
                Some([1.0, 0.0, 0.0])
            } else {
                None
            }
        },
        &|_| false, // Not planar — skip
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(
        result.is_ok(),
        "Non-planar face should be skipped even with CW geometry"
    );
}

// ── Integration ─────────────────────────────────────────────────────────

#[test]
fn valid_cube_passes_orientation() {
    // Full cube created via make_cube — all faces should have consistent orientation.
    use crate::geometry::facade::GeometryView;
    use crate::integration_tests::harness::builders::shapes::unit_cube;

    let cube_result = unit_cube().expect("unit_cube should succeed");
    let solid = cube_result.get_value();
    let arena = solid.topology().arena();

    let result = validate_loop_orientation(
        arena,
        &|v| solid.geometry().get_vertex_position(v).copied(),
        &|_| true,
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(
        result.is_ok(),
        "All faces of a valid cube should have consistent loop orientation"
    );
}
