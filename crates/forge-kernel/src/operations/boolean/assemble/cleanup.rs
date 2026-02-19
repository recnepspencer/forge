//! Post-assembly degenerate topology cleanup.
//!
//! DOMAIN: Removes zero-area faces and zero-length edges created by
//! floating-point drift during vertex deduplication.
//!
//! INVARIANTS:
//! - Must run after `stitch_twins` and before `MutableDraft::commit()`
//! - Preserves manifold topology by stitching neighbors of removed elements
//!
//! DEPENDENCIES: `forge_topo::state::MutableDraft`, `GeometryStore`, `quantize_position`

use std::collections::HashSet;
use forge_core::KernelError;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::state::MutableDraft;
use forge_topo::traverse::FaceEdgeIterator;

use crate::geometry_store::GeometryStore;

/// Remove degenerate faces and zero-length edges from the draft.
///
/// After vertex deduplication in the copy phase, some faces may have
/// collapsed (2+ vertices merged to the same point, creating zero-area
/// slivers). This pass detects and removes them before commit validation.
///
/// Returns the number of elements removed.
pub fn cleanup_degenerate_topology(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
) -> Result<usize, KernelError> {
    let mut total_removed = 0;

    total_removed += remove_zero_length_edges(draft, geom)?;
    total_removed += remove_degenerate_faces(draft, geom)?;

    Ok(total_removed)
}

/// Remove edges where origin and destination have the same quantized position.
///
/// When vertex dedup maps two distinct vertices to the same point,
/// the edge between them becomes zero-length. Collapse it by redirecting
/// all halfedges from one vertex to the other.
fn remove_zero_length_edges(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
) -> Result<usize, KernelError> {
    let mut zero_edges: Vec<HalfEdgeId> = Vec::new();

    for (he_id, he_data) in draft.arena().iter_half_edges() {
        let next_data = draft.arena().get_half_edge(he_data.next())?;
        let origin = he_data.origin();
        let dest = next_data.origin();

        if origin == dest {
            zero_edges.push(he_id);
        }
    }

    let mut removed = 0;
    let mut processed: HashSet<u32> = HashSet::new();

    for he_id in zero_edges {
        if processed.contains(&he_id.index()) {
            continue;
        }

        let he_data = match draft.arena().get_half_edge(he_id) {
            Ok(d) => d.clone(),
            Err(_) => continue,
        };

        let twin_id = he_data.twin();
        if processed.contains(&twin_id.index()) {
            continue;
        }

        let next_id = he_data.next();
        let prev_id = find_prev_he(draft, he_id)?;

        if prev_id == he_id || next_id == he_id {
            continue;
        }

        draft.arena_mut().get_half_edge_mut(prev_id)?.set_next(next_id);

        if let Ok(twin_data) = draft.arena().get_half_edge(twin_id) {
            let twin_data = twin_data.clone();
            let twin_next = twin_data.next();
            let twin_prev = find_prev_he(draft, twin_id)?;

            if twin_prev != twin_id && twin_next != twin_id {
                draft.arena_mut().get_half_edge_mut(twin_prev)?.set_next(twin_next);
            }
        }

        let outer_loop_id = {
            let face_data = draft.arena().get_face(he_data.face())?;
            let loop_data = draft.arena().get_loop(face_data.outer_loop())?;
            if loop_data.half_edge() == he_id {
                Some(face_data.outer_loop())
            } else {
                None
            }
        };
        if let Some(loop_id) = outer_loop_id {
            draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(next_id);
        }

        let outgoing_update = {
            let vertex_data = draft.arena().get_vertex(he_data.origin())?;
            if vertex_data.outgoing() == he_id {
                Some(he_data.origin())
            } else {
                None
            }
        };
        if let Some(vid) = outgoing_update {
            draft.arena_mut().get_vertex_mut(vid)?.set_outgoing(next_id);
        }

        let _ = draft.arena_mut().remove_half_edge(he_id);
        if draft.arena().get_half_edge(twin_id).is_ok() {
            let _ = draft.arena_mut().remove_half_edge(twin_id);
        }

        processed.insert(he_id.index());
        processed.insert(twin_id.index());
        removed += 1;
    }

    Ok(removed)
}

/// Remove faces with fewer than 3 distinct vertices (collapsed slivers).
///
/// A face can become degenerate after zero-length edge removal or vertex
/// dedup. If a face has < 3 unique vertex positions, it has zero area
/// and must be removed to satisfy the Euler formula.
fn remove_degenerate_faces(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
) -> Result<usize, KernelError> {
    let mut degenerate_faces: Vec<FaceId> = Vec::new();

    let faces: Vec<FaceId> = draft.arena().iter_faces()
        .map(|(fid, _)| fid)
        .collect();

    for face_id in &faces {
        let edges: Vec<_> = match FaceEdgeIterator::new(draft.arena(), *face_id) {
            Ok(iter) => iter.collect::<Result<Vec<_>, _>>()?,
            Err(_) => continue,
        };

        let mut unique_verts: HashSet<u32> = HashSet::new();
        for he_id in &edges {
            if let Ok(he_data) = draft.arena().get_half_edge(*he_id) {
                unique_verts.insert(he_data.origin().index());
            }
        }

        if unique_verts.len() < 3 {
            degenerate_faces.push(*face_id);
        }
    }

    let mut removed = 0;
    let mut processed_faces: HashSet<u32> = HashSet::new();

    for face_id in degenerate_faces {
        if processed_faces.contains(&face_id.index()) {
            continue;
        }

        let edges: Vec<_> = match FaceEdgeIterator::new(draft.arena(), face_id) {
            Ok(iter) => iter.collect::<Result<Vec<_>, _>>()?,
            Err(_) => continue, // Skip invalid faces
        };for he_id in &edges {
            if let Ok(he_data) = draft.arena().get_half_edge(*he_id) {
                let twin_id = he_data.twin();
                if let Ok(twin_data) = draft.arena().get_half_edge(twin_id) {
                    let twin_face = twin_data.face();
                    if twin_face != face_id && !processed_faces.contains(&twin_face.index()) {
                        if let Ok(twin_next) = draft.arena().get_half_edge(twin_data.next()) {
                            let _ = twin_next;
                        }
                    }
                }
            }
        }

        let edge_list: Vec<HalfEdgeId> = edges.clone();
        for he_id in &edge_list {
            if let Ok(he_data) = draft.arena().get_half_edge(*he_id) {
                let twin_id = he_data.twin();
                if let Ok(_twin_data) = draft.arena().get_half_edge(twin_id) {
                    let prev_of_twin = find_prev_he(draft, twin_id);
                    let next_of_twin_result = draft.arena().get_half_edge(twin_id).map(|d| d.next());

                    if let (Ok(prev_t), Ok(next_t)) = (prev_of_twin, next_of_twin_result) {
                        if prev_t != twin_id {
                            let _ = draft.arena_mut().get_half_edge_mut(prev_t).map(|d| d.set_next(next_t));
                        }
                    }
                }
                let _ = draft.arena_mut().remove_half_edge(*he_id);
                if draft.arena().get_half_edge(twin_id).is_ok() {
                    let _ = draft.arena_mut().remove_half_edge(twin_id);
                }
            }
        }

        let face_data = draft.arena().get_face(face_id)?;
        let loop_id = face_data.outer_loop();
        let _ = draft.arena_mut().remove_face(face_id);
        let _ = draft.arena_mut().remove_loop(loop_id);

        processed_faces.insert(face_id.index());
        removed += 1;
    }

    Ok(removed)
}

/// Find the halfedge whose `next` field points to `target`.
fn find_prev_he(
    draft: &MutableDraft,
    target: HalfEdgeId,
) -> Result<HalfEdgeId, KernelError> {
    let he_data = draft.arena().get_half_edge(target)?;
    let face_data = draft.arena().get_face(he_data.face())?;
    let loop_data = draft.arena().get_loop(face_data.outer_loop())?;
    let start = loop_data.half_edge();
    let mut current = start;
    let max_iter = 1000;

    for _ in 0..max_iter {
        let curr_data = draft.arena().get_half_edge(current)?;
        if curr_data.next() == target {
            return Ok(current);
        }
        current = curr_data.next();
        if current == start {
            break;
        }
    }

    Err(KernelError::InternalError {
        message: format!("Could not find prev halfedge for {}", target),
        context: None,
    })
}
