//! Parametric surface types and evaluation.
//!
//! DOMAIN: The surface type hierarchy for B-Rep faces. Analytic surfaces
//! (plane, cylinder, cone, sphere, torus) have closed-form evaluation.
//! NURBS surfaces are Phase 7.
//!
//! DEPENDENCIES: serde

pub mod schema;
pub mod eval;

pub use schema::{SurfaceKind, SurfaceData, ParameterDomain, SurfaceRelation};
pub use eval::classify_surface_pair;
