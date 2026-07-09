//! Allocation neighborhood lane: handoff → membership → constraint authority → projection adapters.
//!
//! Lifecycle order:
//! 1. `handoff` admits graph snapshot + measurement basis into neighborhood authority
//! 2. `membership` / `membership_rule` / `operator_contract` derive graph-owned semantics
//! 3. `constraint_*` admits neighborhood into constraint set authority
//! 4. `projection` exposes thin evidence-carrier routing into handoff

// --- handoff (graph snapshot → neighborhood authority) ---
mod handoff;
mod membership;
mod membership_rule;
mod operator_contract;
mod projection;

// --- constraint authority (neighborhood → constraint set) ---
mod constraint_authority;
mod constraint_bound_reconciliation;
mod constraint_child_intrinsic_contribution;
mod constraint_cycle_posture;
mod constraint_durable_resize_input;
mod constraint_edge_assembly;
mod constraint_equal_share_distribution;
mod constraint_normalization;
mod constraint_parent_available_space;
mod constraint_pipeline;
mod constraint_portal_anchor_planning_input;
mod constraint_projection;
mod constraint_scroll_owner_planning_input;
mod constraint_sibling_negotiation;
mod constraint_summary;
mod constraint_viewport_planning_input;

mod denial;
#[cfg(test)]
mod equivalence;
#[cfg(test)]
pub(crate) mod tests;

pub use denial::UiAllocationNeighborhoodDenial;
#[cfg(test)]
pub(crate) use equivalence::equivalent_identity;
