//! Single owner per loop validator.
//!
//! INVARIANT: Every loop must be referenced by exactly one face — either as
//! that face's outer_loop or in exactly one face's inner_loops list.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;
use std::collections::BTreeMap;

use super::vf;

pub(crate) fn validate_single_owner_per_loop(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut owner_count: BTreeMap<u32, (u32, u32)> = BTreeMap::new();

    for (face_id, face_data) in arena.iter_faces() {
        let outer = face_data.outer_loop();
        let entry = owner_count.entry(outer.index()).or_insert((0, face_id.index()));
        entry.0 += 1;
        entry.1 = face_id.index();

        for &il in face_data.inner_loops() {
            let entry = owner_count.entry(il.index()).or_insert((0, face_id.index()));
            entry.0 += 1;
            entry.1 = face_id.index();
        }
    }

    for (loop_id, loop_data) in arena.iter_loops() {
        match owner_count.get(&loop_id.index()) {
            None => {
                return Err(vf("single_owner_per_loop", format!(
                    "Loop {} (face={}) is orphaned: no face claims it via outer_loop or inner_loops",
                    loop_id.index(), loop_data.face().index()
                )));
            }
            Some(&(count, _)) if count > 1 => {
                return Err(vf("single_owner_per_loop", format!(
                    "Loop {} is claimed by {} faces (must be exactly 1)",
                    loop_id.index(), count
                )));
            }
            _ => {}
        }
    }

    Ok(())
}
