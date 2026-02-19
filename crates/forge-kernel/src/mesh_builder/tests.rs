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
