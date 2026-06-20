use super::{
    estimate_graph_read_access_cost, ForgeQueryGraphReadAccessCostEstimate,
    ForgeQueryGraphReadCostEvidence,
};
use crate::runtime::ForgeQueryGraphReadAccessRequirementSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadPlanningObservation {
    edge_read_count: usize,
    access_buffer_allocation_count: usize,
}

impl ForgeQueryGraphReadPlanningObservation {
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
pub struct ForgeQueryGraphReadObservedCostEstimate {
    estimate: ForgeQueryGraphReadAccessCostEstimate,
    planning_observation: ForgeQueryGraphReadPlanningObservation,
}

impl ForgeQueryGraphReadObservedCostEstimate {
    pub fn estimate(&self) -> &ForgeQueryGraphReadAccessCostEstimate {
        &self.estimate
    }

    pub fn planning_observation(&self) -> &ForgeQueryGraphReadPlanningObservation {
        &self.planning_observation
    }

    fn new(
        estimate: ForgeQueryGraphReadAccessCostEstimate,
        planning_observation: ForgeQueryGraphReadPlanningObservation,
    ) -> Self {
        Self {
            estimate,
            planning_observation,
        }
    }
}

pub fn estimate_graph_read_access_cost_with_planning_observation(
    requirements: &ForgeQueryGraphReadAccessRequirementSet,
    evidence: ForgeQueryGraphReadCostEvidence,
) -> ForgeQueryGraphReadObservedCostEstimate {
    ForgeQueryGraphReadObservedCostEstimate::new(
        estimate_graph_read_access_cost(requirements, evidence),
        ForgeQueryGraphReadPlanningObservation::planning_only(),
    )
}
