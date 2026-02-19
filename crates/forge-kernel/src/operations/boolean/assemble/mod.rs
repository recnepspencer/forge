//! Assembly phase of Boolean operations.
//!
//! Handles the reconstruction of valid manifolds from split and classified faces.

pub mod stitch;
pub mod select;
pub mod copy;
pub mod cleanup;
pub mod disjoint;
pub mod merge;

pub use merge::execute_boolean;
pub use merge::execute_boolean_with_overrides;
