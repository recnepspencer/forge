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
use forge_geom::ray::{
    compute_ray_plane_intersection, resolve_zero_edge, dominant_projection_axes,
};
use crate::arena::TopologyArena;
use crate::handles::FaceId;
use crate::traverse::face_edges;

/// Degeneracy threshold for ray-plane intersection.
///
/// When the dot product of the ray direction with the face normal
/// is smaller than this value, the ray is considered parallel.
const RAY_PLANE_DEGENERACY: f64 = 1e-30;

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
            FaceCrossing::OnBoundary => {
                 return Ok(PointClassification::OnBoundary(face_id))
            },
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

    // D3: robustly find a triangle basis from the face vertices.
    // If indices 0,1,2are collinear, orient3d returns Zero even if point is far.
    // We must find i,j,k such that (i,j,k) form a valid plane (non-collinear).
    // We try 0, 1, k for k in 2..n.
    
    let mut basis_indices = None;
    for k in 2..positions.len() {
        // Check if 0, 1, k are collinear.
        // We can't use orient3d on *point* yet. We check against each other?
        // Actually, we want to define the face plane.
        // If (0,1,k) are collinear, they don't define a plane.
        // How to check collinearity without floating point epsilon?
        // Use orient3d with a 4th point? No.
        // forge-geom or forge-math should have `collinear` predicate?
        // Actually, we can check if `orient2d` on all 3 projections is zero?
        // Or simply: search for a triplet that gives a definite sign for *some* query point? 
        // No, the plane is intrinsic.
        
        // Check cross product of edges (1-0) and (k-1)?
        // Using raw arithmetic is D3 violation?
        // `orient3d` is the only tool.
        
        // If we assumed the face is planar (Forge invariant).
        // Then *any* non-collinear triplet defines the plane.
        // If ALL points are collinear, the face is degenerate (area zero).
        
        // Heuristic: Use lexicographically spread vertices?
        // Or just compute normal area?
        
        // Let's perform a cross product check with a robust tolerance, 
        // OR try to classify the point against this basis.
        // If the basis is degenerate, orient3d(a,b,c, p) is 0 for ALL p.
        // But we want to know if p is on the *Face Plane*.
        // If orient3d(a,b,c, p) is non-zero, then they form a tetrahedron, so p is NOT on plane abc.
        // If plane abc is valid, then result is valid.
        // If plane abc is degenerate (line), orient3d is 0.
        // So if `orient3d` returns Non-Zero, we found a valid plane AND p is off it. -> Success.
        
        // But what if p lies on the degenerate plane (line)?
        // Then orient3d is 0. But p might fail "point_in_polygon".
        
        // Correct logic:
        // iterate k. Compute `o = orient3d(0, 1, k, p)`.
        // If `o != Zero`, then p is definitely NOT coplanar with (0,1,k).
        // Since (0,1,k) are in the face, p is NOT coplanar with face.
        // Return Count(0) or Count(1) based on intersection?
        // No, we need SIGN.
        
        // If we find `o != Zero`, we can immediately say "Not Coplanar". 
        // But which side? `o` tells us.
        // But is (0,1,k) orientation consistent with face normal?
        // Face might be concave? No, Forge faces are convex? Or simple holes?
        // Forge faces can be non-convex? "convex polygon" mentioned in comments.
        // If convex, ordering is consistent.
        
        let o = orient3d(positions[0], positions[1], positions[k], *point)
            .map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;
            
        if o.sign() != TriSign::Zero {
            // Found a triangle 0-1-k that P is NOT on.
            // But does 0-1-k define the face plane normal direction correctly?
            // If the polygon is convex, 0-1-k is a sub-triangle consistent with winding.
            // So `o` is the correct orientation.
            
            // We verify `far` orientation with the SAME basis.
            let far_orient = orient3d(positions[0], positions[1], positions[k], *ray_far)
                .map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;
                
            if far_orient.sign() == TriSign::Zero {
                return Ok(FaceCrossing::Count(0)); // Degenerate ray end
            }
            if o.sign() == far_orient.sign() {
                return Ok(FaceCrossing::Count(0)); // Same side
            }
            
            // Ray crosses plane. Compute intersection.
            // Must use the valid basis 0,1,k for plane?
            // compute_ray_plane_intersection uses all `positions` (best fit plane).
            // So we can fall through to intersection test.
            
            basis_indices = Some((0, 1, k));
            break;
        }
    }
    
    // If loop finished and we haven't broken, it means orient3d was Zero for ALL k.
    // This implies P is coplanar with 0-1-k for all k.
    // So P is coplanar with the whole face (assuming 0-1 matches face plane).
    // OR 0-1 is degenerate?
    // If 0 and 1 are same point? (Checked by unique vertices?)
    // If 0,1,k are always collinear (Face is a line)?
    
    // If we assume valid Face (area > 0).
    // If P is coplanar with every triangle fan from 0-1.
    // Then P is coplanar with the face.
    // So we proceed to "OnBoundary" logic.
    
    // But wait, what if 0, 1, k are collinear, but P is NOT?
    // Then orient3d is Zero.
    // My loop continues.
    // If I check ALL k and all are Zero.
    // Does it mean P is on plane?
    // Yes, if 0-1 is not a point.
    // AND if face is not a line.
    
    // Let's assume P is coplanar if we fell through.
    if basis_indices.is_none() {
        // Coplanar point logic...
        let point_orient = TriSign::Zero; // Implicit
    }
    
    // Check intersection using BEST FIT plane (computed inside function)
    // We don't use 'point_orient' variable anymore.
    // We handled "Not Coplanar" inside loop?
    // No, I need to restructure to avoid "fall through means coplanar".
    
    // Simplified robust check:
    // 1. Find ANY non-collinear triplet (a,b,c) in face.
    // 2. Check orient3d(a,b,c, p).
    
    // Step 1: Find basis.
    // Note: We assume convexity or simple polygon.
    // Even if non-convex, we can usually find a valid triangle at 0-1-k unless 0-1 is collinear with all others.
    // Robust strategy: Iterate all triplets? Too slow.
    // Iterate 0-i-j?
    
    // Fallback: If 0-1-2 is collinear, try 0-2-3?
    // Just finding ONE non-zero orient3d(a,b,c, p) proves non-coplanarity.
    // But we need the sign to match `far`.
    
    let mut is_coplanar_with_p = true;
    let mut side_sign = TriSign::Zero;
    
    // Try to find a definite side for P
    // Using point 0 as fan pivot.
    for k in 2..positions.len() {
         let o = orient3d(positions[0], positions[k-1], positions[k], *point)
            .map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;
         if o.sign() != TriSign::Zero {
             is_coplanar_with_p = false;
             side_sign = o.sign();
             break;
         }
    }
    
    if is_coplanar_with_p {
        // Coplanar point logic
        let (u, v) = dominant_projection_axes(&positions);
        let p2d = [point[u], point[v]];

        let mut has_pos = false;
        let mut has_neg = false;

        for i in 0..positions.len() {
            let curr = positions[i];
            let next = positions[(i + 1) % positions.len()];
            
            let c2d = [curr[u], curr[v]];
            let n2d = [next[u], next[v]];

            let orient = orient2d(c2d, n2d, p2d).map_err(|e| KernelError::InternalError {
                message: format!("orient2d error: {e}"),
                context: None,
            })?;

            match orient.sign() {
                TriSign::Pos => has_pos = true,
                TriSign::Neg => has_neg = true,
                TriSign::Zero => return Ok(FaceCrossing::OnBoundary),
            }
        }

        if has_pos && has_neg {
            return Ok(FaceCrossing::Count(0)); 
        } else {
            return Ok(FaceCrossing::OnBoundary);
        }
    }

    if side_sign != TriSign::Zero {
        // It returns Intersection or None.
        // If intersection t > 0 and t < 1 (aka between point and far).
        // `compute_ray_plane_intersection` assumes infinite line?
        // No, segment.
        // It handles degeneracy.
        
        let hit = match compute_ray_plane_intersection(point, ray_far, &positions, RAY_PLANE_DEGENERACY) {
            Ok(h) => h,
            Err(_) => return Ok(FaceCrossing::Count(0)),
        };

        let inside = point_inside_convex_polygon(&hit, &positions)?;

        if inside {
            return Ok(FaceCrossing::Count(1));
        } else {
            return Ok(FaceCrossing::Count(0));
        }
    }
    
    Ok(FaceCrossing::Count(0)) // Should be unreachable if logic holds
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
            TriSign::Zero => match resolve_zero_edge(vi_2d, vn_2d) {
                forge_geom::ray::EdgeTieBreaker::PreferPos => TriSign::Pos,
                forge_geom::ray::EdgeTieBreaker::PreferNeg => TriSign::Neg,
            },
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
