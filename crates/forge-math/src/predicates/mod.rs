//! Geometric predicates with filtered evaluation.
//!
//! Each predicate uses the three-stage [`FilteredEval`](crate::arithmetic::filter::FilteredEval) pipeline to determine
//! the sign of a geometric determinant. Stage 1 (f64 with Shewchuk error bounds)
//! resolves >95% of random inputs. Stage 2 (double-double) resolves >99% of
//! the remainder. Stage 3 (exact rational) resolves everything.
//!
//! # Reference
//!
//! Shewchuk, "Adaptive Precision Floating-Point Arithmetic and Fast Robust
//! Geometric Predicates," Discrete & Computational Geometry, 1997.

pub mod in_sphere;
pub mod orient2d;
pub mod orient3d;

pub use in_sphere::{in_sphere, InSphereInput};
pub use orient2d::{orient2d, Orient2dInput};
pub use orient3d::{orient3d, Orient3dInput};

/// Shewchuk-derived error bound coefficients for static filters.
///
/// For a sum of products, rounding error is bounded by `coeff * Σ|products|`.
pub(crate) const EPSILON: f64 = f64::EPSILON;
pub(crate) const ORIENT2D_ERR_BOUND_A: f64 = (3.0 + 16.0 * EPSILON) * EPSILON;
pub(crate) const ORIENT3D_ERR_BOUND_A: f64 = (7.0 + 56.0 * EPSILON) * EPSILON;
pub(crate) const IN_SPHERE_ERR_BOUND_A: f64 = (16.0 + 256.0 * EPSILON) * EPSILON;
