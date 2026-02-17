//! Post-commit topology validation (Doctrine D4, D6).
//!
//! DOMAIN: Structural integrity checks for halfedge meshes.
//!
//! INVARIANTS:
//! - These checks run automatically on `MutableDraft::commit()`
//! - They are never optional, never skippable
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs)

use forge_core::{KernelError, TopologyError, ErrorContext, ErrorScope};
use crate::arena::TopologyArena;

/// Validate all topology invariants for the arena.
///
/// Called automatically by `MutableDraft::commit()`.
/// Returns `Ok(())` if all invariants hold, or a structured error
/// describing exactly which invariant was violated.
pub fn validate_topology(arena: &TopologyArena) -> Result<(), KernelError> {
    validate_twins(arena)?;
    validate_loops(arena)?;
    validate_euler(arena)?;
    validate_manifold(arena)?;
    Ok(())
}

/// Every halfedge must have a valid twin, and twin.twin must equal self.
fn validate_twins(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let _twin_data = arena.get_half_edge(he_data.twin).map_err(|_| {
            KernelError::TopologyViolation {
                err: TopologyError::MissingTwin {
                    halfedge_index: he_id.index(),
                },
                context: None,
            }
        })?;

        let twin_data = arena.get_half_edge(he_data.twin)?;
        if twin_data.twin != he_id {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::MissingTwin {
                    halfedge_index: he_id.index(),
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "HalfEdge", index: he_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("HalfEdge {} has twin {}, but twin's twin is {} (not self)", he_id, he_data.twin, twin_data.twin),
                }),
            });
        }
    }
    Ok(())
}

/// Every face loop must be closed (following `next` returns to start).
fn validate_loops(arena: &TopologyArena) -> Result<(), KernelError> {
    for (face_id, face_data) in arena.iter_faces() {
        let loop_data = arena.get_loop(face_data.outer_loop)?;
        let start = loop_data.half_edge;
        let mut current = start;
        let max_iterations: usize = 10000;

        let mut found_cycle = false;
        for _ in 0..max_iterations {
            let he = arena.get_half_edge(current)?;
            if he.face != face_id {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::BrokenLoop {
                        face_index: face_id.index(),
                        starting_halfedge: start.index(),
                    },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Entity { entity_kind: "Face", index: face_id.index() },
                        suggested_fixes: Vec::new(),
                        detail: format!("HalfEdge {} in loop of Face {} has face={} (wrong face)", current, face_id, he.face),
                    }),
                });
            }
            current = he.next;
            if current == start {
                found_cycle = true;
                break;
            }
        }

        if !found_cycle {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::BrokenLoop {
                    face_index: face_id.index(),
                    starting_halfedge: start.index(),
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Face", index: face_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Face {} loop starting at {} does not close within iteration limit", face_id, start),
                }),
            });
        }
    }
    Ok(())
}

/// Check the Euler formula: V - E + F = 2S for genus-0 solids with S shells.
///
/// Skips the check if the arena is empty or contains only a degenerate
/// seed (1 vertex, 1 face, 1 halfedge from MVF).
///
/// Counts connected components (shells) by BFS over face adjacency
/// and validates χ = 2S where S = number of shells.
fn validate_euler(arena: &TopologyArena) -> Result<(), KernelError> {
    let vertices = arena.vertex_count();
    let half_edges = arena.half_edge_count();
    let faces = arena.face_count();

    if vertices == 0 && half_edges == 0 && faces == 0 {
        return Ok(());
    }

    if vertices == 1 && half_edges == 1 && faces == 1 {
        return Ok(());
    }

    if half_edges % 2 != 0 {
        return Err(KernelError::TopologyViolation {
            err: TopologyError::NonManifoldEdge {
                edge_index: 0,
            },
            context: Some(ErrorContext {
                scope: ErrorScope::Global,
                suggested_fixes: Vec::new(),
                detail: format!("Odd number of halfedges ({}); every edge needs exactly two halfedges", half_edges),
            }),
        });
    }

    let edges = half_edges / 2;
    let shells = count_shells(arena)?;
    check_euler_formula_multishell(vertices, edges, faces, shells)
}

/// Count connected components (shells) in the topology via BFS.
///
/// Two faces are in the same shell if they share an edge (via twin).
fn count_shells(arena: &TopologyArena) -> Result<usize, KernelError> {
    use std::collections::HashSet;
    use std::collections::VecDeque;

    let all_faces: Vec<crate::handles::FaceId> = arena.iter_faces()
        .map(|(fid, _)| fid)
        .collect();

    if all_faces.is_empty() {
        return Ok(0);
    }

    let mut visited: HashSet<u64> = HashSet::new();
    let mut shell_count: usize = 0;
    let max_iterations: usize = 100_000;
    let mut total_iterations: usize = 0;

    for &start_face in &all_faces {
        let start_key = (u64::from(start_face.generation()) << 32)
            | u64::from(start_face.index());

        if visited.contains(&start_key) {
            continue;
        }

        shell_count += 1;
        let mut queue: VecDeque<crate::handles::FaceId> = VecDeque::new();
        queue.push_back(start_face);
        visited.insert(start_key);

        while let Some(face) = queue.pop_front() {
            total_iterations += 1;
            if total_iterations > max_iterations {
                return Err(KernelError::InternalError {
                    message: "Loop limit exceeded in count_shells".to_string(),
                    context: None,
                });
            }

            let face_data = arena.get_face(face)?;
            let loop_data = arena.get_loop(face_data.outer_loop)?;
            let start_he = loop_data.half_edge;
            let mut current = start_he;

            loop {
                total_iterations += 1;
                if total_iterations > max_iterations {
                    return Err(KernelError::InternalError {
                        message: "Loop limit exceeded in count_shells (inner)".to_string(),
                        context: None,
                    });
                }

                let he_data = arena.get_half_edge(current)?;
                let twin_data = arena.get_half_edge(he_data.twin)?;
                let neighbor_face = twin_data.face;
                let neighbor_key = (u64::from(neighbor_face.generation()) << 32)
                    | u64::from(neighbor_face.index());

                if !visited.contains(&neighbor_key) {
                    visited.insert(neighbor_key);
                    queue.push_back(neighbor_face);
                }

                current = he_data.next;
                if current == start_he {
                    break;
                }
            }
        }
    }

    Ok(shell_count)
}

/// Check the Euler formula: V - E + F = 2S for genus-0 solids with S shells.
pub fn check_euler_formula(
    vertices: usize,
    edges: usize,
    faces: usize,
) -> Result<(), KernelError> {
    check_euler_formula_multishell(vertices, edges, faces, 1)
}

/// Check the Euler formula for a topology with S shells.
fn check_euler_formula_multishell(
    vertices: usize,
    edges: usize,
    faces: usize,
    shells: usize,
) -> Result<(), KernelError> {
    let chi = vertices as i64 - edges as i64 + faces as i64;
    let expected_chi: i64 = 2 * shells as i64;

    if chi != expected_chi {
        return Err(KernelError::TopologyViolation {
            err: TopologyError::EulerFormulaViolation {
                vertices,
                edges,
                faces,
                expected_chi,
                actual_chi: chi,
            },
            context: None,
        });
    }

    Ok(())
}

/// No edge should be shared by more than 2 faces (manifold condition).
///
/// In a valid halfedge mesh, each halfedge pair is shared by exactly 2 faces
/// (or the same face for boundary edges). This is enforced structurally by
/// the twin relationship, but we verify it explicitly.
fn validate_manifold(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_data = arena.get_half_edge(he_data.twin)?;
        if twin_data.twin != he_id {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::NonManifoldEdge {
                    edge_index: 0, // Placeholder
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "HalfEdge", index: he_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("HalfEdge {} twin chain is not bidirectional (non-manifold)", he_id),
                }),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_passes_euler_check() {
        assert!(check_euler_formula(8, 12, 6).is_ok());
    }

    #[test]
    fn tetrahedron_passes_euler_check() {
        assert!(check_euler_formula(4, 6, 4).is_ok());
    }

    #[test]
    fn invalid_mesh_fails_euler_check() {
        let result = check_euler_formula(8, 11, 6);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(
            error,
            KernelError::TopologyViolation {
                err: TopologyError::EulerFormulaViolation {
                    actual_chi: 3,
                    expected_chi: 2,
                    ..
                },
                ..
            }
        ));
    }

    #[test]
    fn empty_arena_passes_validation() {
        let arena = TopologyArena::new();
        assert!(validate_topology(&arena).is_ok());
    }
}
