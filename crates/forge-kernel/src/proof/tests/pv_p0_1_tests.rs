//! PV Suite P0.1 — Geometric Invariant Validation Tests
//!
//! Tests that geometric validation catches:
//! - PV-01: Zero-area faces
//! - PV-02: Zero-length edges
//! - PV-03: Inverted shell (negative signed volume)
//! - PV-04: Degenerate loops (fewer than 3 distinct vertices)
//! - PV-05: Inner loop winding CCW instead of CW
//! - PV-06: Adjacent faces with same-direction half-edges across shared edge

use super::test_support::{insert_test_solid_shell, validate_geometric_invariants_all_faces};
use crate::geometry::facade::{ExactPosition, GeometryStore, GeometryView};
use crate::integration_tests::harness::builders::configs::test_config;
use crate::operations::primitives::make_cube;
use forge_core::{FlatToleranceProvider, KernelError, TopologyError};
use forge_spatial::{validate_geometric_invariants, GeometryContext};
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::validate::{validate_topology, ValidationLevel};

/// Build a position lookup closure from a GeometryStore.
fn position_lookup(store: &GeometryStore) -> impl Fn(VertexId) -> Option<[f64; 3]> + '_ {
    |vertex_id| store.get_vertex_position(vertex_id).copied()
}

/// Build a face-plane lookup closure from a GeometryStore.
fn plane_lookup(
    store: &GeometryStore,
) -> impl Fn(FaceId) -> Option<worth_geom::facade::Plane> + '_ {
    |face_id| store.planes.get(face_id).cloned()
}

/// Build an edge-curve-kind lookup closure from a GeometryStore.
fn curve_lookup(
    store: &GeometryStore,
) -> impl Fn(forge_topo::handles::EdgeId) -> Option<worth_geom::facade::CurveKind> + '_ {
    |edge_id| store.curves.get(edge_id).map(|curve| curve.kind.clone())
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
    let (topo, mut geom) = result.into_value().into_parts();
    let arena = topo.arena();

    let vertices: Vec<(VertexId, [f64; 3])> = arena
        .iter_vertices()
        .filter_map(|(vid, _)| geom.get_vertex_position(vid).map(|pos| (vid, *pos)))
        .collect();

    let plus_x: Vec<_> = vertices.iter().filter(|(_, pos)| pos[0] > 0.0).collect();

    assert!(plus_x.len() >= 4, "Should have 4 +X face vertices");

    // Place all +X face vertices along a line (collinear) so the face
    // has zero area but edges remain non-zero-length.
    for (i, &(vid, _)) in plus_x.iter().enumerate() {
        let t = i as f64;
        geom.positions
            .set(*vid, ExactPosition::from_f64([1.0, t, 0.0]));
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
    let (topo, mut geom) = result.into_value().into_parts();
    let arena = topo.arena();

    let he_id = arena.iter_half_edges().next().unwrap().0;
    let he_data = arena.get_half_edge(he_id).unwrap();
    let origin = he_data.origin();
    let twin_data = arena.get_half_edge(he_data.radial_next()).unwrap();
    let target = twin_data.origin();

    let origin_pos = *geom.get_vertex_position(origin).unwrap();
    geom.positions
        .set(target, ExactPosition::from_f64(origin_pos));

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
    let (topo, mut geom) = result.into_value().into_parts();
    let arena = topo.arena();

    let vertices: Vec<(VertexId, [f64; 3])> = arena
        .iter_vertices()
        .filter_map(|(vid, _)| geom.get_vertex_position(vid).map(|pos| (vid, *pos)))
        .collect();

    for (vid, pos) in &vertices {
        geom.positions
            .set(*vid, ExactPosition::from_f64([-pos[0], -pos[1], -pos[2]]));
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
    use forge_topo::b_rep::{FaceData, HalfEdgeData, LoopData, VertexData};
    use forge_topo::handles::{FaceId, HalfEdgeId};
    use forge_topo::transactions::{DraftConfig, TopologyState};
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

    let he0 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v0,
        placeholder_edge,
    ));
    let he1 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v1,
        placeholder_edge,
    ));
    let he2 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v0,
        placeholder_edge,
    ));
    let arena = draft.arena_mut();

    arena.get_half_edge_mut(he0).unwrap().set_next(he1);
    arena.get_half_edge_mut(he0).unwrap().set_prev(he2);
    arena.get_half_edge_mut(he1).unwrap().set_next(he2);
    arena.get_half_edge_mut(he1).unwrap().set_prev(he0);
    arena.get_half_edge_mut(he2).unwrap().set_next(he0);
    arena.get_half_edge_mut(he2).unwrap().set_prev(he1);

    let loop_id2 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face2 = draft.insert_face(FaceData::new(loop_id2, placeholder_shell));

    let twin0 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face2,
        v1,
        placeholder_edge,
    ));
    let twin1 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face2,
        v0,
        placeholder_edge,
    ));
    let twin2 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face2,
        v2,
        placeholder_edge,
    ));
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
    arena.get_vertex_mut(v0).unwrap().set_primary_disk(he0);
    arena.get_vertex_mut(v1).unwrap().set_primary_disk(he1);
    arena.get_vertex_mut(v2).unwrap().set_primary_disk(twin2);

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
    let (topo, geom) = result.into_value().into_parts();
    let arena = topo.arena();

    let lookup = position_lookup(&geom);
    let planes = plane_lookup(&geom);
    let curves = curve_lookup(&geom);
    let tol = FlatToleranceProvider::new((1e-10f64).sqrt().max(1e-12));
    let ctx = GeometryContext {
        position_fn: &lookup,
        plane_fn: &planes,
        is_planar: &|_| true,
        curve_fn: &curves,
        tolerance_provider: &tol,
    };
    let result = validate_geometric_invariants(arena, &ctx);
    assert!(result.is_ok(), "Valid cube should pass: {:?}", result.err());
}

/// PV-05: An inner loop winding CCW (same as outer loop) is detected.
///
/// Strategy: Build a face with an outer loop and an inner loop,
/// and set their vertex positions so both wind the same direction.
#[test]
fn pv_05_inner_loop_wrong_orientation() {
    use forge_topo::b_rep::{FaceData, HalfEdgeData, LoopData, VertexData};
    use forge_topo::handles::{FaceId, HalfEdgeId};
    use forge_topo::transactions::{DraftConfig, TopologyState};
    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation_with(config);
    let mut geom = GeometryStore::default();

    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = forge_topo::handles::EdgeId::new(0, 0);

    // Outer loop vertices
    let v0 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v1 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v2 = draft.insert_vertex(VertexData::new(placeholder_he));

    // Set positions CCW
    geom.positions
        .set(v0, ExactPosition::from_f64([0.0, 0.0, 0.0]));
    geom.positions
        .set(v1, ExactPosition::from_f64([10.0, 0.0, 0.0]));
    geom.positions
        .set(v2, ExactPosition::from_f64([0.0, 10.0, 0.0]));

    // Inner loop vertices (winding CCW to trigger failure)
    let v3 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v4 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v5 = draft.insert_vertex(VertexData::new(placeholder_he));

    // CCW inner loop!
    geom.positions
        .set(v3, ExactPosition::from_f64([2.0, 2.0, 0.0]));
    geom.positions
        .set(v4, ExactPosition::from_f64([8.0, 2.0, 0.0]));
    geom.positions
        .set(v5, ExactPosition::from_f64([2.0, 8.0, 0.0]));

    let placeholder_shell = insert_test_solid_shell(&mut draft);

    let outer_loop_id = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let inner_loop_id = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));

    let face = draft.insert_face(FaceData::new(outer_loop_id, placeholder_shell));

    // Manually push inner loop
    {
        let arena = draft.arena_mut();
        arena
            .get_face_mut(face)
            .unwrap()
            .loops
            .add_inner(inner_loop_id);
        arena
            .get_shell_mut(placeholder_shell)
            .unwrap()
            .set_representative_face(face);
        arena.get_loop_mut(outer_loop_id).unwrap().set_face(face);
        arena.get_loop_mut(inner_loop_id).unwrap().set_face(face);
    }

    // Outer loop halfedges
    let ho0 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v0,
        placeholder_edge,
    ));
    let ho1 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v1,
        placeholder_edge,
    ));
    let ho2 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v2,
        placeholder_edge,
    ));

    // Inner loop halfedges
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
        arena.get_half_edge_mut(ho0).unwrap().set_next(ho1);
        arena.get_half_edge_mut(ho1).unwrap().set_next(ho2);
        arena.get_half_edge_mut(ho2).unwrap().set_next(ho0);

        // Ensure radial_next loops back to self (boundary edges) to pass boundary tests
        arena.get_half_edge_mut(ho0).unwrap().set_radial_next(ho0);
        arena.get_half_edge_mut(ho1).unwrap().set_radial_next(ho1);
        arena.get_half_edge_mut(ho2).unwrap().set_radial_next(ho2);

        arena.get_half_edge_mut(hi0).unwrap().set_next(hi1);
        arena.get_half_edge_mut(hi1).unwrap().set_next(hi2);
        arena.get_half_edge_mut(hi2).unwrap().set_next(hi0);

        arena.get_half_edge_mut(hi0).unwrap().set_radial_next(hi0);
        arena.get_half_edge_mut(hi1).unwrap().set_radial_next(hi1);
        arena.get_half_edge_mut(hi2).unwrap().set_radial_next(hi2);

        arena
            .get_loop_mut(outer_loop_id)
            .unwrap()
            .set_half_edge(ho0);
        arena
            .get_loop_mut(inner_loop_id)
            .unwrap()
            .set_half_edge(hi0);
    }

    let arena = draft.arena();
    let lookup = position_lookup(&geom);

    // Call validate_loop_orientation explicitly
    let err = forge_spatial::validators::loop_orientation::validate_loop_orientation(
        arena,
        &lookup,
        &|_| true,
        &forge_core::FlatToleranceProvider::new(1e-10),
    );

    assert!(err.is_err(), "Should detect inner loop winding CCW");
    match err.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::OrientationInconsistency { .. },
            ..
        } => {}
        other => panic!("Expected OrientationInconsistency, got: {:?}", other),
    }
}

/// PV-06: Shell orientation consistency detects adjacent faces with parallel half-edges.
///
/// Strategy: Build two faces sharing a geometric edge, but both half-edges
/// run A -> B.
#[test]
fn pv_06_shell_orientation_inconsistency() {
    use forge_topo::b_rep::{FaceData, HalfEdgeData, LoopData, VertexData};
    use forge_topo::handles::{FaceId, HalfEdgeId};
    use forge_topo::transactions::{DraftConfig, TopologyState};
    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation_with(config);
    let mut geom = GeometryStore::default();

    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = forge_topo::handles::EdgeId::new(0, 0);

    // The shared edge vertices
    let v_a = draft.insert_vertex(VertexData::new(placeholder_he));
    let v_b = draft.insert_vertex(VertexData::new(placeholder_he));
    geom.positions
        .set(v_a, ExactPosition::from_f64([0.0, 0.0, 0.0]));
    geom.positions
        .set(v_b, ExactPosition::from_f64([1.0, 0.0, 0.0]));

    // Face 1 outer vertex
    let v_c = draft.insert_vertex(VertexData::new(placeholder_he));
    geom.positions
        .set(v_c, ExactPosition::from_f64([0.0, 1.0, 0.0]));

    // Face 2 outer vertex
    let v_d = draft.insert_vertex(VertexData::new(placeholder_he));
    geom.positions
        .set(v_d, ExactPosition::from_f64([0.0, -1.0, 0.0]));

    let placeholder_shell = insert_test_solid_shell(&mut draft);

    let loop1 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face1 = draft.insert_face(FaceData::new(loop1, placeholder_shell));

    let loop2 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face2 = draft.insert_face(FaceData::new(loop2, placeholder_shell));

    // Face 1: A -> B -> C -> A
    let h1_ab = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face1,
        v_a,
        placeholder_edge,
    ));
    let h1_bc = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face1,
        v_b,
        placeholder_edge,
    ));
    let h1_ca = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face1,
        v_c,
        placeholder_edge,
    ));

    // Face 2: A -> B -> D -> A  (Notice A -> B is same direction as Face 1!)
    let h2_ab = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face2,
        v_a,
        placeholder_edge,
    ));
    let h2_bd = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face2,
        v_b,
        placeholder_edge,
    ));
    let h2_da = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face2,
        v_d,
        placeholder_edge,
    ));

    {
        let arena = draft.arena_mut();

        arena.get_loop_mut(loop1).unwrap().set_half_edge(h1_ab);
        arena.get_loop_mut(loop1).unwrap().set_face(face1);
        arena.get_loop_mut(loop2).unwrap().set_half_edge(h2_ab);
        arena.get_loop_mut(loop2).unwrap().set_face(face2);

        // Face 1 link
        arena.get_half_edge_mut(h1_ab).unwrap().set_next(h1_bc);
        arena.get_half_edge_mut(h1_bc).unwrap().set_next(h1_ca);
        arena.get_half_edge_mut(h1_ca).unwrap().set_next(h1_ab);

        // Face 2 link
        arena.get_half_edge_mut(h2_ab).unwrap().set_next(h2_bd);
        arena.get_half_edge_mut(h2_bd).unwrap().set_next(h2_da);
        arena.get_half_edge_mut(h2_da).unwrap().set_next(h2_ab);

        // Link the shared edge radially
        arena
            .get_half_edge_mut(h1_ab)
            .unwrap()
            .set_radial_next(h2_ab);
        arena
            .get_half_edge_mut(h2_ab)
            .unwrap()
            .set_radial_next(h1_ab);

        // Boundary edges
        arena
            .get_half_edge_mut(h1_bc)
            .unwrap()
            .set_radial_next(h1_bc);
        arena
            .get_half_edge_mut(h1_ca)
            .unwrap()
            .set_radial_next(h1_ca);
        arena
            .get_half_edge_mut(h2_bd)
            .unwrap()
            .set_radial_next(h2_bd);
        arena
            .get_half_edge_mut(h2_da)
            .unwrap()
            .set_radial_next(h2_da);
    }

    let arena = draft.arena();
    let lookup = position_lookup(&geom);

    let err = forge_spatial::validators::shell_orientation::validate_shell_orientation(
        arena,
        &lookup,
        &forge_core::FlatToleranceProvider::new(1e-10),
    );

    assert!(
        err.is_err(),
        "Should detect parallel half-edges in shared geometry"
    );
    match err.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::ValidatorFailure { .. },
            ..
        } => {}
        other => panic!("Expected ValidatorFailure, got: {:?}", other),
    }
}
