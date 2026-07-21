//! Planning lane handoff order: measurement basis → constraint admission → allocation planning
//! → plan topology → equivalence / inspection.

pub mod allocation_planning;
mod allocation_replan_denial;
pub mod execution_plan_input;
mod execution_plan_lowering_authority;
mod execution_plan_lowering_identity;
pub mod plan_equivalence;
pub mod plan_inspection;
pub mod plan_topology;
pub mod query_binding;

mod input_handoff;
mod measurement_basis;
mod plan_allocation;
mod transitions;

/// Capability held only by the planning lane when it materializes candidate truth.
pub(crate) struct UiAllocationCandidateMintAuthority(());

impl UiAllocationCandidateMintAuthority {
    const fn new() -> Self {
        Self(())
    }
}

#[cfg(test)]
pub(crate) use execution_plan_lowering_authority::test_support::facts_below_authority;
pub(crate) use execution_plan_lowering_authority::{
    WorthUiExecutionPlanLoweringAuthority, WorthUiExecutionPlanLoweringAuthorityDenial,
    WorthUiExecutionPlanLoweringFacts,
};
pub(crate) use execution_plan_lowering_identity::WorthUiExecutionPlanLoweringIdentity;
pub(crate) use input_handoff::construct_verified_planning_input_handoff;

pub(crate) use allocation_replan_denial::WorthUiAllocationReplanDenial;
pub(crate) use measurement_basis::collect_planning_measurement_basis;
pub(crate) use plan_allocation::plan_allocation_for_pending_activation;
pub(crate) use plan_allocation::replan_selected_candidates_with_portal;
pub(crate) use plan_allocation::replan_selected_candidates_with_resize;
pub(crate) use transitions::construct_planning_lane_input;
pub use transitions::{WorthUiPlanningLaneAdmissionDenial, WorthUiPlanningLaneInput};
