use super::inventory::S45ExistingHarnessInventory;
use super::requirement_set::S45RoadmapHarnessRequirementSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S45HarnessEntryRequest {
    recovered_root: String,
    source_decision_digest: String,
    s4_completed_lanes: usize,
    s4_required_lanes: usize,
    s4_foundational_exact_counter_assertions: usize,
    roadmap_requirements: S45RoadmapHarnessRequirementSet,
    inventory: S45ExistingHarnessInventory,
}

impl S45HarnessEntryRequest {
    pub(crate) fn new(
        recovered_root: impl Into<String>,
        source_decision_digest: impl Into<String>,
        s4_completed_lanes: usize,
        s4_required_lanes: usize,
        s4_foundational_exact_counter_assertions: usize,
        roadmap_requirements: S45RoadmapHarnessRequirementSet,
        inventory: S45ExistingHarnessInventory,
    ) -> Self {
        Self {
            recovered_root: recovered_root.into(),
            source_decision_digest: source_decision_digest.into(),
            s4_completed_lanes,
            s4_required_lanes,
            s4_foundational_exact_counter_assertions,
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

    pub(crate) const fn s4_completed_lanes(&self) -> usize {
        self.s4_completed_lanes
    }

    pub(crate) const fn s4_required_lanes(&self) -> usize {
        self.s4_required_lanes
    }

    pub(crate) const fn roadmap_requirements(&self) -> &S45RoadmapHarnessRequirementSet {
        &self.roadmap_requirements
    }

    pub(crate) fn into_admitted_parts(
        self,
    ) -> (
        String,
        String,
        usize,
        usize,
        usize,
        S45RoadmapHarnessRequirementSet,
        S45ExistingHarnessInventory,
    ) {
        (
            self.recovered_root,
            self.source_decision_digest,
            self.s4_completed_lanes,
            self.s4_required_lanes,
            self.s4_foundational_exact_counter_assertions,
            self.roadmap_requirements,
            self.inventory,
        )
    }
}
