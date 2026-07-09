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

pub(crate) use input_handoff::construct_verified_planning_input_handoff;

#[cfg(test)]
pub(crate) use measurement_basis::collect_planning_measurement_basis;
pub(crate) use plan_allocation::plan_allocation_for_pending_activation;
pub(crate) use transitions::construct_planning_lane_input;
pub use transitions::{WorthUiPlanningLaneAdmissionDenial, WorthUiPlanningLaneInput};
