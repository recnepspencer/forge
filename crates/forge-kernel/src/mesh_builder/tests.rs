//! Tests for the mesh builder.

use forge_geom::spatial::bsp::{build_convex_polyhedron, BspConfig};
use forge_geom::Plane;
use crate::core::ModelingContext;
use super::eval::build_halfedge_mesh;

/// Build a unit cube centered at origin (±1 on each axis) from 6 planes.
fn build_unit_cube_cell() -> forge_geom::spatial::bsp::ConvexCell {
    let planes = vec![
        Plane::from_point_normal([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, -1.0, 0.0], [0.0, -1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, -1.0], [0.0, 0.0, -1.0]).unwrap(),
    ];
    build_convex_polyhedron(&planes, &BspConfig::default()).unwrap()
}

/// Build a tetrahedron from 4 planes.
fn build_tetrahedron_cell() -> forge_geom::spatial::bsp::ConvexCell {
    let planes = vec![
        Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, -1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]).unwrap(),
    ];
    build_convex_polyhedron(&planes, &BspConfig::default()).unwrap()
}

#[test]
fn cube_cell_has_correct_structure() {
    let cell = build_unit_cube_cell();
    assert_eq!(cell.vertex_count(), 8);
    assert!(cell.face_count() >= 6);
}

#[test]
fn cube_mesh_builds_successfully() {
    let cell = build_unit_cube_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx);
    assert!(result.is_ok(), "Mesh build failed: {:?}", result.err());
}

#[test]
fn cube_mesh_has_correct_vertex_count() {
    let cell = build_unit_cube_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    let arena = result.topology().arena();
    assert_eq!(arena.vertex_count(), 8);
}

#[test]
fn cube_mesh_has_vertex_positions() {
    let cell = build_unit_cube_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    assert_eq!(result.geometry().vertex_position_count(), 8);
}

#[test]
fn cube_mesh_has_face_planes() {
    let cell = build_unit_cube_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    assert!(result.geometry().face_plane_count() > 0);
}

#[test]
fn tetrahedron_cell_has_correct_structure() {
    let cell = build_tetrahedron_cell();
    assert_eq!(cell.vertex_count(), 4);
    assert_eq!(cell.face_count(), 4);
}

#[test]
fn tetrahedron_mesh_builds_successfully() {
    let cell = build_tetrahedron_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx);
    assert!(result.is_ok(), "Mesh build failed: {:?}", result.err());
}

#[test]
fn tetrahedron_mesh_has_correct_vertex_count() {
    let cell = build_tetrahedron_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    let arena = result.topology().arena();
    assert_eq!(arena.vertex_count(), 4);
}

#[test]
fn degenerate_cell_rejected() {
    let planes = vec![
        Plane::try_new([-1.0, 0.0, 0.0], 5.0).unwrap(),
        Plane::try_new([1.0, 0.0, 0.0], 5.0).unwrap(),
    ];
    let cell_result = build_convex_polyhedron(&planes, &BspConfig::default());
    if let Ok(cell) = cell_result {
        let mut ctx = ModelingContext::new();
        let result = build_halfedge_mesh(&cell, &mut ctx);
        assert!(result.is_err());
    }
}

#[test]
fn mesh_result_can_be_destructured() {
    let cell = build_unit_cube_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    let (topo, geom) = result.into_parts();
    assert_eq!(topo.arena().vertex_count(), 8);
    assert_eq!(geom.vertex_position_count(), 8);
}

// ══════════════════════════════════════════════════════════════
// Production-path structural validation tests
//
// These exercise the BSP→mesh pipeline output with the SAME
// invariant checks that the boolean pipeline (MB-N3/MB-N4) violates.
// ══════════════════════════════════════════════════════════════

#[test]
fn cube_mesh_twin_pairs_belong_to_different_faces() {
    let cell = build_unit_cube_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    let arena = result.topology().arena();

    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id == twin_id { continue; }

        let twin_data = arena.get_half_edge(twin_id).unwrap();
        assert_ne!(
            he_data.face(), twin_data.face(),
            "BSP cube: twin pair ({}, {}) both on face {} — OrientationInconsistency",
            he_id.index(), twin_id.index(), he_data.face().index()
        );
    }
}

#[test]
fn cube_mesh_manifold_edges() {
    let cell = build_unit_cube_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    let arena = result.topology().arena();

    let mut edge_faces: std::collections::BTreeMap<(u32, u32), Vec<u32>> =
        std::collections::BTreeMap::new();

    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id == twin_id { continue; }
        let canonical = (he_id.index().min(twin_id.index()), he_id.index().max(twin_id.index()));
        edge_faces.entry(canonical).or_default().push(he_data.face().index());
    }

    for ((lo, hi), faces) in &edge_faces {
        assert_eq!(faces.len(), 2,
            "BSP cube: edge ({},{}) has {} halfedges (expected 2): {:?}",
            lo, hi, faces.len(), faces);
        assert_ne!(faces[0], faces[1],
            "BSP cube: edge ({},{}) non-manifold — both on face {}",
            lo, hi, faces[0]);
    }
}

#[test]
fn cube_mesh_orientation_coherence() {
    let cell = build_unit_cube_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    let arena = result.topology().arena();

    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id == twin_id { continue; }

        let twin_data = arena.get_half_edge(twin_id).unwrap();
        let next_data = arena.get_half_edge(he_data.next()).unwrap();

        assert_eq!(
            next_data.origin(), twin_data.origin(),
            "BSP cube: orientation broken at ({},{}): next.origin={} != twin.origin={}",
            he_id.index(), twin_id.index(),
            next_data.origin().index(), twin_data.origin().index()
        );
    }
}

#[test]
fn cube_mesh_euler_formula() {
    let cell = build_unit_cube_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    let arena = result.topology().arena();

    let v = arena.vertex_count() as i64;
    let e = (arena.half_edge_count() / 2) as i64;
    let f = arena.face_count() as i64;
    let chi = v - e + f;

    assert_eq!(chi, 2,
        "BSP cube Euler: V={} - E={} + F={} = {} (expected χ=2)",
        v, e, f, chi);
}

#[test]
fn tetrahedron_mesh_twin_pairs_belong_to_different_faces() {
    let cell = build_tetrahedron_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    let arena = result.topology().arena();

    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id == twin_id { continue; }

        let twin_data = arena.get_half_edge(twin_id).unwrap();
        assert_ne!(
            he_data.face(), twin_data.face(),
            "BSP tet: twin pair ({}, {}) both on face {}",
            he_id.index(), twin_id.index(), he_data.face().index()
        );
    }
}

#[test]
fn tetrahedron_mesh_euler_formula() {
    let cell = build_tetrahedron_cell();
    let mut ctx = ModelingContext::new();
    let result = build_halfedge_mesh(&cell, &mut ctx).unwrap();
    let arena = result.topology().arena();

    let v = arena.vertex_count() as i64;
    let e = (arena.half_edge_count() / 2) as i64;
    let f = arena.face_count() as i64;
    let chi = v - e + f;

    assert_eq!(v, 4, "BSP tet: V={} (expected 4)", v);
    assert_eq!(e, 6, "BSP tet: E={} (expected 6)", e);
    assert_eq!(f, 4, "BSP tet: F={} (expected 4)", f);
    assert_eq!(chi, 2, "BSP tet: χ={} (expected 2)", chi);
}
