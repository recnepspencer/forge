//! Anonymous data access trait and plane coefficient type.
//!
//! DOMAIN: Allows lower layers (`worth-geom`) to request plane data
//! from higher layers (`forge-kernel`) without depending on them.
//!
//! DEPENDENCIES: `MathError` (error type)

use crate::MathError;
use serde::{Deserialize, Serialize};

/// Validated plane coefficients `[a, b, c, d]` for the equation `ax + by + cz + d = 0`.
///
/// Construction validates that the normal `(a, b, c)` is non-zero.
/// This is the value type returned by `GeometrySource::get_plane`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlaneCoefficients {
    /// Normal x-component.
    a: f64,
    /// Normal y-component.
    b: f64,
    /// Normal z-component.
    c: f64,
    /// Signed offset.
    d: f64,
}

impl PlaneCoefficients {
    /// Create validated plane coefficients.
    ///
    /// Returns `MathError::InvalidInput` if the normal `(a, b, c)` is zero-length.
    pub fn try_new(a: f64, b: f64, c: f64, d: f64) -> Result<Self, MathError> {
        let len_sq = a * a + b * b + c * c;
        if len_sq == 0.0 {
            return Err(MathError::InvalidInput(
                "PlaneCoefficients normal must be non-zero".to_string(),
            ));
        }
        Ok(Self { a, b, c, d })
    }

    /// Create from a raw `[f64; 4]` array `[a, b, c, d]`.
    pub fn from_array(coeffs: [f64; 4]) -> Result<Self, MathError> {
        Self::try_new(coeffs[0], coeffs[1], coeffs[2], coeffs[3])
    }

    /// The normal vector `[a, b, c]`.
    pub fn normal(&self) -> [f64; 3] {
        [self.a, self.b, self.c]
    }

    /// The signed offset `d`.
    pub fn offset(&self) -> f64 {
        self.d
    }

    /// Convert to `[a, b, c, d]` array.
    pub fn to_array(&self) -> [f64; 4] {
        [self.a, self.b, self.c, self.d]
    }
}

impl From<PlaneCoefficients> for [f64; 4] {
    fn from(p: PlaneCoefficients) -> [f64; 4] {
        p.to_array()
    }
}

/// Trait for providers of geometric data.
///
/// Implementations map an integer index to plane equation coefficients.
/// The kernel layer provides the concrete implementation that bridges
/// topology handles to geometric values.
pub trait GeometrySource {
    /// Retrieve plane coefficients by index.
    ///
    /// The plane equation is `a*x + b*y + c*z + d = 0`.
    fn get_plane(&self, index: usize) -> Result<PlaneCoefficients, MathError>;
}
