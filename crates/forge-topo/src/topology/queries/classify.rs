//! Topology classification driven by Simulation of Simplicity (Edelsbrunner–Mücke §4).
//!
//! DOMAIN: Point-in-solid classification via ray parity counting along the +X axis.
//!
//! ALGORITHM: Plücker + YZ-projection winding number with proper SoS tie-breaking.
//!
//! The query point P is mathematically perturbed by P(ε) = (P_x+ε³, P_y+ε¹, P_z+ε²).
//! Because ε is infinitesimal no floating-point arithmetic is done with it — if the
//! base orient2d/orient3d result is exactly zero we read the sign off the ε-polynomial
//! coefficients. This guarantees a consistent, non-zero answer for every degenerate
//! configuration without touching the mesh geometry.
//!
//! Two-pass structure:
//!  1. Tolerance boundary check — calls `classify_point_on_face` per face so that a
//!     point physically on the surface still returns `OnBoundary` (SoS pushes it off).
//!  2. SoS parity count — for each face compute a 2D winding number in the YZ plane
//!     and, if non-zero, a 3D depth check via `sos_orient3d`. Odd count → `Inside`.
//!
//! DEPENDENCIES: `arena`, `handles`, `traverse`, `forge-math` (predicates),
//! `forge-geom` (Aabb, BvhNode)

use forge_core::KernelError;
use forge_math::sign::TriSign;
use forge_math::predicates::{orient2d, orient3d};
use forge_geom::{Aabb, BvhNode};
use crate::arena::TopologyArena;
use crate::handles::FaceId;
use crate::traverse::FaceEdgeIterator;

// ── SpatialAccelerator ────────────────────────────────────────────────────────

/// Trait for spatial acceleration structures (Doctrine D3: optimization firewall).
pub trait SpatialAccelerator {
    /// Return all faces whose bounding boxes intersect the query AABB.
    fn candidates(&self, aabb: &Aabb) -> Vec<FaceId>;
}

impl SpatialAccelerator for BvhNode<FaceId> {
    fn candidates(&self, aabb: &Aabb) -> Vec<FaceId> {
        self.query_aabb(aabb)
    }
}

// ── Public result types ───────────────────────────────────────────────────────

/// Result of classifying a point relative to a solid's boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum PointClassification {
    /// Point is strictly inside the solid.
    Inside {
        /// Optional precision escalation that occurred during classification.
        escalation: Option<forge_math::arithmetic::precision::PrecisionEscalation>,
    },
    /// Point is strictly outside the solid.
    Outside {
        /// Optional precision escalation that occurred during classification.
        escalation: Option<forge_math::arithmetic::precision::PrecisionEscalation>,
    },
    /// Point lies exactly on a boundary face.
    OnBoundary(FaceId),
}

/// Classify a point relative to a face's oriented plane.
///
/// The `orientation` must be a [`forge_math::sign::CertifiedTriSign`] obtained from a
/// geometric predicate (e.g., `orient3d`). Passing a raw `f64`
/// comparison is a compile error — enforcing Doctrine D3.
pub fn classify_point_against_face(
    orientation: forge_math::sign::CertifiedTriSign,
    escalation: Option<forge_math::arithmetic::precision::PrecisionEscalation>,
    face: FaceId,
) -> PointClassification {
    match orientation.sign() {
        TriSign::Pos => PointClassification::Outside { escalation },
        TriSign::Neg => PointClassification::Inside { escalation },
        TriSign::Zero => PointClassification::OnBoundary(face),
    }
}

// ── Main classifier ───────────────────────────────────────────────────────────

/// Classify a point relative to a solid using ray-casting parity counting.
///
/// Casts a conceptually infinite ray from `point` along the +X axis and
/// counts face crossings using the Plücker + YZ-projection method with
/// proper Simulation of Simplicity tie-breaking.
///
/// # Arguments
/// - `arena`: the topology arena containing faces, edges, vertices
/// - `vertex_positions`: maps vertex slot index → 3D position
/// - `spatial_index`: optional BVH to accelerate candidate selection  
/// - `point`: the 3D query point
/// - `tolerance`: linear tolerance for the boundary pre-check
///
/// # Note on `OnBoundary`
/// A point physically on a face surface is detected in **Pass 1** using the
/// tolerance-based check. SoS (Pass 2) intentionally perturbs the point
/// off boundaries, so the pre-check is essential.
pub fn classify_point_in_solid(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    spatial_index: Option<&dyn SpatialAccelerator>,
    point: &[f64; 3],
    tolerance: f64,
) -> Result<PointClassification, KernelError> {
    // ── Pass 1: Tolerance-based boundary check ────────────────────────────────
    // SoS intentionally pushes P off boundaries. We must check physical
    // proximity first so `OnBoundary` is still returnable.
    let pos_fn = |v: crate::handles::VertexId| vertex_positions(v.index()).ok();

    let faces_to_check: Vec<FaceId> = if let Some(bvh) = spatial_index {
        // Use a point-AABB: just the query point itself.
        let pt_aabb = Aabb::from_points(&[*point, *point]).unwrap();
        bvh.candidates(&pt_aabb)
    } else {
        arena.iter_faces().map(|(id, _)| id).collect()
    };

    for &face_id in &faces_to_check {
        match classify_point_on_face(arena, face_id, point, &pos_fn, tolerance)? {
            FacePointClassification::Outside => {}
            _ => return Ok(PointClassification::OnBoundary(face_id)),
        }
    }

    // ── Pass 2: SoS parity counting ──────────────────────────────────────────
    let mut crossing_count: i32 = 0;

    // For the parity count we need ALL faces, not just those near P (the ray
    // is infinite). If we have a BVH we extend the query to also cover faces
    // along the entire +X half-space starting from P.
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

// ── SoS predicates ───────────────────────────────────────────────────────────

/// Orient2d tie-breaker for P(ε) = (P_y + ε¹, P_z + ε²) in the YZ plane.
///
/// Called only when `orient2d(a, b, p_yz)` is exactly zero.
/// `a` and `b` are YZ coordinates `[y, z]` of the edge endpoints.
fn sos_orient2d_tiebreak(a: [f64; 2], b: [f64; 2]) -> TriSign {
    // ε¹ coefficient comes from the P_y perturbation: a[1] − b[1]  (the Z values).
    let delta1 = a[1] - b[1];
    if delta1 > 0.0 { return TriSign::Pos; }
    if delta1 < 0.0 { return TriSign::Neg; }

    // ε² coefficient comes from the P_z perturbation: b[0] − a[0]  (the Y values).
    let delta2 = b[0] - a[0];
    if delta2 > 0.0 { return TriSign::Pos; }
    if delta2 < 0.0 { return TriSign::Neg; }

    TriSign::Zero // Only when A == B (degenerate edge — caller skips this face).
}

/// Full orient3d tie-breaker for P(ε) = (P_x + ε³, P_y + ε¹, P_z + ε²).
///
/// Called only when `orient3d(a, b, c, p)` is exactly zero.
fn sos_orient3d_tiebreak(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Result<TriSign, KernelError> {
    // ε¹ term (P_y perturbation) → orient2d of XZ projection (unchanged sign).
    let (o_xz, _) = orient2d(
        [a[0], a[2]], [b[0], b[2]], [c[0], c[2]],
    ).map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;
    if o_xz.sign() != TriSign::Zero { return Ok(o_xz.sign()); }

    // ε² term (P_z perturbation) → **negated** orient2d of XY projection.
    let (o_xy, _) = orient2d(
        [a[0], a[1]], [b[0], b[1]], [c[0], c[1]],
    ).map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;
    if o_xy.sign() != TriSign::Zero {
        return Ok(match o_xy.sign() {
            TriSign::Pos => TriSign::Neg,
            TriSign::Neg => TriSign::Pos,
            TriSign::Zero => TriSign::Zero,
        });
    }

    // ε³ term (P_x perturbation) → **negated** orient2d of YZ projection.
    let (o_yz, _) = orient2d(
        [a[1], a[2]], [b[1], b[2]], [c[1], c[2]],
    ).map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;
    Ok(match o_yz.sign() {
        TriSign::Pos => TriSign::Neg,
        TriSign::Neg => TriSign::Pos,
        TriSign::Zero => TriSign::Zero, // A, B, C collinear — degenerate face.
    })
}

// ── Winding number component ──────────────────────────────────────────────────

/// Winding-number contribution of a single YZ-plane edge for query point (py, pz).
///
/// Uses the SoS perturbation P_z + ε² so that `az == pz` is treated as
/// A being strictly above the scanline. Returns +1, −1, or 0.
fn sos_edge_crossing_yz(
    py: f64, pz: f64,
    ay: f64, az: f64,
    by: f64, bz: f64,
) -> Result<i32, KernelError> {
    // SoS ε² perturbation on P_z: treat az == pz as A being strictly above.
    let a_above = az > pz;
    let b_above = bz > pz;

    // Both sides of the scanline — no crossing.
    if a_above == b_above {
        return Ok(0);
    }

    // Get the orient2d sign; fall back to SoS tie-breaker if zero.
    let (raw_orient, _) = orient2d([ay, az], [by, bz], [py, pz])
        .map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;

    let sign = if raw_orient.sign() != TriSign::Zero {
        raw_orient.sign()
    } else {
        sos_orient2d_tiebreak([ay, az], [by, bz])
    };

    // Upward edge (a below, b above): counts +1 if P is to the left (Pos orient).
    // Downward edge (a above, b below): counts −1 if P is to the right (Neg orient).
    if !a_above && sign == TriSign::Pos { return Ok(1); }
    if  a_above && sign == TriSign::Neg { return Ok(-1); }

    Ok(0)
}

// ── Per-face intersection ─────────────────────────────────────────────────────

/// Determine if the +X axis ray from `point` intersects a face using SoS.
///
/// Returns `true` when the crossing contributes to the parity count.
fn sos_ray_intersects_face(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    face_id: FaceId,
    point: &[f64; 3],
) -> Result<bool, KernelError> {
    // Collect all vertex positions along the outer loop.
    let mut verts: Vec<[f64; 3]> = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        verts.push(vertex_positions(he.origin().index())?);
    }

    if verts.len() < 3 {
        return Ok(false);
    }

    // ── Step 1: Find a certified non-degenerate basis for the face plane ──────
    // We need a non-collinear triplet for orient3d. We also compute the X-component
    // of the face normal (sign of orient2d of the YZ projection of that triplet).
    // If the x-component is zero the face is parallel to the +X ray — skip.
    let (basis_a, basis_b, basis_c) = match find_nondegenerate_basis(&verts)? {
        Some(b) => b,
        None => return Ok(false), // All collinear — zero-area face.
    };

    // nx_sign = sign of orient2d in YZ of the basis triplet.
    // This encodes the X-component of the face normal.
    let (o_nx, _) = orient2d(
        [basis_a[1], basis_a[2]],
        [basis_b[1], basis_b[2]],
        [basis_c[1], basis_c[2]],
    ).map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;

    let nx_sign = o_nx.sign();
    if nx_sign == TriSign::Zero {
        // Face is perfectly parallel to the +X ray — no crossing.
        return Ok(false);
    }

    // ── Step 2: 2D SoS winding number in the YZ plane ────────────────────────
    let n = verts.len();
    let mut winding: i32 = 0;

    for i in 0..n {
        let v0 = verts[i];
        let v1 = verts[(i + 1) % n];
        winding += sos_edge_crossing_yz(
            point[1], point[2],
            v0[1], v0[2],
            v1[1], v1[2],
        )?;
    }

    if winding == 0 {
        return Ok(false); // Ray misses this face in the YZ projection.
    }

    // ── Step 3: 3D depth check — is the face in front of P along +X? ─────────
    let (o3, _) = orient3d(basis_a, basis_b, basis_c, *point)
        .map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;

    let p_sign = if o3.sign() != TriSign::Zero {
        o3.sign()
    } else {
        sos_orient3d_tiebreak(basis_a, basis_b, basis_c)?
    };

    if p_sign == TriSign::Zero {
        // Degenerate face (basis collinear after all) — skip.
        return Ok(false);
    }

    // Mathematical invariant: the crossing counts if and only if the sign of
    // orient3d(P relative to the face plane) differs from the face's X-normal sign.
    Ok(p_sign != nx_sign)
}

/// Find the first non-collinear triplet of vertices in the list.
///
/// Returns `Some((p0, p1, pk))` where pk is the first vertex that forms a
/// triangle with non-zero area under exact `orient2d` in the YZ plane.
/// Returns `None` if all vertices are collinear (degenerate face).
fn find_nondegenerate_basis(
    verts: &[[f64; 3]],
) -> Result<Option<([f64; 3], [f64; 3], [f64; 3])>, KernelError> {
    if verts.len() < 3 { return Ok(None); }

    let p0 = verts[0];
    let p1 = verts[1];

    for &pk in &verts[2..] {
        // Use YZ projection as the primary test axis.
        let (o, _) = orient2d(
            [p0[1], p0[2]], [p1[1], p1[2]], [pk[1], pk[2]],
        ).map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;

        if o.sign() != TriSign::Zero {
            return Ok(Some((p0, p1, pk)));
        }

        // Fallback: try XY projection.
        let (o_xy, _) = orient2d(
            [p0[0], p0[1]], [p1[0], p1[1]], [pk[0], pk[1]],
        ).map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;

        if o_xy.sign() != TriSign::Zero {
            return Ok(Some((p0, p1, pk)));
        }
    }

    Ok(None) // All collinear.
}

// ── Face-level boundary classifier ───────────────────────────────────────────

/// Result of classifying a point relative to a specific face.
#[derive(Debug, Clone, PartialEq)]
pub enum FacePointClassification {
    /// Point is strictly inside the face interior.
    OnFace,
    /// Point is within tolerance of a boundary edge.
    OnEdge(crate::handles::HalfEdgeId),
    /// Point is within tolerance of a boundary vertex.
    OnVertex(crate::handles::VertexId),
    /// Point is outside the face.
    Outside,
}

/// Classify a 3D point relative to a specific face, with edge/vertex snapping.
///
/// Used in Pass 1 of `classify_point_in_solid` to detect true physical boundary
/// contact before SoS perturbs the point away.
///
/// # Arguments
/// - `arena`: topology arena
/// - `face_id`: the face to test against
/// - `point`: 3D query point
/// - `position_fn`: maps `VertexId` → 3D position
/// - `tolerance`: linear snap distance for vertex/edge proximity
pub fn classify_point_on_face(
    arena: &TopologyArena,
    face_id: crate::handles::FaceId,
    point: &[f64; 3],
    position_fn: &dyn Fn(crate::handles::VertexId) -> Option<[f64; 3]>,
    tolerance: f64,
) -> Result<FacePointClassification, KernelError> {
    let tol_sq = tolerance * tolerance;

    let mut boundary: Vec<(crate::handles::HalfEdgeId, crate::handles::VertexId, [f64; 3])> =
        Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        let v = he.origin();
        let pos = position_fn(v).ok_or_else(|| KernelError::InvalidInput {
            message: format!("classify_point_on_face: no position for vertex {}", v.index()),
            context: None,
        })?;
        boundary.push((he_id, v, pos));
    }

    if boundary.len() < 3 {
        return Ok(FacePointClassification::Outside);
    }

    // Vertex proximity (closest wins).
    for &(_, v, pos) in &boundary {
        let dx = point[0] - pos[0];
        let dy = point[1] - pos[1];
        let dz = point[2] - pos[2];
        if dx*dx + dy*dy + dz*dz <= tol_sq {
            return Ok(FacePointClassification::OnVertex(v));
        }
    }

    // Edge proximity.
    let n = boundary.len();
    for i in 0..n {
        let (he_id, _, a) = boundary[i];
        let (_, _, b) = boundary[(i + 1) % n];
        let ab = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
        let ap = [point[0]-a[0], point[1]-a[1], point[2]-a[2]];
        let t_num = ap[0]*ab[0] + ap[1]*ab[1] + ap[2]*ab[2];
        let t_den = ab[0]*ab[0] + ab[1]*ab[1] + ab[2]*ab[2];
        if t_den > 0.0 {
            let t = (t_num / t_den).clamp(0.0, 1.0);
            let closest = [a[0]+t*ab[0], a[1]+t*ab[1], a[2]+t*ab[2]];
            let dx = point[0]-closest[0];
            let dy = point[1]-closest[1];
            let dz = point[2]-closest[2];
            if dx*dx + dy*dy + dz*dz <= tol_sq {
                return Ok(FacePointClassification::OnEdge(he_id));
            }
        }
    }

    // Coplanarity check (distance to plane).
    let positions: Vec<[f64; 3]> = boundary.iter().map(|&(_, _, p)| p).collect();
    let normal = compute_newell_normal(&positions);
    let mag_sq = normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2];
    if mag_sq > 0.0 {
        let mag = mag_sq.sqrt();
        let n_unit = [normal[0]/mag, normal[1]/mag, normal[2]/mag];
        
        let p0 = positions[0];
        let d = -(n_unit[0]*p0[0] + n_unit[1]*p0[1] + n_unit[2]*p0[2]);
        let dist = (n_unit[0]*point[0] + n_unit[1]*point[1] + n_unit[2]*point[2] + d).abs();
        
        if dist > tolerance {
            return Ok(FacePointClassification::Outside);
        }
    }

    // 2D containment check using winding number.
    let (in_poly, _) = point_in_projected_polygon(point, &positions)?;
    if in_poly {
        Ok(FacePointClassification::OnFace)
    } else {
        Ok(FacePointClassification::Outside)
    }
}

// ── 2D winding number (for classify_point_on_face) ───────────────────────────

/// Strict 2D point-in-polygon test via winding number in the dominant projection.
///
/// This variant does NOT use SoS — it is used only inside `classify_point_on_face`
/// where tolerance-based snapping has already cleared degenerate cases.
fn point_in_projected_polygon(
    hit: &[f64; 3],
    verts: &[[f64; 3]],
) -> Result<(bool, Option<forge_math::arithmetic::precision::PrecisionEscalation>), KernelError> {
    let n = verts.len();
    if n < 3 { return Ok((false, None)); }

    use forge_geom::{dominant_projection_axes, resolve_zero_edge, scanline_edge_crossing, EdgeTieBreaker};

    let (u_axis, v_axis) = dominant_projection_axes(verts);
    let hit_u = hit[u_axis];
    let hit_v = hit[v_axis];

    let mut winding: i32 = 0;
    let mut max_escalation: Option<forge_math::arithmetic::precision::PrecisionEscalation> = None;

    for i in 0..n {
        let j = (i + 1) % n;
        let vi_u = verts[i][u_axis];
        let vi_v = verts[i][v_axis];
        let vj_u = verts[j][u_axis];
        let vj_v = verts[j][v_axis];

        let (orient, esc) = orient2d(
            [vi_u, vi_v],
            [vj_u, vj_v],
            [hit_u, hit_v],
        ).map_err(|e| KernelError::InternalError {
            message: format!("orient2d error: {e}"),
            context: None,
        })?;

        if let Some(e) = &max_escalation {
            if esc.resolved_at > e.resolved_at { max_escalation = Some(esc); }
        } else {
            max_escalation = Some(esc);
        }

        let sign = match orient.sign() {
            TriSign::Zero => match resolve_zero_edge([vi_u, vi_v], [vj_u, vj_v]) {
                EdgeTieBreaker::PreferPos => TriSign::Pos,
                EdgeTieBreaker::PreferNeg => TriSign::Neg,
            },
            s => s,
        };

        match scanline_edge_crossing(vi_v, vj_v, hit_v, sign) {
            Some(true)  => winding += 1,
            Some(false) => winding -= 1,
            None        => {}
        }
    }

    Ok((winding != 0, max_escalation))
}

/// Compute the Newell normal vector for a polygon.
///
/// Returns the unnormalized cross-product accumulation. The magnitude
/// is twice the polygon area.
fn compute_newell_normal(verts: &[[f64; 3]]) -> [f64; 3] {
    let n = verts.len();
    let mut nx = 0.0_f64;
    let mut ny = 0.0_f64;
    let mut nz = 0.0_f64;
    for i in 0..n {
        let curr = verts[i];
        let next = verts[(i + 1) % n];
        nx += (curr[1] - next[1]) * (curr[2] + next[2]);
        ny += (curr[2] - next[2]) * (curr[0] + next[0]);
        nz += (curr[0] - next[0]) * (curr[1] + next[1]);
    }
    [nx, ny, nz]
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use forge_math::sign::CertifiedTriSign;

    #[test]
    fn point_above_plane_classified_outside() {
        let (orientation, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, -1.0],
        ).unwrap();
        let face = FaceId::new(0, 1);
        let result = classify_point_against_face(orientation, None, face);
        assert_eq!(result, PointClassification::Outside { escalation: None });
    }

    #[test]
    fn point_below_plane_classified_inside() {
        let (orientation, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ).unwrap();
        let face = FaceId::new(0, 1);
        let result = classify_point_against_face(orientation, None, face);
        assert_eq!(result, PointClassification::Inside { escalation: None });
    }

    #[test]
    fn coplanar_point_classified_on_boundary() {
        let (orientation, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ).unwrap();
        let face = FaceId::new(0, 1);
        let result = classify_point_against_face(orientation, None, face);
        assert_eq!(result, PointClassification::OnBoundary(face));
    }

    #[test]
    fn sos_orient2d_tiebreak_nonzero_delta1() {
        // a=[0,2] b=[0,0]: delta1 = a[1]-b[1] = 2 > 0 → Pos
        assert_eq!(sos_orient2d_tiebreak([0.0, 2.0], [0.0, 0.0]), TriSign::Pos);
        // a=[0,0] b=[0,2]: delta1 = 0-2 = -2 < 0 → Neg
        assert_eq!(sos_orient2d_tiebreak([0.0, 0.0], [0.0, 2.0]), TriSign::Neg);
    }

    #[test]
    fn sos_orient2d_tiebreak_delta2_fallback() {
        // delta1 == 0, delta2 = b[0]-a[0] = 1 > 0 → Pos
        assert_eq!(sos_orient2d_tiebreak([1.0, 0.0], [2.0, 0.0]), TriSign::Pos);
        // delta2 = 1 - 2 = -1 < 0 → Neg
        assert_eq!(sos_orient2d_tiebreak([2.0, 0.0], [1.0, 0.0]), TriSign::Neg);
    }

    #[test]
    fn sos_edge_no_crossing_same_side() {
        // Both A and B have az > pz → both above → 0.
        let result = sos_edge_crossing_yz(0.0, 0.0, 0.0, 1.0, 1.0, 2.0).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn sos_edge_upward_crossing() {
        // Collinear case: A=(0,-1), B=(0,1), P=(0,0) → orient2d is zero.
        // SoS tiebreak: delta1 = a[1]-b[1] = -1-1 = -2 → Neg.
        // !a_above(true) && Pos: false → no crossing (SoS pushes P off-edge).
        let collinear = sos_edge_crossing_yz(0.0, 0.0, 0.0, -1.0, 0.0, 1.0).unwrap();
        assert_eq!(collinear, 0, "Collinear upward edge: SoS yields Neg, no +1 crossing");

        // Non-collinear upward crossing: A=(0,-1), B=(0,1), P=(-1,0).
        // orient2d([0,-1],[0,1],[-1,0]):
        //   det = (0 - (-1))*(0 - (-1)) - (0 - 0)*(0 - (-1)) = 1*1 - 0 = 1 → Pos
        // !a_above(true) && Pos → +1
        let upward = sos_edge_crossing_yz(-1.0, 0.0, 0.0, -1.0, 0.0, 1.0).unwrap();
        assert_eq!(upward, 1, "P to the left of upward edge → +1");

        // Downward crossing: A=(0,1), B=(0,-1), P=(-1,0).
        // a_above=true, orient2d([0,1],[0,-1],[-1,0]):
        //   det = (0 - (-1))*(-1 - 0) - (0 - 0)*(0 - (-1)) = 1*(-1) - 0 = -1 → Neg
        // a_above(true) && Neg → −1
        let downward = sos_edge_crossing_yz(-1.0, 0.0, 0.0, 1.0, 0.0, -1.0).unwrap();
        assert_eq!(downward, -1, "P to the right of downward edge → -1");
    }
}
