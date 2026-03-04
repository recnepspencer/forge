//! Loop minimum cardinality validator.
//!
//! INVARIANT: Every loop must contain at least 2 halfedges (a degenerate digon).

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_loop_minimum_cardinality(arena: &TopologyArena) -> Result<(), KernelError> {
    let bound = arena.half_edge_count();
    for (loop_id, loop_data) in arena.iter_loops() {
        let start = loop_data.half_edge();
        let mut count = 0usize;
        let mut current = start;
        loop {
            count += 1;
            current = arena.get_half_edge(current)?.next();
            if current == start { break; }
            if count > bound {
                return Err(vf("loop_minimum_cardinality", format!(
                    "Loop {} walk exceeded bound", loop_id.index()
                )));
            }
        }
    }
    Ok(())
}
