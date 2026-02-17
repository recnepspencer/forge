//! Topology validation for commit-time invariant checking.
//!
//! DOMAIN: Structural integrity checks for the halfedge mesh.
//!
//! INVARIANTS:
//! - Twin reciprocity: he.twin.twin == he
//! - Previous consistency: he.prev.next == he
//! - Vertex continuity: next(he).origin == twin(he).origin
//! - Loop closure: following `next` pointers returns to start
//! - Generalized Euler-Poincaré: V - E + F - L = 2(S - G)
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs)

use forge_core::KernelError;
use crate::arena::TopologyArena;

/// Validate the topology of an arena.
///
/// Called automatically by `MutableDraft::commit()`. Runs all structural
/// checks and returns the first error found.
pub fn validate_topology(arena: &TopologyArena) -> Result<(), KernelError> {
    validate_twins(arena)?;
    validate_prev_consistency(arena)?;
    validate_vertex_continuity(arena)?;
    validate_loops(arena)?;
    validate_euler(arena)?;
    Ok(())
}

/// Validate twin reciprocity: for every halfedge, `he.twin.twin == he`.
fn validate_twins(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.twin;

        if he_id == twin_id {
            continue;
        }

        let twin_data = arena.get_half_edge(twin_id).map_err(|_| {
            KernelError::TopologyViolation {
                err: forge_core::TopologyError::MissingTwin {
                    halfedge_index: he_id.index(),
                },
                context: None,
            }
        })?;

        if twin_data.twin != he_id {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::MissingTwin {
                    halfedge_index: he_id.index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity { entity_kind: "HalfEdge", index: he_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Twin reciprocity violated: he[{}].twin = {}, but he[{}].twin = {} (expected {})",
                        he_id.index(), twin_id.index(), twin_id.index(), twin_data.twin.index(), he_id.index()
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Validate previous-pointer consistency: for every halfedge, `he.prev.next == he`.
fn validate_prev_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let prev_data = arena.get_half_edge(he_data.prev)?;
        if prev_data.next != he_id {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: he_id.index(),
                    face_index: he_data.face.index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity { entity_kind: "HalfEdge", index: he_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Prev consistency violated: he[{}].prev = {}, but he[{}].next = {} (expected {})",
                        he_id.index(), he_data.prev.index(), he_data.prev.index(), prev_data.next.index(), he_id.index()
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Validate vertex continuity: the next halfedge's origin must equal the
/// twin's origin (i.e., the target vertex of this halfedge).
///
/// This catches the "spaghetti topology" bug where edges are mis-wired.
fn validate_vertex_continuity(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        if he_id == he_data.twin && he_id == he_data.next {
            continue;
        }

        let twin_data = arena.get_half_edge(he_data.twin)?;
        let next_data = arena.get_half_edge(he_data.next)?;

        if he_id != he_data.twin && next_data.origin != twin_data.origin {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: he_id.index(),
                    face_index: he_data.face.index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity { entity_kind: "HalfEdge", index: he_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Vertex continuity violated: he[{}].next.origin = {} but he[{}].twin.origin = {} (should be equal)",
                        he_id.index(), next_data.origin.index(), he_id.index(), twin_data.origin.index()
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Validate that every face's loop is closed and consistent.
fn validate_loops(arena: &TopologyArena) -> Result<(), KernelError> {
    const MAX_ITER: usize = 100_000;

    for (face_id, face_data) in arena.iter_faces() {
        let loop_data = arena.get_loop(face_data.outer_loop)?;
        let start = loop_data.half_edge;
        let mut current = start;
        let mut count = 0;

        loop {
            let he_data = arena.get_half_edge(current)?;

            if he_data.face != face_id {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::BrokenLoop {
                        starting_halfedge: start.index(),
                        face_index: face_id.index(),
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity { entity_kind: "Face", index: face_id.index() },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Halfedge {} in loop of face {} belongs to face {} instead",
                            current.index(), face_id.index(), he_data.face.index()
                        ),
                    }),
                });
            }

            current = he_data.next;
            count += 1;

            if current == start {
                break;
            }

            if count >= MAX_ITER {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::BrokenLoop {
                        starting_halfedge: start.index(),
                        face_index: face_id.index(),
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity { entity_kind: "Face", index: face_id.index() },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Face {} loop did not close after {} iterations",
                            face_id.index(), MAX_ITER
                        ),
                    }),
                });
            }
        }
    }
    Ok(())
}

/// Validate the classic Euler formula for genus-0 shells.
///
/// V - E + F = 2 for a closed, orientable, genus-0 surface.
///
/// E is computed as half_edge_count / 2 (each manifold edge has two halfedges).
/// Self-loop halfedges (twin == self) are excluded since they don't form real edges.
///
/// Inner-loop (hole) correction will be added when inner loop support lands.
fn validate_euler(arena: &TopologyArena) -> Result<(), KernelError> {
    let v = arena.vertex_count() as i64;
    let f = arena.face_count() as i64;

    if v == 0 && f == 0 {
        return Ok(());
    }

    let mut real_edge_count: i64 = 0;
    for (he_id, he_data) in arena.iter_half_edges() {
        if he_id != he_data.twin && he_id.index() < he_data.twin.index() {
            real_edge_count += 1;
        }
    }

    let euler_char = v - real_edge_count + f;
    let expected = 2_i64;

    if euler_char != expected {
        return Err(KernelError::TopologyViolation {
            err: forge_core::TopologyError::EulerFormulaViolation {
                vertices: v as usize,
                edges: real_edge_count as usize,
                faces: f as usize,
                expected_chi: expected,
                actual_chi: euler_char,
            },
            context: Some(forge_core::ErrorContext {
                scope: forge_core::ErrorScope::Entity { entity_kind: "Solid", index: 0 },
                suggested_fixes: Vec::new(),
                detail: format!(
                    "Euler formula violated: V-E+F = {}-{}+{} = {} (expected {})",
                    v, real_edge_count, f, euler_char, expected
                ),
            }),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::TopologyState;
    use crate::operator::apply_op;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;

    #[test]
    fn empty_arena_validates() {
        let arena = TopologyArena::new();
        assert!(validate_topology(&arena).is_ok());
    }

    #[test]
    fn seed_validates() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let _mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();
        assert!(validate_topology(state.arena()).is_ok());
    }

    #[test]
    fn split_validates() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let state = draft.commit().unwrap();
        assert!(validate_topology(state.arena()).is_ok());
    }
}
