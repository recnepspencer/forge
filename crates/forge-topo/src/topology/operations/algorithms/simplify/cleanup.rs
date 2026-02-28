//! Degenerate topology cleanup.
//!
//! DOMAIN: Removes zero-length logical edges and collapsed faces from a mutable
//! half-edge draft without relying on geometry-layer policy objects.

use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::handles::{FaceId, HalfEdgeId, VertexId};
use crate::transactions::MutableDraft;
use crate::topology::queries::traverse::FaceAllEdgesIterator;

/// Remove degenerate faces and zero-length edges from the draft.
///
/// Returns the total number of logical elements removed
/// (each logical edge = halfedge + twin counted as one).
pub fn cleanup_degenerate_topology(draft: &mut MutableDraft) -> Result<usize, KernelError> {
    let logical_edges_removed = remove_zero_length_edges(draft)?;
    let faces_removed = remove_degenerate_faces(draft)?;
    Ok(logical_edges_removed + faces_removed)
}

fn remove_zero_length_edges(draft: &mut MutableDraft) -> Result<usize, KernelError> {
    let zero_edges = find_zero_length_edges(draft)?;
    let mut removed = 0usize;
    let mut processed: BTreeSet<u32> = BTreeSet::new();

    for he_id in zero_edges {
        if processed.contains(&he_id.index()) {
            continue;
        }

        let he = draft.arena().get_half_edge(he_id)?.clone();
        let twin_unprocessed = !processed.contains(&he.radial_next().index());
        let non_degenerate_loop = he.prev() != he_id && he.next() != he_id;
        if !twin_unprocessed || !non_degenerate_loop {
            continue;
        }

        excise_halfedge(draft, he_id, &he)?;
        excise_twin_halfedge(draft, he.radial_next())?;

        draft.remove_half_edge(he_id)?;
        if draft.arena().get_half_edge(he.radial_next()).is_ok() {
            draft.remove_half_edge(he.radial_next())?;
        }

        processed.insert(he_id.index());
        processed.insert(he.radial_next().index());
        removed += 1;
    }

    Ok(removed)
}

fn find_zero_length_edges(draft: &MutableDraft) -> Result<Vec<HalfEdgeId>, KernelError> {
    let mut result = Vec::new();
    for (he_id, he) in draft.arena().iter_half_edges() {
        let dest = draft.arena().get_half_edge(he.next())?.origin();
        if he.origin() == dest {
            result.push(he_id);
        }
    }
    Ok(result)
}

fn excise_halfedge(
    draft: &mut MutableDraft,
    he_id: HalfEdgeId,
    he_data: &crate::b_rep::HalfEdgeData,
) -> Result<(), KernelError> {
    let next_id = he_data.next();
    let prev_id = he_data.prev();

    draft
        .arena_mut()
        .get_half_edge_mut(prev_id)?
        .set_next(next_id);
    draft
        .arena_mut()
        .get_half_edge_mut(next_id)?
        .set_prev(prev_id);

    repair_loop_pointer(draft, he_data.face(), he_id, next_id)?;
    repair_vertex_outgoing(draft, he_data.origin(), he_id, next_id)?;
    Ok(())
}

fn excise_twin_halfedge(draft: &mut MutableDraft, twin_id: HalfEdgeId) -> Result<(), KernelError> {
    let Ok(twin) = draft.arena().get_half_edge(twin_id) else {
        return Ok(());
    };
    let twin = twin.clone();
    if twin.prev() == twin_id || twin.next() == twin_id {
        return Ok(());
    }

    let twin_next = twin.next();
    let twin_prev = twin.prev();
    draft
        .arena_mut()
        .get_half_edge_mut(twin_prev)?
        .set_next(twin_next);
    draft
        .arena_mut()
        .get_half_edge_mut(twin_next)?
        .set_prev(twin_prev);
    repair_loop_pointer(draft, twin.face(), twin_id, twin_next)?;
    repair_vertex_outgoing(draft, twin.origin(), twin_id, twin_next)?;
    Ok(())
}

fn repair_loop_pointer(
    draft: &mut MutableDraft,
    face_id: FaceId,
    removed_he: HalfEdgeId,
    replacement_he: HalfEdgeId,
) -> Result<(), KernelError> {
    let face = draft.arena().get_face(face_id)?;
    let loop_id = face.outer_loop();
    let loop_he = draft.arena().get_loop(loop_id)?.half_edge();
    if loop_he == removed_he {
        draft
            .arena_mut()
            .get_loop_mut(loop_id)?
            .set_half_edge(replacement_he);
    }
    Ok(())
}

fn repair_vertex_outgoing(
    draft: &mut MutableDraft,
    vertex_id: VertexId,
    removed_he: HalfEdgeId,
    replacement_he: HalfEdgeId,
) -> Result<(), KernelError> {
    let outgoing = draft.arena().get_vertex(vertex_id)?.outgoing();
    if outgoing == removed_he {
        draft
            .arena_mut()
            .get_vertex_mut(vertex_id)?
            .set_outgoing(replacement_he);
    }
    Ok(())
}

fn remove_degenerate_faces(draft: &mut MutableDraft) -> Result<usize, KernelError> {
    let degenerate = find_degenerate_faces(draft)?;
    let mut removed = 0usize;
    let mut processed: BTreeSet<u32> = BTreeSet::new();

    for face_id in degenerate {
        if processed.contains(&face_id.index()) {
            continue;
        }

        let edges = collect_face_edges(draft, face_id)?;
        let deleted_set: BTreeSet<u32> = edges.iter().map(|he| he.index()).collect();
        repair_affected_vertices(draft, &edges, &deleted_set)?;
        remove_face_topology(draft, face_id, &edges)?;
        processed.insert(face_id.index());
        removed += 1;
    }

    Ok(removed)
}

fn find_degenerate_faces(draft: &MutableDraft) -> Result<Vec<FaceId>, KernelError> {
    let faces: Vec<FaceId> = draft.arena().iter_faces().map(|(fid, _)| fid).collect();
    let mut degenerate = Vec::new();

    for face_id in faces {
        let edges = collect_face_edges(draft, face_id)?;
        let unique: BTreeSet<u32> = edges
            .iter()
            .filter_map(|&he| {
                draft
                    .arena()
                    .get_half_edge(he)
                    .ok()
                    .map(|d| d.origin().index())
            })
            .collect();
        if unique.len() < 3 {
            degenerate.push(face_id);
        }
    }
    Ok(degenerate)
}

fn collect_face_edges(
    draft: &MutableDraft,
    face_id: FaceId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let iter = FaceAllEdgesIterator::new(draft.arena(), face_id)?;
    iter.collect::<Result<Vec<_>, _>>()
}

fn repair_affected_vertices(
    draft: &mut MutableDraft,
    edges: &[HalfEdgeId],
    deleted_set: &BTreeSet<u32>,
) -> Result<(), KernelError> {
    let mut needs_repair: BTreeSet<u32> = BTreeSet::new();
    for &he_id in edges {
        let he = draft.arena().get_half_edge(he_id)?;
        let origin = he.origin();
        let outgoing = draft.arena().get_vertex(origin).ok().map(|v| v.outgoing());
        if outgoing
            .map(|o| deleted_set.contains(&o.index()))
            .unwrap_or(false)
        {
            needs_repair.insert(origin.index());
        }
    }

    let replacements: Vec<(VertexId, HalfEdgeId)> = draft
        .arena()
        .iter_half_edges()
        .filter(|(he_id, _)| !deleted_set.contains(&he_id.index()))
        .filter(|(_, he)| needs_repair.contains(&he.origin().index()))
        .map(|(he_id, he)| (he.origin(), he_id))
        .collect();

    for (vid, he_id) in replacements {
        draft.arena_mut().get_vertex_mut(vid)?.set_outgoing(he_id);
    }
    Ok(())
}

fn remove_face_topology(
    draft: &mut MutableDraft,
    face_id: FaceId,
    edges: &[HalfEdgeId],
) -> Result<(), KernelError> {
    for &he_id in edges {
        draft.remove_half_edge(he_id)?;
    }
    let loop_id = draft.arena().get_face(face_id)?.outer_loop();
    draft.remove_face(face_id)?;
    draft.remove_loop(loop_id)?;
    Ok(())
}
