//! Face loop existence validator.
//!
//! INVARIANT: Every face must have a valid outer_loop pointing to a live loop.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_face_has_at_least_one_loop(
    arena: &TopologyArena,
) -> Result<(), KernelError> {
    for (face_id, face_data) in arena.iter_faces() {
        let outer = face_data.loops.outer();
        arena.get_loop(outer).map_err(|_| {
            vf(
                "face_has_at_least_one_loop",
                format!(
                    "Face {} outer_loop {} is deleted/invalid",
                    face_id.index(),
                    outer.index()
                ),
            )
        })?;
    }
    Ok(())
}
