//! Global Polygon Extraction (Maximal Region Extraction).
//!
//! DOMAIN: Replace iterative JoinFaces for coplanar merging with an O(N)
//! "nuke-and-pave" approach. Discovers coplanar face groups, walks their
//! boundary perimeter, deletes the fragments, and rebuilds a single clean face.
//!
//! ALGORITHM:
//!   1. Graph discovery: BFS to find connected coplanar face clusters
//!   2. Boundary walk: collect perimeter vertices via twin-hopping
//!   3. Nuke and pave: delete group, rebuild single face from perimeter
//!
//! DEPENDENCIES: forge_topo (arena, handles), GeometryStore, forge_geom (exact_eq)

use std::collections::{BTreeSet, BTreeMap, VecDeque};

use forge_topo::bitset::EntityBitset;

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_core::tracing::TopologyDelta;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId, ShellId, EdgeId};
use forge_topo::state::{TopologyState, MutableDraft};

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::geometry_store::GeometryStore;

/// Extract and merge coplanar regions using the global polygon approach.
///
/// Discovers all coplanar face groups, walks their perimeter boundaries,
/// then rebuilds each group as a single clean face. Falls back to None
/// (caller should use legacy path) if any group extraction fails.
pub fn extract_coplanar_regions(
    topo: TopologyState,
    geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let groups = discover_coplanar_groups(topo.arena(), geom);
    let mergeable: Vec<_> = groups.into_iter().filter(|g| g.count() >= 2).collect();

    if mergeable.is_empty() {
        return Ok((topo, 0));
    }

    let mut draft = topo.into_mutation();
    let pre_snapshot = ArenaSnapshot::capture(draft.arena());
    let mut total_merged = 0usize;

    for group in &mergeable {
        let perimeter = walk_boundary_perimeter(draft.arena(), group)?;
        if perimeter.len() < 3 {
            return Err(KernelError::InternalError {
                message: format!(
                    "Coplanar group of {} faces produced only {} perimeter vertices",
                    group.count(), perimeter.len()
                ),
                context: None,
            });
        }

        let sample_idx = group.iter_ones().next().ok_or_else(|| KernelError::InternalError {
            message: "Empty coplanar group".to_string(),
            context: None,
        })?;
        let sample_face = FaceId::from_raw_parts(sample_idx, 0);
        let lineage = draft.arena().get_face(sample_face)?.lineage().cloned();

        rebuild_face_from_perimeter(&mut draft, group, &perimeter, lineage.as_ref(), geom)?;
        total_merged += (group.count() - 1) as usize;
    }

    let delta = compute_topology_delta(&pre_snapshot, draft.arena());
    log_extraction(total_merged, mergeable.len(), delta, ctx);

    Ok((draft.commit()?, total_merged))
}

/// Discover connected coplanar face groups via BFS.
///
/// Two faces are in the same group if:
/// 1. They share an edge (via twin pointers)
/// 2. Their plane equations are exactly equal (via `exact_eq`)
fn discover_coplanar_groups(
    arena: &TopologyArena,
    geom: &GeometryStore,
) -> Vec<EntityBitset> {
    let mut visited = EntityBitset::for_faces(arena);
    let mut groups: Vec<EntityBitset> = Vec::new();

    for (face_id, _) in arena.iter_faces() {
        if visited.contains(face_id.index()).unwrap_or(false) {
            let _ = face_id;
        } else {
            let group = bfs_coplanar_cluster(arena, geom, face_id, &mut visited);
            if group.count() >= 2 {
                groups.push(group);
            }
        }
    }

    groups
}

/// BFS from a seed face to find all connected coplanar neighbors.
fn bfs_coplanar_cluster(
    arena: &TopologyArena,
    geom: &GeometryStore,
    seed: FaceId,
    visited: &mut EntityBitset,
) -> EntityBitset {
    let mut group = EntityBitset::for_faces(arena);
    let mut queue = VecDeque::new();

    let seed_plane = match geom.get_face_plane(seed) {
        Some(p) => p,
        None => {
            let _ = visited.insert(seed.index());
            let _ = group.insert(seed.index());
            return group;
        }
    };

    let _ = visited.insert(seed.index());
    let _ = group.insert(seed.index());
    queue.push_back(seed);

    while let Some(current) = queue.pop_front() {
        let neighbors = collect_adjacent_faces(arena, current);
        for neighbor in neighbors {
            if visited.contains(neighbor.index()).unwrap_or(false) {
                let _ = neighbor;
            } else {
                let _ = visited.insert(neighbor.index());
                if let Some(neighbor_plane) = geom.get_face_plane(neighbor) {
                    if forge_geom::primitives::plane::exact_eq(seed_plane, neighbor_plane) {
                        let _ = group.insert(neighbor.index());
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    group
}

/// Collect all face IDs adjacent to `face` via twin pointers.
fn collect_adjacent_faces(arena: &TopologyArena, face: FaceId) -> Vec<FaceId> {
    let mut neighbors = Vec::new();
    let face_data = match arena.get_face(face) {
        Ok(f) => f,
        Err(_) => return neighbors,
    };

    let loop_data = match arena.get_loop(face_data.outer_loop()) {
        Ok(l) => l,
        Err(_) => return neighbors,
    };

    let start = loop_data.half_edge();
    let mut current = start;
    let mut steps = 0usize;

    loop {
        if let Ok(he) = arena.get_half_edge(current) {
            if let Ok(twin) = arena.get_half_edge(he.radial_next()) {
                let twin_face = twin.face();
                if twin_face != face {
                    neighbors.push(twin_face);
                }
            }
            current = he.next();
        } else {
            break;
        }

        steps += 1;
        if current == start || steps > 100_000 {
            break;
        }
    }

    neighbors
}

/// Walk the perimeter of a coplanar group, collecting boundary vertices.
///
/// A half-edge is a "boundary edge" if its twin's face is outside the group.
/// An "internal edge" has its twin inside the group.
///
/// Walk logic: start at any boundary edge, follow `next()`. If `next()` is
/// a boundary edge, continue. If `next()` is internal, twin-hop until
/// landing on the next boundary edge.
fn walk_boundary_perimeter(
    arena: &TopologyArena,
    group: &EntityBitset,
) -> Result<Vec<VertexId>, KernelError> {
    let start_he = find_boundary_edge(arena, group)?;
    let mut perimeter = Vec::new();
    let mut current = start_he;
    let mut steps = 0usize;

    loop {
        let he_data = arena.get_half_edge(current)?;
        perimeter.push(he_data.origin());

        let next_candidate = he_data.next();
        current = advance_to_boundary(arena, group, next_candidate)?;

        steps += 1;
        if current == start_he || steps > 100_000 {
            break;
        }
    }

    if steps > 100_000 {
        return Err(KernelError::InternalError {
            message: "Perimeter walk exceeded maximum iterations".to_string(),
            context: None,
        });
    }

    Ok(perimeter)
}

/// Find any boundary half-edge in the group.
fn find_boundary_edge(
    arena: &TopologyArena,
    group: &EntityBitset,
) -> Result<HalfEdgeId, KernelError> {
    for face_idx in group.iter_ones() {
        let face_id = FaceId::from_raw_parts(face_idx, 0);
        let face_data = arena.get_face(face_id)?;
        let loop_data = arena.get_loop(face_data.outer_loop())?;
        let start = loop_data.half_edge();
        let mut current = start;
        let mut steps = 0usize;

        loop {
            let he = arena.get_half_edge(current)?;
            if is_boundary_edge(arena, group, current) {
                return Ok(current);
            }
            current = he.next();
            steps += 1;
            if current == start || steps > 100_000 {
                break;
            }
        }
    }

    Err(KernelError::InternalError {
        message: "No boundary edge found in coplanar group".to_string(),
        context: None,
    })
}

/// Check if a half-edge is a boundary edge (twin is outside the group).
fn is_boundary_edge(
    arena: &TopologyArena,
    group: &EntityBitset,
    he: HalfEdgeId,
) -> bool {
    let he_data = match arena.get_half_edge(he) {
        Ok(d) => d,
        Err(_) => return false,
    };
    let twin_data = match arena.get_half_edge(he_data.radial_next()) {
        Ok(d) => d,
        Err(_) => return true,
    };
    !group.contains(twin_data.face().index()).unwrap_or(false)
}

/// Advance from a candidate to the next boundary edge, twin-hopping over internals.
fn advance_to_boundary(
    arena: &TopologyArena,
    group: &EntityBitset,
    start: HalfEdgeId,
) -> Result<HalfEdgeId, KernelError> {
    let mut current = start;
    let mut steps = 0usize;

    while !is_boundary_edge(arena, group, current) {
        let he_data = arena.get_half_edge(current)?;
        let twin_id = he_data.radial_next();
        let twin_data = arena.get_half_edge(twin_id)?;
        current = twin_data.next();

        steps += 1;
        if steps > 100_000 {
            return Err(KernelError::InternalError {
                message: "Twin-hop exceeded maximum iterations".to_string(),
                context: None,
            });
        }
    }

    Ok(current)
}

/// Delete all entities in the group and rebuild a single face from perimeter vertices.
// DEFECT(D3): rebuild_face_from_perimeter uses raw insert_radial_pair instead of Euler ops.
fn rebuild_face_from_perimeter(
    draft: &mut MutableDraft,
    group: &EntityBitset,
    perimeter: &[VertexId],
    lineage: Option<&forge_topo::lineage::Lineage>,
    _geom: &GeometryStore,
) -> Result<FaceId, KernelError> {
    let edges_to_delete = collect_group_edges(draft.arena(), group)?;
    let internal_vertices = find_internal_vertices(draft.arena(), group, perimeter)?;

    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let placeholder_loop = forge_topo::handles::LoopId::from_raw_parts(u32::MAX, 0);
    
    let sample_idx = group.iter_ones().next().unwrap();
    let sample_face = FaceId::from_raw_parts(sample_idx, 0);
    let shell = draft.arena().get_face(sample_face)?.shell();

    let new_face = draft.insert_face(
        forge_topo::arena::FaceData::with_lineage(placeholder_loop, shell, lineage.cloned()),
    );
    let new_loop = draft.insert_loop(
        forge_topo::arena::LoopData::new(placeholder_he, new_face),
    );
    draft.arena_mut().get_face_mut(new_face)?.set_outer_loop(new_loop);

    let n = perimeter.len();
    let mut new_half_edges: Vec<HalfEdgeId> = Vec::with_capacity(n);

    for i in 0..n {
        let origin = perimeter[i];
        let (he, twin_he) = draft.insert_radial_pair(
            forge_topo::arena::HalfEdgeData::new(
                placeholder_he, placeholder_he, placeholder_he, new_face, origin, EdgeId::from_raw_parts(u32::MAX, 0),
            ),
            forge_topo::arena::HalfEdgeData::new(
                placeholder_he, placeholder_he, placeholder_he, new_face, perimeter[(i + 1) % n], EdgeId::from_raw_parts(u32::MAX, 0),
            ),
        );
        let edge = draft.insert_edge(forge_topo::arena::EdgeData::new(he));
        draft.arena_mut().get_half_edge_mut(he)?.set_edge(edge);
        draft.arena_mut().get_half_edge_mut(twin_he)?.set_edge(edge);
        
        new_half_edges.push(he);
    }

    for i in 0..n {
        let next_idx = (i + 1) % n;
        let prev_idx = if i == 0 { n - 1 } else { i - 1 };

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(new_half_edges[i])?.set_next(new_half_edges[next_idx]);
        arena.get_half_edge_mut(new_half_edges[i])?.set_prev(new_half_edges[prev_idx]);
    }

    draft.arena_mut().get_loop_mut(new_loop)?.set_half_edge(new_half_edges[0]);

    for &vid in perimeter {
        let matching_he = new_half_edges.iter().find(|&&he_id| {
            draft.arena().get_half_edge(he_id)
                .map(|he| he.origin() == vid)
                .unwrap_or(false)
        });
        if let Some(&he_id) = matching_he {
            draft.arena_mut().get_vertex_mut(vid).ok().map(|v| v.set_outgoing(he_id));
        }
    }

    for &(he_a, he_b) in &edges_to_delete {
        let _ = draft.remove_half_edge(he_a);
        let _ = draft.remove_half_edge(he_b);
    }

    for face_idx in group.iter_ones() {
        let face_id = FaceId::from_raw_parts(face_idx, 0);
        let face_data = draft.arena().get_face(face_id)?;
        let loop_id = face_data.outer_loop();
        let _ = draft.remove_loop(loop_id);
        let _ = draft.remove_face(face_id);
    }

    for &vid in &internal_vertices {
        let _ = draft.remove_vertex(vid);
    }

    Ok(new_face)
}

/// Collect all half-edge pairs belonging to faces in the group.
fn collect_group_edges(
    arena: &TopologyArena,
    group: &EntityBitset,
) -> Result<Vec<(HalfEdgeId, HalfEdgeId)>, KernelError> {
    let mut edges: BTreeSet<(u32, u32)> = BTreeSet::new();
    let mut result = Vec::new();

    for face_idx in group.iter_ones() {
        let face_id = FaceId::from_raw_parts(face_idx, 0);
        let face_data = arena.get_face(face_id)?;
        let loop_data = arena.get_loop(face_data.outer_loop())?;
        let start = loop_data.half_edge();
        let mut current = start;
        let mut steps = 0usize;

        loop {
            let he = arena.get_half_edge(current)?;
            let twin_id = he.radial_next();
            let pair = if current.index() < twin_id.index() {
                (current.index(), twin_id.index())
            } else {
                (twin_id.index(), current.index())
            };

            if !edges.contains(&pair) {
                edges.insert(pair);
                result.push((current, twin_id));
            }

            current = he.next();
            steps += 1;
            if current == start || steps > 100_000 {
                break;
            }
        }
    }

    Ok(result)
}

/// Find vertices that are exclusively internal to the group (not on the perimeter).
fn find_internal_vertices(
    arena: &TopologyArena,
    group: &EntityBitset,
    perimeter: &[VertexId],
) -> Result<Vec<VertexId>, KernelError> {
    let perimeter_set: EntityBitset = {
        let mut bs = EntityBitset::for_vertices(arena);
        for &v in perimeter {
            let _ = bs.insert(v.index());
        }
        bs
    };
    let mut all_vertices = EntityBitset::for_vertices(arena);

    for face_idx in group.iter_ones() {
        let face_id = FaceId::from_raw_parts(face_idx, 0);
        let face_data = arena.get_face(face_id)?;
        let loop_data = arena.get_loop(face_data.outer_loop())?;
        let start = loop_data.half_edge();
        let mut current = start;
        let mut steps = 0usize;

        loop {
            let he = arena.get_half_edge(current)?;
            let _ = all_vertices.insert(he.origin().index());
            current = he.next();
            steps += 1;
            if current == start || steps > 100_000 {
                break;
            }
        }
    }

    all_vertices.difference_with(&perimeter_set);
    Ok(all_vertices.iter_ones().map(|idx| VertexId::from_raw_parts(idx, 0)).collect())
}

/// Log the extraction decision.
fn log_extraction(
    merged_count: usize,
    group_count: usize,
    delta: TopologyDelta,
    ctx: &mut ModelingContext,
) {
    let mut decision = TracedDecision::new(
        DecisionId(0),
        DecisionKind::PolicyApplied {
            policy: forge_core::PolicyKind::CoincidentGeometry,
            default_used: true,
        },
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!(
                "Polygon extraction: merged {} faces across {} coplanar groups",
                merged_count, group_count
            ),
        },
    );
    decision.set_topology_delta(delta);
    ctx.get_decision_log_mut().record(decision);
}
