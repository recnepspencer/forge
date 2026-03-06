//! Radial cycle uniqueness validator.
//!
//! INVARIANT: No halfedge may appear more than once in a radial ring.

use crate::b_rep::EntityBitset;
use crate::b_rep::TopologyArena;
use crate::queries::walk::walk_radial_iter;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_radial_cycle_uniqueness(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut global_checked = EntityBitset::for_half_edges(arena);

    for (start_he, _) in arena.iter_half_edges() {
        if global_checked.contains(start_he.index())? {
            continue;
        }

        let mut ring_seen = std::collections::BTreeSet::new();
        for curr_result in walk_radial_iter(arena, start_he)? {
            let curr = curr_result?;
            global_checked.insert(curr.index())?;
            if !ring_seen.insert(curr.index()) {
                return Err(vf(
                    "radial_cycle_uniqueness",
                    format!(
                        "Radial ring seeded at HE {} contains duplicate HE {} (fractured ring)",
                        start_he.index(),
                        curr.index()
                    ),
                ));
            }
        }
    }
    Ok(())
}
