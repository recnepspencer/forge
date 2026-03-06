//! Face loop membership completeness validator.
//!
//! INVARIANT: Every halfedge assigned to a face must be reachable from
//! one of that face's loops (outer or inner). No floating halfedges.

use crate::b_rep::EntityBitset;
use crate::b_rep::TopologyArena;
use crate::queries::walk::walk_loop_iter;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_face_loop_membership_complete(
    arena: &TopologyArena,
) -> Result<(), KernelError> {
    for (face_id, face_data) in arena.iter_faces() {
        let mut reachable = EntityBitset::for_half_edges(arena);

        // Walk outer loop
        let outer_start = arena.get_loop(face_data.loops.outer())?.half_edge();
        for he_result in walk_loop_iter(arena, outer_start)? {
            let current = he_result?;
            reachable.insert(current.index())?;
        }

        // Walk inner loops
        for &il in face_data.loops.inners() {
            let il_start = arena.get_loop(il)?.half_edge();
            for he_result in walk_loop_iter(arena, il_start)? {
                let current = he_result?;
                reachable.insert(current.index())?;
            }
        }

        for (he_id, he_data) in arena.iter_half_edges() {
            if he_data.face() == face_id && !reachable.contains(he_id.index())? {
                return Err(vf(
                    "face_loop_membership_complete",
                    format!(
                        "HE {} claims face {} but is unreachable from any of that face's loops",
                        he_id.index(),
                        face_id.index()
                    ),
                ));
            }
        }
    }

    Ok(())
}
