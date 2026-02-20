//! DOMAIN: Plane Primitive
//! INVARIANTS:
//! - Plane normals are always non-zero (validated at construction)
//! - Exact rational coefficients enable certified classification (D3)
//! - f64 caches are derived from rationals — never the other way around
//!
//! DEPENDENCIES: `forge-math` (Rational, predicates, sign, error)

pub use eval::{classify_point, classify_point_exact, signed_distance, intersect_three_planes,
               intersect_three_planes_exact, to_plane_relation, exact_eq};

use forge_math::MathError;
use forge_math::arithmetic::Rational;
use serde::{Deserialize, Serialize};

/// A plane in 3D space defined by the equation `ax + by + cz + d = 0`.
///
/// Coefficients are stored as exact `Rational` values. For axis-aligned
/// planes these are trivial integers (zero overhead). For planes derived
/// from intersections, they capture the exact result of rational arithmetic.
///
/// Cached f64 approximations are provided for performance-critical paths
/// (BVH, AABB, rendering) but must NOT drive topology decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plane {
    /// Exact rational coefficients: ax + by + cz + d = 0
    a: Rational,
    b: Rational,
    c: Rational,
    d: Rational,
    /// Cached f64 unit normal (derived from rationals).
    f64_normal: [f64; 3],
    /// Cached f64 offset (derived from rationals, after normalization).
    f64_offset: f64,
}

impl Plane {
    /// Construct a plane from exact rational coefficients.
    ///
    /// Validates that the normal `(a, b, c)` is non-zero.
    pub fn from_rationals(a: Rational, b: Rational, c: Rational, d: Rational) -> Result<Self, MathError> {
        if a.is_zero() && b.is_zero() && c.is_zero() {
            return Err(MathError::InvalidInput(
                "Plane normal must be non-zero".to_string(),
            ));
        }

        let fa = a.to_f64_approx();
        let fb = b.to_f64_approx();
        let fc = c.to_f64_approx();
        let fd = d.to_f64_approx();

        let len = (fa * fa + fb * fb + fc * fc).sqrt();
        let f64_normal = [fa / len, fb / len, fc / len];
        let f64_offset = fd / len;

        Ok(Self { a, b, c, d, f64_normal, f64_offset })
    }

    /// Construct an axis-aligned plane.
    ///
    /// `axis` is 0=X, 1=Y, 2=Z. `sign` is +1 or -1 for the normal direction.
    /// `offset` is the rational offset `d` in `ax + by + cz + d = 0`.
    pub fn axis_aligned(axis: usize, sign: i64, offset: Rational) -> Result<Self, MathError> {
        if axis > 2 {
            return Err(MathError::InvalidInput("axis must be 0, 1, or 2".into()));
        }
        if sign != 1 && sign != -1 {
            return Err(MathError::InvalidInput("sign must be 1 or -1".into()));
        }
        let mut coeffs = [Rational::zero(), Rational::zero(), Rational::zero()];
        coeffs[axis] = Rational::from_integer(sign);
        Self::from_rationals(coeffs[0].clone(), coeffs[1].clone(), coeffs[2].clone(), offset)
    }

    /// Construct from f64 normal and offset (lossless IEEE754 → Rational conversion).
    ///
    /// This is the migration path from all existing `Plane::try_new` call sites.
    /// Every finite f64 has an exact rational representation, so no precision is lost.
    pub fn try_from_f64(normal: [f64; 3], offset: f64) -> Result<Self, MathError> {
        if !normal[0].is_finite() || !normal[1].is_finite() || !normal[2].is_finite() {
            return Err(MathError::InvalidInput(
                "Plane normal contains non-finite values".to_string(),
            ));
        }
        if !offset.is_finite() {
            return Err(MathError::InvalidInput(
                "Plane offset is non-finite".to_string(),
            ));
        }
        let a = Rational::try_from_f64(normal[0])?;
        let b = Rational::try_from_f64(normal[1])?;
        let c = Rational::try_from_f64(normal[2])?;
        let d = Rational::try_from_f64(offset)?;
        Self::from_rationals(a, b, c, d)
    }

    /// Construct from a point on the plane and a normal direction (f64).
    ///
    /// The offset is computed as `d = -(n·p)` in exact rational arithmetic.
    pub fn from_point_normal(
        point: [f64; 3],
        normal: [f64; 3],
    ) -> Result<Self, MathError> {
        let a = Rational::try_from_f64(normal[0])?;
        let b = Rational::try_from_f64(normal[1])?;
        let c = Rational::try_from_f64(normal[2])?;
        let px = Rational::try_from_f64(point[0])?;
        let py = Rational::try_from_f64(point[1])?;
        let pz = Rational::try_from_f64(point[2])?;
        let d = -((&a * &px) + (&b * &py) + (&c * &pz));
        Self::from_rationals(a, b, c, d)
    }

    /// Backwards-compatible constructor (delegates to `try_from_f64`).
    pub fn try_new(normal: [f64; 3], offset: f64) -> Result<Self, MathError> {
        Self::try_from_f64(normal, offset)
    }

    /// The cached unit normal vector (f64 approximation).
    pub fn normal(&self) -> [f64; 3] {
        self.f64_normal
    }

    /// The cached signed offset after normalization (f64 approximation).
    pub fn offset(&self) -> f64 {
        self.f64_offset
    }

    /// The raw (un-normalized) normal as f64, for orient3d paths.
    pub fn raw_normal(&self) -> [f64; 3] {
        [self.a.to_f64_approx(), self.b.to_f64_approx(), self.c.to_f64_approx()]
    }

    /// The raw (un-normalized) offset as f64, for orient3d paths.
    pub fn raw_offset(&self) -> f64 {
        self.d.to_f64_approx()
    }

    /// Exact rational coefficients `(a, b, c, d)`.
    pub fn exact_coefficients(&self) -> (&Rational, &Rational, &Rational, &Rational) {
        (&self.a, &self.b, &self.c, &self.d)
    }

    /// Flip the orientation of the plane (negate all coefficients).
    pub fn flip(&mut self) {
        self.a = -self.a.clone();
        self.b = -self.b.clone();
        self.c = -self.c.clone();
        self.d = -self.d.clone();
        self.f64_normal = [-self.f64_normal[0], -self.f64_normal[1], -self.f64_normal[2]];
        self.f64_offset = -self.f64_offset;
    }
}

/// Result of classifying a point relative to a plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaneRelation {
    /// Point is on the positive side of the plane (same side as normal).
    Above,
    /// Point lies exactly on the plane.
    On,
    /// Point is on the negative side of the plane (opposite to normal).
    Below,
}

// =========================================================================
// EVALUATION LOGIC
// =========================================================================

mod eval {
use forge_math::MathError;
use forge_math::arithmetic::Rational;
use forge_math::linalg::{cross, norm_sq};
use forge_math::predicates::orient3d::orient3d;
use forge_math::sign::{CertifiedTriSign, TriSign};

use super::{Plane, PlaneRelation};

/// Classify a point (f64) relative to a plane using `orient3d` → `CertifiedTriSign`.
///
/// This path is for f64 query points (e.g. from ray casting). For exact
/// classification with rational points, use `classify_point_exact`.
pub fn classify_point(plane: &Plane, point: &[f64; 3]) -> Result<CertifiedTriSign, MathError> {
    let [a, b, c] = compute_reference_points(plane);
    let (sign, _escalation) = orient3d(a, b, c, *point).map_err(|e| {
        MathError::InternalError(
            format!("orient3d failed on plane reference points: {}", e),
        )
    })?;
    Ok(sign)
}

/// Classify a point (exact rational) relative to a plane.
///
/// Computes `sign(a*px + b*py + c*pz + d)` in exact rational arithmetic.
/// No orient3d, no reference points, no floating-point error. Period.
pub fn classify_point_exact(plane: &Plane, point: &[Rational; 3]) -> TriSign {
    let (a, b, c, d) = plane.exact_coefficients();
    let dot = &(&(a * &point[0]) + &(&(b * &point[1]) + &(c * &point[2]))) + d;
    dot.sign()
}

/// Convert a `CertifiedTriSign` to a `PlaneRelation`.
pub fn to_plane_relation(sign: &CertifiedTriSign) -> PlaneRelation {
    match sign.sign() {
        TriSign::Neg => PlaneRelation::Above,
        TriSign::Zero => PlaneRelation::On,
        TriSign::Pos => PlaneRelation::Below,
    }
}

/// Compute the approximate signed distance from a point to a plane.
///
/// This is a geometry-only computation (not topology). The result
/// is approximate (f64) and must NOT be used for topology decisions.
pub fn signed_distance(plane: &Plane, point: &[f64; 3]) -> f64 {
    let n = plane.normal();
    n[0] * point[0] + n[1] * point[1] + n[2] * point[2] + plane.offset()
}

/// Solve for the intersection point of three planes using Cramer's rule (f64).
///
/// The `degeneracy` parameter controls the minimum acceptable |det|.
/// For exact intersection, use `intersect_three_planes_exact`.
pub fn intersect_three_planes(
    p0: &Plane,
    p1: &Plane,
    p2: &Plane,
    degeneracy: f64,
) -> Result<[f64; 3], MathError> {
    let n0 = p0.raw_normal();
    let n1 = p1.raw_normal();
    let n2 = p2.raw_normal();
    let d0 = -p0.raw_offset();
    let d1 = -p1.raw_offset();
    let d2 = -p2.raw_offset();

    let det = forge_math::linalg::det3_rows(n0, n1, n2);

    if det.abs() < degeneracy {
        return Err(MathError::InvalidInput(
            "Three planes are nearly parallel or degenerate (det ≈ 0)".to_string(),
        ));
    }

    let inv_det = 1.0 / det;

    let x = forge_math::linalg::det3_rows([d0, n0[1], n0[2]], [d1, n1[1], n1[2]], [d2, n2[1], n2[2]])
        * inv_det;
    let y = forge_math::linalg::det3_rows([n0[0], d0, n0[2]], [n1[0], d1, n1[2]], [n2[0], d2, n2[2]])
        * inv_det;
    let z = forge_math::linalg::det3_rows([n0[0], n0[1], d0], [n1[0], n1[1], d1], [n2[0], n2[1], d2])
        * inv_det;

    Ok([x, y, z])
}

/// Solve for the exact intersection point of three planes using rational Cramer's rule.
///
/// Returns exact `[Rational; 3]` coordinates. Returns an error only if the
/// three planes are exactly parallel/degenerate (determinant is exactly zero).
pub fn intersect_three_planes_exact(
    p0: &Plane,
    p1: &Plane,
    p2: &Plane,
) -> Result<[Rational; 3], MathError> {
    let (a0, b0, c0, d0) = p0.exact_coefficients();
    let (a1, b1, c1, d1) = p1.exact_coefficients();
    let (a2, b2, c2, d2) = p2.exact_coefficients();

    let det = rational_det3(
        a0, b0, c0,
        a1, b1, c1,
        a2, b2, c2,
    );

    if det.is_zero() {
        return Err(MathError::InvalidInput(
            "Three planes are exactly parallel or degenerate".to_string(),
        ));
    }

    let neg_d0 = -d0;
    let neg_d1 = -d1;
    let neg_d2 = -d2;

    let x = rational_det3(
        &neg_d0, b0, c0,
        &neg_d1, b1, c1,
        &neg_d2, b2, c2,
    ) / det.clone();

    let y = rational_det3(
        a0, &neg_d0, c0,
        a1, &neg_d1, c1,
        a2, &neg_d2, c2,
    ) / det.clone();

    let z = rational_det3(
        a0, b0, &neg_d0,
        a1, b1, &neg_d1,
        a2, b2, &neg_d2,
    ) / det;

    Ok([x, y, z])
}

/// Test whether two planes are exactly identical (after canonical normalization).
///
/// Two planes are the same iff their coefficient vectors `(a, b, c, d)` are
/// proportional with a positive ratio (same orientation). Proportionality is
/// verified by checking that all 6 pairwise cross-products of the 4 coefficients
/// are equal. The reference pair for the sign check is the first non-zero pair
/// found across all four coefficients — anchoring only to the normal (a,b,c)
/// fails when the normal coefficient used as the anchor is zero.
pub fn exact_eq(a: &Plane, b: &Plane) -> bool {
    let (a0, a1, a2, a3) = a.exact_coefficients();
    let (b0, b1, b2, b3) = b.exact_coefficients();

    let all_pairs: [(&Rational, &Rational); 4] = [(a0, b0), (a1, b1), (a2, b2), (a3, b3)];

    let mut positive_ratio = true;
    let mut found_nonzero = false;

    for &(ai, bi) in &all_pairs {
        match (ai.is_zero(), bi.is_zero()) {
            (false, false) => {
                if (ai.clone() * bi.clone()).sign() == forge_math::sign::TriSign::Neg {
                    positive_ratio = false;
                }
                found_nonzero = true;
                break;
            }
            (true, false) | (false, true) => return false,
            (true, true) => {}
        }
    }

    if !found_nonzero || !positive_ratio {
        return false;
    }

    let cross_01 = &(a0.clone() * b1.clone()) == &(a1.clone() * b0.clone());
    let cross_02 = &(a0.clone() * b2.clone()) == &(a2.clone() * b0.clone());
    let cross_03 = &(a0.clone() * b3.clone()) == &(a3.clone() * b0.clone());
    let cross_12 = &(a1.clone() * b2.clone()) == &(a2.clone() * b1.clone());
    let cross_13 = &(a1.clone() * b3.clone()) == &(a3.clone() * b1.clone());
    let cross_23 = &(a2.clone() * b3.clone()) == &(a3.clone() * b2.clone());

    cross_01 && cross_02 && cross_03 && cross_12 && cross_13 && cross_23
}

/// 3×3 determinant in exact rational arithmetic.
fn rational_det3(
    a00: &Rational, a01: &Rational, a02: &Rational,
    a10: &Rational, a11: &Rational, a12: &Rational,
    a20: &Rational, a21: &Rational, a22: &Rational,
) -> Rational {
    let t0 = &(a00.clone() * (a11.clone() * a22.clone())) - &(a00.clone() * (a12.clone() * a21.clone()));
    let t1 = &(a01.clone() * (a12.clone() * a20.clone())) - &(a01.clone() * (a10.clone() * a22.clone()));
    let t2 = &(a02.clone() * (a10.clone() * a21.clone())) - &(a02.clone() * (a11.clone() * a20.clone()));
    t0 + t1 + t2
}

/// Compute three non-collinear reference points on a plane (f64 approximation).
///
/// Used by `classify_point` for f64 query points via orient3d.
/// Not needed for exact classification — `classify_point_exact` operates
/// directly on rational coefficients.
fn compute_reference_points(plane: &Plane) -> [[f64; 3]; 3] {
    let n = plane.raw_normal();
    let d = plane.raw_offset();
    let n_sq = norm_sq(n);

    let origin = [
        -n[0] * d / n_sq,
        -n[1] * d / n_sq,
        -n[2] * d / n_sq,
    ];

    let abs_n = [n[0].abs(), n[1].abs(), n[2].abs()];
    let seed = if abs_n[0] <= abs_n[1] && abs_n[0] <= abs_n[2] {
        [1.0, 0.0, 0.0]
    } else if abs_n[1] <= abs_n[2] {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };

    let u = cross(n, seed);
    let v = cross(n, u);

    let p1 = [origin[0] + u[0], origin[1] + u[1], origin[2] + u[2]];
    let p2 = [origin[0] + v[0], origin[1] + v[1], origin[2] + v[2]];

    [origin, p1, p2]
}

} // end mod eval

#[cfg(test)]
mod tests {
    use forge_math::sign::TriSign;
    use forge_math::arithmetic::Rational;
    use crate::primitives::plane::{Plane, PlaneRelation, classify_point, classify_point_exact,
                                    signed_distance, intersect_three_planes,
                                    intersect_three_planes_exact, to_plane_relation, exact_eq};

    const TEST_DEGENERACY: f64 = 1e-15;
    const TEST_TOLERANCE: f64 = 1e-10;

    #[test]
    fn construct_valid_plane() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0);
        assert!(plane.is_ok());
    }

    #[test]
    fn reject_zero_normal() {
        let plane = Plane::try_new([0.0, 0.0, 0.0], 1.0);
        assert!(plane.is_err());
    }

    #[test]
    fn reject_nan_normal() {
        let plane = Plane::try_new([f64::NAN, 0.0, 1.0], 0.0);
        assert!(plane.is_err());
    }

    #[test]
    fn reject_inf_offset() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], f64::INFINITY);
        assert!(plane.is_err());
    }

    #[test]
    fn reject_neg_inf_normal() {
        let plane = Plane::try_new([f64::NEG_INFINITY, 0.0, 0.0], 1.0);
        assert!(plane.is_err());
    }

    #[test]
    fn from_point_normal_constructs_correctly() {
        let plane = Plane::from_point_normal([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]).unwrap();
        let dist = signed_distance(&plane, &[0.0, 0.0, 5.0]);
        assert!(dist.abs() < TEST_TOLERANCE);
    }

    #[test]
    fn classify_point_above_xy_plane() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
        let sign = classify_point(&plane, &[0.0, 0.0, 5.0]).unwrap();
        assert_eq!(to_plane_relation(&sign), PlaneRelation::Above);
    }

    #[test]
    fn classify_point_below_xy_plane() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
        let sign = classify_point(&plane, &[0.0, 0.0, -3.0]).unwrap();
        assert_eq!(to_plane_relation(&sign), PlaneRelation::Below);
    }

    #[test]
    fn classify_point_on_xy_plane() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
        let sign = classify_point(&plane, &[7.0, -3.0, 0.0]).unwrap();
        assert_eq!(sign.sign(), TriSign::Zero);
    }

    #[test]
    fn signed_distance_positive_above() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
        let dist = signed_distance(&plane, &[0.0, 0.0, 3.0]);
        assert!((dist - 3.0).abs() < TEST_TOLERANCE);
    }

    #[test]
    fn signed_distance_negative_below() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
        let dist = signed_distance(&plane, &[0.0, 0.0, -2.0]);
        assert!((dist + 2.0).abs() < TEST_TOLERANCE);
    }

    #[test]
    fn intersect_axis_aligned_planes_at_origin() {
        let px = Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap();
        let py = Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap();
        let pz = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();

        let point = intersect_three_planes(&px, &py, &pz, TEST_DEGENERACY).unwrap();
        assert!((point[0]).abs() < TEST_TOLERANCE);
        assert!((point[1]).abs() < TEST_TOLERANCE);
        assert!((point[2]).abs() < TEST_TOLERANCE);
    }

    #[test]
    fn intersect_offset_planes_at_known_point() {
        let px = Plane::try_new([1.0, 0.0, 0.0], -3.0).unwrap();
        let py = Plane::try_new([0.0, 1.0, 0.0], -4.0).unwrap();
        let pz = Plane::try_new([0.0, 0.0, 1.0], -5.0).unwrap();

        let point = intersect_three_planes(&px, &py, &pz, TEST_DEGENERACY).unwrap();
        assert!((point[0] - 3.0).abs() < TEST_TOLERANCE);
        assert!((point[1] - 4.0).abs() < TEST_TOLERANCE);
        assert!((point[2] - 5.0).abs() < TEST_TOLERANCE);
    }

    #[test]
    fn intersect_parallel_planes_returns_error() {
        let p0 = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
        let p1 = Plane::try_new([0.0, 0.0, 1.0], -1.0).unwrap();
        let p2 = Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap();

        let result = intersect_three_planes(&p0, &p1, &p2, TEST_DEGENERACY);
        assert!(result.is_err());
    }

    #[test]
    fn cube_planes_produce_correct_vertex_count() {
        let planes = [
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([-1.0, 0.0, 0.0], 1.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, -1.0, 0.0], 1.0).unwrap(),
            Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),
            Plane::try_new([0.0, 0.0, -1.0], 1.0).unwrap(),
        ];

        let mut valid_vertices = 0;
        let triples: [(usize, usize, usize); 8] = [
            (0, 2, 4), (0, 2, 5), (0, 3, 4), (0, 3, 5),
            (1, 2, 4), (1, 2, 5), (1, 3, 4), (1, 3, 5),
        ];

        for (i, j, k) in triples {
            let result = intersect_three_planes(&planes[i], &planes[j], &planes[k], TEST_DEGENERACY);
            if result.is_ok() {
                valid_vertices += 1;
            }
        }

        assert_eq!(valid_vertices, 8);
    }

    #[test]
    fn plane_normal_is_normalized() {
        let plane = Plane::try_new([3.0, 4.0, 0.0], 10.0).unwrap();
        let n = plane.normal();
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        assert!((len - 1.0).abs() < TEST_TOLERANCE);
    }

    #[test]
    fn raw_normal_preserves_original() {
        let plane = Plane::try_new([3.0, 4.0, 0.0], 10.0).unwrap();
        let raw = plane.raw_normal();
        assert!((raw[0] - 3.0).abs() < TEST_TOLERANCE);
        assert!((raw[1] - 4.0).abs() < TEST_TOLERANCE);
    }

    // --- New rational-specific tests ---

    #[test]
    fn exact_eq_identical_planes() {
        let a = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
        let b = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
        assert!(exact_eq(&a, &b));
    }

    #[test]
    fn exact_eq_scaled_planes() {
        let a = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
        let b = Plane::try_new([2.0, 0.0, 0.0], -10.0).unwrap();
        assert!(exact_eq(&a, &b));
    }

    #[test]
    fn exact_eq_opposite_normals_are_different() {
        let a = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
        let b = Plane::try_new([-1.0, 0.0, 0.0], 5.0).unwrap();
        assert!(!exact_eq(&a, &b));
    }

    #[test]
    fn classify_point_exact_on_plane() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], -5.0).unwrap();
        let point = [
            Rational::from_integer(0),
            Rational::from_integer(0),
            Rational::from_integer(5),
        ];
        assert_eq!(classify_point_exact(&plane, &point), TriSign::Zero);
    }

    #[test]
    fn classify_point_exact_above_plane() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
        let point = [
            Rational::from_integer(0),
            Rational::from_integer(0),
            Rational::from_integer(5),
        ];
        assert_eq!(classify_point_exact(&plane, &point), TriSign::Pos);
    }

    #[test]
    fn intersect_three_planes_exact_matches_f64() {
        let px = Plane::try_new([1.0, 0.0, 0.0], -3.0).unwrap();
        let py = Plane::try_new([0.0, 1.0, 0.0], -4.0).unwrap();
        let pz = Plane::try_new([0.0, 0.0, 1.0], -5.0).unwrap();

        let exact = intersect_three_planes_exact(&px, &py, &pz).unwrap();
        assert_eq!(exact[0], Rational::from_integer(3));
        assert_eq!(exact[1], Rational::from_integer(4));
        assert_eq!(exact[2], Rational::from_integer(5));
    }

    #[test]
    fn axis_aligned_plane_constructs_correctly() {
        let plane = Plane::axis_aligned(2, 1, Rational::from_integer(-5)).unwrap();
        let n = plane.normal();
        assert!((n[2] - 1.0).abs() < TEST_TOLERANCE);
        assert!((plane.raw_offset() + 5.0).abs() < TEST_TOLERANCE);
    }
}
