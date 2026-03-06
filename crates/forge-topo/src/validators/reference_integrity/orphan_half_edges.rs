//! Orphan half-edge validator.
//!
//! INVARIANT: Every half-edge in the entire memory arena must be reachable
//! by walking the loops of the faces. No "ghost" geometry floating in the void.

use crate::b_rep::{EntityBitset, TopologyArena};
use crate::queries::walk::walk_loop_iter;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_no_orphan_half_edges(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut reachable = EntityBitset::for_half_edges(arena);

    // Phase 1: Mark all reachable half-edges
    for (_loop_id, loop_data) in arena.iter_loops() {
        let start = loop_data.half_edge();
        for curr_result in walk_loop_iter(arena, start)? {
            let curr = curr_result?;
            if !reachable.insert(curr.index())? {
                // If it was already in the reachable set from THIS loop, we hit
                // the duplicate coedges invariant. If it was from ANOTHER loop,
                // that means a half-edge is in multiple loops! Either way, we just
                // break since earlier validators cover those exact cycle invariants.
                // We just want to mark reachable geometry here.
                break;
            }
        }
    }

    // Phase 2: Sweep and find any live half-edges not marked
    for (he_id, _he_data) in arena.iter_half_edges() {
        if !reachable.contains(he_id.index())? {
            return Err(vf("no_orphan_half_edges", format!(
                "HalfEdge {} is an orphan: it exists in the arena but is not reachable from any Loop",
                he_id.index()
            )));
        }
    }

    Ok(())
}
