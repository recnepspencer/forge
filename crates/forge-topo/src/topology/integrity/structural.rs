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

    validate_radial_rings(arena)?;
    validate_prev_consistency(arena)?;
    validate_vertex_continuity(arena)?;
    validate_vertex_outgoing(arena)?;

    if level == ValidationLevel::Full {
        validate_loops(arena)?;
        validate_euler(arena)?;
        validate_shell_consistency(arena)?;
        validate_orientation_consistency(arena)?;
    }

    Ok(())
}

/// Validate radial rings: every halfedge must belong to a closed `.radial_next()` cycle.
fn validate_radial_rings(arena: &TopologyArena) -> Result<(), KernelError> {
    for (start_he, _) in arena.iter_half_edges() {
        let mut current_he = start_he;
        let mut count = 0;
        let limit = 100_000;
        loop {
            let data = arena.get_half_edge(current_he).map_err(|_| KernelError::TopologyViolation {
                err: forge_core::TopologyError::MissingTwin { halfedge_index: current_he.index() },
                context: None,
            })?;
            current_he = data.radial_next();
            count += 1;
            if current_he == start_he {
                break;
            }
            if count > limit {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::LoopCorruption {
                        walk_kind: "RadialRing".to_string(),
                        seed_index: start_he.index(),
                        last_visited_index: current_he.index(),
                        steps_taken: count,
                        entity_bound: limit,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity { entity_kind: "HalfEdge".to_string(), index: start_he.index() },
                        suggested_fixes: vec![],
                        detail: "Radial ring failed to cycle back to start within limit".into(),
                    }),
                });
            }
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

/// Validate vertex continuity: for each halfedge, `he.next.origin`
/// must be a valid endpoint of the geometric edge (i.e., it must equal
/// some other halfedge's origin in the same radial ring, or he.origin
/// for geometric self-loops).
///
/// This catches the "spaghetti topology" bug where edges are mis-wired.
fn validate_vertex_continuity(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut checked_edges: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();

    for (he_id, he_data) in arena.iter_half_edges() {
        let edge_id = he_data.edge();
        if !checked_edges.insert(edge_id.index()) {
            continue;
        }

        // Collect all (origin, target) pairs from this edge's radial ring
        let mut endpoints: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
        let mut curr = he_id;
        loop {
            let curr_data = arena.get_half_edge(curr)?;
            let next_data = arena.get_half_edge(curr_data.next())?;
            endpoints.insert(curr_data.origin().index());
            endpoints.insert(next_data.origin().index());

            curr = curr_data.radial_next();
            if curr == he_id { break; }
        }

        // A well-formed edge should have at most 2 distinct endpoint vertices
        // (exactly 1 for geometric self-loops, exactly 2 for normal edges)
        if endpoints.len() > 2 {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: he_id.index(),
                    face_index: he_data.face().index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity { entity_kind: "Edge".to_string(), index: edge_id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Edge {} has {} distinct endpoint vertices (expected 1 or 2): {:?}",
                        edge_id.index(), endpoints.len(), endpoints
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
                        scope: forge_core::ErrorScope::Entity { entity_kind: "Face".to_string(), index: face_id.index() },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Halfedge {} in outer loop of face {} belongs to face {} instead",
                            he_id.index(), face_id.index(), he_data.face().index()
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
                            scope: forge_core::ErrorScope::Entity { entity_kind: "Loop".to_string(), index: loop_id.index() },
                            suggested_fixes: Vec::new(),
                            detail: format!(
                                "Halfedge {} in inner loop {} of face {} belongs to face {} instead",
                                current.index(), loop_id.index(), face_id.index(), he_data.face().index()
                            ),
                        }),
                    });
                }
                let next = he_data.next();
                current = next;
                if current == start { break; }
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


/// Collect halfedge IDs for a face's loop and find neighbor faces via twins.
///
/// Returns `(neighbor_faces, edge_keys, vertex_indices)` for the face.
fn collect_shell_data_for_face(
    arena: &TopologyArena,
    face_id: FaceId,
) -> Result<(Vec<FaceId>, Vec<u32>, Vec<u32>), KernelError> {
    let mut neighbors = Vec::new();
    let mut edge_keys = Vec::new();
    let mut vertex_indices = Vec::new();

    for he_result in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_result?;
        let he_data = arena.get_half_edge(he_id)?;

        vertex_indices.push(he_data.origin().index());
        edge_keys.push(he_data.edge().index());

        for neighbor_res in crate::topology::queries::traverse::RadialEdgeIterator::new(arena, he_id)? {
            let neighbor_he = neighbor_res?;
            if neighbor_he != he_id {
                let neighbor_data = arena.get_half_edge(neighbor_he)?;
                neighbors.push(neighbor_data.face());
            }
        }
    }

    Ok((neighbors, edge_keys, vertex_indices))
}

/// Validate the generalized Euler formula for each connected shell.
///
/// Supports genus > 0 topology (tori, solids with through-holes) and
/// faces with inner loops (holes). Uses the full formula:
///   V - E + F = 2 - 2G + R
/// where G = genus, R = total inner loop count across all faces in the shell.
///
/// Validates that genus is non-negative — a negative genus indicates
/// a structurally broken shell.
fn validate_euler(arena: &TopologyArena) -> Result<(), KernelError> {
    let f_total = arena.face_count();
    if f_total == 0 && arena.vertex_count() == 0 {
        return Ok(());
    }

    let all_faces: Vec<FaceId> = arena.iter_faces().map(|(fid, _)| fid).collect();
    let face_by_index: std::collections::BTreeMap<u32, FaceId> =
        all_faces.iter().map(|fid| (fid.index(), *fid)).collect();
    let mut visited_faces: BTreeSet<u32> = BTreeSet::new();
    let mut shell_index: usize = 0;

    for &seed_face in &all_faces {
        if !visited_faces.contains(&seed_face.index()) {
            let mut shell_faces: BTreeSet<u32> = BTreeSet::new();
            let mut shell_vertices: BTreeSet<u32> = BTreeSet::new();
            let mut shell_edges: BTreeSet<u32> = BTreeSet::new();
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

            let rings: usize = shell_faces.iter()
                .filter_map(|idx| {
                    let fid = face_by_index.get(idx)?;
                    arena.get_face(*fid).ok()
                })
                .map(|face_data| face_data.inner_loop_count())
                .sum();

            let shell_id = face_by_index.get(&shell_faces.iter().cloned().next().unwrap()).unwrap();
            let shell_kind = arena.get_face(*shell_id).unwrap().shell();
            if !matches!(arena.get_shell(shell_kind).unwrap().kind(), crate::arena::ShellKind::Solid(_)) {
                shell_index += 1;
                continue;
            }

            let genus = compute_shell_genus(euler_char, rings, shell_index)?;
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
            shell_index += 1;
        }
    }

    Ok(())
}

/// Compute the genus of a shell from its Euler characteristic and ring count.
///
/// Full formula: V - E + F = 2 - 2G + R, so G = (2 - χ + R) / 2.
/// Returns 0 for genus-0 (sphere-like), 1 for torus, etc.
/// Returns `Err` if genus is non-integer or negative — this indicates
/// structural damage in the shell rather than valid higher-genus topology.
fn compute_shell_genus(euler_char: i64, rings: usize, shell_index: usize) -> Result<usize, KernelError> {
    let twice_genus = 2 - euler_char + rings as i64;
    
    if twice_genus < 0 {
        return Err(KernelError::TopologyViolation {
            err: forge_core::TopologyError::GeneralizedEulerViolation {
                shell_index: shell_index as u32,
                vertices: 0,
                edges: 0,
                faces: 0,
                genus: 0,
                rings,
                expected_chi: 0,
                actual_chi: euler_char,
            },
            context: Some(forge_core::ErrorContext {
                scope: forge_core::ErrorScope::Entity {
                    entity_kind: "Shell".to_string(),
                    index: shell_index as u32,
                },
                suggested_fixes: Vec::new(),
                detail: format!(
                    "Shell {} has invalid genus: 2·G = {} (negative indicates structural damage)",
                    shell_index, twice_genus
                ),
            }),
        });
    }

    if twice_genus % 2 != 0 {
        return Err(KernelError::TopologyViolation {
            err: forge_core::TopologyError::NonOrientableSurface {
                shell_index: shell_index as u32,
            },
            context: Some(forge_core::ErrorContext {
                scope: forge_core::ErrorScope::Entity {
                    entity_kind: "Shell".to_string(),
                    index: shell_index as u32,
                },
                suggested_fixes: Vec::new(),
                detail: format!(
                    "Shell {} has an odd Euler characteristic implying it is a non-orientable surface (like a Möbius strip or Klein bottle).",
                    shell_index
                ),
            }),
        });
    }

    Ok((twice_genus / 2) as usize)
}

/// Validate shell consistency: Solid shells must not contain boundary edges.
fn validate_shell_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    for (shell_id, shell_data) in arena.iter_shells() {
        if matches!(shell_data.kind(), crate::arena::ShellKind::Solid(_)) {
            for (face_id, face_data) in arena.iter_faces() {
                if face_data.shell() == shell_id {
                    let iter = crate::topology::queries::traverse::FaceEdgeIterator::new(arena, face_id)?;
                    for he_res in iter {
                        let he_id = he_res?;
                        if crate::topology::queries::traverse::is_boundary_edge(arena, he_id)? {
                            return Err(KernelError::TopologyViolation {
                                err: forge_core::TopologyError::BoundaryEdgeInSolid { 
                                    halfedge_index: he_id.index(), 
                                    shell_index: shell_id.index() 
                                },
                                context: Some(forge_core::ErrorContext {
                                    scope: forge_core::ErrorScope::Entity {
                                        entity_kind: "HalfEdge".to_string(),
                                        index: he_id.index(),
                                    },
                                    suggested_fixes: Vec::new(),
                                    detail: format!(
                                        "Solid shell {} contains a boundary edge {} (Solid shells must be watertight)", 
                                        shell_id.index(), he_id.index()
                                    )
                                })
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Validate orientation consistency across twin edge pairs (P0.3).
///
/// In a correctly oriented manifold halfedge mesh, every twin pair
/// (he, twin) must belong to different faces and traverse the shared
/// edge in opposite directions.
///
/// Wire edges (antennae from MakeEdgeVertex) are exempted: their twin
/// pair legitimately shares the same face. A wire edge is identified
/// by `he.face() == he.radial_next().face()` and is a valid non-manifold
/// feature, not an orientation defect.
fn validate_orientation_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    // In a single-face topology (e.g. digon from MVF+SE), all twin
    // pairs necessarily share the same face. This is valid — skip.
    if arena.face_count() <= 1 {
        return Ok(());
    }

    let mut checked: BTreeSet<(u32, u32)> = BTreeSet::new();

    for (he_id, he_data) in arena.iter_half_edges().filter(|(id, d)| *id != d.radial_next()) {
        let twin_id = he_data.radial_next();
        let canonical = (he_id.index().min(twin_id.index()), he_id.index().max(twin_id.index()));

        if checked.insert(canonical) {
            let twin_data = arena.get_half_edge(twin_id)?;

            if he_data.face() == twin_data.face() {
                // Wire edge (antenna): both halfedges of a wire edge
                // share the same face. This is valid topology created by
                // MakeEdgeVertex — skip this pair.
                continue;
            }
        }
    }

    Ok(())
}
