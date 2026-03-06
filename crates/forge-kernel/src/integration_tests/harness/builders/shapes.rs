//! Shape factories for integration tests.
//!
//! DOMAIN: Creates real BSP-generated solids and returns `SolidEnvelope`.
//! Factories are the equivalent of Laravel's `Factory::create()` — they
//! produce real entities through the real pipeline.
//!
//! Use `with_tracing()` to wrap any factory call and get back a
//! `ModelingContext` alongside the result for observability assertions.

use forge_core::{KernelError, OperationResult};
use forge_topo::handles::{FaceId, HalfEdgeId};

use crate::configuration::facade::ResolvedConfig;
use crate::engine::facade::SolidEnvelope;
use crate::operations::primitives;

use super::configs::test_config;

// ── Core factories ──────────────────────────────────────────────────────────

/// Build a unit cube centered at origin.
pub fn unit_cube() -> Result<OperationResult<SolidEnvelope>, KernelError> {
    cube([0.0; 3], 1.0)
}

/// Build a cube at a given center and size.
pub fn cube(center: [f64; 3], size: f64) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let config = test_config();
    primitives::make_cube(center, size, &config)
}

/// Build a tetrahedron centered at origin.
pub fn tetrahedron() -> Result<OperationResult<SolidEnvelope>, KernelError> {
    tetrahedron_at([0.0; 3], 1.0)
}

/// Build a tetrahedron at a given center and scale.
pub fn tetrahedron_at(
    center: [f64; 3],
    scale: f64,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let config = test_config();
    primitives::make_tetrahedron(center, scale, &config)
}

/// Build an axis-aligned block.
pub fn block(
    center: [f64; 3],
    half_extents: [f64; 3],
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let config = test_config();
    primitives::make_block(center, half_extents, &config)
}

/// Build a regular dodecahedron.
pub fn dodecahedron(
    center: [f64; 3],
    radius: f64,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let config = test_config();
    primitives::make_dodecahedron(center, radius, &config)
}

/// Build a prism (regular polygon extruded along Z).
pub fn prism(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let config = test_config();
    primitives::make_prism(center, sides, radius, height, &config)
}

/// Build a pyramid.
pub fn pyramid(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let config = test_config();
    primitives::make_pyramid(center, sides, radius, height, &config)
}

/// Build a wedge.
pub fn wedge(
    center: [f64; 3],
    half_extents: [f64; 3],
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let config = test_config();
    primitives::make_wedge(center, half_extents, &config)
}

/// Build any shape with a custom config.
pub fn cube_with_config(
    center: [f64; 3],
    size: f64,
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    primitives::make_cube(center, size, config)
}

// ── Topology traversal helpers ──────────────────────────────────────────────

/// Find the first halfedge of a given face.
pub fn first_halfedge_of_face(
    arena: &forge_topo::b_rep::TopologyArena,
    face: FaceId,
) -> Result<HalfEdgeId, KernelError> {
    let hes = arena.halfedges_of_face(face);
    if hes.is_empty() {
        return Err(KernelError::InvalidInput {
            message: format!("Face {} has no halfedges", face.index()),
            context: None,
        });
    }
    Ok(hes[0])
}

/// Collect all halfedge IDs around a face loop by walking `next` pointers.
pub fn collect_face_loop(
    arena: &forge_topo::b_rep::TopologyArena,
    start_he: HalfEdgeId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let mut result = vec![start_he];
    let mut current = arena.get_half_edge(start_he)?.next();

    let max_iterations = 1000;
    let mut i = 0;
    while current != start_he {
        result.push(current);
        current = arena.get_half_edge(current)?.next();
        i += 1;
        if i > max_iterations {
            return Err(KernelError::InvalidInput {
                message: "Loop walk exceeded 1000 iterations — broken loop".to_string(),
                context: None,
            });
        }
    }
    Ok(result)
}
