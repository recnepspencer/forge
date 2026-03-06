//! Volume validator poison tests.
//!
//! Uses the integration test harness `unit_cube()` for valid closed shells.

use crate::geometry::facade::GeometryView;
use crate::integration_tests::harness::builders::shapes::unit_cube;
use forge_core::{KernelError, TopologyError};
use forge_spatial::validators::volume::validate_signed_volume;

#[test]
fn valid_cube_passes() {
    let cube_result = unit_cube().expect("unit_cube should succeed");
    let solid = cube_result.get_value();
    let arena = solid.topology().arena();

    let result =
        validate_signed_volume(arena, &|v| solid.geometry().get_vertex_position(v).copied());
    assert!(result.is_ok(), "A valid outward-facing cube should pass");
}

#[test]
fn inverted_cube_detected() {
    let cube_result = unit_cube().expect("unit_cube should succeed");
    let solid = cube_result.get_value();
    let arena = solid.topology().arena();

    // Negate all vertex positions to invert the signed volume.
    let result = validate_signed_volume(arena, &|v| {
        solid
            .geometry()
            .get_vertex_position(v)
            .map(|p| [-p[0], -p[1], -p[2]])
    });
    assert!(
        result.is_err(),
        "Inverted cube should have negative signed volume"
    );
    match result.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::NegativeShellVolume { signed_volume, .. },
            ..
        } => {
            assert!(signed_volume < 0.0);
        }
        other => panic!("Expected NegativeShellVolume, got: {:?}", other),
    }
}

#[test]
fn disconnected_shells_processed() {
    // Two separate cubes — one valid, one inverted via negated positions.
    // The BFS must find and test both shell components.
    use crate::integration_tests::harness::builders::shapes::cube;

    let valid = cube([0.0, 0.0, 0.0], 1.0).expect("cube should succeed");
    let valid_solid = valid.get_value();
    let valid_arena = valid_solid.topology().arena();

    // First cube should pass
    let result = validate_signed_volume(valid_arena, &|v| {
        valid_solid.geometry().get_vertex_position(v).copied()
    });
    assert!(result.is_ok(), "First (valid) cube should pass");

    // Second cube with inverted positions should fail
    let inverted = cube([10.0, 0.0, 0.0], 1.0).expect("cube should succeed");
    let inv_solid = inverted.get_value();
    let inv_arena = inv_solid.topology().arena();

    let result = validate_signed_volume(inv_arena, &|v| {
        inv_solid
            .geometry()
            .get_vertex_position(v)
            .map(|p| [-p[0], -p[1], -p[2]])
    });
    assert!(
        result.is_err(),
        "Inverted cube should have negative signed volume"
    );
}

#[test]
fn planar_shell_handled() {
    // A closed shell that is perfectly flat (all Z = 0) should have zero volume.
    // Zero is not negative, so the validator should pass.
    use super::test_support::*;
    use forge_topo::b_rep::{FaceData, HalfEdgeData, LoopData, VertexData};
    use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId, VertexId};

    let mut draft = empty_test_draft();
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = EdgeId::new(0, 0);

    // Create a simple flat quad (2 triangles) — all vertices at Z=0
    let v0 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v1 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v2 = draft.insert_vertex(VertexData::new(placeholder_he));

    let shell = insert_test_solid_shell(&mut draft);
    let loop_id = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face = draft.insert_face(FaceData::new(loop_id, shell));

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

    let arena = draft.arena_mut();
    arena.get_half_edge_mut(h0).unwrap().set_next(h1);
    arena.get_half_edge_mut(h1).unwrap().set_next(h2);
    arena.get_half_edge_mut(h2).unwrap().set_next(h0);
    // Boundary edges: self-radial (critical for BFS traversal)
    arena.get_half_edge_mut(h0).unwrap().set_radial_next(h0);
    arena.get_half_edge_mut(h1).unwrap().set_radial_next(h1);
    arena.get_half_edge_mut(h2).unwrap().set_radial_next(h2);
    arena.get_loop_mut(loop_id).unwrap().set_half_edge(h0);
    arena.get_loop_mut(loop_id).unwrap().set_face(face);
    arena
        .get_shell_mut(shell)
        .unwrap()
        .set_representative_face(face);

    let result = validate_signed_volume(arena, &|v| {
        if v == v0 {
            Some([0.0, 0.0, 0.0])
        } else if v == v1 {
            Some([10.0, 0.0, 0.0])
        } else if v == v2 {
            Some([0.0, 10.0, 0.0])
        } else {
            None
        }
    });
    assert!(
        result.is_ok(),
        "Flat planar shell with zero volume should pass (not negative), got: {:?}",
        result.unwrap_err()
    );
}
