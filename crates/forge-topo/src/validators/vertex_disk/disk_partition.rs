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

        let out = v_data.primary_disk();
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
                        "Vertex {} primary disk half-edge {} does not belong to any disk at this vertex.",
                        vid.index(), out.index()
                    ),
                }),
            });
        }

        let rebuilt = crate::queries::vertex_disks::rebuild_disk_entries(arena, vid)?;
        let stored_count = arena.disk_count(vid);
        if stored_count != rebuilt.len() {
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
                        "Vertex {} disk count mismatch: stored={} rebuilt={}",
                        vid.index(),
                        stored_count,
                        rebuilt.len()
                    ),
                }),
            });
        }
    }

    Ok(())
}
