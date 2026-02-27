//! JSON serialization for Forge models.
//!
//! DOMAIN: Versioned JSON serialization/deserialization of `FeatureTree`.
//! INVARIANTS: Schema version must be forward-compatible (reject future versions).
//! DEPENDENCIES: `serde`, `serde_json`, `forge-kernel` (FeatureTree)
//!
//! ## Sub-modules
//!
//! - `schema` — `VersionedModel`, `SCHEMA_VERSION`
//! - `eval` — `save_model`, `load_model` functions
//! - `diff` — Model diffing for version control

pub mod diff;
pub mod eval;
pub mod schema;

#[cfg(test)]
pub(crate) mod tests;

pub use diff::{diff_models, ModelChange};
pub use eval::{load_model, save_model};
pub use schema::{VersionedModel, SCHEMA_VERSION};
