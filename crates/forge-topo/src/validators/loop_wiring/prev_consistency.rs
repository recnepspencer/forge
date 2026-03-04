//! Previous-pointer consistency validator.
//!
//! INVARIANT: For every halfedge, `he.prev.next == he`.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

pub(crate) fn validate_prev_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let prev_data = arena.get_half_edge(he_data.prev())?;
        if prev_data.next() != he_id {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: he_id.index(),
                    face_index: he_data.face().index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "HalfEdge".to_string(),
                        index: he_id.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Prev consistency violated: he[{}].prev = {}, but he[{}].next = {} (expected {})",
                        he_id.index(),
                        he_data.prev().index(),
                        he_data.prev().index(),
                        prev_data.next().index(),
                        he_id.index()
                    ),
                }),
            });
        }
    }
    Ok(())
}
