//! Copy faces from one arena to another.
//!
//! DOMAIN: Transfer topology and geometry between arenas, remapping handles
//! and deduplicating vertices via provenance keys and spatial search.
//!
//! DEPENDENCIES: schema (VertexMatchKey), GeometryStore, forge_topo.
//!
//! INVARIANTS:
//! - Vertex dedup uses 3 layers: local → provenance key → spatial NNS → create new.
//! - Cross-solid vertex collisions merge lineage using `Lineage::merge` (D1).
//! - Direct arena insertion (not Euler operators) for full halfedge control.

use std::collections::BTreeMap;

use forge_core::KernelError;
use forge_topo::arena::{BodyData, LumpData, RegionData, FaceData, HalfEdgeData, LoopData, VertexData, ShellData, EdgeData};
use forge_topo::handles::{BodyId, RegionId, FaceId, VertexId, HalfEdgeId, LoopId, ShellId, EdgeId};
use forge_topo::lineage::{Lineage, OpSignature};
use forge_topo::state::MutableDraft;
use forge_topo::traverse::FaceEdgeIterator;

use crate::geometry_store::GeometryStore;
use crate::operations::boolean::eval::VertexMatchKey;

/// Mutable destination state for the face-copy phase.
///
/// Groups the 6 destination-side objects that always travel together.
/// Source-side `(arena, geom)` stays as separate `&` params.
pub struct CopyContext<'a> {
    pub draft: &'a mut MutableDraft,
    pub geometry: &'a mut GeometryStore,
    pub vertex_dedup: &'a mut VertexDedup,
    pub new_edges: &'a mut Vec<HalfEdgeId>,
    pub global_vertex_map: &'a mut BTreeMap<VertexMatchKey, VertexId>,
    pub spatial_index: &'a mut VertexWelder,
}

/// Local vertex mapping for one solid (old arena → new arena).
pub struct VertexDedup {
    mapping: BTreeMap<VertexId, VertexId>,
}

impl VertexDedup {
    pub fn new() -> Self {
        Self { mapping: BTreeMap::new() }
    }

    pub fn insert(&mut self, old: VertexId, new: VertexId) {
        self.mapping.insert(old, new);
    }

    pub fn get(&self, old: VertexId) -> Option<VertexId> {
        self.mapping.get(&old).copied()
    }
}

/// Vertex position welder with Union-Find transitive clustering.
///
/// Backed by `forge_geom::spatial::epsilon_weld::EpsilonWelder` for
/// spatial hashing and transitive merging. Maintains a parallel
/// `VertexId` mapping so the topology layer can resolve cluster roots.
pub struct VertexWelder {
    welder: forge_geom::spatial::epsilon_weld::EpsilonWelder,
    vertex_ids: Vec<VertexId>,
    weld_tolerance_sq: f64,
}

impl VertexWelder {
    /// Create a new welder scaled to the input geometry.
    ///
    /// Tolerance is `characteristic_scale * 1e-8`.
    pub fn new(characteristic_scale: f64) -> Self {
        let scale = characteristic_scale.max(1e-15);
        let linear_tol = scale * 1e-8;
        Self::with_linear_tolerance(linear_tol)
    }

    /// Create a welder with an explicit linear tolerance.
    pub fn with_linear_tolerance(linear_tol: f64) -> Self {
        let linear_tol = linear_tol.max(1e-15);
        Self {
            welder: forge_geom::spatial::epsilon_weld::EpsilonWelder::new(linear_tol),
            vertex_ids: Vec::new(),
            weld_tolerance_sq: linear_tol * linear_tol,
        }
    }

    /// Find the canonical VertexId for a position, or None if no match.
    pub fn find_nearest(&mut self, pos: &[f64; 3]) -> Option<VertexId> {
        self.welder.find_nearest(pos).map(|root| self.vertex_ids[root])
    }

    /// Register a vertex position for future lookups.
    pub fn insert(&mut self, vid: VertexId, pos: [f64; 3]) {
        let idx = self.welder.add_vertex(pos);
        if idx >= self.vertex_ids.len() {
            self.vertex_ids.push(vid);
        } else {
            self.vertex_ids[idx] = vid;
        }
    }

    /// The scale-proportional squared weld tolerance.
    pub fn weld_tolerance_sq(&self) -> f64 {
        self.weld_tolerance_sq
    }
}

// ── Face copying ─────────────────────────────────────────────────────────────

/// Copy a set of faces from a source arena to a destination draft.
pub fn copy_faces(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    vertex_dedup: &mut VertexDedup,
    new_edges: &mut Vec<HalfEdgeId>,
    global_vertex_map: &mut BTreeMap<VertexMatchKey, VertexId>,
    spatial_index: &mut VertexWelder,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    source_faces: &[FaceId],
    reverse_orientation: bool,
    lineage_copy_tag: &str,
    src_prov: Option<&BTreeMap<VertexId, VertexMatchKey>>,
) -> Result<(), KernelError> {
    let mut shell_map: std::collections::BTreeMap<ShellId, ShellId> = std::collections::BTreeMap::new();
    let destination_body = ensure_destination_body(draft);

    for &src_face in source_faces {
        let src_shell = source_arena.get_face(src_face)?.shell();
        let dest_shell = if let Some(existing_shell) = shell_map.get(&src_shell) {
            *existing_shell
        } else {
            let kind = source_arena.get_shell(src_shell)
                .map(|s| s.kind())
                .unwrap_or(forge_topo::arena::ShellKind::Solid(forge_topo::arena::ShellOrientation::Outer));
            let region = create_destination_region(draft, destination_body)?;
            let shell = draft.insert_shell(ShellData::new(
                FaceId::from_raw_parts(u32::MAX, 0),
                kind,
                region,
            ));
            draft.arena_mut().get_region_mut(region)?.add_shell(shell);
            shell_map.insert(src_shell, shell);
            shell
        };
        
        let new_face = copy_single_face(
            draft, result_geom, vertex_dedup, new_edges, global_vertex_map,
            spatial_index,
            source_arena, source_geom, src_face, reverse_orientation, lineage_copy_tag, src_prov, dest_shell,
        )?;
        
        draft.arena_mut().get_shell_mut(dest_shell).unwrap().set_representative_face(new_face);
    }
    Ok(())
}

fn ensure_destination_body(draft: &mut MutableDraft) -> BodyId {
    if let Some((body_id, _)) = draft.arena().iter_bodies().next() {
        return body_id;
    }
    draft.insert_body(BodyData::new())
}

// DEFECT(D7): Region/Lump/Body hierarchy created manually instead of via MakeLumpRegion.
fn create_destination_region(
    draft: &mut MutableDraft,
    body: BodyId,
) -> Result<RegionId, KernelError> {
    let lump = draft.insert_lump(LumpData::new(body));
    let region = draft.insert_region(RegionData::new(lump));
    draft.arena_mut().get_body_mut(body)?.add_lump(lump);
    draft.arena_mut().get_lump_mut(lump)?.add_region(region);
    Ok(region)
}

/// Copy a single face via direct arena insertion.
// DEFECT(D1): copy_single_face does raw arena insertion (insert_face/insert_half_edge) instead of using certified Euler operations.
fn copy_single_face(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    vertex_dedup: &mut VertexDedup,
    new_edges: &mut Vec<HalfEdgeId>,
    global_vertex_map: &mut BTreeMap<VertexMatchKey, VertexId>,
    spatial_index: &mut VertexWelder,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    src_face: FaceId,
    reverse_orientation: bool,
    lineage_copy_tag: &str,
    src_prov: Option<&BTreeMap<VertexId, VertexMatchKey>>,
    dest_shell: ShellId,
) -> Result<FaceId, KernelError> {
    let new_plane = prepare_face_plane(source_geom, src_face, reverse_orientation)?;
    let src_face_lineage = source_arena.get_face(src_face)
        .ok()
        .and_then(|f| f.lineage().cloned());

    let edges: Vec<_> = FaceEdgeIterator::new(source_arena, src_face)?
        .collect::<Result<Vec<_>, _>>()?;

    if edges.is_empty() {
        return insert_empty_face(draft, result_geom, new_plane);
    }

    let src_verts = collect_source_vertices(source_arena, &edges, reverse_orientation)?;

    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let placeholder_loop = LoopId::from_raw_parts(u32::MAX, 0);

    let copy_op_name = if reverse_orientation {
        format!("boolean_copy_face_{}_rev", lineage_copy_tag)
    } else {
        format!("boolean_copy_face_{}_fwd", lineage_copy_tag)
    };
    let copy_sig = OpSignature::with_id(&copy_op_name, src_face.index() as u64);
    let face_lineage = Some(Lineage::derive_from(&src_face_lineage, copy_sig));
    let face_id = draft.insert_face(FaceData::with_lineage(placeholder_loop, dest_shell, face_lineage));
    result_geom.set_face_plane(face_id, new_plane);

    let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face_id));

    let resolved_verts = resolve_all_vertices(
        draft, result_geom, vertex_dedup, global_vertex_map, spatial_index,
        source_arena, source_geom, &src_verts, src_prov,
    )?;

    let he_ids = insert_halfedges(draft, &resolved_verts, face_id)?;
    wire_halfedge_loop(draft, &he_ids)?;

    draft.arena_mut().get_face_mut(face_id)?.set_outer_loop(loop_id);
    draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(he_ids[0]);

    set_vertex_outgoing_pointers(draft, &he_ids)?;

    new_edges.extend_from_slice(&he_ids);
    Ok(face_id)
}

/// Prepare the face plane, flipping if reverse orientation is needed.
fn prepare_face_plane(
    source_geom: &GeometryStore,
    src_face: FaceId,
    reverse: bool,
) -> Result<forge_geom::Plane, KernelError> {
    let src_plane = source_geom.get_face_plane(src_face).ok_or(KernelError::InvalidInput {
        message: format!("Face {} missing plane", src_face), context: None,
    })?;
    let mut plane = src_plane.clone();
    if reverse { plane.flip(); }
    Ok(plane)
}

/// Insert an empty face (no edges) with the given plane.
fn insert_empty_face(
    draft: &mut MutableDraft,
    geom: &mut GeometryStore,
    plane: forge_geom::Plane,
) -> Result<FaceId, KernelError> {
    let placeholder_loop = LoopId::from_raw_parts(u32::MAX, 0);
    let placeholder_shell = ShellId::from_raw_parts(u32::MAX, 0);
    let face_id = draft.insert_face(FaceData::new(placeholder_loop, placeholder_shell));
    geom.set_face_plane(face_id, plane);
    Ok(face_id)
}

/// Collect source vertex IDs in winding order (reversed if needed).
fn collect_source_vertices(
    source_arena: &forge_topo::arena::TopologyArena,
    edges: &[HalfEdgeId],
    reverse: bool,
) -> Result<Vec<VertexId>, KernelError> {
    let iter_order: Vec<HalfEdgeId> = if reverse {
        edges.iter().rev().copied().collect()
    } else {
        edges.to_vec()
    };

    let mut verts = Vec::with_capacity(iter_order.len());
    for he in &iter_order {
        verts.push(source_arena.get_half_edge(*he)?.origin());
    }
    Ok(verts)
}

/// Resolve all source vertices to destination vertices.
fn resolve_all_vertices(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    vertex_dedup: &mut VertexDedup,
    global_vertex_map: &mut BTreeMap<VertexMatchKey, VertexId>,
    spatial_index: &mut VertexWelder,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    src_verts: &[VertexId],
    src_prov: Option<&BTreeMap<VertexId, VertexMatchKey>>,
) -> Result<Vec<VertexId>, KernelError> {
    let mut resolved = Vec::with_capacity(src_verts.len());
    for &sv in src_verts {
        resolved.push(resolve_vertex(
            draft, result_geom, vertex_dedup, global_vertex_map,
            spatial_index, source_arena, source_geom, sv, src_prov,
        )?);
    }
    Ok(resolved)
}

/// Insert halfedges for each vertex with self-twin placeholders.
fn insert_halfedges(
    draft: &mut MutableDraft,
    verts: &[VertexId],
    face_id: FaceId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let placeholder = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let mut ids = Vec::with_capacity(verts.len());
    for &origin in verts {
        let he_id = draft.insert_half_edge(HalfEdgeData::new(
            placeholder, placeholder, placeholder, face_id, origin, EdgeId::from_raw_parts(u32::MAX, 0),
        ));
        let edge = draft.insert_edge(EdgeData::new(he_id));
        draft.arena_mut().get_half_edge_mut(he_id).unwrap().set_edge(edge);
        draft.arena_mut().get_half_edge_mut(he_id).unwrap().set_radial_next(he_id);
        ids.push(he_id);
    }
    Ok(ids)
}

/// Wire next/prev pointers in a closed loop.
fn wire_halfedge_loop(draft: &mut MutableDraft, he_ids: &[HalfEdgeId]) -> Result<(), KernelError> {
    let n = he_ids.len();
    for i in 0..n {
        let next_i = (i + 1) % n;
        let prev_i = if i == 0 { n - 1 } else { i - 1 };
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(he_ids[i])?.set_next(he_ids[next_i]);
        arena.get_half_edge_mut(he_ids[i])?.set_prev(he_ids[prev_i]);
    }
    Ok(())
}

/// Set outgoing pointer on each vertex to its halfedge.
fn set_vertex_outgoing_pointers(draft: &mut MutableDraft, he_ids: &[HalfEdgeId]) -> Result<(), KernelError> {
    for &he_id in he_ids {
        let origin = draft.arena().get_half_edge(he_id)?.origin();
        draft.arena_mut().get_vertex_mut(origin)?.set_outgoing(he_id);
    }
    Ok(())
}

// ── Vertex resolution ────────────────────────────────────────────────────────

/// Resolve a source vertex to a destination vertex using 4-layer dedup:
/// 1. Local dedup (same solid, already copied)
/// 2. Global VertexMatchKey map (cross-solid gluing via 3-plane provenance)
/// 3. Spatial NNS (fuzzy position fallback)
/// 4. Create new vertex
fn resolve_vertex(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    vertex_dedup: &mut VertexDedup,
    global_vertex_map: &mut BTreeMap<VertexMatchKey, VertexId>,
    spatial_index: &mut VertexWelder,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    src_vertex: VertexId,
    src_prov: Option<&BTreeMap<VertexId, VertexMatchKey>>,
) -> Result<VertexId, KernelError> {
    let pos = source_geom.get_vertex_position(src_vertex).ok_or(
        KernelError::InvalidInput { message: "Missing vertex position".into(), context: None }
    )?;

    if let Some(mapped) = vertex_dedup.get(src_vertex) {
        return Ok(mapped);
    }

    let match_key = src_prov.and_then(|prov| prov.get(&src_vertex).cloned());

    if let Some(ref key) = match_key {
        if let Some(&global_id) = global_vertex_map.get(key) {
            merge_vertex_lineage(draft, source_arena, src_vertex, global_id, "boolean_vertex_merge")?;
            vertex_dedup.insert(src_vertex, global_id);
            return Ok(global_id);
        }
    }

    if let Some(existing_id) = spatial_index.find_nearest(pos) {
        if draft.arena().get_vertex(existing_id).is_ok() {
            merge_vertex_lineage(draft, source_arena, src_vertex, existing_id, "boolean_vertex_merge_spatial")?;
            vertex_dedup.insert(src_vertex, existing_id);
            return Ok(existing_id);
        }
    }

    let vid = create_new_vertex(draft, result_geom, source_arena, source_geom, src_vertex, pos)?;
    vertex_dedup.insert(src_vertex, vid);
    spatial_index.insert(vid, *pos);
    if let Some(key) = match_key {
        global_vertex_map.insert(key, vid);
    }
    Ok(vid)
}

/// Merge lineage from a source vertex into an existing destination vertex.
fn merge_vertex_lineage(
    draft: &mut MutableDraft,
    source_arena: &forge_topo::arena::TopologyArena,
    src_vertex: VertexId,
    dest_vertex: VertexId,
    op_name: &str,
) -> Result<(), KernelError> {
    let src_lineage = source_arena.get_vertex(src_vertex)
        .ok().and_then(|v| v.lineage().cloned());
    let existing_lineage = draft.arena().get_vertex(dest_vertex)
        .ok().and_then(|v| v.lineage().cloned());
    let merged = Lineage::merge(&existing_lineage, &src_lineage, &OpSignature::new(op_name));
    draft.arena_mut().get_vertex_mut(dest_vertex)?.set_lineage(Some(merged));
    Ok(())
}

/// Create a brand new vertex in the destination arena with geometry.
fn create_new_vertex(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    src_vertex: VertexId,
    pos: &[f64; 3],
) -> Result<VertexId, KernelError> {
    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let src_lineage = source_arena.get_vertex(src_vertex)
        .ok().and_then(|v| v.lineage().cloned());
    let vid = draft.insert_vertex(VertexData::with_lineage(placeholder_he, src_lineage));
    result_geom.set_vertex_position(vid, *pos);
    if let Some(exact) = source_geom.get_vertex_position_exact(src_vertex) {
        if let Some(planes) = source_geom.get_vertex_symbolic_planes(src_vertex) {
            result_geom.set_vertex_position_symbolic(vid, exact.clone(), *pos, *planes);
        } else {
            result_geom.set_vertex_position_exact(vid, exact.clone());
        }
    }
    Ok(vid)
}

// ── Identity repair ──────────────────────────────────────────────────────────

/// Pre-stitch identity repair: cluster vertices by position, rewrite
/// halfedge endpoints to use the canonical (lowest-index) VertexId in
/// each cluster.
///
/// This fixes "same position, different VertexId" defects that arise when
/// faces from different sources are copied into the same arena. It's local
/// surgery: face/edge structure is preserved, only vertex endpoints change.
///
/// Returns the number of vertices that were merged into canonical IDs.
pub fn repair_vertex_identity(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
    weld_tolerance: f64,
    ctx: &mut crate::core::ModelingContext,
) -> Result<usize, KernelError> {
    let mut welder = forge_geom::spatial::epsilon_weld::EpsilonWelder::new(weld_tolerance);
    let mut index_to_vid: Vec<VertexId> = Vec::new();

    for (vid, _) in draft.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            let idx = welder.add_vertex(*pos);
            if idx >= index_to_vid.len() {
                index_to_vid.push(vid);
            }
        }
    }

    let mut remap: BTreeMap<u32, VertexId> = BTreeMap::new();
    for (vid, _) in draft.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            if let Some(root) = welder.find_nearest(pos) {
                let canonical = index_to_vid[root];
                if canonical != vid {
                    remap.insert(vid.index(), canonical);
                }
            }
        }
    }

    if remap.is_empty() {
        return Ok(0);
    }

    let merged_count = remap.len();

    let all_he_ids: Vec<forge_topo::handles::HalfEdgeId> = draft.arena()
        .iter_half_edges()
        .map(|(id, _)| id)
        .collect();

    for he_id in &all_he_ids {
        let origin = draft.arena().get_half_edge(*he_id)?.origin();
        if let Some(&canonical) = remap.get(&origin.index()) {
            draft.arena_mut().get_half_edge_mut(*he_id)?.set_origin(canonical);
        }
    }

    let mut decision = forge_core::TracedDecision::new(
        forge_core::DecisionId(merged_count as u64),
        forge_core::DecisionKind::Forced {
            reason: format!("Identity repair: merged {} duplicate vertices", merged_count),
        },
        forge_core::DecisionTier::PolicyApplied,
        0.9,
        forge_core::DecisionContext::Degeneracy {
            description: format!(
                "Pre-stitch identity repair rewrote {} vertex references",
                merged_count,
            ),
        },
    );
    decision.set_entity_scope(forge_core::EntityRef::new(forge_core::EntityKind::Vertex, 0));
    ctx.get_decision_log_mut().record(decision);

    Ok(merged_count)
}
