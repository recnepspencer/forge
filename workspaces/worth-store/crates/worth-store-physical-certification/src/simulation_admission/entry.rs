use worth_store_recovery_physics::PageLsn;
use worth_store_recovery_runtime::RecoveryCompletion;

use super::inventory::ExistingSimulationHarnessInventory;
use super::non_claims::{SimulationHarnessNonClaim, REQUIRED_S45_ENTRY_NON_CLAIMS};
use super::request::SimulationHarnessEntryRequest;
use super::requirement_set::{
    SimulationHarnessRoadmapRequirement, SimulationHarnessRoadmapRequirementSet,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimulationHarnessEntryIdentity {
    recovered_root: String,
    source_decision_digest: String,
    roadmap_requirements: Vec<SimulationHarnessRoadmapRequirement>,
}

impl SimulationHarnessEntryIdentity {
    pub(crate) fn new(
        recovered_root: impl Into<String>,
        source_decision_digest: impl Into<String>,
        roadmap_requirements: &SimulationHarnessRoadmapRequirementSet,
    ) -> Self {
        Self {
            recovered_root: recovered_root.into(),
            source_decision_digest: source_decision_digest.into(),
            roadmap_requirements: roadmap_requirements.canonical_identity_requirements(),
        }
    }

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub fn source_decision_digest(&self) -> &str {
        &self.source_decision_digest
    }

    pub fn roadmap_requirements(&self) -> &[SimulationHarnessRoadmapRequirement] {
        &self.roadmap_requirements
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessEntry {
    identity: SimulationHarnessEntryIdentity,
    recovered_root: String,
    admitted_page_lsn_frontier: Option<PageLsn>,
    source_decision_digest: String,
    recovery_completion: RecoveryCompletion,
    roadmap_requirements: SimulationHarnessRoadmapRequirementSet,
    inventory: ExistingSimulationHarnessInventory,
    non_claims: Vec<SimulationHarnessNonClaim>,
}

impl SimulationHarnessEntry {
    pub(crate) fn from_admitted_request(
        request: SimulationHarnessEntryRequest,
        recovery_completion: RecoveryCompletion,
    ) -> Self {
        let (recovered_root, source_decision_digest, roadmap_requirements, inventory) =
            request.into_admitted_parts();
        let identity = SimulationHarnessEntryIdentity::new(
            recovered_root.clone(),
            source_decision_digest.clone(),
            &roadmap_requirements,
        );
        Self {
            identity,
            recovered_root,
            admitted_page_lsn_frontier: recovery_completion.admitted_page_lsn_frontier(),
            source_decision_digest,
            recovery_completion,
            roadmap_requirements,
            inventory,
            non_claims: REQUIRED_S45_ENTRY_NON_CLAIMS.to_vec(),
        }
    }

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn identity(&self) -> &SimulationHarnessEntryIdentity {
        &self.identity
    }

    pub const fn admitted_page_lsn_frontier(&self) -> Option<PageLsn> {
        self.admitted_page_lsn_frontier
    }

    pub fn source_decision_digest(&self) -> &str {
        &self.source_decision_digest
    }

    pub const fn recovery_completion(&self) -> &RecoveryCompletion {
        &self.recovery_completion
    }

    pub const fn replayed_frames(&self) -> usize {
        self.recovery_completion.replayed_frames()
    }

    pub const fn source_candidate_count(&self) -> usize {
        self.recovery_completion.source_candidate_count()
    }

    pub const fn roadmap_requirements(&self) -> &SimulationHarnessRoadmapRequirementSet {
        &self.roadmap_requirements
    }

    pub const fn inventory(&self) -> &ExistingSimulationHarnessInventory {
        &self.inventory
    }

    pub fn non_claims(&self) -> &[SimulationHarnessNonClaim] {
        &self.non_claims
    }

    pub fn accepts_recovery_completion_and_harness_evidence(&self) -> bool {
        self.roadmap_requirements.is_complete()
            && self
                .non_claims
                .contains(&SimulationHarnessNonClaim::NoPhysicalIsolationCorrectnessClaim)
    }
}
