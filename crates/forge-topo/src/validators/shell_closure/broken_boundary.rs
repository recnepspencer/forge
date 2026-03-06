//! Broken face boundary validator.
//!
//! INVARIANT: Walking a face's outer loop via .next() must close and
//! every halfedge in the walk must claim that face.

use crate::b_rep::TopologyArena;
use crate::queries::walk::walk_loop_iter;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_no_broken_face_boundary(arena: &TopologyArena) -> Result<(), KernelError> {
    for (face_id, face_data) in arena.iter_faces() {
        let mut all_loops = vec![face_data.loops.outer()];
        all_loops.extend_from_slice(face_data.loops.inners());

        for loop_id in all_loops {
            let loop_data = arena.get_loop(loop_id)?;
            let start = loop_data.half_edge();
            for he_result in walk_loop_iter(arena, start)? {
                let current = he_result?;
                let he_data = arena.get_half_edge(current)?;
                if he_data.face() != face_id {
                    return Err(vf(
                        "no_broken_face_boundary",
                        format!(
                            "Face {} loop {}: HE {} claims face {} (boundary mismatch)",
                            face_id.index(),
                            loop_id.index(),
                            current.index(),
                            he_data.face().index()
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}
