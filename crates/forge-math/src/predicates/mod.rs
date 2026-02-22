//! Geometric predicates with adaptive precision arithmetic.
//!
//! Each predicate uses Shewchuk's adaptive cascade to determine the sign
//! of a geometric determinant with exact results and minimal work:
//!
//! 1. **Stage A (f64)** — Shewchuk error bounds — resolves >95% of inputs
//! 2. **Stage B (Expansion)** — first adaptive refinement
//! 3. **Stage C (Expansion)** — full expansion with tail corrections
//!
//! Every evaluation returns [`PrecisionEscalation`](crate::arithmetic::precision::PrecisionEscalation)
//! metadata recording which stage resolved the result.
//!
//! # Reference
//!
//! Shewchuk, "Adaptive Precision Floating-Point Arithmetic and Fast Robust
//! Geometric Predicates," Discrete & Computational Geometry, 1997.

pub mod grid_predicates;
pub mod in_sphere;
pub mod incircle;
pub mod orient2d;
pub mod orient3d;
mod vendored;

pub use grid_predicates::{orient3d_grid, orient2d_grid, classify_point_grid};
pub use in_sphere::{in_sphere, InSphereInput};
pub use incircle::incircle;
pub use orient2d::orient2d;
pub use orient3d::{orient3d, Orient3dInput};
