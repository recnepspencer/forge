//! Public façade for shared kernel validators.
//!
//! DOMAIN: Re-exports production validators for kernel operations.
//! External components depend ONLY on this façade.

// Decision log validators
pub use super::placement::vertex_decisions::validate_vertex_decisions;

// Input validators
pub use super::input::cell::validate_cell;
pub use super::input::dimension::{
    validate_center_and_size, validate_coordinate, validate_dimension,
};
