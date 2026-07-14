use super::inventory::ExistingSimulationHarnessInventory;
use super::requirement_set::SimulationHarnessRoadmapRequirementSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SimulationHarnessEntryRequest {
    recovered_root: String,
    source_decision_digest: String,
    roadmap_requirements: SimulationHarnessRoadmapRequirementSet,
    inventory: ExistingSimulationHarnessInventory,
}

impl SimulationHarnessEntryRequest {
    pub(crate) fn new(
        recovered_root: impl Into<String>,
        source_decision_digest: impl Into<String>,
        roadmap_requirements: SimulationHarnessRoadmapRequirementSet,
        inventory: ExistingSimulationHarnessInventory,
    ) -> Self {
        Self {
            recovered_root: recovered_root.into(),
            source_decision_digest: source_decision_digest.into(),
            roadmap_requirements,
            inventory,
        }
    }

    pub(crate) fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub(crate) fn source_decision_digest(&self) -> &str {
        &self.source_decision_digest
    }

    pub(crate) const fn roadmap_requirements(&self) -> &SimulationHarnessRoadmapRequirementSet {
        &self.roadmap_requirements
    }

    pub(crate) fn into_admitted_parts(
        self,
    ) -> (
        String,
        String,
        SimulationHarnessRoadmapRequirementSet,
        ExistingSimulationHarnessInventory,
    ) {
        (
            self.recovered_root,
            self.source_decision_digest,
            self.roadmap_requirements,
            self.inventory,
        )
    }
}
