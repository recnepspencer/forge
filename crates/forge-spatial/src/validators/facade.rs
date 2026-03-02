//! Public façade for spatial validators.
//!
//! DOMAIN: Re-exports geometric invariant validation functions.
//! External components depend ONLY on this façade.

pub use super::validate_geometric_invariants;
pub use super::completeness::validate_geometry_completeness;
pub use super::area::validate_zero_area_faces;
pub use super::edge_length::validate_zero_length_edges;
pub use super::volume::validate_signed_volume;
