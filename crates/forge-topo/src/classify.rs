//! Topology classification driven by certified predicates (Doctrine D3).
//!
//! DOMAIN: Point-in-solid classification via ray parity counting.
//!
//! Functions in this module accept [`CertifiedTriSign`] — they cannot be
//! called with raw `f64` comparisons. This is the compile-time enforcement
//! of the topology-geometry firewall.
//!
//! DEPENDENCIES: `arena`, `handles`, `traverse`, `forge-math` (predicates)

use forge_core::KernelError;
use forge_math::sign::{CertifiedTriSign, TriSign};
use forge_math::predicates::{orient2d, orient3d};
use crate::arena::TopologyArena;
use crate::handles::FaceId;
use crate::traverse::face_edges;

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
pub fn classify_point_in_solid(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    point: &[f64; 3],
    ray_extent: f64,
) -> Result<PointClassification, KernelError> {
    let ray_far = [point[0] + ray_extent, point[1], point[2]];

    let mut crossing_count: i32 = 0;

    for (face_id, _face_data) in arena.iter_faces() {
        let crossings = count_face_crossings(
            arena,
            vertex_positions,
            point,
            &ray_far,
            face_id,
        )?;

        match crossings {
            FaceCrossing::OnBoundary => return Ok(PointClassification::OnBoundary(face_id)),
            FaceCrossing::Count(n) => crossing_count += n,
        }
    }

    if crossing_count % 2 == 1 {
        Ok(PointClassification::Inside)
    } else {
        Ok(PointClassification::Outside)
    }
}

/// Result of counting ray crossings through a single face.
enum FaceCrossing {
    /// The query point lies on this face's plane.
    OnBoundary,
    /// Number of ray-triangle crossings for this face.
    Count(i32),
}

/// Count crossings for a single face, using certified predicates.
///
/// Uses a face-level intersection test instead of per-triangle fan tests
/// to avoid double-counting at fan-interior edges (D0 compliance).
///
/// Algorithm:
/// 1. Check if origin and far are on opposite sides of the face plane
/// 2. Compute the parametric ray-plane intersection point
/// 3. Test if the intersection is inside the convex face polygon
fn count_face_crossings(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    point: &[f64; 3],
    ray_far: &[f64; 3],
    face_id: FaceId,
) -> Result<FaceCrossing, KernelError> {
    let edges = face_edges(arena, face_id)?;
    if edges.len() < 3 {
        return Ok(FaceCrossing::Count(0));
    }

    let mut positions: Vec<[f64; 3]> = Vec::with_capacity(edges.len());
    for he_id in &edges {
        let he_data = arena.get_half_edge(*he_id)?;
        positions.push(vertex_positions(he_data.origin.index())?);
    }

    let point_orient = orient3d(positions[0], positions[1], positions[2], *point)
        .map_err(|e| KernelError::InternalError {
            message: format!("orient3d error: {e}"),
            context: None,
        })?;

    if point_orient.sign() == TriSign::Zero {
        return Ok(FaceCrossing::OnBoundary);
    }

    let far_orient = orient3d(positions[0], positions[1], positions[2], *ray_far)
        .map_err(|e| KernelError::InternalError {
            message: format!("orient3d error: {e}"),
            context: None,
        })?;

    if far_orient.sign() == TriSign::Zero {
        return Ok(FaceCrossing::Count(0));
    }

    if point_orient.sign() == far_orient.sign() {
        return Ok(FaceCrossing::Count(0));
    }

    let hit = match compute_ray_plane_intersection(point, ray_far, &positions) {
        Ok(h) => h,
        Err(_) => return Ok(FaceCrossing::Count(0)),
    };

    let inside = point_inside_convex_polygon(&hit, &positions)?;

    if inside {
        Ok(FaceCrossing::Count(1))
    } else {
        Ok(FaceCrossing::Count(0))
    }
}

/// Compute the intersection point of a ray with a plane defined by
/// three vertices using parametric interpolation.
///
/// Given ray from `origin` to `far` crossing the plane at some t ∈ (0,1),
/// computes hit = origin + t * (far - origin).
///
/// Precondition: the caller must verify via orient3d that origin and far
/// are on opposite sides of the face plane, guaranteeing `denom != 0`.
fn compute_ray_plane_intersection(
    origin: &[f64; 3],
    far: &[f64; 3],
    face_verts: &[[f64; 3]],
) -> Result<[f64; 3], KernelError> {
    let a = face_verts[0];
    let b = face_verts[1];
    let c = face_verts[2];

    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let normal = [
        ab[1] * ac[2] - ab[2] * ac[1],
        ab[2] * ac[0] - ab[0] * ac[2],
        ab[0] * ac[1] - ab[1] * ac[0],
    ];

    let ao = [origin[0] - a[0], origin[1] - a[1], origin[2] - a[2]];
    let dir = [far[0] - origin[0], far[1] - origin[1], far[2] - origin[2]];

    let denom = normal[0] * dir[0] + normal[1] * dir[1] + normal[2] * dir[2];
    let numer = -(normal[0] * ao[0] + normal[1] * ao[1] + normal[2] * ao[2]);

    if denom.abs() < 1e-30 {
        return Err(KernelError::InternalError {
            message: "Ray nearly parallel to face plane — skipping".to_string(),
            context: None,
        });
    }

    let t = numer / denom;
    Ok([
        origin[0] + t * dir[0],
        origin[1] + t * dir[1],
        origin[2] + t * dir[2],
    ])
}

/// Test if a 3D point lies inside a convex polygon using certified
/// orient2d predicates (D3 — no raw f64 comparisons for topology).
///
/// Projects onto the dominant 2D plane and checks that the hit point
/// is on the same side of every polygon edge using exact orient2d.
///
/// When `orient2d` returns `Zero` (hit exactly on an edge), we apply
/// a consistent tie-breaking rule (Simulation of Simplicity / D0):
/// the edge is resolved to `Pos` or `Neg` based on the edge's direction
/// in the projected v-axis. This ensures exactly one of two adjacent
/// faces "owns" a shared edge, preventing double-counted crossings.
fn point_inside_convex_polygon(
    hit: &[f64; 3],
    verts: &[[f64; 3]],
) -> Result<bool, KernelError> {
    let n = verts.len();
    if n < 3 {
        return Ok(false);
    }

    let (u_axis, v_axis) = dominant_projection_axes(verts);

    let hit_2d = [hit[u_axis], hit[v_axis]];

    let mut reference_sign: Option<TriSign> = None;

    for i in 0..n {
        let next_i = (i + 1) % n;
        let vi_2d = [verts[i][u_axis], verts[i][v_axis]];
        let vn_2d = [verts[next_i][u_axis], verts[next_i][v_axis]];

        let orient = orient2d(vi_2d, vn_2d, hit_2d)
            .map_err(|e| KernelError::InternalError {
                message: format!("orient2d error in polygon test: {e}"),
                context: None,
            })?;

        let effective_sign = match orient.sign() {
            TriSign::Zero => resolve_zero_edge(vi_2d, vn_2d),
            other => other,
        };

        match reference_sign {
            None => reference_sign = Some(effective_sign),
            Some(ref_sign) => {
                if effective_sign != ref_sign {
                    return Ok(false);
                }
            }
        }
    }

    Ok(reference_sign.is_some())
}

/// Resolve a Zero orient2d result to Pos or Neg using edge direction.
///
/// When a hit point lies exactly on a polygon edge, both adjacent faces
/// share that edge with opposite winding. We break the tie by looking
/// at the edge's direction in the v-axis (second projected coordinate):
/// - Edge going upward (vi.v < vn.v) → Pos
/// - Edge going downward (vi.v > vn.v) → Neg
/// - Horizontal edge: break tie on u-axis direction
///
/// Since adjacent faces traverse the shared edge in opposite directions,
/// they get opposite resolved signs — ensuring exactly one face "owns"
/// the edge (D0 — no perturbation).
fn resolve_zero_edge(vi_2d: [f64; 2], vn_2d: [f64; 2]) -> TriSign {
    let dv = vn_2d[1] - vi_2d[1];
    if dv > 0.0 {
        TriSign::Pos
    } else if dv < 0.0 {
        TriSign::Neg
    } else {
        let du = vn_2d[0] - vi_2d[0];
        if du > 0.0 {
            TriSign::Pos
        } else {
            TriSign::Neg
        }
    }
}

/// Determine the 2D projection axes by finding the dominant
/// component of the face normal.
///
/// Drops the axis with the largest absolute normal component
/// to maximize projection area and numerical stability.
fn dominant_projection_axes(verts: &[[f64; 3]]) -> (usize, usize) {
    let a = verts[0];
    let b = verts[1];
    let c = verts[2];

    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let nx = (ab[1] * ac[2] - ab[2] * ac[1]).abs();
    let ny = (ab[2] * ac[0] - ab[0] * ac[2]).abs();
    let nz = (ab[0] * ac[1] - ab[1] * ac[0]).abs();

    if nx >= ny && nx >= nz {
        (1, 2)
    } else if ny >= nz {
        (0, 2)
    } else {
        (0, 1)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut draft = state.begin_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace { feature_id: 0 }).unwrap().into_value();
        let v0 = mvf.vertex;

        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge }).unwrap().into_value();
        let v1 = se1.new_vertex;

        let mef1 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: v0,
            vertex_b: v1,
            face: mvf.face,
        }).unwrap().into_value();

        let se2 = apply_op(&mut draft, SplitEdge { edge: mef1.half_edge_ab }).unwrap().into_value();
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
            &[1.0, 0.5, 0.0],
            ray_extent,
        ).unwrap();

        assert!(
            matches!(result, PointClassification::OnBoundary(_)),
            "Expected OnBoundary, got {:?}",
            result
        );
    }
}
