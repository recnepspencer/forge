//! Planning lane handoff order: measurement basis → constraint admission → allocation planning
//! → plan topology → equivalence / inspection.

#[path = "../allocation_planning/mod.rs"]
pub mod allocation_planning;
#[path = "../execution_plan_input/mod.rs"]
pub mod execution_plan_input;
#[path = "../plan_equivalence/mod.rs"]
pub mod plan_equivalence;
#[path = "../plan_inspection/mod.rs"]
pub mod plan_inspection;
#[path = "../plan_topology/mod.rs"]
pub mod plan_topology;

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

pub(crate) use input_handoff::construct_verified_planning_input_handoff;

pub(crate) use measurement_basis::collect_planning_measurement_basis;
#[cfg(test)]
pub(crate) use plan_allocation::candidate_from_test_planning;
pub(crate) use plan_allocation::plan_allocation_for_pending_activation;
pub(crate) use plan_allocation::replan_selected_candidates_with_portal;
pub(crate) use plan_allocation::replan_selected_candidates_with_resize;
pub(crate) use transitions::construct_planning_lane_input;
pub use transitions::{WorthUiPlanningLaneAdmissionDenial, WorthUiPlanningLaneInput};
