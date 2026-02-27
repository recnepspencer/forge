//! Assembly phase of Boolean operations.
//!
//! Handles the reconstruction of valid manifolds from split and classified faces.

#[cfg(test)]
mod copy_stitch_tests;
pub mod disjoint;
pub mod merge;

pub use merge::{execute_boolean_direct, execute_boolean_with_overrides};
