//! Vertex primary disk validator.
//!
//! INVARIANT: Every vertex's primary disk entry must be valid and point
//! back to that vertex as its origin.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;
use std::collections::BTreeSet;

pub(crate) fn validate_vertex_outgoing(arena: &TopologyArena) -> Result<(), KernelError> {
    for (vid, _) in arena.iter_vertices() {
        let expected_outgoing: BTreeSet<_> = arena
            .iter_half_edges()
            .filter(|(_, data)| data.origin() == vid)
            .map(|(id, _)| id)
            .collect();

        if expected_outgoing.is_empty() {
            continue;
        }

        let disks = crate::queries::vertex_disks::compute_vertex_disks(arena, vid)?;
        let entries = arena.disk_entries(vid)?;

        let mut covered = BTreeSet::new();
        for &entry in &entries {
            let entry_data = arena.get_half_edge(entry).map_err(|_| KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: entry.index(),
                    face_index: 0,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Vertex".to_string(),
                        index: vid.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Vertex {} disk entry {}(gen{}) is stale/deleted",
                        vid.index(),
                        entry.index(),
                        entry.generation()
                    ),
                }),
            })?;

            if entry_data.origin() != vid {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::BrokenLoop {
                        starting_halfedge: entry.index(),
                        face_index: 0,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "Vertex".to_string(),
                            index: vid.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Vertex {} disk entry {} has origin {} (should be {})",
                            vid.index(),
                            entry.index(),
                            entry_data.origin().index(),
                            vid.index()
                        ),
                    }),
                });
            }

            let mut found_disk = None;
            for disk in &disks {
                if disk.contains(&entry) {
                    found_disk = Some(disk);
                    break;
                }
            }

            let Some(disk) = found_disk else {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::BrokenLoop {
                        starting_halfedge: entry.index(),
                        face_index: 0,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "Vertex".to_string(),
                            index: vid.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Vertex {} disk entry {} does not reach any local disk component",
                            vid.index(),
                            entry.index()
                        ),
                    }),
                });
            };

            for &he in disk {
                covered.insert(he);
            }
        }

        if covered != expected_outgoing {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: entries[0].index(),
                    face_index: 0,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Vertex".to_string(),
                        index: vid.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Vertex {} disk entries do not cover all outgoing half-edges (covered={}, expected={})",
                        vid.index(),
                        covered.len(),
                        expected_outgoing.len()
                    ),
                }),
            });
        }
    }
    Ok(())
}
