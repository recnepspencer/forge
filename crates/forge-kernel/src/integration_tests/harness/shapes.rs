//! Shape builders for integration tests.
//!
//! DOMAIN: Creates real BSP-generated solids and returns `SolidEnvelope`
//! (handles extracted lazily via `OnceCell`) for structural tests, or
//! `(SolidEnvelope, ModelingContext)` for observability tests.
//!
//! When lineage or persistent naming lands, update these builders once —
//! all tests absorb the change automatically.

use crate::context::ModelingContext;
use forge_core::KernelError;
use forge_topo::handles::{FaceId, HalfEdgeId};

use crate::configuration::facade::{resolve_config, KernelConfig, ResolvedConfig};
use crate::context::scope::OperationScope;
use crate::engine::facade::SolidEnvelope;
use crate::operations::primitives;

/// Build a default test config.
pub fn test_config() -> ResolvedConfig {
    resolve_config(&KernelConfig::default(), None, None, None).unwrap()
}

/// Build a unit cube centered at origin.
///
/// Returns a `SolidEnvelope` with lazily-extracted handles.
/// Access `envelope.body()`, `envelope.faces()`, etc. for handle inspection.
pub fn unit_cube() -> Result<SolidEnvelope, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    primitives::make_cube([0.0, 0.0, 0.0], 1.0, &mut scope)
}

/// Build a tetrahedron centered at origin.
///
/// Returns a `SolidEnvelope` with lazily-extracted handles.
pub fn tetrahedron() -> Result<SolidEnvelope, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    primitives::make_tetrahedron([0.0, 0.0, 0.0], 1.0, &mut scope)
}

/// Find the first halfedge of a given face by traversal.
///
/// Traverses the face's halfedge list via the index to find a starting halfedge.
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

// ── Traced builders (production-grade observability tests) ──────────────────

/// Build a unit cube with `ModelingContext` (real `DecisionSink`).
///
/// Returns `(SolidEnvelope, ModelingContext)` so observability tests can
/// assert on `DecisionLog`, lineage, spans — everything the production
/// path produces.
pub fn unit_cube_traced() -> Result<(SolidEnvelope, ModelingContext), KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let envelope = {
        let mut scope = OperationScope::new(&config, &mut ctx);
        primitives::make_cube([0.0, 0.0, 0.0], 1.0, &mut scope)?
    };
    Ok((envelope, ctx))
}

/// Build an axis-aligned block with `ModelingContext` (real `DecisionSink`).
pub fn unit_block_traced(
    center: [f64; 3],
    half_extents: [f64; 3],
) -> Result<(SolidEnvelope, ModelingContext), KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let envelope = {
        let mut scope = OperationScope::new(&config, &mut ctx);
        primitives::make_block(center, half_extents, &mut scope)?
    };
    Ok((envelope, ctx))
}

/// Create a fresh `(ResolvedConfig, ModelingContext)` pair for manual pipeline tests.
///
/// The caller can destructure these into an `OperationScope` and drive
/// multi-step operations while keeping access to the `ModelingContext`
/// for post-operation assertions on `DecisionLog`.
pub fn traced_scope() -> (ResolvedConfig, ModelingContext) {
    (test_config(), ModelingContext::new())
}
