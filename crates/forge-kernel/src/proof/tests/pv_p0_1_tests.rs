//! PV Suite P0.1 — Geometric Invariant Validation Tests
//!
//! Tests that geometric validation catches:
//! - PV-01: Zero-area faces
//! - PV-02: Zero-length edges
//! - PV-03: Inverted shell (negative signed volume)
//! - PV-04: Degenerate loops (fewer than 3 distinct vertices)

use super::test_support::{insert_test_solid_shell, validate_geometric_invariants_all_faces};
use crate::geometry_state::GeometryState;
use crate::mesh_builder::make_cube;
use crate::integration_tests::harness::shapes::test_config;
use forge_core::{KernelError, TopologyError};
use forge_topo::handles::VertexId;
use forge_topo::validate::{validate_topology, ValidationLevel};

/// Build a position lookup closure from a GeometryState.
fn position_lookup(store: &GeometryState) -> impl Fn(VertexId) -> Option<[f64; 3]> + '_ {
    |vertex_id| store.get_vertex_position(vertex_id).copied()
}

/// PV-01: A face collapsed to zero area must be detected.
///
/// Strategy: Build a valid cube, then move vertices of one face to be
/// collinear (forming a degenerate face with zero area, but edges still
/// have non-zero length).
#[test]
fn pv_01_zero_area_face_detection() {
    let config = test_config();
    let result = make_cube([0.0, 0.0, 0.0], 2.0, &config).unwrap();
    let (topo, mut geom, _brep) = result.into_parts();
    let arena = topo.arena();

    let vertices: Vec<(VertexId, [f64; 3])> = arena
        .iter_vertices()
        .filter_map(|(vid, _)| geom.get_vertex_position(vid).map(|pos| (vid, *pos)))
        .collect();

    let plus_x: Vec<_> = vertices.iter().filter(|(_, pos)| pos[0] > 0.0).collect();

    assert!(plus_x.len() >= 4, "Should have 4 +X face vertices");

    for &(vid, _) in &plus_x {
        geom.set_vertex_position(*vid, [1.0, 0.0, 0.0]);
    }

    let lookup = position_lookup(&geom);
    let err = validate_geometric_invariants_all_faces(arena, &lookup, 1e-10, 1e-20);

    assert!(err.is_err(), "Should detect zero-area face");
    match err.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::ZeroAreaFace { .. },
            ..
        } => {}
        other => panic!("Expected ZeroAreaFace, got: {:?}", other),
    }
}

/// PV-02: An edge collapsed to zero length must be detected.
///
/// Strategy: Build a valid cube, then set two adjacent vertex positions
/// to the same point.
#[test]
fn pv_02_zero_length_edge_detection() {
    let config = test_config();
    let result = make_cube([0.0, 0.0, 0.0], 2.0, &config).unwrap();
    let (topo, mut geom, _brep) = result.into_parts();
    let arena = topo.arena();

    let he_id = arena.iter_half_edges().next().unwrap().0;
    let he_data = arena.get_half_edge(he_id).unwrap();
    let origin = he_data.origin();
    let twin_data = arena.get_half_edge(he_data.radial_next()).unwrap();
    let target = twin_data.origin();

    let origin_pos = *geom.get_vertex_position(origin).unwrap();
    geom.set_vertex_position(target, origin_pos);

    let lookup = position_lookup(&geom);
    let err = validate_geometric_invariants_all_faces(arena, &lookup, 1e-20, 1e-12);

    assert!(err.is_err(), "Should detect zero-length edge");
    match err.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::ZeroLengthEdge { .. },
            ..
        } => {}
        other => panic!("Expected ZeroLengthEdge, got: {:?}", other),
    }
}

/// PV-03: A shell with inverted normals has negative signed volume.
///
/// Strategy: Build a cube, then negate all vertex positions through
/// the origin to invert the winding order without changing face loops.
#[test]
fn pv_03_inverted_shell_signed_volume() {
    let config = test_config();
    let result = make_cube([0.0, 0.0, 0.0], 2.0, &config).unwrap();
    let (topo, mut geom, _brep) = result.into_parts();
    let arena = topo.arena();

    let vertices: Vec<(VertexId, [f64; 3])> = arena
        .iter_vertices()
        .filter_map(|(vid, _)| geom.get_vertex_position(vid).map(|pos| (vid, *pos)))
        .collect();

    for (vid, pos) in &vertices {
        geom.set_vertex_position(*vid, [-pos[0], -pos[1], -pos[2]]);
    }

    let lookup = position_lookup(&geom);
    let err = validate_geometric_invariants_all_faces(arena, &lookup, 1e-20, 1e-20);

    assert!(err.is_err(), "Should detect negative signed volume");
    match err.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::NegativeShellVolume { .. },
            ..
        } => {}
        other => panic!("Expected NegativeShellVolume, got: {:?}", other),
    }
}

/// PV-04: A face loop with fewer than 3 distinct vertices is degenerate.
///
/// Strategy: Build a 3-edge loop where one vertex appears twice,
/// giving 3 edges but only 2 distinct vertices.
#[test]
fn pv_04_degenerate_loop_detection() {
    use forge_topo::arena::{FaceData, HalfEdgeData, LoopData, VertexData};
    use forge_topo::handles::{FaceId, HalfEdgeId};
    use forge_topo::state::{DraftConfig, TopologyState};
    use forge_topo::validate::ValidationLevel;

    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation_with(config);

    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);

    let v0 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v1 = draft.insert_vertex(VertexData::new(placeholder_he));

    let v2 = draft.insert_vertex(VertexData::new(placeholder_he));

    let placeholder_shell = insert_test_solid_shell(&mut draft);
    let arena = draft.arena_mut();
    let placeholder_edge = forge_topo::handles::EdgeId::new(0, 0);

    let loop_id = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face = draft.insert_face(FaceData::new(loop_id, placeholder_shell));
    let arena = draft.arena_mut();
    arena
        .get_shell_mut(placeholder_shell)
        .unwrap()
        .set_representative_face(face);

    let he0 = draft.insert_half_edge(
        HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            v0,
            placeholder_edge,
        ),
    );
    let he1 = draft.insert_half_edge(
        HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            v1,
            placeholder_edge,
        ),
    );
    let he2 = draft.insert_half_edge(
        HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            v0,
            placeholder_edge,
        ),
    );
    let arena = draft.arena_mut();

    arena.get_half_edge_mut(he0).unwrap().set_next(he1);
    arena.get_half_edge_mut(he0).unwrap().set_prev(he2);
    arena.get_half_edge_mut(he1).unwrap().set_next(he2);
    arena.get_half_edge_mut(he1).unwrap().set_prev(he0);
    arena.get_half_edge_mut(he2).unwrap().set_next(he0);
    arena.get_half_edge_mut(he2).unwrap().set_prev(he1);

    let loop_id2 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face2 = draft.insert_face(FaceData::new(loop_id2, placeholder_shell));

    let twin0 = draft.insert_half_edge(
        HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face2,
            v1,
            placeholder_edge,
        ),
    );
    let twin1 = draft.insert_half_edge(
        HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face2,
            v0,
            placeholder_edge,
        ),
    );
    let twin2 = draft.insert_half_edge(
        HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face2,
            v2,
            placeholder_edge,
        ),
    );
    let arena = draft.arena_mut();

    arena.get_half_edge_mut(twin0).unwrap().set_next(twin1);
    arena.get_half_edge_mut(twin0).unwrap().set_prev(twin2);
    arena.get_half_edge_mut(twin1).unwrap().set_next(twin2);
    arena.get_half_edge_mut(twin1).unwrap().set_prev(twin0);
    arena.get_half_edge_mut(twin2).unwrap().set_next(twin0);
    arena.get_half_edge_mut(twin2).unwrap().set_prev(twin1);

    arena.get_half_edge_mut(he0).unwrap().set_radial_next(twin0);
    arena.get_half_edge_mut(twin0).unwrap().set_radial_next(he0);
    arena.get_half_edge_mut(he1).unwrap().set_radial_next(twin1);
    arena.get_half_edge_mut(twin1).unwrap().set_radial_next(he1);
    arena.get_half_edge_mut(he2).unwrap().set_radial_next(twin2);
    arena.get_half_edge_mut(twin2).unwrap().set_radial_next(he2);

    arena.get_loop_mut(loop_id).unwrap().set_half_edge(he0);
    arena.get_loop_mut(loop_id).unwrap().set_face(face);
    arena.get_loop_mut(loop_id2).unwrap().set_half_edge(twin0);
    arena.get_loop_mut(loop_id2).unwrap().set_face(face2);
    arena.get_vertex_mut(v0).unwrap().set_outgoing(he0);
    arena.get_vertex_mut(v1).unwrap().set_outgoing(he1);
    arena.get_vertex_mut(v2).unwrap().set_outgoing(twin2);

    let err = validate_topology(arena, ValidationLevel::Full);

    assert!(
        err.is_err(),
        "Should detect degenerate/broken loop topology"
    );
    match err.unwrap_err() {
        KernelError::TopologyViolation { .. } => {}
        other => panic!("Expected TopologyViolation, got: {:?}", other),
    }
}

/// Positive control: A valid cube passes all geometric invariants.
#[test]
fn valid_cube_passes_geometric_invariants() {
    let config = test_config();
    let result = make_cube([0.0, 0.0, 0.0], 2.0, &config).unwrap();
    let (topo, geom, _brep) = result.into_parts();
    let arena = topo.arena();

    let lookup = position_lookup(&geom);
    let result = validate_geometric_invariants_all_faces(arena, &lookup, 1e-10, 1e-12);
    assert!(result.is_ok(), "Valid cube should pass: {:?}", result.err());
}
