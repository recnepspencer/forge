mod authority;
mod constraint_authority;
mod constraint_bound_reconciliation;
mod constraint_child_intrinsic_contribution;
mod constraint_cycle_posture;
mod constraint_durable_resize_input;
mod constraint_equal_share_distribution;
mod constraint_normalization;
mod constraint_parent_available_space;
mod constraint_portal_anchor_planning_input;
mod constraint_projection;
mod constraint_sibling_negotiation;
mod constraint_scroll_owner_planning_input;
mod constraint_summary;
mod constraint_viewport_planning_input;
mod denial;
#[cfg(test)]
mod equivalence;
mod membership;
mod projection;

pub use denial::UiAllocationNeighborhoodDenial;
#[cfg(test)]
pub(crate) use equivalence::equivalent_identity;
