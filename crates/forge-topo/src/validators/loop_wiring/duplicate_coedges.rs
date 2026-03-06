//! Duplicate coedge detection validator.
//!
//! INVARIANT: No halfedge may appear more than once in the same loop walk.

use crate::b_rep::TopologyArena;
use crate::queries::walk::walk_loop_iter;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_no_duplicate_coedges_in_loop(
    arena: &TopologyArena,
) -> Result<(), KernelError> {
    for (loop_id, loop_data) in arena.iter_loops() {
        let start = loop_data.half_edge();
        let mut seen = std::collections::BTreeSet::new();
        for he_result in walk_loop_iter(arena, start)? {
            let current = he_result?;
            if !seen.insert(current.index()) {
                return Err(vf(
                    "no_duplicate_coedges_in_loop",
                    format!(
                        "Loop {} contains duplicate halfedge {} (cycle corruption)",
                        loop_id.index(),
                        current.index()
                    ),
                ));
            }
        }
    }
    Ok(())
}
