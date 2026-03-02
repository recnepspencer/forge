//! Ray-casting parity classifier for point-in-solid queries.
//!
//! DOMAIN: Determine whether a 3D point is inside, outside, or on the boundary
//!         of a solid represented as a closed halfedge mesh.
//!
//! ALGORITHM: Two-pass structure:
//!   Pass 1: Tolerance boundary check via `classify_point_on_face` — catches
//!           points physically on the surface before SoS perturbs them off.
//!   Pass 2: SoS parity count — casts a +X axis ray, counts face crossings
//!           using Plücker + YZ-projection winding number with SoS tie-breaking.
//!           Odd count → Inside.
//!
//! DEPENDENCIES: forge-topo (arena, handles, FaceEdgeIterator),
//!               forge-geom (Aabb), forge-math (orient2d, orient3d, TriSign).

use forge_core::{KernelError, ToleranceProvider};
use forge_geom::Aabb;
use forge_math::predicates::{orient2d, orient3d};
use forge_math::sign::TriSign;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;

use super::point_on_face::{classify_point_on_face, FacePointClassification};
use super::schema::{PointClassification, SpatialAccelerator};
use super::sos::{sos_edge_crossing_yz, sos_orient2d_tiebreak, sos_orient3d_tiebreak};

/// Classify a point relative to a solid using ray-casting parity counting.
///
/// # Arguments
/// - `arena`: topology arena
/// - `vertex_positions`: maps vertex slot index → 3D position
/// - `spatial_index`: optional BVH to accelerate candidate selection
/// - `point`: the 3D query point
/// - `tolerance_provider`: per-entity tolerance for the boundary pre-check
// DEFECT(D8): classify_point_in_solid has no multi-direction retry if first ray hits degenerate components.
pub fn classify_point_in_solid(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    spatial_index: Option<&dyn SpatialAccelerator>,
    point: &[f64; 3],
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<PointClassification, KernelError> {
    let pos_fn = |v: VertexId| vertex_positions(v.index()).ok();

    let faces_to_check: Vec<FaceId> = if let Some(bvh) = spatial_index {
        let pt_aabb = Aabb::from_points(&[*point, *point]).unwrap();
        bvh.candidates(&pt_aabb)
    } else {
        arena.iter_faces().map(|(id, _)| id).collect()
    };

    for &face_id in &faces_to_check {
        match classify_point_on_face(arena, face_id, point, &pos_fn, tolerance_provider)? {
            FacePointClassification::Outside => {}
            _ => return Ok(PointClassification::OnBoundary(face_id)),
        }
    }

    let mut crossing_count: i32 = 0;
    let all_faces: Vec<FaceId> = arena.iter_faces().map(|(id, _)| id).collect();
    for face_id in all_faces {
        if sos_ray_intersects_face(arena, vertex_positions, face_id, point)? {
            crossing_count += 1;
        }
    }

    if crossing_count % 2 == 1 {
        Ok(PointClassification::Inside { escalation: None })
    } else {
        Ok(PointClassification::Outside { escalation: None })
    }
}

/// Resolve a classification ambiguity by symmetric perturbation along a direction.
///
/// Samples `point ± epsilon * direction` and re-runs `classify_point_in_solid`.
/// Returns `Some(classification)` only when both samples agree exactly.
pub fn classify_point_with_perturbation(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    spatial_index: Option<&dyn SpatialAccelerator>,
    point: &[f64; 3],
    direction: [f64; 3],
    epsilon: f64,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<Option<PointClassification>, KernelError> {
    let pos_sample = [
        point[0] + epsilon * direction[0],
        point[1] + epsilon * direction[1],
        point[2] + epsilon * direction[2],
    ];
    let neg_sample = [
        point[0] - epsilon * direction[0],
        point[1] - epsilon * direction[1],
        point[2] - epsilon * direction[2],
    ];

    let pos_class = classify_point_in_solid(
        arena,
        vertex_positions,
        spatial_index,
        &pos_sample,
        tolerance_provider,
    )?;
    let neg_class = classify_point_in_solid(
        arena,
        vertex_positions,
        spatial_index,
        &neg_sample,
        tolerance_provider,
    )?;

    if pos_class == neg_class {
        Ok(Some(pos_class))
    } else {
        Ok(None)
    }
}

/// Determine if the +X axis ray from `point` intersects a face using SoS.
fn sos_ray_intersects_face(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    face_id: FaceId,
    point: &[f64; 3],
) -> Result<bool, KernelError> {
    let mut verts: Vec<[f64; 3]> = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        verts.push(vertex_positions(he.origin().index())?);
    }

    if verts.len() < 3 {
        return Ok(false);
    }

    let (basis_a, basis_b, basis_c) = match find_nondegenerate_basis(&verts)? {
        Some(b) => b,
        None => return Ok(false),
    };

    let (o_nx, _) = orient2d(
        [basis_a[1], basis_a[2]],
        [basis_b[1], basis_b[2]],
        [basis_c[1], basis_c[2]],
    )
    .map_err(|e| KernelError::InternalError {
        message: e.to_string(),
        context: None,
    })?;

    let nx_sign = o_nx.sign();
    if nx_sign == TriSign::Zero {
        return Ok(false);
    }

    let n = verts.len();
    let mut winding: i32 = 0;
    for i in 0..n {
        let v0 = verts[i];
        let v1 = verts[(i + 1) % n];
        winding += sos_edge_crossing_yz(point[1], point[2], v0[1], v0[2], v1[1], v1[2])?;
    }

    if winding == 0 {
        return Ok(false);
    }

    let (o3, _) =
        orient3d(basis_a, basis_b, basis_c, *point).map_err(|e| KernelError::InternalError {
            message: e.to_string(),
            context: None,
        })?;

    let p_sign = if o3.sign() != TriSign::Zero {
        o3.sign()
    } else {
        sos_orient3d_tiebreak(basis_a, basis_b, basis_c)?
    };

    if p_sign == TriSign::Zero {
        return Ok(false);
    }

    Ok(p_sign != nx_sign)
}

/// Find the first non-collinear triplet of vertices in the list.
fn find_nondegenerate_basis(
    verts: &[[f64; 3]],
) -> Result<Option<([f64; 3], [f64; 3], [f64; 3])>, KernelError> {
    if verts.len() < 3 {
        return Ok(None);
    }

    let p0 = verts[0];
    let p1 = verts[1];

    for &pk in &verts[2..] {
        let (o, _) = orient2d([p0[1], p0[2]], [p1[1], p1[2]], [pk[1], pk[2]]).map_err(|e| {
            KernelError::InternalError {
                message: e.to_string(),
                context: None,
            }
        })?;

        if o.sign() != TriSign::Zero {
            return Ok(Some((p0, p1, pk)));
        }

        let (o_xy, _) = orient2d([p0[0], p0[1]], [p1[0], p1[1]], [pk[0], pk[1]]).map_err(|e| {
            KernelError::InternalError {
                message: e.to_string(),
                context: None,
            }
        })?;

        if o_xy.sign() != TriSign::Zero {
            return Ok(Some((p0, p1, pk)));
        }
    }

    Ok(None)
}


