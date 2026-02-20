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

use std::collections::BTreeSet;
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
    _geom: &GeometryStore,
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
    let mut processed: BTreeSet<u32> = BTreeSet::new();

    for he_id in zero_edges {
        if !processed.contains(&he_id.index()) {
            if let Ok(he_data) = draft.arena().get_half_edge(he_id) {
                let he_data = he_data.clone();
                let twin_id = he_data.twin();
                if !processed.contains(&twin_id.index()) {
                    let next_id = he_data.next();
                    let prev_id = he_data.prev();

                    if prev_id != he_id && next_id != he_id {
                        draft.arena_mut().get_half_edge_mut(prev_id)?.set_next(next_id);
                        draft.arena_mut().get_half_edge_mut(next_id)?.set_prev(prev_id);

                        if let Ok(twin_data) = draft.arena().get_half_edge(twin_id) {
                            let twin_data = twin_data.clone();
                            let twin_next = twin_data.next();
                            let twin_prev = twin_data.prev();

                            if twin_prev != twin_id && twin_next != twin_id {
                                draft.arena_mut().get_half_edge_mut(twin_prev)?.set_next(twin_next);
                                draft.arena_mut().get_half_edge_mut(twin_next)?.set_prev(twin_prev);
                            }
                        }

                        let outer_loop_id_primary = {
                            let face_data = draft.arena().get_face(he_data.face())?;
                            let loop_data = draft.arena().get_loop(face_data.outer_loop())?;
                            if loop_data.half_edge() == he_id {
                                Some(face_data.outer_loop())
                            } else {
                                None
                            }
                        };
                        if let Some(loop_id) = outer_loop_id_primary {
                            draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(next_id);
                        }

                        let outgoing_update_primary = {
                            let vertex_data = draft.arena().get_vertex(he_data.origin())?;
                            if vertex_data.outgoing() == he_id {
                                Some(he_data.origin())
                            } else {
                                None
                            }
                        };
                        if let Some(vid) = outgoing_update_primary {
                            draft.arena_mut().get_vertex_mut(vid)?.set_outgoing(next_id);
                        }

                        if let Ok(twin_data) = draft.arena().get_half_edge(twin_id) {
                            let twin_data = twin_data.clone();
                            let twin_next = twin_data.next();
                            
                            let outer_loop_id_twin = {
                                let face_data = draft.arena().get_face(twin_data.face())?;
                                let loop_data = draft.arena().get_loop(face_data.outer_loop())?;
                                if loop_data.half_edge() == twin_id {
                                    Some(face_data.outer_loop())
                                } else {
                                    None
                                }
                            };
                            if let Some(loop_id) = outer_loop_id_twin {
                                draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(twin_next);
                            }

                            let outgoing_update_twin = {
                                let vertex_data = draft.arena().get_vertex(twin_data.origin())?;
                                if vertex_data.outgoing() == twin_id {
                                    Some(twin_data.origin())
                                } else {
                                    None
                                }
                            };
                            if let Some(vid) = outgoing_update_twin {
                                draft.arena_mut().get_vertex_mut(vid)?.set_outgoing(twin_next);
                            }
                        }

                        let _ = draft.arena_mut().remove_half_edge(he_id);
                        if draft.arena().get_half_edge(twin_id).is_ok() {
                            let _ = draft.arena_mut().remove_half_edge(twin_id);
                        }

                        processed.insert(he_id.index());
                        processed.insert(twin_id.index());
                        removed += 1;
                    }
                }
            }
        }
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
    _geom: &GeometryStore,
) -> Result<usize, KernelError> {
    let mut degenerate_faces: Vec<FaceId> = Vec::new();

    let faces: Vec<FaceId> = draft.arena().iter_faces()
        .map(|(fid, _)| fid)
        .collect();

    for face_id in &faces {
        if let Ok(iter) = FaceEdgeIterator::new(draft.arena(), *face_id) {
            let edges: Vec<_> = iter.collect::<Result<Vec<_>, _>>()?;

            let mut unique_verts: BTreeSet<u32> = BTreeSet::new();
            for he_id in &edges {
                if let Ok(he_data) = draft.arena().get_half_edge(*he_id) {
                    unique_verts.insert(he_data.origin().index());
                }
            }

            if unique_verts.len() < 3 {
                degenerate_faces.push(*face_id);
            }
        }
    }

    let mut removed = 0;
    let mut processed_faces: BTreeSet<u32> = BTreeSet::new();

    for face_id in degenerate_faces {
        if !processed_faces.contains(&face_id.index()) {
            if let Ok(iter) = FaceEdgeIterator::new(draft.arena(), face_id) {
                let edges: Vec<_> = iter.collect::<Result<Vec<_>, _>>()?;

                let mut deleted_he_set: BTreeSet<u32> = BTreeSet::new();
                for he_id in &edges {
                    deleted_he_set.insert(he_id.index());
                }

                let mut affected_vertices: BTreeSet<u32> = BTreeSet::new();
                for he_id in &edges {
                    if let Ok(he_data) = draft.arena().get_half_edge(*he_id) {
                        let origin = he_data.origin();
                        let outgoing = draft.arena().get_vertex(origin)
                            .map(|v| v.outgoing())
                            .ok();
                        if outgoing.map(|o| deleted_he_set.contains(&o.index())).unwrap_or(false) {
                            affected_vertices.insert(origin.index());
                        }
                    }
                }

                let replacements: Vec<(VertexId, HalfEdgeId)> = draft.arena().iter_half_edges()
                    .filter(|(he_id, _)| !deleted_he_set.contains(&he_id.index()))
                    .filter(|(_, he_data)| affected_vertices.contains(&he_data.origin().index()))
                    .map(|(he_id, he_data)| (he_data.origin(), he_id))
                    .collect();

                for (vid, he_id) in &replacements {
                    affected_vertices.remove(&vid.index());
                    let _ = draft.arena_mut().get_vertex_mut(*vid).map(|v| v.set_outgoing(*he_id));
                }

                for he_id in &edges {
                    let _ = draft.arena_mut().remove_half_edge(*he_id);
                }

                let face_data = draft.arena().get_face(face_id)?;
                let loop_id = face_data.outer_loop();
                let _ = draft.arena_mut().remove_face(face_id);
                let _ = draft.arena_mut().remove_loop(loop_id);

                processed_faces.insert(face_id.index());
                removed += 1;
            }
        }
    }

    Ok(removed)
}

