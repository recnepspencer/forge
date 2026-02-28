//! Half-edge and loop wiring invariant validators.
//!
//! DOMAIN: Structural invariants for the half-edge data structure —
//! twin symmetry, next/prev symmetry, loop closure, and edge-endpoint
//! consistency with loop vertices.
//!
//! VALIDATORS (from validators.md §2):
//! - ValidateTwinSymmetry
//! - ValidateNextPrevSymmetry
//! - ValidateLoopClosure
//! - ValidateLoopIsSimpleTopologically
//! - ValidateEdgeEndpointsMatchCoedgeVertices
//! - ValidateConsistentEdgeSenseAcrossCoedges
//! - ValidateFaceLoopMembershipComplete
//!
//! DEPENDENCIES: `arena`, `handles`, `queries::traverse`

use crate::b_rep::TopologyArena;
use crate::b_rep::EntityBitset;
use crate::queries::traverse::FaceEdgeIterator;
use forge_core::KernelError;

/// Validate previous-pointer consistency: for every halfedge, `he.prev.next == he`.
pub(crate) fn validate_prev_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let prev_data = arena.get_half_edge(he_data.prev())?;
        if prev_data.next() != he_id {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: he_id.index(),
                    face_index: he_data.face().index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "HalfEdge".to_string(),
                        index: he_id.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Prev consistency violated: he[{}].prev = {}, but he[{}].next = {} (expected {})",
                        he_id.index(),
                        he_data.prev().index(),
                        he_data.prev().index(),
                        prev_data.next().index(),
                        he_id.index()
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Validate vertex continuity: for each halfedge, `he.next.origin`
/// must be a valid endpoint of the geometric edge (i.e., it must equal
/// some other halfedge's origin in the same radial ring, or he.origin
/// for geometric self-loops).
///
/// This catches the "spaghetti topology" bug where edges are mis-wired.
pub fn validate_vertex_continuity(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut checked_halfedges = EntityBitset::for_half_edges(arena);

    for (he_id, he_data) in arena.iter_half_edges() {
        if checked_halfedges.contains(he_id.index())? {
            continue;
        }

        checked_halfedges.insert(he_id.index())?;

        // Unify the edge explicitly for the error message, even though
        // validate_radial_edge_consistency ensures it's uniform per ring.
        let edge_id = he_data.edge();

        // Collect all (origin, target) pairs from this edge's radial ring
        let mut endpoints = EntityBitset::for_vertices(arena);
        let mut curr = he_id;
        loop {
            checked_halfedges.insert(curr.index())?;
            let curr_data = arena.get_half_edge(curr)?;
            let next_data = arena.get_half_edge(curr_data.next())?;
            endpoints.insert(curr_data.origin().index())?;
            endpoints.insert(next_data.origin().index())?;

            curr = curr_data.radial_next();
            if curr == he_id {
                break;
            }
        }

        // A well-formed edge should have at most 2 distinct endpoint vertices
        // (exactly 1 for geometric self-loops, exactly 2 for normal edges)
        if endpoints.count() > 2 {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: he_id.index(),
                    face_index: he_data.face().index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Edge".to_string(),
                        index: edge_id.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Edge {} has {} distinct endpoint vertices (expected 1 or 2)",
                        edge_id.index(),
                        endpoints.count()
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Validate that every face's loop is closed and each halfedge belongs to the correct face.
pub(crate) fn validate_loops(arena: &TopologyArena) -> Result<(), KernelError> {
    for (face_id, face_data) in arena.iter_faces() {
        // Validate outer loop
        for he_result in FaceEdgeIterator::new(arena, face_id)? {
            let he_id = he_result?;
            let he_data = arena.get_half_edge(he_id)?;

            if he_data.face() != face_id {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::BrokenLoop {
                        starting_halfedge: he_id.index(),
                        face_index: face_id.index(),
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "Face".to_string(),
                            index: face_id.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Halfedge {} in outer loop of face {} belongs to face {} instead",
                            he_id.index(),
                            face_id.index(),
                            he_data.face().index()
                        ),
                    }),
                });
            }
        }

        // Validate inner loops (holes)
        let bound = arena.half_edge_count();
        for &loop_id in face_data.inner_loops() {
            let loop_data = arena.get_loop(loop_id)?;
            let start = loop_data.half_edge();
            let mut current = start;
            let mut steps = 0usize;

            loop {
                let he_data = arena.get_half_edge(current)?;
                if he_data.face() != face_id {
                    return Err(KernelError::TopologyViolation {
                        err: forge_core::TopologyError::BrokenLoop {
                            starting_halfedge: current.index(),
                            face_index: face_id.index(),
                        },
                        context: Some(forge_core::ErrorContext {
                            scope: forge_core::ErrorScope::Entity {
                                entity_kind: "Loop".to_string(),
                                index: loop_id.index(),
                            },
                            suggested_fixes: Vec::new(),
                            detail: format!(
                                "Halfedge {} in inner loop {} of face {} belongs to face {} instead",
                                current.index(),
                                loop_id.index(),
                                face_id.index(),
                                he_data.face().index()
                            ),
                        }),
                    });
                }
                let next = he_data.next();
                current = next;
                if current == start {
                    break;
                }
                steps += 1;
                if steps > bound {
                    return Err(KernelError::TopologyViolation {
                        err: forge_core::TopologyError::LoopCorruption {
                            walk_kind: "validate_inner_loop".into(),
                            seed_index: start.index(),
                            last_visited_index: current.index(),
                            steps_taken: steps,
                            entity_bound: bound,
                        },
                        context: None,
                    });
                }
            }
        }
    }
    Ok(())
}
