//! DOMAIN: Plane Primitive
//! INVARIANTS:
//! - Plane normals are always non-zero (validated at construction)
//! - Exact rational coefficients enable certified classification (D3)
//! - f64 caches are derived from rationals — never the other way around
//!
//! DEPENDENCIES: `forge-math` (Rational, predicates, sign, error)

mod eval;

pub use eval::{classify_point, classify_point_exact, signed_distance, intersect_three_planes,
               intersect_three_planes_exact, intersect_edge_plane, to_plane_relation, exact_eq, coplanar_eq,
               are_parallel_exact, normals_aligned_exact};

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

#[cfg(test)]
mod tests;
