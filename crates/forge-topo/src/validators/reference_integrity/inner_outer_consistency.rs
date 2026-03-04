//! Inner/outer loop consistency validator.
//!
//! INVARIANT: A face's outer_loop must NOT be an inner loop of any face.
//! Each loop's face pointer must match the owning face.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;
use std::collections::BTreeSet;

use super::vf;

pub(crate) fn validate_inner_outer_loop_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut outer_loops = BTreeSet::new();
    for (_face_id, face_data) in arena.iter_faces() {
        outer_loops.insert(face_data.outer_loop().index());
    }

    for (face_id, face_data) in arena.iter_faces() {
        for &il in face_data.inner_loops() {
            if outer_loops.contains(&il.index()) {
                return Err(vf("inner_outer_loop_consistency", format!(
                    "Face {} has inner loop {} which is also an outer_loop of another face",
                    face_id.index(), il.index()
                )));
            }
        }

        let outer = face_data.outer_loop();
        let outer_data = arena.get_loop(outer)?;
        if outer_data.face() != face_id {
            return Err(vf("inner_outer_loop_consistency", format!(
                "Face {} outer_loop {} has loop.face() = {} (mismatch)",
                face_id.index(), outer.index(), outer_data.face().index()
            )));
        }

        for &il in face_data.inner_loops() {
            let il_data = arena.get_loop(il)?;
            if il_data.face() != face_id {
                return Err(vf("inner_outer_loop_consistency", format!(
                    "Face {} inner_loop {} has loop.face() = {} (mismatch)",
                    face_id.index(), il.index(), il_data.face().index()
                )));
            }
        }
    }

    Ok(())
}
