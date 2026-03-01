//! Kernel-layer query traits.
//!
//! DOMAIN: Named interfaces that features and operations query for
//! configuration values, geometry lookups, and observability hooks.
//! Algorithms depend on these traits instead of reaching into
//! `ModelingContext` or `GeometryState` directly, following the
//! Adapter Rule (§6).
//!
//! ## Structure
//!
//! ```text
//! queries/
//! ├── mod.rs           ← this file (Table of Contents)
//! ├── tolerance.rs     ← ToleranceQueries: spatial, geometry, sampling, gap, scale
//! ├── validation.rs    ← ValidationQuery: checkpoint config access
//! ├── policy.rs        ← PolicyQuery: policy pre-validation
//! ├── geometry.rs      ← FaceGeometryQuery, VertexGeometryQuery
//! ├── tracing.rs       ← DecisionQuery, WarningQuery
//! └── tracing.rs       ← DecisionQuery, WarningQuery
//! ```

pub mod geometry;
pub mod policy;
pub mod tolerance;
pub mod tracing;
pub mod validation;
