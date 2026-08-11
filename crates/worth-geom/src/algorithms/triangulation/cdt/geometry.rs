//! Exact 2D geometric predicates and triangle-shape operations for CDT.
//!
//! DOMAIN: Private geometric substrate shared by CDT insertion, constraint,
//! and exterior phases. Topology-driving decisions remain exact-predicate
//! results from `worth_math`.

use worth_math::predicates::orient2d::orient2d;
use worth_math::sign::TriSign;

use super::{Triangle, VIdx};

/// Compute the bounding box of a point set.
pub(super) fn bounding_box(vertices: &[[f64; 2]]) -> ([f64; 2], [f64; 2]) {
    let mut min = [f64::INFINITY, f64::INFINITY];
    let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY];
    for v in vertices {
        min[0] = min[0].min(v[0]);
        min[1] = min[1].min(v[1]);
        max[0] = max[0].max(v[0]);
        max[1] = max[1].max(v[1]);
    }
    (min, max)
}

/// Check if triangle `tri` contains edge (a, b) in either direction.
pub(super) fn triangle_has_edge(tri: &Triangle, a: VIdx, b: VIdx) -> bool {
    for i in 0..3 {
        let ea = tri.v[i];
        let eb = tri.v[(i + 1) % 3];
        if (ea == a && eb == b) || (ea == b && eb == a) {
            return true;
        }
    }
    false
}

/// Find the vertex of `tri` opposite to edge (a, b).
pub(super) fn triangle_opposite_vertex(tri: &Triangle, a: VIdx, b: VIdx) -> Option<VIdx> {
    for &v in &tri.v {
        if v != a && v != b {
            return Some(v);
        }
    }
    None
}

/// Centroid of a triangle in 2D.
pub(super) fn triangle_centroid(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> [f64; 2] {
    [(a[0] + b[0] + c[0]) / 3.0, (a[1] + b[1] + c[1]) / 3.0]
}

/// Test if two line segments properly cross (not just touch).
///
/// Uses exact `orient2d` predicates for robustness.
pub(super) fn segments_cross(a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2]) -> bool {
    let o1 = orient2d(a, b, c).ok().map(|(s, _)| s.sign());
    let o2 = orient2d(a, b, d).ok().map(|(s, _)| s.sign());
    let o3 = orient2d(c, d, a).ok().map(|(s, _)| s.sign());
    let o4 = orient2d(c, d, b).ok().map(|(s, _)| s.sign());

    match (o1, o2, o3, o4) {
        (Some(s1), Some(s2), Some(s3), Some(s4)) => {
            s1 != s2
                && s1 != TriSign::Zero
                && s2 != TriSign::Zero
                && s3 != s4
                && s3 != TriSign::Zero
                && s4 != TriSign::Zero
        }
        _ => false,
    }
}

/// Point-in-polygon test using winding number with exact orient2d.
pub(super) fn point_in_polygon_2d(
    pt: &[f64; 2],
    vertices: &[[f64; 2]],
    boundary: &[usize],
) -> Result<bool, worth_math::error::MathError> {
    let n = boundary.len();
    let mut winding = 0i32;

    for i in 0..n {
        let a = vertices[boundary[i]];
        let b = vertices[boundary[(i + 1) % n]];

        if a[1] <= pt[1] {
            if b[1] > pt[1] {
                let (s, _) = orient2d(a, b, *pt)?;
                if s.sign() == TriSign::Pos {
                    winding += 1;
                }
            }
        } else if b[1] <= pt[1] {
            let (s, _) = orient2d(a, b, *pt)?;
            if s.sign() == TriSign::Neg {
                winding -= 1;
            }
        }
    }

    Ok(winding != 0)
}
