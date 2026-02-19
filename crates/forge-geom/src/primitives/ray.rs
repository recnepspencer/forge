//! Ray-geometry intersection and projection utilities.
//!
//! DOMAIN: Ray-plane intersection, 2D projection axis selection,
//! and zero-edge resolution for point-in-solid classification.
//!
//! DEPENDENCIES: `forge-math` (sign types, error)
//!
//! INVARIANTS:
//! - All floating-point geometry computations live here, not in `forge-topo`
//! - Degeneracy thresholds are explicit parameters, never hardcoded

pub use eval::{compute_ray_plane_intersection, resolve_zero_edge, EdgeTieBreaker, dominant_projection_axes, scanline_edge_crossing};

// =========================================================================
// EVALUATION LOGIC
// =========================================================================

mod eval {
use forge_math::MathError;
use forge_math::sign::TriSign;


/// Compute the intersection point of a ray with a plane defined by
/// three vertices using parametric interpolation.
///
/// The `degeneracy` parameter controls the minimum acceptable |denom|.
pub fn compute_ray_plane_intersection(
    origin: &[f64; 3],
    far: &[f64; 3],
    face_verts: &[[f64; 3]],
    degeneracy: f64,
) -> Result<[f64; 3], MathError> {
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
        return Err(MathError::InternalError(
            "Ray nearly parallel to face plane — skipping".to_string(),
        ));
    }

    let t = numer / denom;
    Ok([
        origin[0] + t * dir[0],
        origin[1] + t * dir[1],
        origin[2] + t * dir[2],
    ])
}

/// Tie-breaking hint for zero-edge resolution (not a certified sign).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeTieBreaker {
    /// Prefer positive orientation (e.g., "outside").
    PreferPos,
    /// Prefer negative orientation (e.g., "inside").
    PreferNeg,
}

/// Resolve a Zero orient2d result to a consistent tie-breaker using edge direction.
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

/// Determine if a directed polygon edge crosses a horizontal scanline.
pub fn scanline_edge_crossing(
    vi_v: f64,
    vj_v: f64,
    hit_v: f64,
    edge_sign: TriSign,
) -> Option<bool> {
    if vi_v <= hit_v {
        if vj_v > hit_v && edge_sign == TriSign::Pos {
            return Some(true);
        }
    } else if vj_v <= hit_v && edge_sign == TriSign::Neg {
        return Some(false);
    }
    None
}
}
