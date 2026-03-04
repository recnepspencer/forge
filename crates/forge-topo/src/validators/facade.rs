//! Public API surface for the validators domain.
//!
//! External components depend ONLY on this facade.
//! Internal subdirectory structure is hidden.

pub use super::validate::{validate_topology, ValidationLevel};
pub use super::shell_closure::validate_manifold_edges;
pub use super::radial_edge::validate_radial_edge_consistency;
pub use super::loop_wiring::validate_vertex_continuity;
