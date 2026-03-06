//! Public API surface for the validators domain.
//!
//! External components depend ONLY on this facade.
//! Internal subdirectory structure is hidden.

pub use super::loop_wiring::validate_vertex_continuity;
pub use super::radial_edge::validate_radial_edge_consistency;
pub use super::shell_closure::validate_manifold_edges;
pub use super::validate::{validate_topology, ValidationLevel};

pub use super::group_policy_runtime::topology_context_from_shell_metadata;
pub use super::group_policy_runtime::GroupPolicyRuntime;
pub use super::invariant_group::{invariant_ids, InvariantGroup, InvariantTier};
pub use super::invariant_id::{
    validator_for, InvariantContract, InvariantId, InvariantRelation, ValidatorCost, ValidatorEntry,
};
