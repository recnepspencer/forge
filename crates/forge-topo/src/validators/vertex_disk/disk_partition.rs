//! Validate Vertex Disk Partition
//!
//! INVARIANT: For any vertex, all half-edges with it as their origin must
//! belong to a well-defined set of disjoint vertex disks (orbits of `twin -> next`).
//! Every outgoing half-edge must be a member of exactly one disk. This ensures
//! the local topology around a pinch-point is completely and cleanly partitioned.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;
use std::collections::BTreeSet;

pub(crate) fn validate_vertex_disk_partition(arena: &TopologyArena) -> Result<(), KernelError> {
    for (vid, v_data) in arena.iter_vertices() {
        let expected_outgoing: BTreeSet<_> = arena
            .iter_half_edges()
            .filter(|(_, data)| data.origin() == vid)
            .map(|(id, _)| id)
            .collect();

        if expected_outgoing.is_empty() {
            continue;
        }

        let mut all_visited = BTreeSet::new();
        let mut disk_count = 0;
        let mut expected_iter = expected_outgoing.iter().copied();

        // Partition into disks by tracing `twin -> next`
        while let Some(start) = expected_iter.find(|id| !all_visited.contains(id)) {
            let (disk, _) = super::disk_walker::collect_disk(arena, start)?;
            all_visited.extend(disk);
            disk_count += 1;
        }

        // Validate partition covers everything. The while loop above guarantees
        // we cover all `expected_outgoing`. We just need to ensure `outgoing()`
        // points to a valid disk entry.
        let out = v_data.outgoing();
        if !expected_outgoing.contains(&out) {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: out.index(),
                    face_index: 0,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Vertex".to_string(),
                        index: vid.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Vertex {} outgoing half-edge {} does not belong to any disk at this vertex.",
                        vid.index(), out.index()
                    ),
                }),
            });
        }
        
        // Furthermore, in a fully correct partition, every halfedge visited is expected.
        let extra_visited: Vec<_> = all_visited.difference(&expected_outgoing).collect();
        if !extra_visited.is_empty() {
             return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: out.index(),
                    face_index: 0,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Vertex".to_string(),
                        index: vid.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Vertex {} disks contain half-edges that do not originate at this vertex: {:?}",
                        vid.index(), extra_visited
                    ),
                }),
            });
        }
    }

    Ok(())
}
