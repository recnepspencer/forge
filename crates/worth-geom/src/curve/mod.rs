//! 3D edge curve types and evaluation.
//!
//! DOMAIN: The curve type hierarchy for B-Rep edges. Analytic curves
//! (line, circle, ellipse) have closed-form evaluation. Symbolic intersection
//! curves store their parent surfaces for aerospace-grade correctness.
//!
//! DEPENDENCIES: serde

pub mod eval;
pub mod schema;

pub use schema::{CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation, SurfaceIndex};
