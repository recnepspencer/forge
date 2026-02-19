//! DOMAIN: Plane Primitive
//! INVARIANTS:
//! - Plane normals are always non-zero (validated at construction)
//! - Point classification uses `orient3d` → `CertifiedTriSign` (D3)
//! - No raw f64 comparisons for topology decisions (D0)
//!
//! DEPENDENCIES: `forge-math` (predicates, sign, error)

pub use eval::{classify_point, signed_distance, intersect_three_planes, to_plane_relation, is_coplanar};

use forge_math::MathError;
use serde::{Deserialize, Serialize};

/// A plane in 3D space defined by the equation `n·p + d = 0`.
///
/// The normal vector `[a, b, c]` and offset `d` are stored as `f64`.
/// Exact rational fallback is deferred to the predicate call-site
/// (through the filtered evaluation pipeline in `forge-math`).
///
/// # Construction
///
/// Use [`Plane::try_new`] which validates that the normal is non-zero.
/// This ensures the plane is always geometrically meaningful.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plane {
    /// Unit normal vector `[a, b, c]` (normalized at construction).
    normal: [f64; 3],
    /// Signed offset `d` such that `a*x + b*y + c*z + d = 0`.
    offset: f64,
    /// Original (un-normalized) normal, preserved for exact arithmetic.
    raw_normal: [f64; 3],
    /// Original offset before normalization.
    raw_offset: f64,
}

impl Plane {
    /// Construct a plane from normal `[a, b, c]` and offset `d`.
    ///
    /// The equation is `a*x + b*y + c*z + d = 0`.
    /// Returns `MathError::InvalidInput` if the normal is zero-length.
    pub fn try_new(normal: [f64; 3], offset: f64) -> Result<Self, MathError> {
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
        let len_sq = normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2];
        if len_sq == 0.0 {
            return Err(MathError::InvalidInput(
                "Plane normal must be non-zero".to_string(),
            ));
        }
        let len = len_sq.sqrt();
        let unit_normal = [normal[0] / len, normal[1] / len, normal[2] / len];
        let unit_offset = offset / len;

        Ok(Self {
            normal: unit_normal,
            offset: unit_offset,
            raw_normal: normal,
            raw_offset: offset,
        })
    }

    /// Construct a plane from a point on the plane and a normal direction.
    ///
    /// The offset is computed as `d = -(n·p)`.
    pub fn from_point_normal(
        point: [f64; 3],
        normal: [f64; 3],
    ) -> Result<Self, MathError> {
        let offset = -(normal[0] * point[0] + normal[1] * point[1] + normal[2] * point[2]);
        Self::try_new(normal, offset)
    }

    /// The unit normal vector.
    pub fn normal(&self) -> [f64; 3] {
        self.normal
    }

    /// The signed offset (after normalization).
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// The raw (un-normalized) normal, for exact arithmetic paths.
    pub fn raw_normal(&self) -> [f64; 3] {
        self.raw_normal
    }

    /// The raw (un-normalized) offset, for exact arithmetic paths.
    pub fn raw_offset(&self) -> f64 {
        self.raw_offset
    }

    /// Flip the orientation of the plane (negate normal and offset).
    pub fn flip(&mut self) {
        self.normal = [-self.normal[0], -self.normal[1], -self.normal[2]];
        self.offset = -self.offset;
        self.raw_normal = [-self.raw_normal[0], -self.raw_normal[1], -self.raw_normal[2]];
        self.raw_offset = -self.raw_offset;
    }
}

/// Result of classifying a point relative to a plane.
///
/// Derived from `CertifiedTriSign` — the classification is always
/// backed by a certified predicate evaluation (D3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaneRelation {
    /// Point is on the positive side of the plane (same side as normal).
    Above,
    /// Point lies exactly on the plane (genuine coincidence, not noise).
    On,
    /// Point is on the negative side of the plane (opposite to normal).
    Below,
}

// =========================================================================
// EVALUATION LOGIC
// =========================================================================

mod eval {
use forge_math::MathError;
use forge_math::linalg::{cross, det3_rows, norm_sq};
use forge_math::predicates::orient3d::orient3d;
use forge_math::sign::CertifiedTriSign;

use super::{Plane, PlaneRelation};

/// Classify a point relative to a plane using `orient3d` → `CertifiedTriSign`.
///
/// This is the certified path (D3): the classification flows through the
/// filtered predicate pipeline, not raw f64 comparisons.
///
/// # Errors
///
/// Returns `MathError::InternalError` if orient3d fails.
pub fn classify_point(plane: &Plane, point: &[f64; 3]) -> Result<CertifiedTriSign, MathError> {
    let [a, b, c] = compute_reference_points(plane);
    orient3d(a, b, c, *point).map_err(|e| {
        MathError::InternalError(
            format!("orient3d failed on plane reference points: {}", e),
        )
    })
}

/// Convert a `CertifiedTriSign` to a `PlaneRelation`.
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
/// The `degeneracy` parameter controls the minimum acceptable |det|.
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

    let det = det3_rows(n0, n1, n2);

    if det.abs() < degeneracy {
        return Err(MathError::InvalidInput(
            "Three planes are nearly parallel or degenerate (det ≈ 0)".to_string(),
        ));
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
pub fn is_coplanar(a: &Plane, b: &Plane, angle_epsilon: f64, offset_epsilon: f64) -> bool {
    let na = a.normal();
    let nb = b.normal();
    
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
        (da - db).abs() < offset_epsilon
    } else {
        (da + db).abs() < offset_epsilon
    }
}
} // end mod eval

#[cfg(test)]
mod tests {
    use forge_math::sign::TriSign;
    use crate::primitives::plane::{Plane, PlaneRelation, classify_point, signed_distance, intersect_three_planes, to_plane_relation};

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
        assert!((raw[0] - 3.0).abs() < TEST_DEGENERACY);
        assert!((raw[1] - 4.0).abs() < TEST_DEGENERACY);
    }
}
