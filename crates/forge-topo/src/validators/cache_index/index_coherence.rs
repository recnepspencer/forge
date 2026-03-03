//! Index coherence validator — face→halfedges bag vs loop walks.
//!
//! DOMAIN: For each face, walks all loops (outer + inner) and collects
//! the set of halfedges reachable by following `next` pointers. Compares
//! this set against the `face_halfedges` bag index. Reports drift if
//! they differ — catching Euler operators that rewire halfedges but
//! forget to update the reverse index.

use std::collections::BTreeSet;
use crate::b_rep::TopologyArena;
use forge_core::KernelError;

/// Validate that `face_halfedges` index matches loop-walked halfedges.
pub(crate) fn validate_index_coherence(arena: &TopologyArena) -> Result<(), KernelError> {
    for (face_id, face_data) in arena.iter_faces() {
        let indexed: BTreeSet<_> = arena.halfedges_of_face(face_id)
            .iter()
            .copied()
            .collect();

        let mut walked = BTreeSet::new();
        let bound = arena.half_edge_count();

        // Walk outer loop
        let outer_loop = arena.get_loop(face_data.outer_loop())?;
        walk_loop_into(arena, outer_loop.half_edge(), bound, &mut walked)?;

        // Walk inner loops (holes)
        for &loop_id in face_data.inner_loops() {
            let inner_loop = arena.get_loop(loop_id)?;
            walk_loop_into(arena, inner_loop.half_edge(), bound, &mut walked)?;
        }

        if indexed != walked {
            let in_index_not_walked: Vec<_> = indexed.difference(&walked).collect();
            let in_walk_not_indexed: Vec<_> = walked.difference(&indexed).collect();
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: face_data.outer_loop().index(),
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

/// Walk a loop starting at `start`, inserting each visited halfedge into `out`.
fn walk_loop_into(
    arena: &TopologyArena,
    start: crate::handles::HalfEdgeId,
    bound: usize,
    out: &mut BTreeSet<crate::handles::HalfEdgeId>,
) -> Result<(), KernelError> {
    let mut current = start;
    let mut steps = 0;
    loop {
        out.insert(current);
        let he = arena.get_half_edge(current)?;
        current = he.next();
        steps += 1;
        if current == start || steps > bound {
            break;
        }
    }
    Ok(())
}
