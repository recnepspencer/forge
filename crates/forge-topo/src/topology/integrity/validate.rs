//! Topology validation for commit-time invariant checking.
//!
//! DOMAIN: Structural integrity checks for the halfedge mesh.
//!
//! INVARIANTS:
//! - Twin reciprocity: he.twin.twin == he
//! - Previous consistency: he.prev.next == he
//! - Vertex continuity: next(he).origin == twin(he).origin
//! - Loop closure: following `next` pointers returns to start
//! - Euler formula: V - E + F = 2 per connected shell (supports multi-shell solids)
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs)

use forge_core::KernelError;
use crate::arena::TopologyArena;

/// Validation strictness level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationLevel {
    /// No checks. Trust the operations blindly (fastest).
    None,
    /// Only check local connectivity invariants (twins, prev/next).
    /// Used for Release builds.
    Minimal,
    /// Full global validity checks (Euler formula, loop closure).
    /// Used for Debug/Test builds.
    Full,
}

impl Default for ValidationLevel {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            ValidationLevel::Full
        } else {
            ValidationLevel::Minimal
        }
    }
}


/// Validate the topology of an arena with the specified strictness.
///
/// Called automatically by `MutableDraft::commit()`. Runs checks based on `level`.
pub fn validate_topology(arena: &TopologyArena, level: ValidationLevel) -> Result<(), KernelError> {
    if level == ValidationLevel::None {
        return Ok(());
    }

    // Level::Minimal checks
    validate_twins(arena)?;
    validate_prev_consistency(arena)?;
    validate_vertex_continuity(arena)?;
    validate_vertex_outgoing(arena)?;

    if level == ValidationLevel::Full {
        validate_loops(arena)?;
        validate_euler(arena)?;
    }

    Ok(())
}

/// Validate twin reciprocity: for every halfedge, `he.twin.twin == he`.
fn validate_twins(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.twin();

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

        if twin_data.twin() != he_id {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::MissingTwin {
                    halfedge_index: he_id.index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity { entity_kind: "HalfEdge".to_string(), index: he_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Twin reciprocity violated: he[{}].twin = {}, but he[{}].twin = {} (expected {})",
                        he_id.index(), twin_id.index(), twin_id.index(), twin_data.twin().index(), he_id.index()
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
        let prev_data = arena.get_half_edge(he_data.prev())?;
        if prev_data.next() != he_id {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: he_id.index(),
                    face_index: he_data.face().index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity { entity_kind: "HalfEdge".to_string(), index: he_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Prev consistency violated: he[{}].prev = {}, but he[{}].next = {} (expected {})",
                        he_id.index(), he_data.prev().index(), he_data.prev().index(), prev_data.next().index(), he_id.index()
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
        if he_id == he_data.twin() && he_id == he_data.next() {
            continue;
        }

        let twin_data = arena.get_half_edge(he_data.twin())?;
        let next_data = arena.get_half_edge(he_data.next())?;

        if he_id != he_data.twin() && next_data.origin() != twin_data.origin() {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: he_id.index(),
                    face_index: he_data.face().index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity { entity_kind: "HalfEdge".to_string(), index: he_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Vertex continuity violated: he[{}].next.origin = {} but he[{}].twin.origin = {} (should be equal)",
                        he_id.index(), next_data.origin().index(), he_id.index(), twin_data.origin().index()
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Validate that every vertex's outgoing halfedge is valid and points back to the vertex.
fn validate_vertex_outgoing(arena: &TopologyArena) -> Result<(), KernelError> {
    for (vid, v_data) in arena.iter_vertices() {
        let out = v_data.outgoing();

        let out_data = arena.get_half_edge(out).map_err(|_| {
            KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: out.index(),
                    face_index: 0,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity { entity_kind: "Vertex".to_string(), index: vid.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Vertex {} outgoing halfedge {}(gen{}) is stale/deleted",
                        vid.index(), out.index(), out.generation()
                    ),
                }),
            }
        })?;

        if out_data.origin() != vid {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: out.index(),
                    face_index: 0,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity { entity_kind: "Vertex".to_string(), index: vid.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Vertex {} outgoing halfedge {} has origin {} (should be {})",
                        vid.index(), out.index(), out_data.origin().index(), vid.index()
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
        let loop_data = arena.get_loop(face_data.outer_loop())?;
        let start = loop_data.half_edge();
        let mut current = start;
        let mut count = 0;

        loop {
            let he_data = arena.get_half_edge(current)?;

            if he_data.face() != face_id {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::BrokenLoop {
                        starting_halfedge: start.index(),
                        face_index: face_id.index(),
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity { entity_kind: "Face".to_string(), index: face_id.index() },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Halfedge {} in loop of face {} belongs to face {} instead",
                            current.index(), face_id.index(), he_data.face().index()
                        ),
                    }),
                });
            }

            current = he_data.next();
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
                        scope: forge_core::ErrorScope::Entity { entity_kind: "Face".to_string(), index: face_id.index() },
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

/// Validate the Euler formula for genus-0, closed, orientable shells.
///
/// Supports multi-shell solids (e.g., disjoint union results) by decomposing
/// the mesh into connected components via face-twin adjacency BFS. Each
/// shell must independently satisfy V - E + F = 2.
///
/// A mesh with N disconnected shells has global chi = 2N. Rather than
/// checking that, we validate per-shell to give precise error reporting.
fn validate_euler(arena: &TopologyArena) -> Result<(), KernelError> {
    use std::collections::{HashSet, VecDeque};
    use crate::handles::FaceId;

    let f_total = arena.face_count();
    if f_total == 0 && arena.vertex_count() == 0 {
        return Ok(());
    }

    let all_faces: Vec<FaceId> = arena.iter_faces().map(|(fid, _)| fid).collect();
    let mut visited_faces: HashSet<u32> = HashSet::with_capacity(f_total);
    let mut shell_index: usize = 0;

    for &seed_face in &all_faces {
        if visited_faces.contains(&seed_face.index()) {
            continue;
        }

        let mut shell_faces: HashSet<u32> = HashSet::new();
        let mut shell_vertices: HashSet<u32> = HashSet::new();
        let mut shell_edges: HashSet<(u32, u32)> = HashSet::new();
        let mut queue: VecDeque<FaceId> = VecDeque::new();

        queue.push_back(seed_face);
        shell_faces.insert(seed_face.index());

        while let Some(face_id) = queue.pop_front() {
            let face_data = arena.get_face(face_id)?;
            let loop_data = arena.get_loop(face_data.outer_loop())?;
            let start_he = loop_data.half_edge();
            let mut current_he = start_he;
            let max_iter = 100_000;

            for _ in 0..max_iter {
                let he_data = arena.get_half_edge(current_he)?;

                shell_vertices.insert(he_data.origin().index());

                if current_he != he_data.twin() {
                    let lo = current_he.index().min(he_data.twin().index());
                    let hi = current_he.index().max(he_data.twin().index());
                    shell_edges.insert((lo, hi));

                    let twin_data = arena.get_half_edge(he_data.twin())?;
                    let neighbor_face = twin_data.face();
                    if !shell_faces.contains(&neighbor_face.index()) {
                        shell_faces.insert(neighbor_face.index());
                        queue.push_back(neighbor_face);
                    }
                }

                current_he = he_data.next();
                if current_he == start_he {
                    break;
                }
            }
        }

        visited_faces.extend(&shell_faces);

        let sv = shell_vertices.len() as i64;
        let se = shell_edges.len() as i64;
        let sf = shell_faces.len() as i64;
        let euler_char = sv - se + sf;
        let expected = 2_i64;

        if euler_char != expected {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::EulerFormulaViolation {
                    vertices: sv as usize,
                    edges: se as usize,
                    faces: sf as usize,
                    expected_chi: expected,
                    actual_chi: euler_char,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Shell".to_string(),
                        index: shell_index as u32,
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Shell {} Euler formula violated: V-E+F = {}-{}+{} = {} (expected {})",
                        shell_index, sv, se, sf, euler_char, expected
                    ),
                }),
            });
        }

        shell_index += 1;
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
        assert!(validate_topology(&arena, ValidationLevel::Full).is_ok());
    }

    #[test]
    fn seed_validates() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let _mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();
        assert!(validate_topology(state.arena(), ValidationLevel::Full).is_ok());
    }

    #[test]
    fn split_validates() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let state = draft.commit().unwrap();
        assert!(validate_topology(state.arena(), ValidationLevel::Full).is_ok());
    }
}
