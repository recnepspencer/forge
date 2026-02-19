//! Copy faces from one arena to another.
//!
//! Handles the transfer of topology and geometry, remapping handles,
//! and deduplicating vertices based on `VertexMatchKey` (3-plane signature)
//! with a spatial nearest-neighbor fallback for vertices lacking provenance.
//!
//! When a cross-solid vertex collision is found (same VertexMatchKey or
//! same geometric position within tolerance), lineages are merged using
//! `Lineage::merge` to preserve traceability (D1).
//!
//! Uses direct arena insertion (same pattern as `mesh_builder/eval.rs`)
//! rather than Euler operators, giving full control over halfedge wiring.

use std::collections::HashMap;

use forge_core::KernelError;
use forge_topo::arena::{FaceData, HalfEdgeData, LoopData};
use forge_topo::handles::{FaceId, VertexId, HalfEdgeId, LoopId};
use forge_topo::lineage::{Lineage, OpSignature};
use forge_topo::state::MutableDraft;
use forge_topo::traverse::FaceEdgeIterator;

use crate::geometry_store::GeometryStore;
use crate::operations::boolean::eval::VertexMatchKey;

/// Helper to map vertices from old arena to new arena (local to one solid).
pub struct VertexDedup {
    mapping: HashMap<VertexId, VertexId>,
}

impl VertexDedup {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, old: VertexId, new: VertexId) {
        self.mapping.insert(old, new);
    }
    
    pub fn get(&self, old: VertexId) -> Option<VertexId> {
        self.mapping.get(&old).copied()
    }
}

/// Vertex position index with fuzzy nearest-neighbor search.
///
/// Replaces the old `PositionKey` grid which suffered from boundary
/// straddling. Uses squared Euclidean distance with a fixed tolerance,
/// guaranteeing that geometrically coincident vertices always unify
/// regardless of floating-point noise from the split computation.
const SPATIAL_GRID_CELL_SIZE: f64 = 1e-5;

pub struct SpatialVertexIndex {
    grid: std::collections::HashMap<[i64; 3], Vec<(VertexId, [f64; 3])>>,
}

/// Squared distance tolerance for vertex matching (1e-6 linear ≈ 1e-12 squared).
///
/// This must be loose enough to unify vertices that went through
/// floating-point arithmetic in prior boolean operations (where
/// sub-nanometer positional noise accumulates), but tight enough
/// to never merge geometrically distinct vertices.
const VERTEX_WELD_TOLERANCE_SQ: f64 = 1e-12;

impl SpatialVertexIndex {
    pub fn new() -> Self {
        Self { grid: std::collections::HashMap::new() }
    }

    fn cell_coords(pos: &[f64; 3]) -> [i64; 3] {
        [
            (pos[0] / SPATIAL_GRID_CELL_SIZE).floor() as i64,
            (pos[1] / SPATIAL_GRID_CELL_SIZE).floor() as i64,
            (pos[2] / SPATIAL_GRID_CELL_SIZE).floor() as i64,
        ]
    }

    /// Find the nearest vertex within tolerance. Returns `None` if
    /// no existing vertex is close enough.
    pub fn find_nearest(&self, pos: &[f64; 3]) -> Option<VertexId> {
        let center_cell = Self::cell_coords(pos);
        let mut best: Option<(VertexId, f64)> = None;

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let cell = [center_cell[0] + dx, center_cell[1] + dy, center_cell[2] + dz];
                    if let Some(entries) = self.grid.get(&cell) {
                        for &(vid, ref p) in entries {
                            let diff_x = pos[0] - p[0];
                            let diff_y = pos[1] - p[1];
                            let diff_z = pos[2] - p[2];
                            let dist_sq = diff_x * diff_x + diff_y * diff_y + diff_z * diff_z;
                            if dist_sq <= VERTEX_WELD_TOLERANCE_SQ {
                                match best {
                                    None => best = Some((vid, dist_sq)),
                                    Some((_, best_d)) if dist_sq < best_d => best = Some((vid, dist_sq)),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        best.map(|(vid, _)| vid)
    }

    /// Register a vertex position for future lookups.
    pub fn insert(&mut self, vid: VertexId, pos: [f64; 3]) {
        let cell = Self::cell_coords(&pos);
        self.grid.entry(cell).or_insert_with(Vec::new).push((vid, pos));
    }
}

/// Copy a set of faces from a source arena to a destination draft.
///
/// - `global_vertex_map`: Map of VertexMatchKey → New VertexId (shared across entire boolean op).
/// - `spatial_index`: Position-based fallback for vertices without provenance.
/// - `src_prov`: Optional provenance map (Source VertexId → VertexMatchKey) for cross-solid gluing.
pub fn copy_faces(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    vertex_dedup: &mut VertexDedup,
    new_edges: &mut Vec<HalfEdgeId>,
    global_vertex_map: &mut HashMap<VertexMatchKey, VertexId>,
    spatial_index: &mut SpatialVertexIndex,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    source_faces: &[FaceId],
    reverse_orientation: bool,
    src_prov: Option<&HashMap<VertexId, VertexMatchKey>>,
) -> Result<(), KernelError> {
    
    for &src_face in source_faces {
        copy_single_face(
            draft, result_geom, vertex_dedup, new_edges, global_vertex_map,
            spatial_index,
            source_arena, source_geom, src_face, reverse_orientation, src_prov,
        )?;
    }
    
    Ok(())
}

/// Copy a single face via direct arena insertion.
///
/// Algorithm (mirrors `mesh_builder::insert_faces_and_loops`):
/// 1. Collect source vertices in order (reversed if needed)
/// 2. Resolve each vertex via local dedup → global match key → spatial NNS → create new
/// 3. Insert FaceData, LoopData, and one HalfEdgeData per vertex
/// 4. Wire next/prev pointers in a closed loop
/// 5. Record all created HalfEdgeIds for later twin stitching
fn copy_single_face(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    vertex_dedup: &mut VertexDedup,
    new_edges: &mut Vec<HalfEdgeId>,
    global_vertex_map: &mut HashMap<VertexMatchKey, VertexId>,
    spatial_index: &mut SpatialVertexIndex,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    src_face: FaceId,
    reverse_orientation: bool,
    src_prov: Option<&HashMap<VertexId, VertexMatchKey>>,
) -> Result<FaceId, KernelError> {
    
    let src_plane = source_geom.get_face_plane(src_face).ok_or(KernelError::InvalidInput {
        message: format!("Face {} missing plane", src_face),
        context: None,
    })?;
    
    let mut new_plane = src_plane.clone();
    if reverse_orientation {
        new_plane.flip();
    }
    
    // 2. Copy all half-edges for the face
    let edges: Vec<_> = FaceEdgeIterator::new(source_arena, src_face)?
        .collect::<Result<Vec<_>, _>>()?;
    if edges.is_empty() {
        let _placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
        let placeholder_loop = LoopId::from_raw_parts(u32::MAX, 0);
        let face_id = draft.arena_mut().insert_face(FaceData::new(
            placeholder_loop,
        ));
        result_geom.set_face_plane(face_id, new_plane);
        return Ok(face_id);
    }
    
    let iter_order: Vec<HalfEdgeId> = if reverse_orientation {
        edges.iter().rev().copied().collect()
    } else {
        edges.to_vec()
    };
    
    let mut src_verts = Vec::with_capacity(iter_order.len());
    for he in &iter_order {
        let he_data = source_arena.get_half_edge(*he)?;
        src_verts.push(he_data.origin());
    }
    
    let num_verts = src_verts.len();
    
    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let placeholder_loop = LoopId::from_raw_parts(u32::MAX, 0);
    
    let face_id = draft.arena_mut().insert_face(FaceData::new(
        placeholder_loop,
    ));
    result_geom.set_face_plane(face_id, new_plane);
    
    let loop_id = draft.arena_mut().insert_loop(LoopData::new(
        placeholder_he,
        face_id,
    ));
    
    let mut resolved_verts = Vec::with_capacity(num_verts);
    for &sv in &src_verts {
        let new_v_id = resolve_vertex(
            draft, result_geom, vertex_dedup, global_vertex_map,
            spatial_index,
            source_arena, source_geom, sv, src_prov,
        )?;
        resolved_verts.push(new_v_id);
    }
    
    let mut he_ids = Vec::with_capacity(num_verts);
    for &origin in &resolved_verts {
        let he_id = draft.arena_mut().insert_half_edge(HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face_id,
            origin,
        ));
        he_ids.push(he_id);
    }
    
    for i in 0..num_verts {
        let next_i = (i + 1) % num_verts;
        let prev_i = if i == 0 { num_verts - 1 } else { i - 1 };
        
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(he_ids[i])?.set_next(he_ids[next_i]);
        arena.get_half_edge_mut(he_ids[i])?.set_prev(he_ids[prev_i]);
    }
    
    draft.arena_mut().get_face_mut(face_id)?.set_outer_loop(loop_id);
    draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(he_ids[0]);
    
    for &he_id in &he_ids {
        let origin = draft.arena().get_half_edge(he_id)?.origin();
        draft.arena_mut().get_vertex_mut(origin)?.set_outgoing(he_id);
    }
    
    new_edges.extend_from_slice(&he_ids);
    
    Ok(face_id)
}

/// Resolve a source vertex to a destination vertex, using dedup layers:
/// 1. Local dedup (same solid, already copied)
/// 2. Global VertexMatchKey map (cross-solid vertex gluing via 3-plane provenance)
/// 3. Spatial nearest-neighbor search (fuzzy position fallback)
/// 4. Create new vertex
///
/// When a cross-solid match is found (step 2 or 3), the existing vertex's
/// lineage is merged with the source vertex's lineage using `Lineage::merge` (D1).
fn resolve_vertex(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    vertex_dedup: &mut VertexDedup,
    global_vertex_map: &mut HashMap<VertexMatchKey, VertexId>,
    spatial_index: &mut SpatialVertexIndex,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    src_vertex: VertexId,
    src_prov: Option<&HashMap<VertexId, VertexMatchKey>>,
) -> Result<VertexId, KernelError> {
    let pos = source_geom.get_vertex_position(src_vertex).ok_or(
        KernelError::InvalidInput { message: "Missing vertex position".into(), context: None }
    )?;
    
    if let Some(mapped) = vertex_dedup.get(src_vertex) {
        return Ok(mapped);
    }
    
    let match_key = src_prov.and_then(|prov| prov.get(&src_vertex).cloned());
    
    if let Some(ref key) = match_key {
        if let Some(global_id) = global_vertex_map.get(key) {
            let src_lineage = source_arena.get_vertex(src_vertex)
                .ok()
                .and_then(|v| v.lineage().cloned());
            let existing_lineage = draft.arena().get_vertex(*global_id)
                .ok()
                .and_then(|v| v.lineage().cloned());
            
            let merge_sig = OpSignature::new("boolean_vertex_merge");
            let merged = Lineage::merge(&existing_lineage, &src_lineage, &merge_sig);
            let v_mut = draft.arena_mut().get_vertex_mut(*global_id)?;
            v_mut.set_lineage(Some(merged));
            if v_mut.provenance().is_none() {
                v_mut.set_provenance(Some(*key.planes()));
            }
            
            vertex_dedup.insert(src_vertex, *global_id);
            return Ok(*global_id);
        }
    }
    
    // Fallback: fuzzy nearest-neighbor search for vertices without provenance
    if let Some(existing_id) = spatial_index.find_nearest(pos) {
        if draft.arena().get_vertex(existing_id).is_ok() {
            let src_lineage = source_arena.get_vertex(src_vertex)
                .ok()
                .and_then(|v| v.lineage().cloned());
            let existing_lineage = draft.arena().get_vertex(existing_id)
                .ok()
                .and_then(|v| v.lineage().cloned());
            let merge_sig = OpSignature::new("boolean_vertex_merge_spatial");
            let merged = Lineage::merge(&existing_lineage, &src_lineage, &merge_sig);
            draft.arena_mut().get_vertex_mut(existing_id)?.set_lineage(Some(merged));
            
            vertex_dedup.insert(src_vertex, existing_id);
            return Ok(existing_id);
        }
    }
    
    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let src_lineage = source_arena.get_vertex(src_vertex)
        .ok()
        .and_then(|v| v.lineage().cloned());
    let vid = draft.arena_mut().insert_vertex(forge_topo::arena::VertexData::with_lineage(
        placeholder_he,
        src_lineage,
    ));
    result_geom.set_vertex_position(vid, *pos);
    
    // Carry provenance: prefer the match key from the split phase,
    // fall back to the source vertex's stored provenance (from prior booleans).
    if let Some(ref key) = match_key {
        draft.arena_mut().get_vertex_mut(vid)?.set_provenance(Some(*key.planes()));
    } else if let Ok(src_v) = source_arena.get_vertex(src_vertex) {
        if let Some(stored) = src_v.provenance() {
            draft.arena_mut().get_vertex_mut(vid)?.set_provenance(Some(*stored));
        }
    }
    
    vertex_dedup.insert(src_vertex, vid);
    spatial_index.insert(vid, *pos);
    if let Some(key) = match_key {
        global_vertex_map.insert(key, vid);
    }
    
    Ok(vid)
}

