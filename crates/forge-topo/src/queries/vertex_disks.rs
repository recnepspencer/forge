//! Canonical vertex disk queries for manifold and NMT vertices.
//!
//! DOMAIN: Partition outgoing half-edges at a vertex into connected
//! disk cycles/components using radial/disk walks.

use std::collections::{BTreeSet, VecDeque};

use forge_core::{ErrorContext, ErrorScope, KernelError, TopologyError};
use smallvec::SmallVec;

use crate::b_rep::TopologyArena;
use crate::handles::{HalfEdgeId, VertexId};

fn outgoing_at_vertex(arena: &TopologyArena, vertex: VertexId) -> Result<Vec<HalfEdgeId>, KernelError> {
    let mut outgoing = Vec::new();

    for (he_id, he_data) in arena.iter_half_edges() {
        if he_data.origin() == vertex {
            outgoing.push(he_id);
        }
    }

    outgoing.sort_by_key(|he| he.index());
    outgoing.dedup();
    Ok(outgoing)
}

fn disk_component(
    arena: &TopologyArena,
    vertex: VertexId,
    seed: HalfEdgeId,
    outgoing_set: &BTreeSet<HalfEdgeId>,
) -> Result<BTreeSet<HalfEdgeId>, KernelError> {
    let mut disk = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(seed);
    disk.insert(seed);

    let bound = arena.half_edge_count().max(1);
    println!("DEBUG: disk_component seed={} vertex={} out_set={:?}", seed.index(), vertex.index(), outgoing_set.iter().map(|h| h.index()).collect::<Vec<_>>());

    while let Some(curr) = queue.pop_front() {
        let curr_data = arena.get_half_edge(curr)?;
        if curr_data.origin() != vertex {
            continue;
        }

        // A vertex disk is the connected component of corners at the vertex.
        // The corner is formed by the incoming half-edge and the outgoing half-edge.
        let out_he = curr;
        let in_he = curr_data.prev();
        println!("DEBUG:   curr={} in_he={}", out_he.index(), in_he.index());

        // 1. Walk the entire radial ring of the outgoing edge
        let mut r = out_he;
        for step in 0..=bound {
            let r_data = arena.get_half_edge(r)?;
            
            // If this half-edge is outgoing, it belongs to the disk directly
            if r_data.origin() == vertex {
                if outgoing_set.contains(&r) && disk.insert(r) {
                    queue.push_back(r);
                }
            }
            
            // If this half-edge is incoming, its .next() is an outgoing half-edge
            let r_next = r_data.next();
            if outgoing_set.contains(&r_next) {
                if disk.insert(r_next) {
                    queue.push_back(r_next);
                }
            }

            r = r_data.radial_next();
            if r == out_he { break; }
            if step == bound {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::BrokenLoop { starting_halfedge: out_he.index(), face_index: 0 },
                    context: None,
                });
            }
        }

        // 2. Walk the entire radial ring of the incoming edge
        let mut r = in_he;
        for step in 0..=bound {
            let r_data = arena.get_half_edge(r)?;
            
            if r_data.origin() == vertex {
                if outgoing_set.contains(&r) && disk.insert(r) {
                    queue.push_back(r);
                }
            }
            
            let r_next = r_data.next();
            if outgoing_set.contains(&r_next) {
                if disk.insert(r_next) {
                    queue.push_back(r_next);
                }
            }

            r = r_data.radial_next();
            if r == in_he { break; }
            if step == bound {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::BrokenLoop { starting_halfedge: in_he.index(), face_index: 0 },
                    context: None,
                });
            }
        }
    }

    Ok(disk)
}

/// Partition all outgoing half-edges at a vertex into connected disk cycles/components.
pub fn compute_vertex_disks(
    arena: &TopologyArena,
    vertex: VertexId,
) -> Result<Vec<Vec<HalfEdgeId>>, KernelError> {
    let outgoing = outgoing_at_vertex(arena, vertex)?;
    if outgoing.is_empty() {
        return Ok(Vec::new());
    }

    let outgoing_set: BTreeSet<_> = outgoing.iter().copied().collect();
    let mut visited = BTreeSet::new();
    let mut disks = Vec::new();

    for seed in outgoing {
        if visited.contains(&seed) {
            continue;
        }
        let component = disk_component(arena, vertex, seed, &outgoing_set)?;
        visited.extend(component.iter().copied());
        let mut disk_vec: Vec<_> = component.into_iter().collect();
        disk_vec.sort_by_key(|he| he.index());
        disks.push(disk_vec);
    }

    disks.sort_by_key(|disk| disk.first().map(|he| he.index()).unwrap_or(u32::MAX));
    Ok(disks)
}

/// Is this vertex manifold (exactly 1 disk cycle)?
pub fn is_vertex_manifold(
    arena: &TopologyArena,
    vertex: VertexId,
) -> Result<bool, KernelError> {
    Ok(compute_vertex_disks(arena, vertex)?.len() <= 1)
}

/// Slow-but-correct recomputation of disk entries from scratch.
///
/// Collects outgoing half-edges via `vertex_halfedges`, partitions into
/// connected disk components, and returns one deterministic representative
/// (minimum half-edge index) per component.
pub fn rebuild_disk_entries(
    arena: &TopologyArena,
    vertex: VertexId,
) -> Result<SmallVec<[HalfEdgeId; 1]>, KernelError> {
    let disks = compute_vertex_disks(arena, vertex)?;
    let mut entries: SmallVec<[HalfEdgeId; 1]> = SmallVec::new();

    for disk in disks {
        if let Some(&repr) = disk.first() {
            entries.push(repr);
        }
    }

    entries.sort_by_key(|he| he.index());
    entries.dedup();
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use crate::b_rep::ShellKind;
    use super::*;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::transactions::TopologyState;

    #[test]
    fn manifold_vertex_rebuilds_single_disk_entry() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();
        let _se = draft.execute(SplitEdge { edge: mvf.half_edge }).unwrap().into_value();

        let entries = rebuild_disk_entries(draft.arena(), mvf.vertex).unwrap();
        assert_eq!(entries.len(), 1);
        assert!(is_vertex_manifold(draft.arena(), mvf.vertex).unwrap());
    }

    #[test]
    fn adversarial_shared_vertex_rebuilds_multiple_disk_entries() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let a = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();
        let b = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();

        let second_ring: Vec<_> = draft
            .arena()
            .iter_half_edges()
            .filter(|(_, data)| data.origin() == b.vertex)
            .map(|(id, _)| id)
            .collect();

        for he in second_ring {
            draft
                .arena_mut()
                .get_half_edge_mut(he)
                .unwrap()
                .set_origin(a.vertex);
        }

        let entries = rebuild_disk_entries(draft.arena(), a.vertex).unwrap();
        assert!(entries.len() >= 2);
        assert!(!is_vertex_manifold(draft.arena(), a.vertex).unwrap());
    }
}
