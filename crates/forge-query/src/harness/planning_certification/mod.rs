mod bundles;
mod fixtures;
mod canonical_rows;
mod rejection_rows;
mod tests;

use super::planning_matrix::{
    MilestoneThreePlanningCertificationArtifact, PlanningCertificationMatrix,
};

pub struct MilestoneThreePlanningCertificationAdapter;

impl MilestoneThreePlanningCertificationAdapter {
    pub fn planner_executor_binding_parity_certification_artifact(
    ) -> MilestoneThreePlanningCertificationArtifact {
        Self::planner_executor_binding_parity_test().into_milestone_three_artifact()
    }

    pub fn planner_executor_binding_parity_test() -> PlanningCertificationMatrix {
        PlanningCertificationMatrix {
            suite_name: "Planner / Executor / Binding Parity Test",
            rows: canonical_rows::canonical_rows(),
            rejection_rows: rejection_rows::rejection_rows(),
        }
    }
}
