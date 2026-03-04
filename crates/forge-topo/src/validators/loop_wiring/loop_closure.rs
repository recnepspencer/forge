//! Loop closure validator.
//!
//! INVARIANT: Every face's loop is closed and each halfedge belongs
//! to the correct face. Validates both outer and inner loops.

use crate::b_rep::TopologyArena;
use crate::queries::traverse::FaceEdgeIterator;
use forge_core::KernelError;

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
