//! Index coherence validator — face→halfedges bag vs loop walks.
//!
//! DOMAIN: For each face, walks all loops (outer + inner) and collects
//! the set of halfedges reachable by following `next` pointers. Compares
//! this set against the `face_halfedges` bag index. Reports drift if
//! they differ — catching Euler operators that rewire halfedges but
//! forget to update the reverse index.

use crate::b_rep::TopologyArena;
use crate::queries::walk::collect_loop;
use forge_core::KernelError;
use std::collections::BTreeSet;

/// Validate that `face_halfedges` index matches loop-walked halfedges.
pub(crate) fn validate_index_coherence(arena: &TopologyArena) -> Result<(), KernelError> {
    for (face_id, face_data) in arena.iter_faces() {
        let indexed: BTreeSet<_> = arena.halfedges_of_face(face_id).iter().copied().collect();

        let mut walked = BTreeSet::new();

        // Walk outer loop
        let outer_loop = arena.get_loop(face_data.loops.outer())?;
        for he in collect_loop(arena, outer_loop.half_edge())? {
            walked.insert(he);
        }

        // Walk inner loops (holes)
        for &loop_id in face_data.loops.inners() {
            let inner_loop = arena.get_loop(loop_id)?;
            for he in collect_loop(arena, inner_loop.half_edge())? {
                walked.insert(he);
            }
        }

        if indexed != walked {
            let in_index_not_walked: Vec<_> = indexed.difference(&walked).collect();
            let in_walk_not_indexed: Vec<_> = walked.difference(&indexed).collect();
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: face_data.loops.outer().index(),
                    face_index: face_id.index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Face".to_string(),
                        index: face_id.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Index coherence drift for face {}: {} in index but not in loops, {} in loops but not indexed",
                        face_id.index(),
                        in_index_not_walked.len(),
                        in_walk_not_indexed.len(),
                    ),
                }),
            });
        }
    }

    Ok(())
}
