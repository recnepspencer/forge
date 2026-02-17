//! Shared test fixtures for Boolean operation tests.
//!
//! Centralizes `build_cube` and `face_centroid` so every test module
//! uses the same construction logic without duplication.

use forge_geom::bsp::{build_convex_polyhedron, BspConfig};
use forge_geom::plane::Plane;
use forge_topo::state::TopologyState;

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use crate::mesh_builder::build_halfedge_mesh;
use super::eval::compute_face_centroid;
use super::schema::{BooleanInput, BooleanOp, BooleanResult};
use super::assemble::execute_boolean;

/// Build a cube mesh from 6 axis-aligned planes.
pub fn build_cube(
    center: [f64; 3],
    half_size: f64,
) -> (TopologyState, GeometryStore) {
    let planes = vec![
        Plane::from_point_normal(
            [center[0] + half_size, center[1], center[2]],
            [1.0, 0.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0] - half_size, center[1], center[2]],
            [-1.0, 0.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1] + half_size, center[2]],
            [0.0, 1.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1] - half_size, center[2]],
            [0.0, -1.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1], center[2] + half_size],
            [0.0, 0.0, 1.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1], center[2] - half_size],
            [0.0, 0.0, -1.0],
        ).unwrap(),
    ];
    let cell = build_convex_polyhedron(&planes, &BspConfig::default()).unwrap();
    let mut ctx = ModelingContext::new();
    build_halfedge_mesh(&cell, &mut ctx).unwrap().into_parts()
}

/// Compute face centroid for test assertions (wraps the shared eval function).
pub fn face_centroid(
    arena: &forge_topo::arena::TopologyArena,
    geom: &GeometryStore,
    face: forge_topo::handles::FaceId,
) -> [f64; 3] {
    compute_face_centroid(arena, geom, face).unwrap()
}

/// Execute a boolean and return the result, panicking with context on failure.
pub fn run_boolean(
    center_a: [f64; 3],
    half_a: f64,
    center_b: [f64; 3],
    half_b: f64,
    op: BooleanOp,
) -> BooleanResult {
    let (topo_a, geom_a) = build_cube(center_a, half_a);
    let (topo_b, geom_b) = build_cube(center_b, half_b);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, op);
    execute_boolean(input).unwrap_or_else(|e| {
        panic!("Boolean {:?} failed: {:?}", op, e);
    })
}

/// Attempt a boolean, returning the Result for tests that expect errors.
pub fn try_boolean(
    center_a: [f64; 3],
    half_a: f64,
    center_b: [f64; 3],
    half_b: f64,
    op: BooleanOp,
) -> Result<BooleanResult, forge_core::KernelError> {
    let (topo_a, geom_a) = build_cube(center_a, half_a);
    let (topo_b, geom_b) = build_cube(center_b, half_b);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, op);
    execute_boolean(input)
}
