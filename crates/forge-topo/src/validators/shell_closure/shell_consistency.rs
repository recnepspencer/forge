//! Shell consistency validator (solid shells must be watertight).
//!
//! INVARIANT: Solid shells must not contain boundary edges.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

pub(crate) fn validate_shell_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    for (shell_id, shell_data) in arena.iter_shells() {
        if matches!(shell_data.kind(), crate::b_rep::ShellKind::Solid(_)) {
            for (face_id, face_data) in arena.iter_faces() {
                if face_data.shell() == shell_id {
                    let iter = crate::queries::traverse::FaceEdgeIterator::new(arena, face_id)?;
                    for he_res in iter {
                        let he_id = he_res?;
                        if crate::queries::traverse::is_boundary_edge(arena, he_id)? {
                            return Err(KernelError::TopologyViolation {
                                err: forge_core::TopologyError::BoundaryEdgeInSolid {
                                    halfedge_index: he_id.index(),
                                    shell_index: shell_id.index(),
                                },
                                context: Some(forge_core::ErrorContext {
                                    scope: forge_core::ErrorScope::Entity {
                                        entity_kind: "HalfEdge".to_string(),
                                        index: he_id.index(),
                                    },
                                    suggested_fixes: Vec::new(),
                                    detail: format!(
                                        "Solid shell {} contains a boundary edge {} (Solid shells must be watertight)",
                                        shell_id.index(),
                                        he_id.index()
                                    ),
                                }),
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
