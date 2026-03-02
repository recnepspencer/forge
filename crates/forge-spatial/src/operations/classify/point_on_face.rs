//! Boundary proximity pre-check for point-on-face classification.
//!
//! DOMAIN: Determine whether a 3D point lies within tolerance of a specific
//!         face — vertex sphere, edge tube, or face plane — before SoS is
//!         applied (which would push the point off the boundary).
//!
//! DEPENDENCIES: forge-topo (arena, handles, FaceEdgeIterator),
//!               forge-core (KernelError, ToleranceProvider),
//!               forge-geom (dominant_projection_axes, scanline_edge_crossing).

use forge_core::{KernelError, ToleranceProvider};
use forge_geom::{
    dominant_projection_axes, resolve_zero_edge, scanline_edge_crossing, EdgeTieBreaker,
};
use forge_math::predicates::orient2d;
use forge_math::sign::TriSign;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;

/// Result of classifying a point relative to a specific face.
#[derive(Debug, Clone, PartialEq)]
pub enum FacePointClassification {
    /// Point is strictly inside the face interior.
    OnFace,
    /// Point is within tolerance of a boundary edge.
    OnEdge(HalfEdgeId),
    /// Point is within tolerance of a boundary vertex.
    OnVertex(VertexId),
    /// Point is outside the face.
    Outside,
}

/// Classify a 3D point relative to a specific face, with per-entity tolerance snapping.
///
/// Used in Pass 1 of `classify_point_in_solid` to detect true physical boundary
/// contact before SoS perturbs the point away. Checks vertex sphere proximity,
/// edge tube proximity, and coplanarity in that order.
pub fn classify_point_on_face(
    arena: &TopologyArena,
    face_id: FaceId,
    point: &[f64; 3],
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<FacePointClassification, KernelError> {
    let mut boundary: Vec<(HalfEdgeId, VertexId, [f64; 3])> = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        let v = he.origin();
        let pos = position_fn(v).ok_or_else(|| KernelError::InvalidInput {
            message: format!(
                "classify_point_on_face: no position for vertex {}",
                v.index()
            ),
            context: None,
        })?;
        boundary.push((he_id, v, pos));
    }

    if boundary.len() < 3 {
        return Ok(FacePointClassification::Outside);
    }

    for &(_, v, pos) in &boundary {
        let vtol = tolerance_provider.vertex_tolerance(v.index(), v.generation());
        let tol_sq = vtol * vtol;
        let dx = point[0] - pos[0];
        let dy = point[1] - pos[1];
        let dz = point[2] - pos[2];
        if dx * dx + dy * dy + dz * dz <= tol_sq {
            return Ok(FacePointClassification::OnVertex(v));
        }
    }

    let n = boundary.len();
    for i in 0..n {
        let (he_id, _, a) = boundary[i];
        let (_, _, b) = boundary[(i + 1) % n];
        let he = arena.get_half_edge(he_id)?;
        let edge_id = he.edge();
        let etol = tolerance_provider.edge_tolerance(edge_id.index(), edge_id.generation());
        let tol_sq = etol * etol;
        let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let ap = [point[0] - a[0], point[1] - a[1], point[2] - a[2]];
        let t_num = ap[0] * ab[0] + ap[1] * ab[1] + ap[2] * ab[2];
        let t_den = ab[0] * ab[0] + ab[1] * ab[1] + ab[2] * ab[2];
        if t_den > 0.0 {
            let t = (t_num / t_den).clamp(0.0, 1.0);
            let closest = [a[0] + t * ab[0], a[1] + t * ab[1], a[2] + t * ab[2]];
            let dx = point[0] - closest[0];
            let dy = point[1] - closest[1];
            let dz = point[2] - closest[2];
            if dx * dx + dy * dy + dz * dz <= tol_sq {
                return Ok(FacePointClassification::OnEdge(he_id));
            }
        }
    }

    let min_vtol = boundary
        .iter()
        .map(|&(_, v, _)| tolerance_provider.vertex_tolerance(v.index(), v.generation()))
        .fold(f64::MAX, f64::min);

    let positions: Vec<[f64; 3]> = boundary.iter().map(|&(_, _, p)| p).collect();
    let normal = compute_newell_normal(&positions);
    let mag_sq = normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2];
    if mag_sq > 0.0 {
        let mag = mag_sq.sqrt();
        let n_unit = [normal[0] / mag, normal[1] / mag, normal[2] / mag];
        let p0 = positions[0];
        let d = -(n_unit[0] * p0[0] + n_unit[1] * p0[1] + n_unit[2] * p0[2]);
        let dist = (n_unit[0] * point[0] + n_unit[1] * point[1] + n_unit[2] * point[2] + d).abs();
        if dist > min_vtol {
            return Ok(FacePointClassification::Outside);
        }
    }

    let (in_poly, _) = point_in_projected_polygon(point, &positions)?;
    if in_poly {
        Ok(FacePointClassification::OnFace)
    } else {
        Ok(FacePointClassification::Outside)
    }
}

/// Strict 2D point-in-polygon test via winding number in the dominant projection.
///
/// Does NOT use SoS — used only inside `classify_point_on_face` where
/// tolerance-based snapping has already cleared degenerate cases.
fn point_in_projected_polygon(
    hit: &[f64; 3],
    verts: &[[f64; 3]],
) -> Result<
    (
        bool,
        Option<forge_math::arithmetic::precision::PrecisionEscalation>,
    ),
    KernelError,
> {
    let n = verts.len();
    if n < 3 {
        return Ok((false, None));
    }

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

        let (orient, esc) = orient2d([vi_u, vi_v], [vj_u, vj_v], [hit_u, hit_v]).map_err(|e| {
            KernelError::InternalError {
                message: format!("orient2d error: {e}"),
                context: None,
            }
        })?;

        if let Some(e) = &max_escalation {
            if esc.resolved_at > e.resolved_at {
                max_escalation = Some(esc);
            }
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
            Some(true) => winding += 1,
            Some(false) => winding -= 1,
            None => {}
        }
    }

    Ok((winding != 0, max_escalation))
}

/// Compute the unnormalized Newell normal for a polygon.
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
