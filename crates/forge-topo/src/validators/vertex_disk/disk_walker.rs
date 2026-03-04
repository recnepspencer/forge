//! Vertex disk traversal logic.
//!
//! Provides a unified, robust way to traverse vertex umbrella disks,
//! handling open boundaries (sheet shells) and non-manifold topologies
//! (arbitrary co-edge orientations).

use crate::b_rep::TopologyArena;
use crate::handles::HalfEdgeId;
use forge_core::KernelError;
use std::collections::BTreeSet;

/// Traverses a vertex disk starting at `start_id`.
/// `start_id` MUST be an outgoing half-edge (origin == V).
/// Returns all outgoing half-edges belonging to this disk, and whether the disk
/// forms a closed cycle.
pub(crate) fn collect_disk(
    arena: &TopologyArena,
    start_id: HalfEdgeId,
) -> Result<(BTreeSet<HalfEdgeId>, bool), KernelError> {
    let mut disk = BTreeSet::new();
    let v_origin = arena.get_half_edge(start_id)?.origin();
    let bound = arena.half_edge_count().max(1);

    let mut current = start_id;
    let mut forward_closed = false;

    // 1. Walk Forward via `current` edge (Outgoing V -> A)
    for step in 0..=bound {
        disk.insert(current);
        let rad_next = arena.get_half_edge(current)?.radial_next();
        let rad_next_data = arena.get_half_edge(rad_next)?;

        if rad_next_data.origin() == v_origin {
            break; // Open boundary forward
        }

        current = rad_next_data.next();
        if current == start_id {
            forward_closed = true;
            break;
        }
        
        if step == bound {
           return Err(KernelError::TopologyViolation {
               err: forge_core::TopologyError::BrokenLoop { starting_halfedge: start_id.index(), face_index: 0 },
               context: None,
           });
        }
    }

    if forward_closed {
        return Ok((disk, true));
    }

    // 2. Walk Backward via `current.prev()` edge (Incoming B -> V)
    current = start_id;
    for step in 0..=bound {
        // If we just inserted it, wait, we already inserted start_id.
        let prev = arena.get_half_edge(current)?.prev();

        // Find radial_prev of `prev`
        let mut rad_prev = prev;
        for _ in 0..=bound {
            let next_rad = arena.get_half_edge(rad_prev)?.radial_next();
            if next_rad == prev {
                break;
            }
            rad_prev = next_rad;
        }

        let rad_prev_data = arena.get_half_edge(rad_prev)?;

        if rad_prev_data.origin() != v_origin {
            break; // Open boundary backward
        }

        current = rad_prev;
        if !disk.insert(current) {
            break; // Reached already visited half-edge
        }
        
        if step == bound {
           return Err(KernelError::TopologyViolation {
               err: forge_core::TopologyError::BrokenLoop { starting_halfedge: start_id.index(), face_index: 0 },
               context: None,
           });
        }
    }

    Ok((disk, false))
}
