//! 2D polygon overlap detection for coplanar face analysis.
//!
//! DOMAIN: Stateless 2D polygon intersection tests.
//! DEPENDENCIES: `forge-math` (cross_2d)
//! INVARIANTS: All functions are pure — no topology, no policy, no thresholds.

use forge_math::linalg::cross_2d;

/// Choose the two axes for 2D projection by dropping the dominant normal axis.
///
/// Given a 3D normal vector, returns the two axis indices that produce
/// the least-distorted 2D projection of polygons lying in that plane.
pub fn dominant_projection_axes(normal: [f64; 3]) -> (usize, usize) {
    let abs_n = [normal[0].abs(), normal[1].abs(), normal[2].abs()];
    if abs_n[0] >= abs_n[1] && abs_n[0] >= abs_n[2] {
        (1, 2)
    } else if abs_n[1] >= abs_n[2] {
        (0, 2)
    } else {
        (0, 1)
    }
}

/// Test if two 2D polygons overlap in area.
///
/// Checks for genuine area overlap using three tests:
///   1. Any vertex of A is strictly inside B (winding number)
///   2. Any vertex of B is strictly inside A (winding number)
///   3. Any edge pair from A and B properly crosses
///
/// Returns false for edge-only or vertex-only contact (no shared area).
/// Both polygons must be simple (non-self-intersecting).
pub fn polygons_overlap_2d(poly_a: &[[f64; 2]], poly_b: &[[f64; 2]]) -> bool {
    for pt in poly_a {
        if point_strictly_inside_polygon(pt, poly_b) {
            return true;
        }
    }
    for pt in poly_b {
        if point_strictly_inside_polygon(pt, poly_a) {
            return true;
        }
    }

    for i in 0..poly_a.len() {
        let a0 = &poly_a[i];
        let a1 = &poly_a[(i + 1) % poly_a.len()];
        for j in 0..poly_b.len() {
            let b0 = &poly_b[j];
            let b1 = &poly_b[(j + 1) % poly_b.len()];
            if segments_properly_cross(a0, a1, b0, b1) {
                return true;
            }
        }
    }

    let centroid_a = polygon_centroid(poly_a);
    if point_strictly_inside_polygon(&centroid_a, poly_b) {
        return true;
    }
    let centroid_b = polygon_centroid(poly_b);
    if point_strictly_inside_polygon(&centroid_b, poly_a) {
        return true;
    }

    false
}

/// Compute the centroid (arithmetic mean of vertices) of a polygon.
fn polygon_centroid(poly: &[[f64; 2]]) -> [f64; 2] {
    let n = poly.len() as f64;
    let sx: f64 = poly.iter().map(|p| p[0]).sum();
    let sy: f64 = poly.iter().map(|p| p[1]).sum();
    [sx / n, sy / n]
}

/// Test if a point is strictly inside a simple polygon.
///
/// Two-phase approach:
///   1. **Boundary rejection**: if the point is within `eps` of any polygon edge,
///      return false. This catches vertices, edge midpoints, and near-boundary cases.
///   2. **Winding number**: run the standard Dan Sunday winding number algorithm
///      on the remaining cases to determine interior vs exterior.
///
/// The standard winding number alone cannot distinguish boundary from interior
/// (it gives inconsistent results for on-boundary points depending on edge
/// orientation). The explicit boundary check resolves this.
pub fn point_strictly_inside_polygon(pt: &[f64; 2], poly: &[[f64; 2]]) -> bool {
    let n = poly.len();
    let boundary_eps_sq = 1e-20;

    for i in 0..n {
        let v0 = &poly[i];
        let v1 = &poly[(i + 1) % n];
        if point_near_segment_sq(pt, v0, v1) < boundary_eps_sq {
            return false;
        }
    }

    winding_number(pt, poly) != 0
}

/// Squared distance from point to line segment.
fn point_near_segment_sq(pt: &[f64; 2], a: &[f64; 2], b: &[f64; 2]) -> f64 {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let ap = [pt[0] - a[0], pt[1] - a[1]];
    let ab_len_sq = ab[0] * ab[0] + ab[1] * ab[1];

    if ab_len_sq < 1e-30 {
        return ap[0] * ap[0] + ap[1] * ap[1];
    }

    let t = (ap[0] * ab[0] + ap[1] * ab[1]) / ab_len_sq;
    let t_clamped = t.max(0.0).min(1.0);
    let proj = [a[0] + t_clamped * ab[0], a[1] + t_clamped * ab[1]];
    let dx = pt[0] - proj[0];
    let dy = pt[1] - proj[1];
    dx * dx + dy * dy
}

/// Standard Dan Sunday winding number algorithm.
///
/// For each edge Vᵢ→Vᵢ₊₁:
///   - Upward crossing (Vᵢ.y ≤ P.y < Vᵢ₊₁.y): if isLeft > 0, winding++
///   - Downward crossing (Vᵢ₊₁.y ≤ P.y < Vᵢ.y): if isLeft < 0, winding--
///
/// isLeft = cross_2d(edge, to_pt) = (V₁-V₀)×(P-V₀)
fn winding_number(pt: &[f64; 2], poly: &[[f64; 2]]) -> i32 {
    let n = poly.len();
    let mut winding: i32 = 0;

    for i in 0..n {
        let v0 = &poly[i];
        let v1 = &poly[(i + 1) % n];

        let edge = [v1[0] - v0[0], v1[1] - v0[1]];
        let to_pt = [pt[0] - v0[0], pt[1] - v0[1]];
        let is_left = cross_2d(edge, to_pt);

        if v0[1] <= pt[1] {
            if v1[1] > pt[1] {
                if is_left > 0.0 {
                    winding += 1;
                }
            }
        } else if v1[1] <= pt[1] {
            if is_left < 0.0 {
                winding -= 1;
            }
        }
    }

    winding
}

/// Test if two line segments properly cross (transversal intersection).
///
/// Uses the orientation test: segments AB and CD properly cross iff
///   orient(A,B,C) ≠ orient(A,B,D) AND orient(C,D,A) ≠ orient(C,D,B)
/// with all four orientations being non-degenerate.
///
/// Returns false for collinear, touching, or endpoint-only contact.
pub fn segments_properly_cross(
    a: &[f64; 2], b: &[f64; 2],
    c: &[f64; 2], d: &[f64; 2],
) -> bool {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let ac = [c[0] - a[0], c[1] - a[1]];
    let ad = [d[0] - a[0], d[1] - a[1]];

    let cd = [d[0] - c[0], d[1] - c[1]];
    let ca = [a[0] - c[0], a[1] - c[1]];
    let cb = [b[0] - c[0], b[1] - c[1]];

    let d1 = cross_2d(ab, ac);
    let d2 = cross_2d(ab, ad);
    let d3 = cross_2d(cd, ca);
    let d4 = cross_2d(cd, cb);

    let eps = 1e-14;
    if d1.abs() < eps || d2.abs() < eps || d3.abs() < eps || d4.abs() < eps {
        return false;
    }

    (d1 > 0.0) != (d2 > 0.0) && (d3 > 0.0) != (d4 > 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dominant_axes_z_normal_drops_z() {
        assert_eq!(dominant_projection_axes([0.0, 0.0, 1.0]), (0, 1));
    }

    #[test]
    fn dominant_axes_x_normal_drops_x() {
        assert_eq!(dominant_projection_axes([1.0, 0.0, 0.0]), (1, 2));
    }

    #[test]
    fn dominant_axes_y_normal_drops_y() {
        assert_eq!(dominant_projection_axes([0.0, 1.0, 0.0]), (0, 2));
    }

    #[test]
    fn point_inside_unit_square() {
        let square = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        assert!(point_strictly_inside_polygon(&[0.5, 0.5], &square));
    }

    #[test]
    fn point_outside_unit_square() {
        let square = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        assert!(!point_strictly_inside_polygon(&[2.0, 0.5], &square));
    }

    #[test]
    fn point_on_edge_not_inside() {
        let square = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        assert!(!point_strictly_inside_polygon(&[0.5, 0.0], &square));
    }

    #[test]
    fn point_on_vertex_not_inside() {
        let square = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        assert!(!point_strictly_inside_polygon(&[0.0, 0.0], &square));
    }

    #[test]
    fn identical_squares_overlap() {
        let sq = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        assert!(polygons_overlap_2d(&sq, &sq));
    }

    #[test]
    fn overlapping_squares_detected() {
        let a = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let b = [[0.5, 0.5], [1.5, 0.5], [1.5, 1.5], [0.5, 1.5]];
        assert!(polygons_overlap_2d(&a, &b));
    }

    #[test]
    fn edge_touching_squares_no_overlap() {
        let a = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let b = [[1.0, 0.0], [2.0, 0.0], [2.0, 1.0], [1.0, 1.0]];
        assert!(!polygons_overlap_2d(&a, &b));
    }

    #[test]
    fn vertex_touching_squares_no_overlap() {
        let a = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let b = [[1.0, 1.0], [2.0, 1.0], [2.0, 2.0], [1.0, 2.0]];
        assert!(!polygons_overlap_2d(&a, &b));
    }

    #[test]
    fn disjoint_squares_no_overlap() {
        let a = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let b = [[5.0, 5.0], [6.0, 5.0], [6.0, 6.0], [5.0, 6.0]];
        assert!(!polygons_overlap_2d(&a, &b));
    }

    #[test]
    fn one_inside_other_detected() {
        let outer = [[-2.0, -2.0], [2.0, -2.0], [2.0, 2.0], [-2.0, 2.0]];
        let inner = [[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]];
        assert!(polygons_overlap_2d(&outer, &inner));
        assert!(polygons_overlap_2d(&inner, &outer));
    }

    #[test]
    fn cross_overlap_detected() {
        let a = [[-0.5, -2.0], [0.5, -2.0], [0.5, 2.0], [-0.5, 2.0]];
        let b = [[-2.0, -0.5], [2.0, -0.5], [2.0, 0.5], [-2.0, 0.5]];
        assert!(polygons_overlap_2d(&a, &b));
    }

    #[test]
    fn segments_cross_basic() {
        assert!(segments_properly_cross(
            &[0.0, 0.0], &[1.0, 1.0],
            &[0.0, 1.0], &[1.0, 0.0],
        ));
    }

    #[test]
    fn segments_parallel_no_cross() {
        assert!(!segments_properly_cross(
            &[0.0, 0.0], &[1.0, 0.0],
            &[0.0, 1.0], &[1.0, 1.0],
        ));
    }

    #[test]
    fn segments_share_endpoint_no_cross() {
        assert!(!segments_properly_cross(
            &[0.0, 0.0], &[1.0, 0.0],
            &[1.0, 0.0], &[1.0, 1.0],
        ));
    }

    #[test]
    fn segments_t_junction_no_cross() {
        assert!(!segments_properly_cross(
            &[0.0, 0.0], &[2.0, 0.0],
            &[1.0, 0.0], &[1.0, 1.0],
        ));
    }

    #[test]
    fn cube_face_edge_touching_no_overlap() {
        let face_a = [[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]];
        let face_b = [[-0.5, 0.5], [0.5, 0.5], [0.5, 1.5], [-0.5, 1.5]];
        assert!(!polygons_overlap_2d(&face_a, &face_b));
    }
}
