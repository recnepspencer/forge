//! Evaluation logic for the Plane primitive.

use forge_core::KernelError;
use forge_math::linalg::{cross, det3_rows, norm_sq};
use forge_math::predicates::orient3d::orient3d;
use forge_math::sign::CertifiedTriSign;

use super::schema::{Plane, PlaneRelation};

/// Classify a point relative to a plane using `orient3d` → `CertifiedTriSign`.
///
/// This is the certified path (D3): the classification flows through the
/// filtered predicate pipeline, not raw f64 comparisons.
///
/// The classification uses three reference points on the plane plus the
/// query point to form an `orient3d` evaluation.
///
/// # Errors
///
/// Returns `KernelError::InternalError` if orient3d fails (e.g. due to
/// degenerate reference points from a non-finite plane).
pub fn classify_point(plane: &Plane, point: &[f64; 3]) -> Result<CertifiedTriSign, KernelError> {
    let [a, b, c] = compute_reference_points(plane);
    orient3d(a, b, c, *point).map_err(|e| {
        KernelError::InternalError {
            message: format!("orient3d failed on plane reference points: {}", e),
            context: None,
        }
    })
}

/// Convert a `CertifiedTriSign` to a `PlaneRelation`.
///
/// `orient3d` returns `Neg` when `d` is on the positive side of the
/// oriented plane through `(a, b, c)`, and `Pos` for the negative side.
/// This matches the standard "left-hand rule" orientation convention.
pub fn to_plane_relation(sign: &CertifiedTriSign) -> PlaneRelation {
    match sign.sign() {
        forge_math::sign::TriSign::Neg => PlaneRelation::Above,
        forge_math::sign::TriSign::Zero => PlaneRelation::On,
        forge_math::sign::TriSign::Pos => PlaneRelation::Below,
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

/// Solve for the intersection point of three planes using Cramer's rule.
///
/// Given planes `a*x + b*y + c*z + d = 0` for each plane, solves the
/// 3×3 linear system. Returns `KernelError::InvalidInput` if the
/// planes are parallel or nearly so (determinant ≈ 0).
///
/// The `degeneracy` parameter controls the minimum acceptable |det|.
pub fn intersect_three_planes(
    p0: &Plane,
    p1: &Plane,
    p2: &Plane,
    degeneracy: f64,
) -> Result<[f64; 3], KernelError> {
    let n0 = p0.raw_normal();
    let n1 = p1.raw_normal();
    let n2 = p2.raw_normal();
    let d0 = -p0.raw_offset();
    let d1 = -p1.raw_offset();
    let d2 = -p2.raw_offset();

    let det = det3_rows(n0, n1, n2);

    if det.abs() < degeneracy {
        return Err(KernelError::InvalidInput {
            message: "Three planes are nearly parallel or degenerate (det ≈ 0)".to_string(),
            context: None,
        });
    }

    let inv_det = 1.0 / det;

    let x = det3_rows([d0, n0[1], n0[2]], [d1, n1[1], n1[2]], [d2, n2[1], n2[2]])
        * inv_det;
    let y = det3_rows([n0[0], d0, n0[2]], [n1[0], d1, n1[2]], [n2[0], d2, n2[2]])
        * inv_det;
    let z = det3_rows([n0[0], n0[1], d0], [n1[0], n1[1], d1], [n2[0], n2[1], d2])
        * inv_det;

    Ok([x, y, z])
}

/// Compute three non-collinear reference points on a plane.
///
/// These points are used for `orient3d` classification.
/// The strategy: pick two directions orthogonal to the normal,
/// then generate three points at `origin`, `origin + u`, `origin + v`.
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

/// Check if two planes are coplanar within the given tolerances.
///
/// * `angle_epsilon`: Tolerance for parallelism (squared magnitude of cross product).
/// * `offset_epsilon`: Tolerance for offset difference.
///
/// Returns true if planes have same or opposite orientation and matching offsets.
pub fn is_coplanar(a: &Plane, b: &Plane, angle_epsilon: f64, offset_epsilon: f64) -> bool {
    let na = a.normal();
    let nb = b.normal();
    
    // Check parallelism using cross product magnitude
    let cx = na[1] * nb[2] - na[2] * nb[1];
    let cy = na[2] * nb[0] - na[0] * nb[2];
    let cz = na[0] * nb[1] - na[1] * nb[0];
    let cross_sq = cx * cx + cy * cy + cz * cz;

    if cross_sq > angle_epsilon {
        return false;
    }

    let dot = na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2];
    let da = a.raw_offset();
    let db = b.raw_offset();

    if dot > 0.0 {
        // Same orientation
        (da - db).abs() < offset_epsilon
    } else {
        // Opposite orientation
        (da + db).abs() < offset_epsilon
    }
}
