use forge_store_recovery_physics::{PageLsn, RecoveryCounterSnapshot};

use super::inventory::S45ExistingHarnessInventory;
use super::non_claims::{S45HarnessNonClaim, REQUIRED_S45_ENTRY_NON_CLAIMS};
use super::request::S45HarnessEntryRequest;
use super::requirement_set::{S45RoadmapHarnessRequirement, S45RoadmapHarnessRequirementSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct S45SimulationHarnessEntryIdentity {
    recovered_root: String,
    source_decision_digest: String,
    roadmap_requirements: Vec<S45RoadmapHarnessRequirement>,
}

impl S45SimulationHarnessEntryIdentity {
    pub(crate) fn new(
        recovered_root: impl Into<String>,
        source_decision_digest: impl Into<String>,
        roadmap_requirements: &S45RoadmapHarnessRequirementSet,
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

    pub fn roadmap_requirements(&self) -> &[S45RoadmapHarnessRequirement] {
        &self.roadmap_requirements
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45SimulationHarnessEntry {
    identity: S45SimulationHarnessEntryIdentity,
    recovered_root: String,
    admitted_page_lsn_frontier: Option<PageLsn>,
    source_decision_digest: String,
    recovery_counters: RecoveryCounterSnapshot,
    s4_completed_lanes: usize,
    s4_required_lanes: usize,
    s4_foundational_exact_counter_assertions: usize,
    roadmap_requirements: S45RoadmapHarnessRequirementSet,
    inventory: S45ExistingHarnessInventory,
    non_claims: Vec<S45HarnessNonClaim>,
}

impl S45SimulationHarnessEntry {
    pub(crate) fn from_admitted_request(
        request: S45HarnessEntryRequest,
        admitted_page_lsn_frontier: Option<PageLsn>,
        recovery_counters: RecoveryCounterSnapshot,
    ) -> Self {
        let (
            recovered_root,
            source_decision_digest,
            s4_completed_lanes,
            s4_required_lanes,
            s4_foundational_exact_counter_assertions,
            roadmap_requirements,
            inventory,
        ) = request.into_admitted_parts();
        let identity = S45SimulationHarnessEntryIdentity::new(
            recovered_root.clone(),
            source_decision_digest.clone(),
            &roadmap_requirements,
        );
        Self {
            identity,
            recovered_root,
            admitted_page_lsn_frontier,
            source_decision_digest,
            recovery_counters,
            s4_completed_lanes,
            s4_required_lanes,
            s4_foundational_exact_counter_assertions,
            roadmap_requirements,
            inventory,
            non_claims: REQUIRED_S45_ENTRY_NON_CLAIMS.to_vec(),
        }
    }

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn identity(&self) -> &S45SimulationHarnessEntryIdentity {
        &self.identity
    }

    pub const fn admitted_page_lsn_frontier(&self) -> Option<PageLsn> {
        self.admitted_page_lsn_frontier
    }

    pub fn source_decision_digest(&self) -> &str {
        &self.source_decision_digest
    }

    pub const fn recovery_counters(&self) -> RecoveryCounterSnapshot {
        self.recovery_counters
    }

    pub const fn s4_completed_lanes(&self) -> usize {
        self.s4_completed_lanes
    }

    pub const fn s4_required_lanes(&self) -> usize {
        self.s4_required_lanes
    }

    pub const fn s4_foundational_exact_counter_assertions(&self) -> usize {
        self.s4_foundational_exact_counter_assertions
    }

    pub const fn roadmap_requirements(&self) -> &S45RoadmapHarnessRequirementSet {
        &self.roadmap_requirements
    }

    pub const fn inventory(&self) -> &S45ExistingHarnessInventory {
        &self.inventory
    }

    pub fn non_claims(&self) -> &[S45HarnessNonClaim] {
        &self.non_claims
    }

    pub fn accepts_only_s4_closeout_and_roadmap2_harness_evidence(&self) -> bool {
        self.s4_completed_lanes == self.s4_required_lanes
            && self.roadmap_requirements.is_complete()
            && self
                .non_claims
                .contains(&S45HarnessNonClaim::NoS5PhysicalIsolationCorrectnessClaim)
    }
}
