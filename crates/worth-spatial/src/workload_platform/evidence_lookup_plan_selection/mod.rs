mod counters;
mod error;
mod plan_row;
mod query_posture;
mod selected_plan;
mod selection;
mod strategy;
mod topology_posture;

#[cfg(test)]
mod tests;

pub use counters::EvidenceLookupPlanSelectionCounters;
pub use error::{EvidenceLookupPlanSelectionError, EvidenceLookupPlanSelectionErrorKind};
pub use plan_row::{EvidenceLookupPlanRowOutcome, EvidenceLookupSelectedPlanRow};
pub use query_posture::{
    EvidenceLookupPlanQueryPosture, EvidenceLookupPlanQueryPostureState,
    EvidenceLookupPlanQuerySurface,
};
pub use selected_plan::EvidenceLookupSelectedPlan;
pub use selection::select_evidence_lookup_plan;
pub use strategy::{EvidenceLookupSelectedStrategy, EvidenceLookupSelectedStrategyKind};
pub use topology_posture::{
    EvidenceLookupPlanTopologyPosture, EvidenceLookupPlanTopologyPostureState,
};
