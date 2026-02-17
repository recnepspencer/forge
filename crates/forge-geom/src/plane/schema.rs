//! Data definitions for the Plane primitive.

use forge_core::KernelError;

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
#[derive(Debug, Clone)]
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
    /// Returns `KernelError::InvalidInput` if the normal is zero-length.
    pub fn try_new(normal: [f64; 3], offset: f64) -> Result<Self, KernelError> {
        if !normal[0].is_finite() || !normal[1].is_finite() || !normal[2].is_finite() {
            return Err(KernelError::InvalidInput {
                message: "Plane normal contains non-finite values".to_string(),
                context: None,
            });
        }
        if !offset.is_finite() {
            return Err(KernelError::InvalidInput {
                message: "Plane offset is non-finite".to_string(),
                context: None,
            });
        }
        let len_sq = normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2];
        if len_sq == 0.0 {
            return Err(KernelError::InvalidInput {
                message: "Plane normal must be non-zero".to_string(),
                context: None,
            });
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
    ) -> Result<Self, KernelError> {
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
