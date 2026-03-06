//! Binding completeness validation.
//!
//! DOMAIN: Checks that every topology entity required to have geometry
//! actually has a binding. Accepts callbacks instead of a concrete
//! geometry store, keeping forge-spatial decoupled from forge-kernel.
//!
//! DEPENDENCIES: forge-core (KernelError), forge-topo (TopologyArena, handles).

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{EdgeId, FaceId, VertexId};

/// Validate that every topology entity has required geometry assigned.
///
/// **Required checks** (always run):
/// - Every face has a plane binding.
/// - Every vertex has a position binding.
///
/// **Optional checks** (run when callback is `Some`):
/// - Every face has a surface binding (`has_face_surface`).
/// - Every edge has a curve binding (`has_edge_curve`).
///
/// Making surface/curve checks optional preserves backward compatibility —
/// existing callers pass `None` and continue working. New callers opt in
/// to stricter Phase 2b+ completeness requirements.
///
/// ```ignore
/// validate_geometry_completeness(
///     arena,
///     &|f| store.planes.contains(f),
///     &|v| store.positions.contains(v),
///     Some(&|f| store.surfaces.contains(f)),
///     Some(&|e| store.curves.contains(e)),
/// )?;
/// ```
pub fn validate_geometry_completeness(
    arena: &TopologyArena,
    has_face_plane: &dyn Fn(FaceId) -> bool,
    has_vertex_position: &dyn Fn(VertexId) -> bool,
    has_face_surface: Option<&dyn Fn(FaceId) -> bool>,
    has_edge_curve: Option<&dyn Fn(EdgeId) -> bool>,
) -> Result<(), KernelError> {
    for (face_id, _) in arena.iter_faces() {
        if !has_face_plane(face_id) {
            return Err(KernelError::InternalError {
                message: format!("Face {} has no plane binding", face_id),
                context: None,
            });
        }

        if let Some(check) = has_face_surface {
            if !check(face_id) {
                return Err(KernelError::InternalError {
                    message: format!("Face {} has no surface binding", face_id),
                    context: None,
                });
            }
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

    if let Some(check) = has_edge_curve {
        for (edge_id, _) in arena.iter_edges() {
            if !check(edge_id) {
                return Err(KernelError::InternalError {
                    message: format!("Edge {} has no curve binding", edge_id),
                    context: None,
                });
            }
        }
    }

    Ok(())
}
