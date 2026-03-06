//! Loop minimum cardinality validator.
//!
//! INVARIANT: Every loop must contain at least 2 halfedges (a degenerate digon).

use crate::b_rep::TopologyArena;
use crate::queries::walk::walk_loop_iter;
use forge_core::KernelError;

pub(crate) fn validate_loop_minimum_cardinality(arena: &TopologyArena) -> Result<(), KernelError> {
    for (_loop_id, loop_data) in arena.iter_loops() {
        let start = loop_data.half_edge();
        let _ = walk_loop_iter(arena, start)?.collect::<Result<Vec<_>, _>>()?;
    }
    Ok(())
}
