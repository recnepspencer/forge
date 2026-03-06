//! Vertex disk traversal logic.
//!
//! Provides a unified, robust way to traverse vertex umbrella disks,
//! handling open boundaries (sheet shells) and non-manifold topologies
//! (arbitrary co-edge orientations).

use crate::b_rep::TopologyArena;
use crate::handles::HalfEdgeId;
use crate::queries::walk::collect_vertex_disk;
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
    let v_origin = arena.get_half_edge(start_id)?.origin();
    let disk: BTreeSet<HalfEdgeId> = collect_vertex_disk(arena, v_origin, start_id)?
        .into_iter()
        .collect();
    let mut closed = true;
    for &he in &disk {
        let radial = arena.get_half_edge(he)?.radial_next();
        if arena.get_half_edge(radial)?.origin() == v_origin {
            closed = false;
            break;
        }
    }
    Ok((disk, closed))
}
