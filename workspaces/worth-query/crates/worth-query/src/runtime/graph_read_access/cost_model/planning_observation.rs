use super::{
    estimate_graph_read_access_cost, WorthQueryGraphReadAccessCostEstimate,
    WorthQueryGraphReadCostEvidence,
};
use crate::runtime::WorthQueryGraphReadAccessRequirementSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadPlanningObservation {
    edge_read_count: usize,
    access_buffer_allocation_count: usize,
}

impl WorthQueryGraphReadPlanningObservation {
    pub fn edge_read_count(&self) -> usize {
        self.edge_read_count
    }

    pub fn access_buffer_allocation_count(&self) -> usize {
        self.access_buffer_allocation_count
    }

    fn planning_only() -> Self {
        Self {
            edge_read_count: 0,
            access_buffer_allocation_count: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadObservedCostEstimate {
    estimate: WorthQueryGraphReadAccessCostEstimate,
    planning_observation: WorthQueryGraphReadPlanningObservation,
}

impl WorthQueryGraphReadObservedCostEstimate {
    pub fn estimate(&self) -> &WorthQueryGraphReadAccessCostEstimate {
        &self.estimate
    }

    pub fn planning_observation(&self) -> &WorthQueryGraphReadPlanningObservation {
        &self.planning_observation
    }

    fn new(
        estimate: WorthQueryGraphReadAccessCostEstimate,
        planning_observation: WorthQueryGraphReadPlanningObservation,
    ) -> Self {
        Self {
            estimate,
            planning_observation,
        }
    }
}

pub fn estimate_graph_read_access_cost_with_planning_observation(
    requirements: &WorthQueryGraphReadAccessRequirementSet,
    evidence: WorthQueryGraphReadCostEvidence,
) -> WorthQueryGraphReadObservedCostEstimate {
    WorthQueryGraphReadObservedCostEstimate::new(
        estimate_graph_read_access_cost(requirements, evidence),
        WorthQueryGraphReadPlanningObservation::planning_only(),
    )
}
