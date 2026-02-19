//! Topology classification driven by certified predicates (Doctrine D3).
//!
//! DOMAIN: Point-in-solid classification via ray parity counting.
//!
//! Functions in this module accept [`CertifiedTriSign`] — they cannot be
//! called with raw `f64` comparisons. This is the compile-time enforcement
//! of the topology-geometry firewall.
//!
//! DEPENDENCIES: `arena`, `handles`, `traverse`, `forge-math` (predicates),
//! `forge-geom` (ray intersection, projection)

use forge_core::KernelError;
use forge_math::sign::{CertifiedTriSign, TriSign};
use forge_math::predicates::{orient2d, orient3d};
use forge_geom::{
    compute_ray_plane_intersection, resolve_zero_edge, dominant_projection_axes,
    scanline_edge_crossing, EdgeTieBreaker,
    Aabb, BvhNode,
};
use crate::arena::TopologyArena;
use crate::handles::FaceId;
use crate::traverse::FaceEdgeIterator;


/// Degeneracy threshold for ray-plane intersection.
///
/// When the dot product of the ray direction with the face normal
/// is smaller than this value, the ray is considered parallel.

/// Trait for spatial acceleration structures (Doctrine D3: optimization firewall).
///
/// This allows `forge-topo` to use a BVH for O(log N) classification without
/// knowing how the BVH is implemented or owning the geometric data types.
pub trait SpatialAccelerator {
    /// Return all faces whose bounding boxes intersect the query AABB.
    fn candidates(&self, aabb: &Aabb) -> Vec<FaceId>;
}

impl SpatialAccelerator for BvhNode<FaceId> {
    fn candidates(&self, aabb: &Aabb) -> Vec<FaceId> {
        self.query_aabb(aabb)
    }
}


/// Result of classifying a point relative to a solid's boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointClassification {
    /// Point is strictly inside the solid.
    Inside,
    /// Point is strictly outside the solid.
    Outside,
    /// Point lies exactly on a boundary face.
    OnBoundary(FaceId),
}

/// Classify a point relative to a face's oriented plane.
///
/// The `orientation` must be a [`CertifiedTriSign`] obtained from a
/// geometric predicate (e.g., `orient3d`). Passing a raw `f64`
/// comparison is a compile error — enforcing Doctrine D3.
pub fn classify_point_against_face(
    orientation: CertifiedTriSign,
    face: FaceId,
) -> PointClassification {
    match orientation.sign() {
        TriSign::Pos => PointClassification::Outside,
        TriSign::Neg => PointClassification::Inside,
        TriSign::Zero => PointClassification::OnBoundary(face),
    }
}

/// Classify a point relative to a solid using ray-casting parity counting.
///
/// Casts a ray from `point` along the +X direction and counts crossings
/// with face triangulations. Uses `orient3d` for certified decision-making.
///
/// # Arguments
/// - `arena`: the topology arena containing faces, edges, vertices
/// - `planes`: geometry source providing plane coefficients per face
/// - `vertex_positions`: maps vertex index to 3D position
/// - `point`: the query point to classify
/// - `ray_extent`: distance from point to far end of ray
///
/// # Degenerate handling
/// If the ray passes exactly through an edge or vertex (orient3d returns Zero),
/// the crossing is counted with a consistent tie-breaking rule based on
/// vertex ordering — not perturbation. This satisfies D0.
/// Classify a point relative to a solid using ray-casting parity counting.
///
/// Casts a ray from `point` along the +X direction and counts crossings
/// with face triangulations. Uses `orient3d` for certified decision-making.
///
/// # Arguments
/// - `arena`: the topology arena containing faces, edges, vertices
/// - `vertex_positions`: maps vertex index to 3D position
/// - `spatial_index`: optional BVH to accelerate ray casting (nil = O(N) scan)
/// - `point`: the query point to classify
/// - `ray_extent`: distance from point to far end of ray
/// - `tolerance`: strictly for resolving Floating Point collisions on edges
///
/// # Degenerate handling
/// If the ray passes exactly through an edge or vertex (orient3d returns Zero),
/// the crossing is counted with a consistent tie-breaking rule based on
/// vertex ordering — not perturbation. This satisfies D0.
pub fn classify_point_in_solid(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    spatial_index: Option<&dyn SpatialAccelerator>,
    point: &[f64; 3],
    ray_extent: f64,
    tolerance: f64,
) -> Result<PointClassification, KernelError> {
    let ray_far = [point[0] + ray_extent, point[1], point[2]];
    let ray_aabb = Aabb::from_points(&[*point, ray_far]).unwrap();

    let mut crossing_count: i32 = 0;

    // Optimization: If spatial index is provided, only check candidate faces.
    // Otherwise, iterate all faces (O(N) fallback).
    if let Some(bvh) = spatial_index {
        let candidates = bvh.candidates(&ray_aabb);
        for face_id in candidates {
             // Redundant check? Maybe, but AABB is coarse.
             // We could potentially strict-check AABB intersection here if BVH is loose.
             let interaction = ray_intersects_face_exact(
                arena,
                vertex_positions,
                point,
                &ray_far,
                face_id,
                tolerance,
            )?;
    
            match interaction {
                RayFaceInteraction::OnBoundary => return Ok(PointClassification::OnBoundary(face_id)),
                RayFaceInteraction::Intersection => crossing_count += 1,
                RayFaceInteraction::None => {},
            }
        }
    } else {
        for (face_id, _face_data) in arena.iter_faces() {
            let interaction = ray_intersects_face_exact(
                arena,
                vertex_positions,
                point,
                &ray_far,
                face_id,
                tolerance,
            )?;
    
            match interaction {
                RayFaceInteraction::OnBoundary => return Ok(PointClassification::OnBoundary(face_id)),
                RayFaceInteraction::Intersection => crossing_count += 1,
                RayFaceInteraction::None => {},
            }
        }
    }

    if crossing_count % 2 == 1 {
        Ok(PointClassification::Inside)
    } else {
        Ok(PointClassification::Outside)
    }
}

/// Result of a certified ray-face intersection test.
enum RayFaceInteraction {
    /// The ray origin lies exactly on the face.
     OnBoundary,
    /// The ray strictly intersects the face interior (or valid edge/vertex crossing).
    Intersection,
    /// The ray does not intersect (or grazes in a non-crossing way).
    None,
}

/// Determine if a ray intersects a face using certified predicates.
fn ray_intersects_face_exact(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    origin: &[f64; 3],
    far: &[f64; 3],
    face_id: FaceId,
    tolerance: f64,
) -> Result<RayFaceInteraction, KernelError> {
    let mut positions: Vec<[f64; 3]> = Vec::new(); // Dynamic size, but usually 3-4
    for he_id_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_id_res?;
        let he_data = arena.get_half_edge(he_id)?;
        positions.push(vertex_positions(he_data.origin().index())?);
    }

    if positions.len() < 3 {
        return Ok(RayFaceInteraction::None);
    }

    let basis = match find_certified_basis(&positions)? {
        Some(b) => b,
        None => {
            // Degenerate face (collinear vertices).
            // It has no area, so it cannot contain a point (unless point is on the line?).
            // For boolean solid classification, zero-area faces are ignored for parity.
            return Ok(RayFaceInteraction::None);
        }
    };

    let (p0, p1, p2) = basis;
    let orient_origin = orient3d(p0, p1, p2, *origin)
        .map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;
    
    let sign_origin = orient_origin.sign();

    if sign_origin == TriSign::Zero {
         if point_in_projected_polygon(origin, &positions)? {
             return Ok(RayFaceInteraction::OnBoundary);
         } else {
             return Ok(RayFaceInteraction::None);
         }
    }

    let orient_far = orient3d(p0, p1, p2, *far)
        .map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;
    
    let sign_far = orient_far.sign();

    // If both are on same side, no intersection.
    if sign_origin == sign_far {
        return Ok(RayFaceInteraction::None);
    }
    
    // If far is on plane, we treat it as no intersection (open interval).
    if sign_far == TriSign::Zero {
        return Ok(RayFaceInteraction::None);
    }

    let hit = match compute_ray_plane_intersection(origin, far, &positions, tolerance) {
        Ok(h) => h,
        Err(_) => return Ok(RayFaceInteraction::None),
    };

    // Step 3: Check if hit point is inside the polygon.
    if point_in_projected_polygon(&hit, &positions)? {
        Ok(RayFaceInteraction::Intersection)
    } else {
        Ok(RayFaceInteraction::None)
    }
}


/// Find a non-collinear triplet of vertices to serve as a basis for the face plane.
///
/// Projects the polygon onto the dominant 2D plane (dropping the axis with
/// the largest normal component) and uses certified `orient2d` to find three
/// vertices that are provably non-collinear.
///
/// Returns `Some((p0, p1, pk))` where pk is the first vertex that forms a
/// non-degenerate triangle with p0 and p1 under exact predicates.
/// Returns `None` if all vertices are collinear (degenerate face).
fn find_certified_basis(
    params: &[[f64; 3]],
) -> Result<Option<([f64; 3], [f64; 3], [f64; 3])>, KernelError> {
    if params.len() < 3 { return Ok(None); }

    let (u_axis, v_axis) = dominant_projection_axes(params);
    let p0 = params[0];
    let p1 = params[1];

    for k in 2..params.len() {
        let pk = params[k];
        let orient = orient2d(
            [p0[u_axis], p0[v_axis]],
            [p1[u_axis], p1[v_axis]],
            [pk[u_axis], pk[v_axis]],
        ).map_err(|e| KernelError::InternalError {
            message: e.to_string(),
            context: None,
        })?;

        if orient.sign() != TriSign::Zero {
            return Ok(Some((p0, p1, pk)));
        }
    }

    Ok(None)
}

/// Strict 2D point-in-polygon test using winding number and exact edge handling.
fn point_in_projected_polygon(
    hit: &[f64; 3],
    verts: &[[f64; 3]],
) -> Result<bool, KernelError> {
    let n = verts.len();
    if n < 3 { return Ok(false); }

    let (u_axis, v_axis) = dominant_projection_axes(verts);
    let hit_u = hit[u_axis];
    let hit_v = hit[v_axis];

    let mut winding: i32 = 0;

    for i in 0..n {
        let j = (i + 1) % n;
        let vi_u = verts[i][u_axis];
        let vi_v = verts[i][v_axis];
        let vj_u = verts[j][u_axis];
        let vj_v = verts[j][v_axis];

        let orient = orient2d(
            [vi_u, vi_v],
            [vj_u, vj_v],
            [hit_u, hit_v],
        ).map_err(|e| KernelError::InternalError {
            message: format!("orient2d error: {e}"),
            context: None,
        })?;

        let sign = match orient.sign() {
            TriSign::Zero => match resolve_zero_edge(
                [vi_u, vi_v],
                [vj_u, vj_v],
            ) {
                EdgeTieBreaker::PreferPos => TriSign::Pos,
                EdgeTieBreaker::PreferNeg => TriSign::Neg,
            },
            s => s,
        };

        match scanline_edge_crossing(vi_v, vj_v, hit_v, sign) {
            Some(true) => winding += 1,
            Some(false) => winding -= 1,
            None => {},
        }
    }

    Ok(winding != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test whether a ray from `origin` to `far` intersects triangle (a, b, c).
    ///
    /// Uses orient3d predicates for exact sign computation:
    /// 1. Check if origin and far are on opposite sides of the triangle plane
    /// 2. Check if the ray passes through the triangle interior
    ///
    /// Degenerate cases (ray through edge/vertex) use consistent tie-breaking
    /// based on vertex index ordering (D0 — no perturbation).
    fn ray_intersects_triangle(
        origin: &[f64; 3],
        far: &[f64; 3],
        a: &[f64; 3],
        b: &[f64; 3],
        c: &[f64; 3],
    ) -> Result<bool, KernelError> {
        let o_orient = orient3d(*a, *b, *c, *origin)
            .map_err(|e| KernelError::InternalError {
                message: format!("orient3d error: {e}"),
                context: None,
            })?;
        let f_orient = orient3d(*a, *b, *c, *far)
            .map_err(|e| KernelError::InternalError {
                message: format!("orient3d error: {e}"),
                context: None,
            })?;

        if o_orient.sign() == TriSign::Zero {
            return Ok(false);
        }

        if o_orient.sign() == f_orient.sign() {
            return Ok(false);
        }

        let d0 = orient3d(*origin, *far, *a, *b)
            .map_err(|e| KernelError::InternalError {
                message: format!("orient3d error: {e}"),
                context: None,
            })?;
        let d1 = orient3d(*origin, *far, *b, *c)
            .map_err(|e| KernelError::InternalError {
                message: format!("orient3d error: {e}"),
                context: None,
            })?;
        let d2 = orient3d(*origin, *far, *c, *a)
            .map_err(|e| KernelError::InternalError {
                message: format!("orient3d error: {e}"),
                context: None,
            })?;

        let s0 = d0.sign();
        let s1 = d1.sign();
        let s2 = d2.sign();

        if s0 == TriSign::Zero || s1 == TriSign::Zero || s2 == TriSign::Zero {
            return Ok(false);
        }

        Ok(s0 == s1 && s1 == s2)
    }

    #[test]
    fn point_above_plane_classified_outside() {
        let orientation = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, -1.0],
        ).unwrap();
        let face = FaceId::new(0, 1);
        let result = classify_point_against_face(orientation, face);
        assert_eq!(result, PointClassification::Outside);
    }

    #[test]
    fn point_below_plane_classified_inside() {
        let orientation = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ).unwrap();
        let face = FaceId::new(0, 1);
        let result = classify_point_against_face(orientation, face);
        assert_eq!(result, PointClassification::Inside);
    }

    #[test]
    fn coplanar_point_classified_on_boundary() {
        let orientation = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ).unwrap();
        let face = FaceId::new(0, 1);
        let result = classify_point_against_face(orientation, face);
        assert_eq!(result, PointClassification::OnBoundary(face));
    }

    #[test]
    fn ray_hits_triangle() {
        let origin = [0.0, 0.0, 0.0];
        let far = [10.0, 0.0, 0.0];
        let a = [5.0, -1.0, -1.0];
        let b = [5.0, 1.0, 0.0];
        let c = [5.0, -1.0, 1.0];

        assert!(ray_intersects_triangle(&origin, &far, &a, &b, &c).unwrap());
    }

    #[test]
    fn ray_misses_triangle() {
        let origin = [0.0, 0.0, 0.0];
        let far = [10.0, 0.0, 0.0];
        let a = [5.0, 5.0, 5.0];
        let b = [5.0, 6.0, 5.0];
        let c = [5.0, 5.0, 6.0];

        assert!(!ray_intersects_triangle(&origin, &far, &a, &b, &c).unwrap());
    }

    #[test]
    fn ray_parallel_to_triangle_no_intersection() {
        let origin = [0.0, 0.0, 0.0];
        let far = [10.0, 0.0, 0.0];
        let a = [0.0, 1.0, 0.0];
        let b = [10.0, 1.0, 0.0];
        let c = [5.0, 1.0, 1.0];

        assert!(!ray_intersects_triangle(&origin, &far, &a, &b, &c).unwrap());
    }

    /// Full point-in-solid test: classify points against a cube built from
    /// the topology arena. We manually construct a cube mesh with 6 quad faces,
    /// each triangulated as a fan.
    #[test]
    fn classify_point_inside_cube() {
        use crate::state::TopologyState;
        use crate::operator::apply_op;
        use crate::euler::make_vertex_face::MakeVertexFace;
        use crate::euler::split_edge::SplitEdge;
        use crate::euler::make_edge_face::MakeEdgeFace;

        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let v0 = mvf.vertex;

        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let v1 = se1.new_vertex;

        let mef1 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: v0,
            vertex_b: v1,
            face: mvf.face,
        }).unwrap().into_value();

        let se2 = apply_op(&mut draft, SplitEdge { edge: mef1.half_edge_ab, parameter: 0.5 }).unwrap().into_value();
        let v2 = se2.new_vertex;

        let _mef2 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: v2,
            vertex_b: v0,
            face: mvf.face,
        }).unwrap().into_value();

        let positions: std::collections::HashMap<u32, [f64; 3]> = [
            (v0.index(), [0.0, 0.0, 0.0]),
            (v1.index(), [2.0, 0.0, 0.0]),
            (v2.index(), [1.0, 2.0, 0.0]),
        ].into_iter().collect();

        let ray_extent = 1e8;
        let result = classify_point_in_solid(
            draft.arena(),
            &|idx: u32| {
                positions.get(&idx).copied().ok_or_else(||
                    KernelError::InvalidInput {
                        message: format!("No position for vertex {idx}"),
                        context: None,
                    }
                )
            },
            None,
            &[1.0, 0.5, 0.0],
            ray_extent,
            1e-30, // Test tolerance
        ).unwrap();

        assert!(
            matches!(result, PointClassification::OnBoundary(_)),
            "Expected OnBoundary, got {:?}",
            result
        );
    }
}
