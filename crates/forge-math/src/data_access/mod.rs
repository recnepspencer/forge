//! Data-access abstractions for cross-layer communication.
//!
//! DOMAIN: Anonymous data access traits that allow lower layers
//! (`forge-geom`) to request data from higher layers (`forge-kernel`)
//! without depending on them.
//!
//! DEPENDENCIES: `MathError` (error type)
//!
//! ## Contents
//!
//! - `PlaneCoefficients` — Validated `[a, b, c, d]` plane equation
//! - `GeometrySource` — Trait mapping index → plane coefficients

mod schema;

pub use schema::{GeometrySource, PlaneCoefficients};
