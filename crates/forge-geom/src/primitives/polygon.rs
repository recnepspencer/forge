//! Polygon geometry algorithms.

/// Compute polygon area using the Newell method (3D cross product sum).
///
/// Works for arbitrary planar polygons embedded in 3D space.
/// Returns the magnitude of half the sum of edge cross products.
pub fn compute_polygon_area(vertices: &[[f64; 3]]) -> f64 {
    let n = vertices.len();
    if n < 3 {
        return 0.0;
    }

    let mut cross = [0.0_f64; 3];

    for i in 0..n {
        let j = (i + 1) % n;
        let vi = &vertices[i];
        let vj = &vertices[j];
        cross[0] += (vi[1] - vj[1]) * (vi[2] + vj[2]);
        cross[1] += (vi[2] - vj[2]) * (vi[0] + vj[0]);
        cross[2] += (vi[0] - vj[0]) * (vi[1] + vj[1]);
    }

    let magnitude = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
    magnitude * 0.5
}

/// Compute the centroid of a polygon by averaging vertex positions.
///
/// For a convex polygon, this is always strictly interior.
/// Returns None if vertices is empty.
pub fn compute_polygon_centroid(vertices: &[[f64; 3]]) -> Option<[f64; 3]> {
    let n = vertices.len();
    if n == 0 {
        return None;
    }

    let mut sum = [0.0_f64; 3];
    for v in vertices {
        sum[0] += v[0];
        sum[1] += v[1];
        sum[2] += v[2];
    }

    let inv = 1.0 / n as f64;
    Some([sum[0] * inv, sum[1] * inv, sum[2] * inv])
}

/// Compute the centroid of the largest-area triangle in a fan decomposition.
///
/// For face polygons that may be concave, the vertex-average centroid can lie
/// outside the polygon. This function fans from vertex 0, finds the triangle
/// with the largest area (via cross product magnitude), and returns its centroid.
///
/// The centroid of any triangle is always strictly interior to that triangle,
/// so the result is guaranteed to lie inside the convex hull of three polygon
/// vertices (though not necessarily inside the polygon for highly concave shapes,
/// the largest triangle heuristic works well in practice for post-split faces).
///
/// Returns `None` if fewer than 3 vertices or all triangles are degenerate.
pub fn compute_largest_triangle_centroid(vertices: &[[f64; 3]]) -> Option<[f64; 3]> {
    if vertices.len() < 3 {
        return None;
    }

    let v0 = vertices[0];
    let mut best_area_sq = -1.0_f64;
    let mut best_centroid: Option<[f64; 3]> = None;

    for i in 1..vertices.len() - 1 {
        let v1 = vertices[i];
        let v2 = vertices[i + 1];

        let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
        let c = forge_math::linalg::cross(e1, e2);
        let area_sq = forge_math::linalg::norm_sq(c);

        if area_sq > best_area_sq {
            best_area_sq = area_sq;
            best_centroid = Some([
                (v0[0] + v1[0] + v2[0]) / 3.0,
                (v0[1] + v1[1] + v2[1]) / 3.0,
                (v0[2] + v1[2] + v2[2]) / 3.0,
            ]);
        }
    }

    if best_area_sq <= 0.0 {
        return None;
    }

    best_centroid
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TOLERANCE: f64 = 1e-10;

    #[test]
    fn unit_square_area_is_one() {
        let verts = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let area = compute_polygon_area(&verts);
        assert!(
            (area - 1.0).abs() < TEST_TOLERANCE,
            "Expected 1.0, got {area}"
        );
    }

    #[test]
    fn centroid_of_unit_square() {
        let verts = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let centroid = compute_polygon_centroid(&verts).unwrap();
        assert!((centroid[0] - 0.5).abs() < TEST_TOLERANCE);
        assert!((centroid[1] - 0.5).abs() < TEST_TOLERANCE);
        assert!((centroid[2] - 0.0).abs() < TEST_TOLERANCE);
    }

    #[test]
    fn largest_triangle_centroid_of_triangle() {
        let verts = [[0.0, 0.0, 0.0], [3.0, 0.0, 0.0], [0.0, 3.0, 0.0]];
        let centroid = compute_largest_triangle_centroid(&verts).unwrap();
        assert!((centroid[0] - 1.0).abs() < TEST_TOLERANCE);
        assert!((centroid[1] - 1.0).abs() < TEST_TOLERANCE);
        assert!((centroid[2] - 0.0).abs() < TEST_TOLERANCE);
    }

    #[test]
    fn largest_triangle_centroid_picks_biggest_triangle() {
        let verts = [
            [0.0, 0.0, 0.0],
            [0.001, 0.0, 0.0],
            [10.0, 0.0, 0.0],
            [10.0, 10.0, 0.0],
        ];
        let centroid = compute_largest_triangle_centroid(&verts).unwrap();
        assert!(
            centroid[0] > 3.0,
            "Should pick third triangle, got x={}",
            centroid[0]
        );
    }

    #[test]
    fn largest_triangle_centroid_returns_none_for_degenerate() {
        let verts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        assert!(compute_largest_triangle_centroid(&verts).is_none());
    }

    #[test]
    fn largest_triangle_centroid_returns_none_for_collinear() {
        let verts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        assert!(compute_largest_triangle_centroid(&verts).is_none());
    }

    #[test]
    fn largest_triangle_centroid_3d_tilted() {
        let verts = [[0.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0]];
        let centroid = compute_largest_triangle_centroid(&verts).unwrap();
        let expected = [1.0 / 3.0, 1.0 / 3.0, 2.0 / 3.0];
        assert!((centroid[0] - expected[0]).abs() < TEST_TOLERANCE);
        assert!((centroid[1] - expected[1]).abs() < TEST_TOLERANCE);
        assert!((centroid[2] - expected[2]).abs() < TEST_TOLERANCE);
    }
}
