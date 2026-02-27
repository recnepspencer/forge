//! Assembly phase of Boolean operations.
//!
//! Handles the reconstruction of valid manifolds from split and classified faces.

pub mod stitch;
pub mod copy;
pub mod cleanup;
pub mod disjoint;
pub mod merge;
#[cfg(test)]
mod copy_stitch_tests;

pub use merge::execute_boolean_direct;
pub use merge::execute_boolean_with_overrides;
pub use merge::execute_boolean_with_engine;
pub mod rebuild_face;
