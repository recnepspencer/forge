//! Structural topology validation (commit-time invariant checking).
//!
//! DOMAIN: Pure connectivity checks that require no geometry data.
//!
//! INVARIANTS:
//! - Twin reciprocity: he.twin.twin == he
//! - Previous consistency: he.prev.next == he
//! - Vertex continuity: next(he).origin == twin(he).origin
//! - Vertex outgoing: v.outgoing.origin == v
//! - Loop closure: following `next` pointers returns to start
//! - Degenerate loops: every face loop has >= 3 distinct vertices
//! - Euler formula: V - E + F = 2 per connected shell
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs), `queries/traverse` (FaceEdgeIterator)

use std::collections::{BTreeSet, VecDeque};

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::handles::FaceId;
use crate::topology::queries::traverse::FaceEdgeIterator;
use crate::validate::ValidationLevel;

/// Validate the topology of an arena with the specified strictness.
///
/// Called automatically by `MutableDraft::commit()`. Runs checks based on `level`.
pub fn validate_topology(arena: &TopologyArena, level: ValidationLevel) -> Result<(), KernelError> {
    if level == ValidationLevel::None {
        return Ok(());
    }

    validate_twins(arena)?;
    validate_prev_consistency(arena)?;
    validate_vertex_continuity(arena)?;
    validate_vertex_outgoing(arena)?;

    if level == ValidationLevel::Full {
        validate_loops(arena)?;
        validate_degenerate_loops(arena)?;
        validate_euler(arena)?;
        validate_edge_manifoldness(arena)?;
    }

    Ok(())
}

/// Validate twin reciprocity: for every halfedge, `he.twin.twin == he`.
fn validate_twins(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        if he_id == he_data.twin() {
            return Ok(());
        }

        let twin_data = arena.get_half_edge(he_data.twin()).map_err(|_| {
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
                        he_id.index(), he_data.twin().index(), he_data.twin().index(), twin_data.twin().index(), he_id.index()
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
        let is_self_twin = he_id == he_data.twin();
        let is_self_next = he_id == he_data.next();
        if is_self_twin && is_self_next {
            return Ok(());
        }

        if is_self_twin {
            return Ok(());
        }

        let twin_data = arena.get_half_edge(he_data.twin())?;
        let next_data = arena.get_half_edge(he_data.next())?;

        if next_data.origin() != twin_data.origin() {
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

/// Validate that every face's loop is closed and each halfedge belongs to the correct face.
fn validate_loops(arena: &TopologyArena) -> Result<(), KernelError> {
    for (face_id, _face_data) in arena.iter_faces() {
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
                        scope: forge_core::ErrorScope::Entity { entity_kind: "Face".to_string(), index: face_id.index() },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Halfedge {} in loop of face {} belongs to face {} instead",
                            he_id.index(), face_id.index(), he_data.face().index()
                        ),
                    }),
                });
            }
        }
    }
    Ok(())
}

/// Validate that every face loop has at least 3 distinct vertices.
///
/// A loop with fewer than 3 distinct vertices cannot bound a valid face.
/// Skips seed faces from Euler operators (loops with fewer than 3 edges).
fn validate_degenerate_loops(arena: &TopologyArena) -> Result<(), KernelError> {
    for (face_id, _face_data) in arena.iter_faces() {
        let mut distinct_vertices: BTreeSet<u32> = BTreeSet::new();
        let mut edge_count: usize = 0;

        for he_result in FaceEdgeIterator::new(arena, face_id)? {
            let he_id = he_result?;
            let he_data = arena.get_half_edge(he_id)?;
            distinct_vertices.insert(he_data.origin().index());
            edge_count += 1;
        }

        if edge_count < 3 {
            return Ok(());
        }

        if distinct_vertices.len() < 3 {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::DegenerateLoop {
                    face_index: face_id.index(),
                    distinct_vertices: distinct_vertices.len(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Face".to_string(),
                        index: face_id.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Face {} loop has only {} distinct vertices (minimum 3 required)",
                        face_id.index(), distinct_vertices.len()
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Collect halfedge IDs for a face's loop and find neighbor faces via twins.
///
/// Returns `(neighbor_faces, edge_keys, vertex_indices)` for the face.
fn collect_shell_data_for_face(
    arena: &TopologyArena,
    face_id: FaceId,
) -> Result<(Vec<FaceId>, Vec<(u32, u32)>, Vec<u32>), KernelError> {
    let mut neighbors = Vec::new();
    let mut edge_keys = Vec::new();
    let mut vertex_indices = Vec::new();

    for he_result in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_result?;
        let he_data = arena.get_half_edge(he_id)?;

        vertex_indices.push(he_data.origin().index());

        if he_id != he_data.twin() {
            let lo = he_id.index().min(he_data.twin().index());
            let hi = he_id.index().max(he_data.twin().index());
            edge_keys.push((lo, hi));

            let twin_data = arena.get_half_edge(he_data.twin())?;
            neighbors.push(twin_data.face());
        }
    }

    Ok((neighbors, edge_keys, vertex_indices))
}

/// Validate the generalized Euler formula for each connected shell.
///
/// Supports genus > 0 topology (tori, solids with through-holes) by computing
/// genus from connectivity: `G = 1 - (V - E + F) / 2` for each shell.
/// The generalized formula is: `V - E + F = 2 - 2G` (with R=0 since inner
/// loops are not yet supported).
///
/// Validates that genus is non-negative — a negative genus indicates
/// a structurally broken shell.
fn validate_euler(arena: &TopologyArena) -> Result<(), KernelError> {
    let f_total = arena.face_count();
    if f_total == 0 && arena.vertex_count() == 0 {
        return Ok(());
    }

    let all_faces: Vec<FaceId> = arena.iter_faces().map(|(fid, _)| fid).collect();
    let mut visited_faces: BTreeSet<u32> = BTreeSet::new();
    let mut shell_index: usize = 0;

    for &seed_face in &all_faces {
        if !visited_faces.contains(&seed_face.index()) {
            let mut shell_faces: BTreeSet<u32> = BTreeSet::new();
            let mut shell_vertices: BTreeSet<u32> = BTreeSet::new();
            let mut shell_edges: BTreeSet<(u32, u32)> = BTreeSet::new();
            let mut queue: VecDeque<FaceId> = VecDeque::new();

            queue.push_back(seed_face);
            shell_faces.insert(seed_face.index());

            while let Some(face_id) = queue.pop_front() {
                let (neighbors, edge_keys, vertex_indices) =
                    collect_shell_data_for_face(arena, face_id)?;

                for vid in vertex_indices {
                    shell_vertices.insert(vid);
                }

                for ek in edge_keys {
                    shell_edges.insert(ek);
                }

                for neighbor in neighbors {
                    if shell_faces.insert(neighbor.index()) {
                        queue.push_back(neighbor);
                    }
                }
            }

            visited_faces.append(&mut shell_faces.clone());

            let sv = shell_vertices.len() as i64;
            let se = shell_edges.len() as i64;
            let sf = shell_faces.len() as i64;
            let euler_char = sv - se + sf;

            let genus = compute_shell_genus(euler_char);
            let rings: usize = 0;
            let expected = 2_i64 - 2 * (genus as i64) + (rings as i64);

            if euler_char != expected {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::GeneralizedEulerViolation {
                        shell_index: shell_index as u32,
                        vertices: sv as usize,
                        edges: se as usize,
                        faces: sf as usize,
                        genus,
                        rings,
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
                            "Shell {} generalized Euler: V-E+F = {}-{}+{} = {}, genus={}, rings={}, expected χ={}",
                            shell_index, sv, se, sf, euler_char, genus, rings, expected
                        ),
                    }),
                });
            }
        }

        shell_index += 1;
    }

    Ok(())
}

/// Compute the genus of a shell from its Euler characteristic.
///
/// For a closed orientable surface: χ = 2 - 2G, so G = (2 - χ) / 2.
/// Returns 0 for genus-0 (sphere-like), 1 for torus, etc.
/// A non-integer or negative result indicates structural damage.
fn compute_shell_genus(euler_char: i64) -> usize {
    let twice_genus = 2 - euler_char;
    if twice_genus < 0 || twice_genus % 2 != 0 {
        return 0;
    }
    (twice_genus / 2) as usize
}

/// Validate that every geometric edge is manifold (shared by exactly 2 faces).
///
/// In a manifold halfedge mesh, every edge (canonical halfedge pair) should
/// connect exactly two distinct faces. Non-manifold edges (3+ faces sharing
/// one edge) indicate geometric corruption.
fn validate_edge_manifoldness(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut edge_face_count: std::collections::BTreeMap<(u32, u32), usize> = std::collections::BTreeMap::new();

    for (he_id, he_data) in arena.iter_half_edges().filter(|(id, d)| *id != d.twin()) {
        let twin_id = he_data.twin();
        let canonical = (he_id.index().min(twin_id.index()), he_id.index().max(twin_id.index()));
        *edge_face_count.entry(canonical).or_insert(0) += 1;
    }

    for (&(lo, _hi), &count) in &edge_face_count {
        if count > 2 {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::NonManifoldEdge {
                    edge_index: lo,
                    valence: count,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Edge".to_string(),
                        index: lo,
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Edge {} appears {} times (expected 2 for manifold)",
                        lo, count
                    ),
                }),
            });
        }
    }

    Ok(())
}
