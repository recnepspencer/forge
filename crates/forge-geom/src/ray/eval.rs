//! Evaluation logic for ray-geometry operations.
//!
//! These functions were extracted from `forge-topo/classify.rs` to enforce
//! the topology-geometry firewall (Doctrine D3). All floating-point
//! geometry stays in `forge-geom`; topology only consumes results.

use forge_core::KernelError;
use forge_math::sign::TriSign;

/// Compute the intersection point of a ray with a plane defined by
/// three vertices using parametric interpolation.
///
/// Given ray from `origin` to `far` crossing the plane at some t ∈ (0,1),
/// computes hit = origin + t * (far - origin).
///
/// The `degeneracy` parameter controls the minimum acceptable |denom|.
/// If the ray is nearly parallel to the plane (|denom| < degeneracy),
/// an error is returned.
///
/// Precondition: the caller should verify via orient3d that origin and far
/// are on opposite sides of the face plane for a meaningful intersection.
pub fn compute_ray_plane_intersection(
    origin: &[f64; 3],
    far: &[f64; 3],
    face_verts: &[[f64; 3]],
    degeneracy: f64,
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

    if denom.abs() < degeneracy {
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

/// Tie-breaking hint for zero-edge resolution (not a certified sign).
///
/// This enum is used to deterministically resolve the "on-edge" case (Zero)
/// without returning a `TriSign`, which would imply a certified geometric
/// predicate. This enforces the Topology-Geometry Firewall (D3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeTieBreaker {
    /// Prefer positive orientation (e.g., "outside").
    PreferPos,
    /// Prefer negative orientation (e.g., "inside").
    PreferNeg,
}

/// Resolve a Zero orient2d result to a consistent tie-breaker using edge direction.
///
/// When a hit point lies exactly on a polygon edge, both adjacent faces
/// share that edge with opposite winding. We break the tie by looking
/// at the edge's direction in the v-axis (second projected coordinate):
/// - Edge going upward (vi.v < vn.v) → PreferPos
/// - Edge going downward (vi.v > vn.v) → PreferNeg
/// - Horizontal edge: break tie on u-axis direction
///
/// Since adjacent faces traverse the shared edge in opposite directions,
/// they get opposite resolved signs — ensuring exactly one face "owns"
/// the edge (D0 — no perturbation).
pub fn resolve_zero_edge(vi_2d: [f64; 2], vn_2d: [f64; 2]) -> EdgeTieBreaker {
    let dv = vn_2d[1] - vi_2d[1];
    if dv > 0.0 {
        EdgeTieBreaker::PreferPos
    } else if dv < 0.0 {
        EdgeTieBreaker::PreferNeg
    } else {
        let du = vn_2d[0] - vi_2d[0];
        if du > 0.0 {
            EdgeTieBreaker::PreferPos
        } else {
            EdgeTieBreaker::PreferNeg
        }
    }
}

/// Determine the 2D projection axes by finding the dominant
/// component of the face normal.
///
/// Drops the axis with the largest absolute normal component
/// to maximize projection area and numerical stability.
pub fn dominant_projection_axes(verts: &[[f64; 3]]) -> (usize, usize) {
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
