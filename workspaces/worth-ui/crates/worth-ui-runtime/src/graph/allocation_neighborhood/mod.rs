//! Allocation neighborhood lane: handoff → membership → constraint authority → projection adapters.
//!
//! Lifecycle order:
//! 1. `handoff` admits graph snapshot + measurement basis into neighborhood authority
//! 2. `membership` / `membership_rule` / `operator_contract` derive graph-owned semantics
//! 3. `constraint_*` admits neighborhood into constraint set authority
//! 4. `projection` exposes thin evidence-carrier routing into handoff

// --- handoff (graph snapshot → neighborhood authority) ---
#[path = "replan_selection/transaction/active_replan_index.rs"]
mod active_replan_index;
#[path = "admission/admitted_constraint_basis.rs"]
mod admitted_constraint_basis;
#[path = "admission/catalog_basis.rs"]
mod catalog_basis;
#[path = "admission/catalog_delta.rs"]
mod catalog_delta;
#[path = "replan_selection/transaction/graph_neighborhood_footprint.rs"]
mod graph_neighborhood_footprint;
#[path = "admission/handoff.rs"]
mod handoff;
#[path = "membership/membership.rs"]
mod membership;
#[path = "membership/membership_rule.rs"]
mod membership_rule;
#[path = "admission/operator_contract.rs"]
mod operator_contract;
#[path = "activation_handoff/projection.rs"]
mod projection;
#[path = "replan_selection/transaction/replan_authority.rs"]
mod replan_authority;
#[path = "replan_selection/transaction/replan_consequence.rs"]
mod replan_consequence;
mod replan_selection;
#[path = "replan_selection/transaction/replan_target_set.rs"]
mod replan_target_set;
#[path = "replan_selection/transaction/replan_transaction_sealing.rs"]
mod replan_transaction_sealing;

pub(crate) struct UiAllocationNeighborhoodMintAuthority(());

impl UiAllocationNeighborhoodMintAuthority {
    pub(in crate::graph::allocation_neighborhood) fn mint() -> Self {
        Self(())
    }
}

// --- constraint authority (neighborhood → constraint set) ---
#[path = "constraint_authority/admission/constraint_authority.rs"]
mod constraint_authority;
#[path = "constraint_authority/derivation/constraint_bound_reconciliation.rs"]
mod constraint_bound_reconciliation;
#[path = "constraint_authority/derivation/constraint_child_intrinsic_contribution.rs"]
mod constraint_child_intrinsic_contribution;
#[path = "constraint_authority/admission/constraint_cycle_posture.rs"]
mod constraint_cycle_posture;
#[path = "constraint_authority/derivation/constraint_durable_resize_input.rs"]
mod constraint_durable_resize_input;
#[path = "constraint_authority/derivation/constraint_edge_assembly.rs"]
mod constraint_edge_assembly;
#[path = "constraint_authority/derivation/constraint_equal_share_distribution.rs"]
mod constraint_equal_share_distribution;
#[path = "constraint_authority/admission/constraint_mint_authority.rs"]
mod constraint_mint_authority;
#[path = "constraint_authority/admission/constraint_normalization.rs"]
mod constraint_normalization;
#[path = "constraint_authority/derivation/constraint_parent_available_space.rs"]
mod constraint_parent_available_space;
mod constraint_pipeline;
#[path = "constraint_authority/integration/constraint_portal_anchor_planning_input.rs"]
mod constraint_portal_anchor_planning_input;
#[path = "constraint_authority/admission/constraint_projection.rs"]
mod constraint_projection;
#[path = "constraint_authority/integration/constraint_scroll_owner_planning_input.rs"]
mod constraint_scroll_owner_planning_input;
mod constraint_sibling_negotiation;
#[path = "constraint_authority/admission/constraint_summary.rs"]
mod constraint_summary;
#[path = "constraint_authority/derivation/constraint_viewport_planning_input.rs"]
mod constraint_viewport_planning_input;
#[path = "constraint_authority/integration/scroll_planning_authority.rs"]
mod scroll_planning_authority;

pub(crate) use admitted_constraint_basis::{
    UiAdmittedAllocationConstraintBasis, UiAllocationConstraintProvenance,
};
pub(crate) use constraint_mint_authority::UiGraphConstraintMintAuthority;
pub(crate) use scroll_planning_authority::UiGraphScrollPlanningAuthority;

#[path = "admission/denial.rs"]
mod denial;
#[cfg(test)]
#[path = "membership/equivalence.rs"]
mod equivalence;
#[cfg(test)]
pub(crate) mod tests;

pub(crate) use activation_lifecycle::UiGraphNeighborhoodActivationTransition;
pub use catalog_basis::{
    UiAdmittedAllocationCatalogBasisSet, UiAllocationCatalogBasisAdmissionDenial,
};
pub use catalog_delta::{
    UiAdmittedAllocationCatalogDelta, UiAllocationCatalogDeltaAdmissionDenial,
};
pub use denial::UiAllocationNeighborhoodDenial;
#[cfg(test)]
pub(crate) use equivalence::equivalent_identity;
pub(crate) use graph_neighborhood_footprint::UiGraphNeighborhoodFootprint;
pub use replan_authority::UiAdmittedAllocationInvalidationTargetSet;
pub(crate) use replan_authority::{
    UiAdmittedAllocationInvalidationTarget, UiAdmittedAllocationPlanReference,
    UiGraphReplanAdmission, UiGraphReplanAuthority, UiGraphReplanTargetDisposition,
    UiReplanGenerationKey,
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
#[path = "activation_handoff/activation_lifecycle.rs"]
mod activation_lifecycle;
