//! Previous-pointer consistency validator.
//!
//! INVARIANT: For every halfedge, `he.prev.next == he`.

use crate::b_rep::TopologyArena;
use crate::handles::HalfEdgeId;
use forge_core::KernelError;

pub(crate) fn validate_prev_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let prev_id = he_data.prev();

        // Sentinel detection: if prev points to DANGLING, the halfedge was
        // never properly wired. Report this clearly rather than crashing
        // with a StaleHandle from the arena lookup.
        if prev_id == HalfEdgeId::DANGLING {
            return Err(super::super::shared::vf(
                "prev_consistency",
                format!(
                    "HE[{}].prev is DANGLING (u32::MAX) — halfedge was never wired. \
                     Face: {}, Origin: {}",
                    he_id.index(),
                    he_data.face().index(),
                    he_data.origin().index(),
                ),
            ));
        }

        let prev_data = arena.get_half_edge(prev_id).map_err(|_| {
            super::super::shared::vf(
                "prev_consistency",
                format!(
                    "HE[{}].prev = {} references a deleted/invalid halfedge. \
                     Face: {}, Origin: {}",
                    he_id.index(),
                    prev_id.index(),
                    he_data.face().index(),
                    he_data.origin().index(),
                ),
            )
        })?;

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
