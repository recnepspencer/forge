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

#[cfg(test)]
mod tests {
    use super::*;

    /// Default tolerance for polygon tests.
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
        assert!((area - 1.0).abs() < TEST_TOLERANCE, "Expected 1.0, got {area}");
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
}
