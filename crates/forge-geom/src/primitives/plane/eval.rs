//! Pure evaluation logic for plane operations.
//!
//! DOMAIN: Point classification, plane intersection, and plane comparison.
//!
//! DEPENDENCIES: `forge-math` (Rational, predicates, sign, linalg)

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

    let reference = all_pairs.iter().find_map(|&(ai, bi)| {
        match (ai.is_zero(), bi.is_zero()) {
            (false, false) => Some((ai.clone() * bi.clone()).sign()),
            _ => None,
        }
    });

    let has_mismatch = all_pairs.iter().any(|&(ai, bi)| {
        matches!((ai.is_zero(), bi.is_zero()), (true, false) | (false, true))
    });

    if has_mismatch {
        return false;
    }

    let positive_ratio = matches!(reference, Some(forge_math::sign::TriSign::Pos));
    if !positive_ratio {
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

/// Test whether two planes represent the same geometric surface.
///
/// Unlike `exact_eq` (which requires same normal direction, i.e. same
/// half-space orientation), `coplanar_eq` returns true for both same-direction
/// AND anti-parallel normals. This is the correct test for detecting shared
/// geometric planes in boolean operations, where two touching faces have
/// opposite normals.
///
/// Uses exact rational arithmetic — no tolerances.
/// Delegates the normal parallelism check to `are_parallel_exact`.
pub fn coplanar_eq(a: &Plane, b: &Plane) -> bool {
    if !are_parallel_exact(a, b) {
        return false;
    }

    let (a0, a1, a2, a3) = a.exact_coefficients();
    let (b0, b1, b2, b3) = b.exact_coefficients();

    let check_a = &(a0 * b3) - &(b0 * a3);
    if !check_a.is_zero() {
        return false;
    }
    let check_b = &(a1 * b3) - &(b1 * a3);
    if !check_b.is_zero() {
        return false;
    }
    let check_c = &(a2 * b3) - &(b2 * a3);
    if !check_c.is_zero() {
        return false;
    }

    true
}

/// Test whether two planes have parallel normals (same or opposite direction).
///
/// Uses exact rational arithmetic — the cross product `n1 × n2` is computed
/// in rational coordinates and checked for exact zero. No tolerance needed.
///
/// This is the D3-compliant replacement for float-based parallelism checks.
/// It returns `true` for both same-direction and anti-parallel normals.
pub fn are_parallel_exact(a: &Plane, b: &Plane) -> bool {
    let (a0, a1, a2, _) = a.exact_coefficients();
    let (b0, b1, b2, _) = b.exact_coefficients();

    let cx = &(a0 * b1) - &(a1 * b0);
    let cy = &(a1 * b2) - &(a2 * b1);
    let cz = &(a0 * b2) - &(a2 * b0);

    cx.is_zero() && cy.is_zero() && cz.is_zero()
}

/// Test whether two planes have normals pointing in the same general direction.
///
/// Returns `true` if the exact rational dot product of the normals is positive
/// (angle between normals is less than 90°). Uses exact arithmetic — no tolerance.
///
/// This is the D3-compliant replacement for `normals_aligned(raw_normal(), raw_normal())`.
pub fn normals_aligned_exact(a: &Plane, b: &Plane) -> bool {
    let (a0, a1, a2, _) = a.exact_coefficients();
    let (b0, b1, b2, _) = b.exact_coefficients();

    let dot = &(&(a0 * b0) + &(a1 * b1)) + &(a2 * b2);
    dot.sign() == forge_math::sign::TriSign::Pos
}
