//! Assembly phase of Boolean operations.
//!
//! Handles the reconstruction of valid manifolds from split and classified faces.

pub mod cleanup;
pub mod copy;
#[cfg(test)]
mod copy_stitch_tests;
pub mod disjoint;
pub mod merge;
pub mod stitch;

pub use merge::execute_boolean_direct;
pub use merge::execute_boolean_with_engine;
pub use merge::execute_boolean_with_overrides;
pub mod rebuild_face;
