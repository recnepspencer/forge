//! Public façade for shared kernel validators.
//!
//! DOMAIN: Re-exports production validators for kernel operations.
//! External components depend ONLY on this façade.

pub use super::placement::vertex_decisions::validate_vertex_decisions;
