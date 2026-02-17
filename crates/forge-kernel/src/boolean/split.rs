//! Face splitting along plane-plane intersections (corefinement).
//!
//! For two planar solids, computes the intersection line between each
//! pair of non-parallel face planes, clips the line to both face
//! boundaries, and inserts split edges using Euler operators.
//!
//! During splitting, all newly created vertex positions are recorded
//! into a `SharedVertexMap` so the assembly phase can deduplicate
//! vertices across arenas by position.
//!
//! All crossing decisions use `classify_point` → `CertifiedTriSign` (D3).

use std::collections::HashMap;

use forge_core::KernelError;
use forge_geom::plane::{Plane, classify_point, signed_distance};
use forge_math::sign::TriSign;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId, HalfEdgeId};
use forge_topo::state::{TopologyState, MutableDraft};
use forge_topo::traverse::face_edges;
use forge_topo::operator::apply_op;
use forge_topo::euler::split_edge::SplitEdge;
use forge_topo::euler::make_edge_face::MakeEdgeFace;

use crate::geometry_store::GeometryStore;
use super::eval::{quantize_position, planes_are_parallel};

/// Maps quantized 3D positions to vertex IDs from each arena.
///
/// After splitting both solids, this map records which vertices in the
/// target arena and tool arena correspond to the same spatial point.
/// The assembly phase uses this to merge cross-arena vertices.
pub struct SharedVertexMap {
    /// Quantized position → target VertexId.
    target_vertices: HashMap<[i64; 3], VertexId>,
    /// Quantized position → tool VertexId.
    tool_vertices: HashMap<[i64; 3], VertexId>,
}

impl SharedVertexMap {
    /// Create an empty map.
    fn new() -> Self {
        Self {
            target_vertices: HashMap::new(),
            tool_vertices: HashMap::new(),
        }
    }

    /// Record all vertex positions from a topology + geometry pair.
    fn record_all_vertices(
        &mut self,
        arena: &TopologyArena,
        geometry: &GeometryStore,
        is_target: bool,
    ) {
        for (vid, _) in arena.iter_vertices() {
            if let Some(pos) = geometry.get_vertex_position(vid) {
                let qpos = quantize_position(pos);
                if is_target {
                    self.target_vertices.insert(qpos, vid);
                } else {
                    self.tool_vertices.insert(qpos, vid);
                }
            }
        }
    }


}

/// Result of the entire split phase across both solids.
pub struct SplitPhaseResult {
    /// Updated target topology (faces may have been split).
    target_topology: TopologyState,
    /// Updated target geometry.
    target_geometry: GeometryStore,
    /// Updated tool topology (faces may have been split).
    tool_topology: TopologyState,
    /// Updated tool geometry.
    tool_geometry: GeometryStore,
    /// Total face splits performed.
    split_count: usize,
    /// Vertex correspondence map across both arenas.
    shared_vertices: SharedVertexMap,
}

impl SplitPhaseResult {
    /// Number of face splits performed across both solids.
    pub fn split_count(&self) -> usize {
        self.split_count
    }

    /// Consume and return owned parts including the shared vertex map.
    pub fn into_parts(self) -> (TopologyState, GeometryStore, TopologyState, GeometryStore, SharedVertexMap) {
        (
            self.target_topology,
            self.target_geometry,
            self.tool_topology,
            self.tool_geometry,
            self.shared_vertices,
        )
    }
}

/// Split all faces of both solids along their mutual intersections.
///
/// Both solids are split by the union of ALL planes from both operands.
/// This ensures edge-compatible face decompositions: every boundary
/// edge in one solid's selection has a matching edge in the other.
///
/// After splitting, builds a SharedVertexMap by recording all vertex
/// positions from both results. Vertices at the same spatial point
/// (created by splitting at the same intersection) will be linked.
pub fn split_all_faces(
    target_topo: TopologyState,
    target_geom: GeometryStore,
    tool_topo: TopologyState,
    tool_geom: GeometryStore,
) -> Result<SplitPhaseResult, KernelError> {
    // Use default config for now, until passed in
    let config = crate::core::ToleranceConfig::default();
    split_all_faces_with_config(target_topo, target_geom, tool_topo, tool_geom, &config)
}

/// Internal implementation of split_all_faces with explicit config.
pub fn split_all_faces_with_config(
    target_topo: TopologyState,
    target_geom: GeometryStore,
    tool_topo: TopologyState,
    tool_geom: GeometryStore,
    config: &crate::core::ToleranceConfig,
) -> Result<SplitPhaseResult, KernelError> {
    let mut total_splits = 0usize;

    let target_planes = collect_face_planes(target_topo.arena(), &target_geom)?;
    let tool_planes = collect_face_planes(tool_topo.arena(), &tool_geom)?;

    let mut all_planes: Vec<Plane> = Vec::with_capacity(target_planes.len() + tool_planes.len());
    all_planes.extend(tool_planes.iter().cloned());
    all_planes.extend(target_planes.iter().cloned());
    
    eprintln!("Splitting with {} planes (Target: {}, Tool: {})", all_planes.len(), target_planes.len(), tool_planes.len());

    let (split_target_topo, split_target_geom, target_splits) =
        split_solid_by_planes(target_topo, target_geom, &all_planes, config)?;
    total_splits += target_splits;
    
    eprintln!("Target splits: {}. Faces: {}", target_splits, split_target_topo.arena().face_count());

    let (split_tool_topo, split_tool_geom, tool_splits) =
        split_solid_by_planes(tool_topo, tool_geom, &all_planes, config)?;
    total_splits += tool_splits;

    eprintln!("Tool splits: {}. Faces: {}", tool_splits, split_tool_topo.arena().face_count());

    let mut shared_vertices = SharedVertexMap::new();
    shared_vertices.record_all_vertices(split_target_topo.arena(), &split_target_geom, true);
    shared_vertices.record_all_vertices(split_tool_topo.arena(), &split_tool_geom, false);

    Ok(SplitPhaseResult {
        target_topology: split_target_topo,
        target_geometry: split_target_geom,
        tool_topology: split_tool_topo,
        tool_geometry: split_tool_geom,
        split_count: total_splits,
        shared_vertices,
    })
}

/// Collect all face planes from a solid.
fn collect_face_planes(
    arena: &TopologyArena,
    geometry: &GeometryStore,
) -> Result<Vec<Plane>, KernelError> {
    let mut planes = Vec::new();
    for (face_id, _) in arena.iter_faces() {
        let plane = geometry.get_face_plane(face_id).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("Face {} has no associated plane", face_id),
                context: None,
            }
        })?;
        planes.push(plane.clone());
    }
    Ok(planes)
}

/// Split all faces of a solid against a set of cutting planes.
///
/// Iterates cutting planes and for each one, attempts to split every
/// face of the solid. New faces produced by splits are also checked
/// against remaining cutting planes.
fn split_solid_by_planes(
    topo: TopologyState,
    geom: GeometryStore,
    cutting_planes: &[Plane],
    config: &crate::core::ToleranceConfig,
) -> Result<(TopologyState, GeometryStore, usize), KernelError> {
    let mut current_topo = topo;
    let mut current_geom = geom;
    let mut total_splits = 0usize;

    for cut_plane in cutting_planes {
        let (new_topo, new_geom, splits) =
            split_solid_by_single_plane(current_topo, current_geom, cut_plane, config)?;
        current_topo = new_topo;
        current_geom = new_geom;
        total_splits += splits;
    }

    Ok((current_topo, current_geom, total_splits))
}

/// Vertex dedup map used during a single split pass.
///
/// Tracks quantized 3D positions → existing VertexIds within the draft.
/// When two cutting planes would create a vertex at the same corner point,
/// the second split reuses the existing vertex instead of creating a duplicate.
struct SplitVertexDedup {
    by_position: HashMap<[i64; 3], VertexId>,
}

impl SplitVertexDedup {
    /// Create an empty dedup map.
    fn new() -> Self {
        Self {
            by_position: HashMap::new(),
        }
    }

    /// Look up an existing vertex at this quantized position.
    fn find(&self, pos: &[f64; 3]) -> Option<VertexId> {
        self.by_position.get(&quantize_position(pos)).copied()
    }

    /// Record a vertex at a quantized position.
    fn insert(&mut self, pos: &[f64; 3], vid: VertexId) {
        self.by_position.insert(quantize_position(pos), vid);
    }
}

/// Split all faces of a solid against one cutting plane.
///
/// Collects the face IDs up front, then attempts to split each one.
/// Newly created faces from a split inherit the parent's plane and
/// will be checked in subsequent cutting-plane passes.
fn split_solid_by_single_plane(
    topo: TopologyState,
    mut geom: GeometryStore,
    cut_plane: &Plane,
    config: &crate::core::ToleranceConfig,
) -> Result<(TopologyState, GeometryStore, usize), KernelError> {
    let face_ids: Vec<FaceId> = topo.arena().iter_faces()
        .map(|(fid, _)| fid)
        .collect();

    let mut draft = topo.begin_mutation();
    let mut splits = 0usize;
    let mut vertex_dedup = SplitVertexDedup::new();

    for (vid, _) in draft.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            vertex_dedup.insert(pos, vid);
        }
    }

    for face_id in face_ids {
        let face_plane = geom.get_face_plane(face_id).cloned();
        let Some(face_plane) = face_plane else { continue };

        if planes_are_parallel(&face_plane, cut_plane) {
            continue;
        }

        let did_split = split_face_by_plane(
            &mut draft, &mut geom, &mut vertex_dedup, face_id, &face_plane, cut_plane, config,
        )?;

        if did_split {
            splits += 1;
        }
    }

    let committed = draft.commit()?;
    Ok((committed, geom, splits))
}


/// A point where a cutting plane intersects a face's boundary.
///
/// This enum encodes the binary truth from `classify_point`:
/// - `Zero` → the plane passes through an existing vertex (reuse it)
/// - `Pos/Neg` opposite signs → the plane slices an edge interior (insert new vertex)
enum CutPoint {
    /// The plane passes exactly through an existing vertex (`TriSign::Zero`).
    Existing(VertexId),
    /// The plane cleanly slices the interior of a halfedge (`Pos → Neg` or `Neg → Pos`).
    NewOnEdge(HalfEdgeId),
}

/// Split a single face along the intersection with a cutting plane.
///
/// Uses `classify_point` (→ `CertifiedTriSign`) for D3-compliant
/// crossing detection. Only splits when the plane genuinely divides
/// the face: vertices must exist on both sides (Pos AND Neg).
fn split_face_by_plane(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    vertex_dedup: &mut SplitVertexDedup,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    config: &crate::core::ToleranceConfig,
) -> Result<bool, KernelError> {
    if !has_vertices_on_both_sides(draft.arena(), geometry, face, cut_plane)? {
        return Ok(false);
    }

    let cut_points = find_cut_points_certified(draft.arena(), geometry, face, cut_plane, vertex_dedup, config)?;

    if cut_points.len() < 2 {
        return Ok(false);
    }

    let vertex_a = match resolve_cut_point(&cut_points[0], draft, geometry, vertex_dedup, cut_plane) {
        Ok(v) => v,
        Err(_) => return Ok(false),
    };
    let vertex_b = match resolve_cut_point(&cut_points[1], draft, geometry, vertex_dedup, cut_plane) {
        Ok(v) => v,
        Err(_) => return Ok(false),
    };

    if vertex_a == vertex_b {
        return Ok(false);
    }

    let mef_result = apply_op(draft, MakeEdgeFace {
        vertex_a,
        vertex_b,
        face,
    })?;

    let new_face = mef_result.get_value().new_face;
    geometry.set_face_plane(new_face, face_plane.clone());

    Ok(true)
}

/// Check whether a face has vertices on strictly both sides of a cutting plane.
///
/// Returns `true` only if at least one vertex classifies as `Pos` AND
/// at least one classifies as `Neg`. Faces where all vertices are
/// on one side (or all Zero) are not intersected by the plane.
fn has_vertices_on_both_sides(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    cut_plane: &Plane,
) -> Result<bool, KernelError> {
    let edges = face_edges(arena, face)?;
    let mut has_pos = false;
    let mut has_neg = false;

    for he_id in &edges {
        let he_data = arena.get_half_edge(*he_id)?;
        let pos = geometry.get_vertex_position(he_data.origin).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", he_data.origin),
                context: None,
            }
        })?;

        let sign = classify_point(cut_plane, pos)
            .map_err(|e| KernelError::InternalError {
                message: format!("classify_point failed: {}", e),
                context: None,
            })?
            .sign();

        match sign {
            TriSign::Pos => has_pos = true,
            TriSign::Neg => has_neg = true,
            TriSign::Zero => {}
        }

        if has_pos && has_neg {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Resolve a `CutPoint` into a `VertexId`.
///
/// `Existing` vertices are returned directly.
/// `NewOnEdge` vertices are created via `SplitEdge` + interpolation,
/// unless a vertex already exists at the same quantized position
/// (detected via `SplitVertexDedup`).
fn resolve_cut_point(
    cut_point: &CutPoint,
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    vertex_dedup: &mut SplitVertexDedup,
    cut_plane: &Plane,
) -> Result<VertexId, KernelError> {
    match cut_point {
        CutPoint::Existing(vid) => Ok(*vid),
        CutPoint::NewOnEdge(he_id) => {
            let (new_vertex, _) = insert_new_vertex_on_edge(draft, geometry, vertex_dedup, *he_id, cut_plane)?;
            Ok(new_vertex)
        }
    }
}

/// Find where the cutting plane intersects a face's boundary.
///
/// Uses `face_edges` for loop traversal and classifies each vertex
/// via `classify_point`.
/// - `TriSign::Zero` → the cutting plane passes through this vertex exactly
/// - Strict `Pos → Neg` or `Neg → Pos` transition → the plane slices the edge
///
/// When an edge-crossing would produce a vertex at a quantized position
/// that already exists in `vertex_dedup` (from a prior cutting-plane pass),
/// locates the existing vertex on the face boundary and emits `Existing`
/// instead of `NewOnEdge` to prevent degenerate zero-length edges.
fn find_cut_points_certified(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    cut_plane: &Plane,
    vertex_dedup: &SplitVertexDedup,
    config: &crate::core::ToleranceConfig,
) -> Result<Vec<CutPoint>, KernelError> {
    let edges = face_edges(arena, face)?;
    let mut cut_points = Vec::new();

    for he_id in &edges {
        let he_data = arena.get_half_edge(*he_id)?;
        let origin = he_data.origin;
        let next_he = he_data.next;
        let dest = arena.get_half_edge(next_he)?.origin;

        let pos_origin = geometry.get_vertex_position(origin).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", origin),
                context: None,
            }
        })?;
        let pos_dest = geometry.get_vertex_position(dest).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", dest),
                context: None,
            }
        })?;

        let o_sign = classify_point(cut_plane, pos_origin)
            .map_err(|e| KernelError::InternalError {
                message: format!("classify_point failed for origin: {}", e),
                context: None,
            })?
            .sign();
        let d_sign = classify_point(cut_plane, pos_dest)
            .map_err(|e| KernelError::InternalError {
                message: format!("classify_point failed for dest: {}", e),
                context: None,
            })?
            .sign();

        if o_sign == TriSign::Zero {
            cut_points.push(CutPoint::Existing(origin));
        } else if (o_sign == TriSign::Pos && d_sign == TriSign::Neg)
            || (o_sign == TriSign::Neg && d_sign == TriSign::Pos)
        {
            let intersection_pos = compute_edge_plane_intersection(pos_origin, pos_dest, cut_plane, config.get_edge_split_degeneracy());
            let Some(intersection_pos) = intersection_pos else {
                continue;
            };
            if let Some(existing_vid) = vertex_dedup.find(&intersection_pos) {
                if is_vertex_on_face(arena, face, existing_vid)? {
                    cut_points.push(CutPoint::Existing(existing_vid));
                    continue;
                }
            }
            cut_points.push(CutPoint::NewOnEdge(*he_id));
        }
    }

    Ok(cut_points)
}

/// Compute the intersection position of an edge with a cutting plane.
///
/// Uses signed distance interpolation. Only called for strict
/// `Pos → Neg` transitions, so the denominator is structurally non-zero.
fn compute_edge_plane_intersection(
    origin_pos: &[f64; 3],
    dest_pos: &[f64; 3],
    cut_plane: &Plane,
    degeneracy_threshold: f64,
) -> Option<[f64; 3]> {
    let dist_origin = signed_distance(cut_plane, origin_pos);
    let dist_dest = signed_distance(cut_plane, dest_pos);
    let denom = dist_origin - dist_dest;
    if denom.abs() < degeneracy_threshold {
        return None;
    }
    let t = dist_origin / denom;
    let pos = [
        origin_pos[0] + t * (dest_pos[0] - origin_pos[0]),
        origin_pos[1] + t * (dest_pos[1] - origin_pos[1]),
        origin_pos[2] + t * (dest_pos[2] - origin_pos[2]),
    ];
    if !pos[0].is_finite() || !pos[1].is_finite() || !pos[2].is_finite() {
        return None;
    }
    Some(pos)
}

/// Check whether a vertex is on a face's boundary loop.
fn is_vertex_on_face(
    arena: &TopologyArena,
    face: FaceId,
    vertex: VertexId,
) -> Result<bool, KernelError> {
    let edges = face_edges(arena, face)?;
    for he_id in &edges {
        let he_data = arena.get_half_edge(*he_id)?;
        if he_data.origin == vertex {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Insert a new vertex at the intersection of a halfedge with the cutting plane.
///
/// Uses `signed_distance` for interpolation position (geometry-only,
/// not topology) after the topology decision is already certified.
/// This function is only called for strict `Pos → Neg` transitions,
/// so the denominator is structurally non-zero.
///
/// If a vertex already exists at the computed quantized position
/// (from a prior split by a different cutting plane), the edge is
/// still topologically split, but the new vertex is merged with the
/// existing one to avoid degenerate zero-length edges.
fn insert_new_vertex_on_edge(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    vertex_dedup: &mut SplitVertexDedup,
    half_edge: HalfEdgeId,
    cut_plane: &Plane,
) -> Result<(VertexId, HalfEdgeId), KernelError> {
    let he_data = draft.arena().get_half_edge(half_edge)?;
    let origin = he_data.origin;
    let next_he = he_data.next;
    let dest = draft.arena().get_half_edge(next_he)?.origin;

    let origin_pos = geometry.get_vertex_position(origin)
        .copied()
        .ok_or_else(|| KernelError::InvalidInput {
            message: "Missing origin position for edge split".to_string(),
            context: None,
        })?;
    let dest_pos = geometry.get_vertex_position(dest)
        .copied()
        .ok_or_else(|| KernelError::InvalidInput {
            message: "Missing dest position for edge split".to_string(),
            context: None,
        })?;

    let dist_origin = signed_distance(cut_plane, &origin_pos);
    let dist_dest = signed_distance(cut_plane, &dest_pos);
    let denom = dist_origin - dist_dest;

    let t = dist_origin / denom;
    let intersection_pos = [
        origin_pos[0] + t * (dest_pos[0] - origin_pos[0]),
        origin_pos[1] + t * (dest_pos[1] - origin_pos[1]),
        origin_pos[2] + t * (dest_pos[2] - origin_pos[2]),
    ];

    if !intersection_pos[0].is_finite() || !intersection_pos[1].is_finite() || !intersection_pos[2].is_finite() {
        return Err(KernelError::InternalError {
            message: "Edge-plane intersection produced non-finite position".to_string(),
            context: None,
        });
    }

    let se_result = apply_op(draft, SplitEdge { edge: half_edge })?;
    let new_vertex = se_result.get_value().new_vertex;
    let new_he = se_result.get_value().he_mb;

    geometry.set_vertex_position(new_vertex, intersection_pos);
    vertex_dedup.insert(&intersection_pos, new_vertex);

    Ok((new_vertex, new_he))
}
