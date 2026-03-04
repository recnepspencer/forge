//! Duplicate coedge detection validator.
//!
//! INVARIANT: No halfedge may appear more than once in the same loop walk.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_no_duplicate_coedges_in_loop(arena: &TopologyArena) -> Result<(), KernelError> {
    let bound = arena.half_edge_count();
    for (loop_id, loop_data) in arena.iter_loops() {
        let start = loop_data.half_edge();
        let mut seen = std::collections::BTreeSet::new();
        let mut current = start;
        loop {
            if !seen.insert(current.index()) {
                return Err(vf("no_duplicate_coedges_in_loop", format!(
                    "Loop {} contains duplicate halfedge {} (cycle corruption)",
                    loop_id.index(), current.index()
                )));
            }
            current = arena.get_half_edge(current)?.next();
            if current == start { break; }
            if seen.len() > bound {
                return Err(vf("no_duplicate_coedges_in_loop", format!(
                    "Loop {} walk exceeded bound", loop_id.index()
                )));
            }
        }
    }
    Ok(())
}
