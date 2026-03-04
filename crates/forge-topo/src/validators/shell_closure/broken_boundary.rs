//! Broken face boundary validator.
//!
//! INVARIANT: Walking a face's outer loop via .next() must close and
//! every halfedge in the walk must claim that face.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_no_broken_face_boundary(arena: &TopologyArena) -> Result<(), KernelError> {
    let bound = arena.half_edge_count();

    for (face_id, face_data) in arena.iter_faces() {
        let outer = face_data.outer_loop();
        let outer_data = arena.get_loop(outer)?;
        let start = outer_data.half_edge();
        let mut current = start;
        let mut steps = 0;

        loop {
            let he_data = arena.get_half_edge(current)?;
            if he_data.face() != face_id {
                return Err(vf("no_broken_face_boundary", format!(
                    "Face {} outer loop: HE {} claims face {} (boundary mismatch)",
                    face_id.index(), current.index(), he_data.face().index()
                )));
            }
            current = he_data.next();
            if current == start { break; }
            steps += 1;
            if steps > bound {
                return Err(vf("no_broken_face_boundary", format!(
                    "Face {} outer loop walk from HE {} did not close within {} steps",
                    face_id.index(), start.index(), bound
                )));
            }
        }
    }
    Ok(())
}
