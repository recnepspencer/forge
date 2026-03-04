//! Vertex outgoing reachability validator.
//!
//! INVARIANT: For every vertex V, every half-edge in the arena whose
//! `origin == V` must be reachable by walking the vertex's disk cycle
//! starting from `V.outgoing()`.
//!
//! This catches the "radial isolation" bug where batch face constructors
//! (MakeFaceFromVertices, MakeFaceInShellFromVertices, MakeLoopInFaceFromVertices)
//! skip wiring a new half-edge into the vertex disk when the vertex already
//! has an outgoing pointer.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

/// Validate that every half-edge with `origin == v` is reachable from
/// `v.outgoing()` via the vertex disk cycle.
///
/// The vertex disk cycle walks: from outgoing, follow `radial_next` to
/// get the twin, then `next` to get the next outgoing half-edge, repeat.
/// Every half-edge originating at V must appear in this cycle.
///
/// # Errors
///
/// Returns `KernelError::TopologyViolation` if any half-edge is unreachable.
pub(crate) fn validate_vertex_outgoing_reachability(
    arena: &TopologyArena,
) -> Result<(), KernelError> {
    use std::collections::BTreeSet;

    for (vid, v_data) in arena.iter_vertices() {
        let outgoing = v_data.outgoing();

        // Collect ALL half-edges in the arena that originate at this vertex.
        let mut expected: BTreeSet<crate::handles::HalfEdgeId> = BTreeSet::new();
        for (he_id, he_data) in arena.iter_half_edges() {
            if he_data.origin() == vid {
                expected.insert(he_id);
            }
        }

        if expected.is_empty() {
            // Vertex has no half-edges referencing it. This is caught by
            // other validators (orphan detection). Skip here.
            continue;
        }

        // Walk the vertex disk cycle from outgoing and collect reachable half-edges.
        let mut reachable: BTreeSet<crate::handles::HalfEdgeId> = BTreeSet::new();
        let mut current = outgoing;
        let bound = arena.half_edge_count().max(1);

        for step in 0..=bound {
            if reachable.contains(&current) {
                // We've looped back. The cycle is closed.
                break;
            }

            let he_data = arena.get_half_edge(current).map_err(|_| {
                KernelError::TopologyViolation {
                    err: forge_core::TopologyError::BrokenLoop {
                        starting_halfedge: outgoing.index(),
                        face_index: 0,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "Vertex".to_string(),
                            index: vid.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Vertex disk walk hit deleted half-edge {} at step {}",
                            current.index(),
                            step
                        ),
                    }),
                }
            })?;

            if he_data.origin() == vid {
                reachable.insert(current);
            }

            // Advance: twin → next gives the next outgoing half-edge at this vertex.
            let twin = he_data.radial_next();
            let next_outgoing = arena.get_half_edge(twin)?.next();
            current = next_outgoing;

            if step == bound {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::BrokenLoop {
                        starting_halfedge: outgoing.index(),
                        face_index: 0,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "Vertex".to_string(),
                            index: vid.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Vertex {} disk cycle did not close after {} steps",
                            vid.index(),
                            bound
                        ),
                    }),
                });
            }
        }

        // Check for unreachable half-edges.
        let unreachable: Vec<_> = expected.difference(&reachable).collect();
        if !unreachable.is_empty() {
            let indices: Vec<u32> = unreachable.iter().map(|he| he.index()).collect();
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: outgoing.index(),
                    face_index: 0,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Vertex".to_string(),
                        index: vid.index(),
                    },
                    suggested_fixes: vec![
                        "Wire the new half-edge into the vertex disk cycle via radial_next linkage".to_string(),
                    ],
                    detail: format!(
                        "Vertex {} has {} half-edge(s) unreachable from outgoing {}: {:?}",
                        vid.index(),
                        unreachable.len(),
                        outgoing.index(),
                        indices,
                    ),
                }),
            });
        }
    }

    Ok(())
}
