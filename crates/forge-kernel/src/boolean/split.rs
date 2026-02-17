//! Face splitting along plane-plane intersections.
//!
//! For two planar solids, computes the intersection line between each
//! pair of non-parallel face planes, clips the line to both face
//! boundaries, and inserts split edges using Euler operators.
//!
//! All crossing decisions use `classify_point` → `CertifiedTriSign` (D3).

use forge_core::KernelError;
use forge_geom::plane::{Plane, classify_point, signed_distance};
use forge_math::sign::TriSign;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId, HalfEdgeId};
use forge_topo::state::{TopologyState, MutableDraft};
use forge_topo::operator::apply_op;
use forge_topo::euler::split_edge::SplitEdge;
use forge_topo::euler::make_edge_face::MakeEdgeFace;

use crate::geometry_store::GeometryStore;

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
}

impl SplitPhaseResult {
    /// Number of face splits performed across both solids.
    pub fn split_count(&self) -> usize {
        self.split_count
    }

    /// Consume and return owned parts.
    pub fn into_parts(self) -> (TopologyState, GeometryStore, TopologyState, GeometryStore) {
        (
            self.target_topology,
            self.target_geometry,
            self.tool_topology,
            self.tool_geometry,
        )
    }
}

/// Split all faces of both solids along their mutual intersections.
///
/// For each face in the target, checks every face in the tool for
/// plane-plane intersections, and splits faces that cross the other's
/// plane. Repeats for tool faces against target planes.
pub fn split_all_faces(
    target_topo: TopologyState,
    target_geom: GeometryStore,
    tool_topo: TopologyState,
    tool_geom: GeometryStore,
) -> Result<SplitPhaseResult, KernelError> {
    let mut total_splits = 0usize;

    let target_planes = collect_face_planes(target_topo.arena(), &target_geom)?;
    let tool_planes = collect_face_planes(tool_topo.arena(), &tool_geom)?;

    let (split_target_topo, split_target_geom, target_splits) =
        split_solid_by_planes(target_topo, target_geom, &tool_planes)?;
    total_splits += target_splits;

    let (split_tool_topo, split_tool_geom, tool_splits) =
        split_solid_by_planes(tool_topo, tool_geom, &target_planes)?;
    total_splits += tool_splits;

    Ok(SplitPhaseResult {
        target_topology: split_target_topo,
        target_geometry: split_target_geom,
        tool_topology: split_tool_topo,
        tool_geometry: split_tool_geom,
        split_count: total_splits,
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
) -> Result<(TopologyState, GeometryStore, usize), KernelError> {
    let mut current_topo = topo;
    let mut current_geom = geom;
    let mut total_splits = 0usize;

    for cut_plane in cutting_planes {
        let (new_topo, new_geom, splits) =
            split_solid_by_single_plane(current_topo, current_geom, cut_plane)?;
        current_topo = new_topo;
        current_geom = new_geom;
        total_splits += splits;
    }

    Ok((current_topo, current_geom, total_splits))
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
) -> Result<(TopologyState, GeometryStore, usize), KernelError> {
    let face_ids: Vec<FaceId> = topo.arena().iter_faces()
        .map(|(fid, _)| fid)
        .collect();

    let mut draft = topo.begin_mutation();
    let mut splits = 0usize;

    for face_id in face_ids {
        let face_plane = geom.get_face_plane(face_id).cloned();
        let Some(face_plane) = face_plane else { continue };

        if planes_are_parallel(&face_plane, cut_plane) {
            continue;
        }

        let did_split = split_face_by_plane(
            &mut draft, &mut geom, face_id, &face_plane, cut_plane,
        )?;

        if did_split {
            splits += 1;
        }
    }

    let committed = draft.commit()?;
    Ok((committed, geom, splits))
}

/// Check if two planes are parallel (cross product of normals ≈ zero).
fn planes_are_parallel(a: &Plane, b: &Plane) -> bool {
    let na = a.normal();
    let nb = b.normal();

    let cross = [
        na[1] * nb[2] - na[2] * nb[1],
        na[2] * nb[0] - na[0] * nb[2],
        na[0] * nb[1] - na[1] * nb[0],
    ];

    let len_sq = cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2];
    len_sq < 1e-20
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
/// crossing detection. Returns `true` if the face was split.
fn split_face_by_plane(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
) -> Result<bool, KernelError> {
    let cut_points = find_cut_points_certified(draft.arena(), geometry, face, cut_plane)?;

    if cut_points.len() < 2 {
        return Ok(false);
    }

    let vertex_a = resolve_cut_point(&cut_points[0], draft, geometry, cut_plane)?;
    let vertex_b = resolve_cut_point(&cut_points[1], draft, geometry, cut_plane)?;

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

/// Resolve a `CutPoint` into a `VertexId`.
///
/// `Existing` vertices are returned directly.
/// `NewOnEdge` vertices are created via `SplitEdge` + interpolation.
fn resolve_cut_point(
    cut_point: &CutPoint,
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    cut_plane: &Plane,
) -> Result<VertexId, KernelError> {
    match cut_point {
        CutPoint::Existing(vid) => Ok(*vid),
        CutPoint::NewOnEdge(he_id) => {
            let (new_vertex, _) = insert_new_vertex_on_edge(draft, geometry, *he_id, cut_plane)?;
            Ok(new_vertex)
        }
    }
}

/// Find where the cutting plane intersects a face's boundary.
///
/// Walks the face loop and classifies each vertex via `classify_point`.
/// - `TriSign::Zero` → the cutting plane passes through this vertex exactly
/// - Strict `Pos → Neg` or `Neg → Pos` transition → the plane slices the edge
fn find_cut_points_certified(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    cut_plane: &Plane,
) -> Result<Vec<CutPoint>, KernelError> {
    let face_data = arena.get_face(face)?;
    let loop_data = arena.get_loop(face_data.outer_loop)?;
    let start_he = loop_data.half_edge;
    let mut cut_points = Vec::new();
    let mut current = start_he;
    let max_iterations: usize = 1000;

    for _ in 0..max_iterations {
        let he_data = arena.get_half_edge(current)?;
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
            cut_points.push(CutPoint::NewOnEdge(current));
        }

        current = next_he;
        if current == start_he {
            return Ok(cut_points);
        }
    }

    Err(KernelError::InternalError {
        message: "Loop limit exceeded in find_cut_points_certified".to_string(),
        context: None,
    })
}

/// Insert a new vertex at the intersection of a halfedge with the cutting plane.
///
/// Uses `signed_distance` for interpolation position (geometry-only,
/// not topology) after the topology decision is already certified.
/// This function is only called for strict `Pos → Neg` transitions,
/// so the denominator is structurally non-zero.
fn insert_new_vertex_on_edge(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
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

    let se_result = apply_op(draft, SplitEdge { edge: half_edge })?;
    let new_vertex = se_result.get_value().new_vertex;
    let new_he = se_result.get_value().he_mb;

    geometry.set_vertex_position(new_vertex, intersection_pos);

    Ok((new_vertex, new_he))
}
