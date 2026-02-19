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

pub mod schema;
pub mod eval;
pub mod diff;

#[cfg(test)]
pub(crate) mod tests;

pub use schema::{VersionedModel, SCHEMA_VERSION};
pub use eval::{save_model, load_model};
pub use diff::{ModelChange, diff_models};
