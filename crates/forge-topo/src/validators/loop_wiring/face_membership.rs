//! Face loop membership completeness validator.
//!
//! INVARIANT: Every halfedge assigned to a face must be reachable from
//! one of that face's loops (outer or inner). No floating halfedges.

use crate::b_rep::TopologyArena;
use crate::b_rep::EntityBitset;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_face_loop_membership_complete(arena: &TopologyArena) -> Result<(), KernelError> {
    let bound = arena.half_edge_count();

    for (face_id, face_data) in arena.iter_faces() {
        let mut reachable = EntityBitset::for_half_edges(arena);

        // Walk outer loop
        let outer_start = arena.get_loop(face_data.outer_loop())?.half_edge();
        let mut current = outer_start;
        let mut steps = 0;
        loop {
            reachable.insert(current.index())?;
            current = arena.get_half_edge(current)?.next();
            if current == outer_start { break; }
            steps += 1;
            if steps > bound { break; }
        }

        // Walk inner loops
        for &il in face_data.inner_loops() {
            let il_start = arena.get_loop(il)?.half_edge();
            let mut current = il_start;
            let mut steps = 0;
            loop {
                reachable.insert(current.index())?;
                current = arena.get_half_edge(current)?.next();
                if current == il_start { break; }
                steps += 1;
                if steps > bound { break; }
            }
        }

        for (he_id, he_data) in arena.iter_half_edges() {
            if he_data.face() == face_id && !reachable.contains(he_id.index())? {
                return Err(vf("face_loop_membership_complete", format!(
                    "HE {} claims face {} but is unreachable from any of that face's loops",
                    he_id.index(), face_id.index()
                )));
            }
        }
    }

    Ok(())
}
