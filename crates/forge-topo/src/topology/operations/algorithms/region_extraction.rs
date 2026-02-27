//! Face-group region extraction helpers.
//!
//! DOMAIN: Read-only graph-walk utilities for contiguous face groups used by
//! higher-level region extraction and coplanar merge algorithms.
//!
//! INVARIANTS:
//! - Deterministic iteration order
//! - Corruption-safe traversal via query iterators
//! - No topology mutation

use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, VertexId};
use crate::operator::apply_op;
use crate::state::MutableDraft;
use crate::topology::bitset::EntityBitset;
use crate::topology::operations::euler::join_faces::JoinFaces;
use crate::topology::queries::traverse::FaceAllEdgesIterator;

/// Walk the boundary perimeter of a face group and collect perimeter vertices.
///
/// A half-edge is a group-boundary half-edge when its radial-adjacent face is
/// outside `group` (or the edge is globally boundary/self-radial).
pub fn walk_face_group_boundary_perimeter(
    arena: &TopologyArena,
    group: &EntityBitset,
) -> Result<Vec<VertexId>, KernelError> {
    let start_he = find_group_boundary_edge(arena, group)?;
    let mut perimeter = Vec::new();
    let mut current = start_he;
    let mut steps = 0usize;

    loop {
        let he_data = arena.get_half_edge(current)?;
        perimeter.push(he_data.origin());

        let next_candidate = he_data.next();
        current = advance_to_group_boundary(arena, group, next_candidate)?;

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

/// Collect unique half-edge radial pairs touched by faces in a group.
pub fn collect_face_group_edges(
    arena: &TopologyArena,
    group: &EntityBitset,
) -> Result<Vec<(HalfEdgeId, HalfEdgeId)>, KernelError> {
    let mut edges: BTreeSet<(u32, u32)> = BTreeSet::new();
    let mut result = Vec::new();

    for face_idx in group.iter_ones() {
        let face_id = FaceId::from_raw_parts(face_idx, 0);
        for he_res in FaceAllEdgesIterator::new(arena, face_id)? {
            let current = he_res?;
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
        }
    }

    Ok(result)
}

/// Find vertices used by the group but not on the provided perimeter.
pub fn find_face_group_internal_vertices(
    arena: &TopologyArena,
    group: &EntityBitset,
    perimeter: &[VertexId],
) -> Result<Vec<VertexId>, KernelError> {
    let perimeter_set = {
        let mut bs = EntityBitset::for_vertices(arena);
        for &v in perimeter {
            bs.insert(v.index())?;
        }
        bs
    };
    let mut all_vertices = EntityBitset::for_vertices(arena);

    for face_idx in group.iter_ones() {
        let face_id = FaceId::from_raw_parts(face_idx, 0);
        for he_res in FaceAllEdgesIterator::new(arena, face_id)? {
            let current = he_res?;
            let he = arena.get_half_edge(current)?;
            all_vertices.insert(he.origin().index())?;
        }
    }

    all_vertices.difference_with(&perimeter_set);
    Ok(all_vertices
        .iter_ones()
        .map(|idx| VertexId::from_raw_parts(idx, 0))
        .collect())
}

/// Merge a contiguous face group using iterative `JoinFaces` across internal edges.
///
/// This is the topology-preserving replacement for "nuke-and-pave" region rebuild.
/// The surviving face remains one of the original group faces.
pub fn merge_face_group_by_join_faces(
    draft: &mut MutableDraft,
    group: &EntityBitset,
) -> Result<FaceId, KernelError> {
    let mut active: BTreeSet<u32> = group.iter_ones().collect();
    if active.is_empty() {
        return Err(KernelError::InternalError {
            message: "merge_face_group_by_join_faces: empty face group".to_string(),
            context: None,
        });
    }

    while active.len() > 1 {
        let candidates = collect_internal_group_half_edges(draft.arena(), &active)?;
        let mut merged = false;

        for he in candidates {
            let he_data = match draft.arena().get_half_edge(he) {
                Ok(data) => data,
                Err(_) => continue,
            };
            let twin = he_data.radial_next();
            if twin == he {
                continue;
            }
            let twin_data = match draft.arena().get_half_edge(twin) {
                Ok(data) => data,
                Err(_) => continue,
            };
            let face_survive = he_data.face();
            let face_remove = twin_data.face();
            if face_survive == face_remove {
                continue;
            }
            if !active.contains(&face_survive.index()) || !active.contains(&face_remove.index()) {
                continue;
            }

            match apply_op(draft, JoinFaces { edge: he }) {
                Ok(exec) => {
                    let out = exec.into_value();
                    let removed = active.remove(&face_remove.index());
                    debug_assert!(
                        removed,
                        "merge_face_group_by_join_faces: removed face must be present in active set"
                    );
                    // `surviving_face` is often already active; duplicate insert is fine.
                    active.insert(out.surviving_face.index());
                    merged = true;
                    break;
                }
                Err(_) => {
                    continue;
                }
            }
        }

        if !merged {
            return Err(KernelError::InternalError {
                message: format!(
                    "merge_face_group_by_join_faces: unable to merge remaining {} faces",
                    active.len()
                ),
                context: None,
            });
        }
    }

    let surviving_idx = *active
        .iter()
        .next()
        .ok_or_else(|| KernelError::InternalError {
            message: "merge_face_group_by_join_faces: no surviving face".to_string(),
            context: None,
        })?;
    let surviving = draft
        .arena()
        .iter_faces()
        .find_map(|(fid, _)| (fid.index() == surviving_idx).then_some(fid))
        .ok_or_else(|| KernelError::InternalError {
            message: format!(
                "merge_face_group_by_join_faces: surviving face {} not found",
                surviving_idx
            ),
            context: None,
        })?;
    Ok(surviving)
}

fn collect_internal_group_half_edges(
    arena: &TopologyArena,
    active_faces: &BTreeSet<u32>,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let mut seen_edges: BTreeSet<u32> = BTreeSet::new();
    let mut candidates = Vec::new();

    for &face_idx in active_faces {
        let face_id = match arena
            .iter_faces()
            .find_map(|(fid, _)| (fid.index() == face_idx).then_some(fid))
        {
            Some(fid) => fid,
            None => continue,
        };

        for he_res in FaceAllEdgesIterator::new(arena, face_id)? {
            let he = he_res?;
            let he_data = arena.get_half_edge(he)?;
            let edge_id = he_data.edge().index();
            if seen_edges.contains(&edge_id) {
                continue;
            }
            let twin = he_data.radial_next();
            if twin == he {
                continue;
            }
            let twin_data = arena.get_half_edge(twin)?;
            if he_data.face() == twin_data.face() {
                continue;
            }
            if active_faces.contains(&he_data.face().index())
                && active_faces.contains(&twin_data.face().index())
            {
                let canonical = if he.index() <= twin.index() { he } else { twin };
                candidates.push(canonical);
                let was_new = seen_edges.insert(edge_id);
                debug_assert!(
                    was_new,
                    "collect_internal_group_half_edges: edge should not be inserted twice after contains check"
                );
            }
        }
    }

    candidates.sort_by_key(|he| he.index());
    candidates.dedup_by_key(|he| he.index());
    Ok(candidates)
}

fn find_group_boundary_edge(
    arena: &TopologyArena,
    group: &EntityBitset,
) -> Result<HalfEdgeId, KernelError> {
    for face_idx in group.iter_ones() {
        let face_id = FaceId::from_raw_parts(face_idx, 0);
        for he_res in FaceAllEdgesIterator::new(arena, face_id)? {
            let he_id = he_res?;
            if is_face_group_boundary_half_edge(arena, group, he_id)? {
                return Ok(he_id);
            }
        }
    }

    Err(KernelError::InternalError {
        message: "No boundary edge found in face group".to_string(),
        context: None,
    })
}

/// Check whether a half-edge lies on the boundary of a face group.
///
/// Returns `true` when the radial-adjacent half-edge is outside the group, or
/// when the edge is self-radial (global sheet boundary).
pub fn is_face_group_boundary_half_edge(
    arena: &TopologyArena,
    group: &EntityBitset,
    he: HalfEdgeId,
) -> Result<bool, KernelError> {
    let he_data = arena.get_half_edge(he)?;
    let twin_id = he_data.radial_next();

    if twin_id == he {
        return Ok(true);
    }

    let twin_data = arena.get_half_edge(twin_id)?;
    Ok(!group.contains(twin_data.face().index()).unwrap_or(false))
}

fn advance_to_group_boundary(
    arena: &TopologyArena,
    group: &EntityBitset,
    start: HalfEdgeId,
) -> Result<HalfEdgeId, KernelError> {
    let mut current = start;
    let mut steps = 0usize;

    while !is_face_group_boundary_half_edge(arena, group, current)? {
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

#[cfg(test)]
mod tests {
    use super::merge_face_group_by_join_faces;
    use crate::bitset::EntityBitset;
    use crate::euler::make_edge_face::MakeEdgeFace;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;
    use crate::operator::apply_op;
    use crate::state::TopologyState;

    #[test]
    fn merge_face_group_by_join_faces_merges_two_faces() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let mef = apply_op(
            &mut draft,
            MakeEdgeFace {
                vertex_a: mvf.vertex,
                vertex_b: se.new_vertex,
                face: mvf.face,
            },
        )
        .unwrap()
        .into_value();

        let mut group = EntityBitset::for_faces(draft.arena());
        group
            .insert(mvf.face.index())
            .expect("bitset capacity must cover fixture faces");
        group
            .insert(mef.new_face.index())
            .expect("bitset capacity must cover fixture faces");

        let surviving = merge_face_group_by_join_faces(&mut draft, &group).unwrap();

        assert_eq!(draft.arena().face_count(), 1);
        assert!(draft.arena().get_face(surviving).is_ok());
    }
}
