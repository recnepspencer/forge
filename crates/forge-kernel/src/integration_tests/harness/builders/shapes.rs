//! Shape factories for integration tests.
//!
//! DOMAIN: Creates real BSP-generated solids and returns `SolidEnvelope`.
//! Factories are the equivalent of Laravel's `Factory::create()` — they
//! produce real entities through the real pipeline.
//!
//! Use `with_tracing()` to wrap any factory call and get back a
//! `ModelingContext` alongside the result for observability assertions.

use crate::context::ModelingContext;
use forge_core::KernelError;
use forge_topo::handles::{FaceId, HalfEdgeId};

use crate::configuration::facade::ResolvedConfig;
use crate::context::scope::OperationScope;
use crate::engine::facade::SolidEnvelope;
use crate::operations::primitives;

use super::configs::test_config;

// ── Generic tracing wrapper ─────────────────────────────────────────────────

/// Run any factory function with full `ModelingContext` tracing.
///
/// Instead of writing a `_traced()` variant for every factory, wrap
/// the call:
///
/// ```rust,ignore
/// let (cube, ctx) = with_tracing(|scope| {
///     primitives::make_cube([0.0; 3], 1.0, scope)
/// })?;
/// ```
pub fn with_tracing<F, T>(f: F) -> Result<(T, ModelingContext), KernelError>
where
    F: FnOnce(&mut OperationScope<'_>) -> Result<T, KernelError>,
{
    let config = test_config();
    with_tracing_config(&config, f)
}

/// Run any factory function with tracing and a custom config.
pub fn with_tracing_config<F, T>(
    config: &ResolvedConfig,
    f: F,
) -> Result<(T, ModelingContext), KernelError>
where
    F: FnOnce(&mut OperationScope<'_>) -> Result<T, KernelError>,
{
    let mut ctx = ModelingContext::new();
    let result = {
        let mut scope = OperationScope::new(config, &mut ctx);
        f(&mut scope)?
    };
    Ok((result, ctx))
}

// ── Core factories ──────────────────────────────────────────────────────────

/// Build a unit cube centered at origin.
pub fn unit_cube() -> Result<SolidEnvelope, KernelError> {
    cube([0.0; 3], 1.0)
}

/// Build a cube at a given center and size.
pub fn cube(center: [f64; 3], size: f64) -> Result<SolidEnvelope, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    primitives::make_cube(center, size, &mut scope)
}

/// Build a tetrahedron centered at origin.
pub fn tetrahedron() -> Result<SolidEnvelope, KernelError> {
    tetrahedron_at([0.0; 3], 1.0)
}

/// Build a tetrahedron at a given center and scale.
pub fn tetrahedron_at(center: [f64; 3], scale: f64) -> Result<SolidEnvelope, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    primitives::make_tetrahedron(center, scale, &mut scope)
}

/// Build an axis-aligned block.
pub fn block(center: [f64; 3], half_extents: [f64; 3]) -> Result<SolidEnvelope, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    primitives::make_block(center, half_extents, &mut scope)
}

/// Build a regular dodecahedron.
pub fn dodecahedron(center: [f64; 3], radius: f64) -> Result<SolidEnvelope, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    primitives::make_dodecahedron(center, radius, &mut scope)
}

/// Build a prism (regular polygon extruded along Z).
pub fn prism(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<SolidEnvelope, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    primitives::make_prism(center, sides, radius, height, &mut scope)
}

/// Build a pyramid.
pub fn pyramid(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<SolidEnvelope, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    primitives::make_pyramid(center, sides, radius, height, &mut scope)
}

/// Build a wedge.
pub fn wedge(
    center: [f64; 3],
    half_extents: [f64; 3],
) -> Result<SolidEnvelope, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    primitives::make_wedge(center, half_extents, &mut scope)
}

/// Build any shape with a custom config.
pub fn cube_with_config(
    center: [f64; 3],
    size: f64,
    config: &ResolvedConfig,
) -> Result<SolidEnvelope, KernelError> {
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(config, &mut ctx);
    primitives::make_cube(center, size, &mut scope)
}

// ── Traced convenience functions ────────────────────────────────────────────

/// Build a unit cube with tracing.
pub fn unit_cube_traced() -> Result<(SolidEnvelope, ModelingContext), KernelError> {
    with_tracing(|scope| primitives::make_cube([0.0; 3], 1.0, scope))
}

/// Build an axis-aligned block with tracing.
pub fn unit_block_traced(
    center: [f64; 3],
    half_extents: [f64; 3],
) -> Result<(SolidEnvelope, ModelingContext), KernelError> {
    with_tracing(|scope| primitives::make_block(center, half_extents, scope))
}

/// Create a fresh `(ResolvedConfig, ModelingContext)` pair for manual tests.
pub fn traced_scope() -> (ResolvedConfig, ModelingContext) {
    (test_config(), ModelingContext::new())
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
