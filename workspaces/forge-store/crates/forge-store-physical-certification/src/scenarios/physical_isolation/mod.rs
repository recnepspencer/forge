mod context;
mod fixture;
mod lane;
mod observer;
mod replay;

pub use crate::{physical_isolation_required_mutation_rows, PhysicalIsolationMutationEvidence};
pub use context::{
    physical_isolation_ci_certification_context_without_lane_registration,
    physical_isolation_ci_certification_planning_context,
    physical_isolation_context_without_lane_registration, physical_isolation_planning_context,
};
pub use lane::{physical_isolation_lanes, PhysicalIsolationHarnessLane};
pub use observer::{observe_physical_isolation_trace, PhysicalIsolationTraceFixtures};
pub use replay::{assemble_physical_isolation_replay_bundle, physical_isolation_coverage_matrix};
