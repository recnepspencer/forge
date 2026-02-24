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
use crate::topology::bitset::EntityBitset;
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
            let _ = bs.insert(v.index());
        }
        bs
    };
    let mut all_vertices = EntityBitset::for_vertices(arena);

    for face_idx in group.iter_ones() {
        let face_id = FaceId::from_raw_parts(face_idx, 0);
        for he_res in FaceAllEdgesIterator::new(arena, face_id)? {
            let current = he_res?;
            let he = arena.get_half_edge(current)?;
            let _ = all_vertices.insert(he.origin().index());
        }
    }

    all_vertices.difference_with(&perimeter_set);
    Ok(all_vertices.iter_ones().map(|idx| VertexId::from_raw_parts(idx, 0)).collect())
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
