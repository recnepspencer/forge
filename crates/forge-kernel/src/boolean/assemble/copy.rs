//! Face copying and vertex deduplication logic.

use std::collections::HashMap;
use forge_core::KernelError;
use forge_topo::arena::{TopologyArena, FaceData, LoopData, HalfEdgeData, VertexData};
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId, LoopId};
use forge_topo::state::MutableDraft;
use forge_topo::traverse::face_edges;

use crate::geometry_store::GeometryStore;
use crate::boolean::eval::quantize_position;

/// Maps quantized vertex position → new VertexId in the result arena.
///
/// Provides cross-arena vertex deduplication: when the same 3D point
/// exists in both the target and tool arenas (at split boundaries),
/// both map to the same new VertexId.
pub struct VertexDedup {
    /// Quantized position → new VertexId.
    by_position: HashMap<[i64; 3], VertexId>,
}

impl VertexDedup {
    /// Create an empty dedup table.
    pub fn new() -> Self {
        Self {
            by_position: HashMap::new(),
        }
    }

    /// Get or insert a vertex at the given position.
    ///
    /// Returns the new VertexId for this position. If a vertex was
    /// already inserted at this position (from the other arena),
    /// returns the existing ID.
    pub fn get_or_insert(
        &mut self,
        pos: &[f64; 3],
        draft: &mut MutableDraft,
        result_geom: &mut GeometryStore,
    ) -> VertexId {
        let qpos = quantize_position(pos);

        if let Some(&existing) = self.by_position.get(&qpos) {
            // eprintln!("VertexDedup: Reuse {:?} -> {}", pos, existing);
            return existing;
        }

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let new_vid = draft.arena_mut().insert_vertex(VertexData {
            outgoing: placeholder_he,
            lineage: None,
        });
        self.by_position.insert(qpos, new_vid);
        result_geom.set_vertex_position(new_vid, *pos);
        // eprintln!("VertexDedup: Insert {:?} -> {}", pos, new_vid);
        new_vid
    }
}

/// Copy faces from a source arena into the draft.
///
/// For each selected face:
/// 1. Insert vertices with position-based dedup (shared across arenas)
/// 2. Insert halfedges with placeholder twins
/// 3. Wire prev/next pointers
/// 4. Record all new halfedge IDs for twin stitching
pub fn copy_faces(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    vertex_dedup: &mut VertexDedup,
    all_new_he_ids: &mut Vec<HalfEdgeId>,
    source_arena: &TopologyArena,
    source_geom: &GeometryStore,
    selected_faces: &[FaceId],
    reverse: bool,
) -> Result<(), KernelError> {
    let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
    let placeholder_loop = LoopId::new(u32::MAX, 0);

    for &face_id in selected_faces {
        let face_data = source_arena.get_face(face_id)?;

        let mut loop_halfedges = face_edges(source_arena, face_id)?;
        if reverse {
            loop_halfedges.reverse();
        }

        let new_face = draft.arena_mut().insert_face(FaceData {
            outer_loop: placeholder_loop,
            lineage: face_data.lineage.clone(),
        });

        if let Some(plane) = source_geom.get_face_plane(face_id) {
            let mut final_plane = plane.clone();
            if reverse {
                 // Flip the plane normal and offset
                 // Plane equation: n.p + d = 0
                 // Flipped: (-n).p + (-d) = 0
                 let n = final_plane.raw_normal();
                 let d = final_plane.raw_offset();
                 // We re-construct to ensure normalization logic holds
                 if let Ok(p) = forge_geom::plane::Plane::try_new([-n[0], -n[1], -n[2]], -d) {
                     final_plane = p;
                 } else {
                     eprintln!("Failed to flip plane for reversed face {}", face_id);
                 }
            }
            result_geom.set_face_plane(new_face, final_plane);
        }

        let new_loop = draft.arena_mut().insert_loop(LoopData {
            half_edge: placeholder_he,
            face: new_face,
        });

        let mut new_he_ids = Vec::with_capacity(loop_halfedges.len());
        for &old_he_id in &loop_halfedges {
            let he_data = source_arena.get_half_edge(old_he_id)?;
            let pos = source_geom.get_vertex_position(he_data.origin).ok_or_else(|| {
                KernelError::InvalidInput {
                    message: format!("No position for vertex {}", he_data.origin),
                    context: None,
                }
            })?;

            let new_origin = vertex_dedup.get_or_insert(pos, draft, result_geom);

            let new_he = draft.arena_mut().insert_half_edge(HalfEdgeData {
                twin: placeholder_he,
                next: placeholder_he,
                prev: placeholder_he,
                face: new_face,
                origin: new_origin,
                lineage: he_data.lineage.clone(),
            });
            new_he_ids.push(new_he);
            all_new_he_ids.push(new_he);
        }

        let he_count = new_he_ids.len();
        for i in 0..he_count {
            let next_i = (i + 1) % he_count;
            let prev_i = if i == 0 { he_count - 1 } else { i - 1 };

            let arena = draft.arena_mut();
            arena.get_half_edge_mut(new_he_ids[i])?.next = new_he_ids[next_i];
            arena.get_half_edge_mut(new_he_ids[i])?.prev = new_he_ids[prev_i];
        }

        if !new_he_ids.is_empty() {
            draft.arena_mut().get_face_mut(new_face)?.outer_loop = new_loop;
            draft.arena_mut().get_loop_mut(new_loop)?.half_edge = new_he_ids[0];
        }

        for &new_he in &new_he_ids {
            let origin = draft.arena().get_half_edge(new_he)?.origin;
            draft.arena_mut().get_vertex_mut(origin)?.outgoing = new_he;
        }
    }

    Ok(())
}
