//! Public façade for spatial validators.
//!
//! DOMAIN: Re-exports geometric invariant validation functions.
//! External components depend ONLY on this façade.

pub use super::validate_geometric_invariants;
pub use super::completeness::validate_geometry_completeness;
pub use super::area::validate_zero_area_faces;
pub use super::edge_length::validate_zero_length_edges;
pub use super::loop_orientation::validate_loop_orientation;
pub use super::shell_orientation::validate_shell_orientation;
pub use super::volume::validate_signed_volume;
pub use super::dispatch::{
    spatial_validator_for, validate_all_spatial_invariants, SpatialValidatorEntry,
};

