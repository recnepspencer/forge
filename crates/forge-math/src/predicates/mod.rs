//! Geometric predicates with filtered evaluation.
//!
//! Each predicate uses the four-stage [`FilteredEval`](crate::arithmetic::filter::FilteredEval)
//! pipeline to determine the sign of a geometric determinant:
//!
//! 1. **f64** with Shewchuk error bounds — resolves >95% of random inputs
//! 2. **Interval** with ULP-widened bounds — resolves >99% of remaining
//! 3. **Double-double** (~106-bit) — resolves >99.9% of remaining
//! 4. **Exact rational** — resolves everything
//!
//! Every evaluation returns [`PrecisionEscalation`](crate::arithmetic::filter::PrecisionEscalation)
//! metadata recording which stage resolved the result.
//!
//! # Reference
//!
//! Shewchuk, "Adaptive Precision Floating-Point Arithmetic and Fast Robust
//! Geometric Predicates," Discrete & Computational Geometry, 1997.

pub mod in_sphere;
pub mod orient2d;
pub mod orient3d;
pub mod grid_predicates;

pub use in_sphere::{in_sphere, InSphereInput};
pub use orient2d::{orient2d, Orient2dInput};
pub use orient3d::{orient3d, Orient3dInput};
pub use grid_predicates::{orient3d_grid, orient2d_grid, classify_point_grid};

/// Shewchuk-derived error bound coefficients for static filters.
///
/// For a sum of products, rounding error is bounded by `coeff * Σ|products|`.
pub(crate) const EPSILON: f64 = f64::EPSILON;
pub(crate) const ORIENT2D_ERR_BOUND_A: f64 = (3.0 + 16.0 * EPSILON) * EPSILON;
pub(crate) const ORIENT3D_ERR_BOUND_A: f64 = (7.0 + 56.0 * EPSILON) * EPSILON;
pub(crate) const IN_SPHERE_ERR_BOUND_A: f64 = (16.0 + 256.0 * EPSILON) * EPSILON;
