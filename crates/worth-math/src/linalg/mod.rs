//! Pure linear algebra primitives for 3D vectors and 3×3 matrices.
//!
//! These are the shared building blocks used by `worth-geom` (plane
//! intersections, vertex resolution, reference frames) and eventually
//! by line-plane, surface-surface, and triangle tests.
//!
//! All functions are deterministic, allocation-free, and side-effect-free.

/// Dot product of two 3D vectors.
pub fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Component-wise subtraction of two 3D vectors: `a - b`.
pub fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Scalar multiplication of a 3D vector.
pub fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

/// Component-wise minimum of two 3D vectors.
pub fn component_min(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])]
}

/// Component-wise maximum of two 3D vectors.
pub fn component_max(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]
}

/// Cross product of two 3D vectors.
pub fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Determinant of a 3×3 matrix given its three row vectors.
pub fn det3_rows(r0: [f64; 3], r1: [f64; 3], r2: [f64; 3]) -> f64 {
    r0[0] * (r1[1] * r2[2] - r1[2] * r2[1]) - r0[1] * (r1[0] * r2[2] - r1[2] * r2[0])
        + r0[2] * (r1[0] * r2[1] - r1[1] * r2[0])
}

/// Squared Euclidean norm of a 3D vector.
pub fn norm_sq(v: [f64; 3]) -> f64 {
    v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
}

/// Euclidean norm of a 3D vector.
pub fn norm(v: [f64; 3]) -> f64 {
    norm_sq(v).sqrt()
}

/// Squared Euclidean norm of a 2D vector.
pub fn norm_sq_2d(v: [f64; 2]) -> f64 {
    v[0] * v[0] + v[1] * v[1]
}

/// Euclidean norm of a 2D vector.
pub fn norm_2d(v: [f64; 2]) -> f64 {
    norm_sq_2d(v).sqrt()
}

/// Squared Euclidean distance between two 3D points.
pub fn distance_sq(a: [f64; 3], b: [f64; 3]) -> f64 {
    norm_sq(sub(a, b))
}

pub fn normalize_checked(v: [f64; 3]) -> Option<[f64; 3]> {
    let len = norm(v);
    if !len.is_finite() || len == 0.0 {
        return None;
    }
    Some([v[0] / len, v[1] / len, v[2] / len])
}

pub fn normalize_checked_2d(v: [f64; 2]) -> Option<[f64; 2]> {
    let len = norm_2d(v);
    if !len.is_finite() || len == 0.0 {
        return None;
    }
    Some([v[0] / len, v[1] / len])
}

/// Compute a reference direction perpendicular to the given vector.
pub fn compute_perpendicular_direction(n: [f64; 3]) -> [f64; 3] {
    let candidates = [[1.0_f64, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let mut best = candidates[0];
    let mut min_dot = f64::MAX;
    for c in &candidates {
        let d = dot(*c, n).abs();
        if d < min_dot {
            min_dot = d;
            best = *c;
        }
    }
    let proj = sub(best, scale(n, dot(best, n)));
    normalize_checked(proj).unwrap_or(best)
}

/// 2D cross product (perpendicular dot product).
///
/// Equivalent to the z-component of the 3D cross product of
/// `(a[0], a[1], 0)` and `(b[0], b[1], 0)`, or the determinant
/// of the 2×2 matrix `[a | b]`.
///
/// Sign convention: positive when `b` is counter-clockwise from `a`.
pub fn cross_2d(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

/// Compute a spatial hash from a 3D position.
///
/// Quantizes coordinates to a grid and hashes them for
/// deterministic spatial tie-breaking.
///
/// # Parameters
/// - `position`: raw f64 coordinates
/// - `grid_scale`: multiplier for quantization (e.g. 1e6 for micrometer grid on meter unit)
pub fn compute_spatial_hash(position: &[f64; 3], grid_scale: f64) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let quantized = [
        (position[0] * grid_scale).round() as i64,
        (position[1] * grid_scale).round() as i64,
        (position[2] * grid_scale).round() as i64,
    ];

    quantized.iter().fold(FNV_OFFSET, |h, &coord| {
        h.wrapping_mul(FNV_PRIME) ^ (coord as u64)
    })
}

/// Check whether two 3D normals point in the same general direction.
///
/// Returns `true` if `dot(a, b) > 0`, i.e. the angle between them is less than 90°.
pub fn normals_aligned(a: [f64; 3], b: [f64; 3]) -> bool {
    dot(a, b) > 0.0
}

/// Compute the projection direction for the intersection line of two planes.
///
/// Returns `cross(normal_a, normal_b)`, normalized if longer than `min_len`.
/// Falls back to `compute_perpendicular_direction(normal_b)` when the planes
/// are parallel (cross product too short). This is the canonical reference
/// direction for sorting vertices along a plane-cut chord.
///
/// Used by: Boolean face splitting (sort cut vertices), Fillet sweep direction,
/// NURBS trim curve orientation.
pub fn plane_cut_direction(normal_a: [f64; 3], normal_b: [f64; 3], min_len_sq: f64) -> [f64; 3] {
    let dir = cross(normal_a, normal_b);
    if norm_sq(dir) > min_len_sq {
        dir
    } else {
        compute_perpendicular_direction(normal_b)
    }
}

/// Sort 3D points by their signed projection onto `direction`.
///
/// Points are ordered from most-negative to most-positive projection.
/// Ties are broken by the original order (stable sort).
///
/// Used to order cut vertices along a plane chord before pairing them
/// for MakeEdgeFace insertion. Also useful for fillet arc endpoint ordering.
pub fn sort_points_along_direction<T: Clone>(
    mut items: Vec<(T, [f64; 3])>,
    direction: [f64; 3],
) -> Vec<(T, [f64; 3])> {
    items.sort_by(|(_, pa), (_, pb)| {
        let ta = dot(*pa, direction);
        let tb = dot(*pb, direction);
        ta.partial_cmp(&tb).unwrap_or(std::cmp::Ordering::Equal)
    });
    items
}

/// Lexicographic ordering of two 3D points by (x, y, z).
///
/// Produces a total order stable under any coordinate permutation.
/// Used wherever 3D points need deterministic sorting independent of
/// a specific projection direction: NURBS control point dedup, fillet
/// endpoint ordering, interval canonicalization, curve sampling dedup.
pub fn compare_points_lex(a: &[f64; 3], b: &[f64; 3]) -> std::cmp::Ordering {
    a[0].partial_cmp(&b[0])
        .unwrap_or(std::cmp::Ordering::Equal)
        .then_with(|| a[1].partial_cmp(&b[1]).unwrap_or(std::cmp::Ordering::Equal))
        .then_with(|| a[2].partial_cmp(&b[2]).unwrap_or(std::cmp::Ordering::Equal))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_orthogonal_is_zero() {
        let result = dot([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!((result).abs() < 1e-15);
    }

    #[test]
    fn dot_parallel_is_product_of_lengths() {
        let result = dot([3.0, 0.0, 0.0], [4.0, 0.0, 0.0]);
        assert!((result - 12.0).abs() < 1e-15);
    }

    #[test]
    fn cross_axis_aligned() {
        let result = cross([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!((result[0]).abs() < 1e-15);
        assert!((result[1]).abs() < 1e-15);
        assert!((result[2] - 1.0).abs() < 1e-15);
    }

    #[test]
    fn cross_anticommutative() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let ab = cross(a, b);
        let ba = cross(b, a);
        assert!((ab[0] + ba[0]).abs() < 1e-15);
        assert!((ab[1] + ba[1]).abs() < 1e-15);
        assert!((ab[2] + ba[2]).abs() < 1e-15);
    }

    #[test]
    fn det3_identity_rows() {
        let result = det3_rows([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        assert!((result - 1.0).abs() < 1e-15);
    }

    #[test]
    fn det3_singular_is_zero() {
        let result = det3_rows([1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!((result).abs() < 1e-15);
    }

    #[test]
    fn norm_sq_unit_vector() {
        let result = norm_sq([1.0, 0.0, 0.0]);
        assert!((result - 1.0).abs() < 1e-15);
    }

    #[test]
    fn normalize_checked_unit_result() {
        let result = normalize_checked([3.0, 4.0, 0.0]).unwrap();
        let len = norm(result);
        assert!((len - 1.0).abs() < 1e-15);
    }

    #[test]
    fn normalize_checked_2d_unit_result() {
        let result = normalize_checked_2d([3.0, 4.0]).unwrap();
        let len = norm_2d(result);
        assert!((len - 1.0).abs() < 1e-15);
    }

    #[test]
    fn normalize_checked_zero_returns_none() {
        let result = normalize_checked([0.0, 0.0, 0.0]);
        assert!(result.is_none());
    }

    #[test]
    fn normalize_checked_nan_returns_none() {
        let result = normalize_checked([f64::NAN, 1.0, 0.0]);
        assert!(result.is_none());
    }

    #[test]
    fn normalize_checked_inf_returns_none() {
        let result = normalize_checked([f64::INFINITY, 0.0, 0.0]);
        assert!(result.is_none());
    }

    #[test]
    fn spatial_hash_is_deterministic() {
        let pos = [1.5, 2.5, 3.5];
        let scale = 1e6;
        assert_eq!(
            compute_spatial_hash(&pos, scale),
            compute_spatial_hash(&pos, scale)
        );
    }

    #[test]
    fn spatial_hash_differs_for_different_positions() {
        let scale = 1e6;
        let a = compute_spatial_hash(&[1.0, 0.0, 0.0], scale);
        let b = compute_spatial_hash(&[0.0, 1.0, 0.0], scale);
        assert_ne!(a, b);
    }
    #[test]
    fn cross_2d_axis_aligned() {
        let result = cross_2d([1.0, 0.0], [0.0, 1.0]);
        assert!((result - 1.0).abs() < 1e-15);
    }

    #[test]
    fn cross_2d_anticommutative() {
        let a = [3.0, 7.0];
        let b = [2.0, 5.0];
        let ab = cross_2d(a, b);
        let ba = cross_2d(b, a);
        assert!((ab + ba).abs() < 1e-15);
    }

    #[test]
    fn cross_2d_parallel_is_zero() {
        let result = cross_2d([2.0, 4.0], [3.0, 6.0]);
        assert!(result.abs() < 1e-15);
    }

    #[test]
    fn cross_2d_matches_3d_z_component() {
        let a2 = [3.0, 7.0];
        let b2 = [2.0, 5.0];
        let result_2d = cross_2d(a2, b2);
        let result_3d = cross([3.0, 7.0, 0.0], [2.0, 5.0, 0.0]);
        assert!((result_2d - result_3d[2]).abs() < 1e-15);
    }

    #[test]
    fn normals_aligned_same_direction() {
        assert!(normals_aligned([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    }

    #[test]
    fn normals_aligned_opposite_direction() {
        assert!(!normals_aligned([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn normals_aligned_orthogonal() {
        assert!(!normals_aligned([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    }

    #[test]
    fn normals_aligned_acute_angle() {
        assert!(normals_aligned([1.0, 0.0, 0.0], [0.5, 0.5, 0.0]));
    }
}
