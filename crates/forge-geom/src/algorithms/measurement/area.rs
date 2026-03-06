//! Triangle area measurement in 3D.
//!
//! DOMAIN: Computes the area of a triangle from three 3D vertex positions
//! using the cross-product magnitude formula. Pure geometry, no topology.

use forge_math::linalg;

/// 3D triangle area via cross-product magnitude: `0.5 * |AB × AC|`.
///
/// This is the canonical triangle area function — all kernel code that
/// needs triangle areas must call this, never inline the math.
#[inline]
pub fn triangle_area_3d(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3]) -> f64 {
    let ab = linalg::sub(*b, *a);
    let ac = linalg::sub(*c, *a);
    let cross = linalg::cross(ab, ac);
    0.5 * linalg::norm(cross)
}

/// Sine of the dihedral angle between two unit normals.
///
/// For unit vectors `na` and `nb`, the dihedral sine is
/// `sqrt(1 - dot(na, nb)²)`, clamped to `[0, 1]`.
///
/// Returns `(sin_angle, dot)` so callers can use the dot product
/// for convexity classification without recomputing it.
#[inline]
pub fn dihedral_sine(na: &[f64; 3], nb: &[f64; 3]) -> (f64, f64) {
    let dot = linalg::dot(*na, *nb);
    let sin_sq = (1.0 - dot * dot).max(0.0);
    (sin_sq.sqrt(), dot)
}

/// Build an orthonormal tangent frame `(u, v)` perpendicular to normal `n`.
///
/// Uses `forge_math::linalg::compute_perpendicular_direction` for the
/// initial candidate, then cross-products for the second axis.
pub fn tangent_frame(n: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    let u = linalg::compute_perpendicular_direction(*n);
    let u = linalg::normalize_checked(linalg::cross(*n, u))
        .unwrap_or(u);
    let v = linalg::cross(*n, u);
    (u, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_right_triangle_area() {
        let area = triangle_area_3d(
            &[0.0, 0.0, 0.0],
            &[1.0, 0.0, 0.0],
            &[0.0, 1.0, 0.0],
        );
        assert!((area - 0.5).abs() < 1e-15);
    }

    #[test]
    fn degenerate_collinear_triangle() {
        let area = triangle_area_3d(
            &[0.0, 0.0, 0.0],
            &[1.0, 0.0, 0.0],
            &[2.0, 0.0, 0.0],
        );
        assert!(area.abs() < 1e-15);
    }

    #[test]
    fn dihedral_perpendicular_normals() {
        let (sin_angle, dot) = dihedral_sine(
            &[1.0, 0.0, 0.0],
            &[0.0, 1.0, 0.0],
        );
        assert!((sin_angle - 1.0).abs() < 1e-15);
        assert!(dot.abs() < 1e-15);
    }

    #[test]
    fn dihedral_parallel_normals() {
        let (sin_angle, _dot) = dihedral_sine(
            &[0.0, 0.0, 1.0],
            &[0.0, 0.0, 1.0],
        );
        assert!(sin_angle.abs() < 1e-15);
    }

    #[test]
    fn tangent_frame_orthogonal() {
        let n = [0.0, 0.0, 1.0];
        let (u, v) = tangent_frame(&n);
        assert!(linalg::dot(u, n).abs() < 1e-14);
        assert!(linalg::dot(v, n).abs() < 1e-14);
        assert!(linalg::dot(u, v).abs() < 1e-14);
    }
}
