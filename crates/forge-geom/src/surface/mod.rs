//! Parametric surface types and evaluation.
//!
//! DOMAIN: The surface type hierarchy for B-Rep faces. Analytic surfaces
//! (plane, cylinder, cone, sphere, torus) have closed-form evaluation.
//! NURBS surfaces are Phase 7.
//!
//! DEPENDENCIES: serde

pub mod schema;
pub mod eval;
pub mod traits;
pub mod trim_ops;

pub use schema::{SurfaceKind, SurfaceData, ParameterDomain, SurfaceRelation};
pub use eval::classify_surface_pair;
pub use traits::EvaluateSurface;
pub use trim_ops::{TrimCurveOps, TrimOverlapResult};
