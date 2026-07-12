//! Allocation neighborhood lane: handoff → membership → constraint authority → projection adapters.
//!
//! Lifecycle order:
//! 1. `handoff` admits graph snapshot + measurement basis into neighborhood authority
//! 2. `membership` / `membership_rule` / `operator_contract` derive graph-owned semantics
//! 3. `constraint_*` admits neighborhood into constraint set authority
//! 4. `projection` exposes thin evidence-carrier routing into handoff

// --- handoff (graph snapshot → neighborhood authority) ---
mod admitted_constraint_basis;
mod catalog_basis;
mod handoff;
mod membership;
mod membership_rule;
mod operator_contract;
mod projection;
mod replan_authority;
mod replan_consequence;
mod replan_selection;
mod replan_target_set;
mod replan_transaction_sealing;

pub(crate) struct UiAllocationNeighborhoodMintAuthority(());

impl UiAllocationNeighborhoodMintAuthority {
    pub(in crate::graph::allocation_neighborhood) fn mint() -> Self {
        Self(())
    }
}

// --- constraint authority (neighborhood → constraint set) ---
mod constraint_authority;
mod constraint_bound_reconciliation;
mod constraint_child_intrinsic_contribution;
mod constraint_cycle_posture;
mod constraint_durable_resize_input;
mod constraint_edge_assembly;
mod constraint_equal_share_distribution;
mod constraint_mint_authority;
mod constraint_normalization;
mod constraint_parent_available_space;
mod constraint_pipeline;
mod constraint_portal_anchor_planning_input;
mod constraint_projection;
mod constraint_scroll_owner_planning_input;
mod constraint_sibling_negotiation;
mod constraint_summary;
mod constraint_viewport_planning_input;
mod scroll_planning_authority;

pub(crate) use admitted_constraint_basis::{UiAdmittedAllocationConstraintBasis, UiAllocationConstraintProvenance};
pub(crate) use constraint_mint_authority::UiGraphConstraintMintAuthority;
pub(crate) use scroll_planning_authority::UiGraphScrollPlanningAuthority;

mod denial;
#[cfg(test)]
mod equivalence;
#[cfg(test)]
pub(crate) mod tests;

pub(crate) use activation_lifecycle::UiGraphNeighborhoodActivationTransition;
pub use catalog_basis::{
    UiAdmittedAllocationCatalogBasisSet, UiAllocationCatalogBasisAdmissionDenial,
};
pub use denial::UiAllocationNeighborhoodDenial;
#[cfg(test)]
pub(crate) use equivalence::equivalent_identity;
pub use replan_authority::UiAdmittedAllocationInvalidationTargetSet;
pub(crate) use replan_authority::{
    UiAdmittedAllocationInvalidationTarget, UiAdmittedAllocationPlanReference,
    UiGraphNeighborhoodFootprint, UiGraphReplanAdmission, UiGraphReplanAuthority,
    UiGraphReplanTargetDisposition, UiReplanGenerationKey,
};
pub(crate) use replan_selection::select_replan_neighborhoods;
pub use replan_selection::{
    UiAdmittedReplanNeighborhood, UiAdmittedReplanNeighborhoodSet, UiReplanLocalityDenial,
    UiReplanLocalityProof, UiReplanNeighborhoodSelectionCounters, UiReplanOverlapDisposition,
    UiReplanRootPosture, UiReplanWidenReason,
};
pub(crate) use replan_selection::{
    UiGraphReplanConsequences, UiGraphReplanTransactionBasis, UiPortalReplanConsequence,
    UiScrollReplanConsequence,
};
mod activation_lifecycle;
