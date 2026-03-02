//! Binding completeness validation.
//!
//! DOMAIN: Checks that every topology entity required to have geometry
//! actually has a binding. Accepts callbacks instead of a concrete
//! geometry store, keeping forge-spatial decoupled from forge-kernel.
//!
//! DEPENDENCIES: forge-core (KernelError), forge-topo (TopologyArena, handles).

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};

/// Validate that every topology entity has required geometry assigned.
///
/// Currently checks: every face has a plane, every vertex has a position.
/// Surfaces, curves, and coedges are optional (planar Phase 1 may not have them).
///
/// The `has_face_plane` and `has_vertex_position` callbacks abstract over
/// the concrete geometry store — any type that can answer "does this entity
/// have geometry?" can be validated.
///
/// ```ignore
/// validate_geometry_completeness(
///     arena,
///     &|f| store.planes.contains(f),
///     &|v| store.positions.contains(v),
/// )?;
/// ```
pub fn validate_geometry_completeness(
    arena: &TopologyArena,
    has_face_plane: &dyn Fn(FaceId) -> bool,
    has_vertex_position: &dyn Fn(VertexId) -> bool,
) -> Result<(), KernelError> {
    for (face_id, _) in arena.iter_faces() {
        if !has_face_plane(face_id) {
            return Err(KernelError::InternalError {
                message: format!("Face {} has no plane binding", face_id),
                context: None,
            });
        }
    }

    for (vertex_id, _) in arena.iter_vertices() {
        if !has_vertex_position(vertex_id) {
            return Err(KernelError::InternalError {
                message: format!("Vertex {} has no position binding", vertex_id),
                context: None,
            });
        }
    }

    Ok(())
}
