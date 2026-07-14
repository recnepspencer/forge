use crate::{
    IntegrityCloseoutEvidenceFamily, PhysicalIntegrityAcceptanceSuite,
    PhysicalIntegrityCloseoutSuite, PhysicalProofOracleKind, PhysicalScenarioDriverKind,
    PhysicalScenarioObserverKind, PhysicalScenarioPlanIdentity, RoadmapLaneFamily,
};
use worth_store_contracts::StableDigest;
use worth_store_recovery_physics::IntegrityHandoffCounters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIntegrityCloseoutReport {
    acceptance_suite_count: usize,
    evidence_family_count: usize,
    harness_lane: RoadmapLaneFamily,
    suite_harnesses: Vec<IntegrityCloseoutHarnessSummary>,
    recovery_handoff_identity: StableDigest,
    recovery_counters: IntegrityHandoffCounters,
    no_raw_bytes_crossed: bool,
    recovery_claimed: bool,
    later_sequence_semantic_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityCloseoutHarnessSummary {
    acceptance_suite: PhysicalIntegrityAcceptanceSuite,
    evidence_family: IntegrityCloseoutEvidenceFamily,
    transcript_identity: PhysicalScenarioPlanIdentity,
    lane_family: RoadmapLaneFamily,
    driver_families: Vec<PhysicalScenarioDriverKind>,
    observer_families: Vec<PhysicalScenarioObserverKind>,
    oracle_families: Vec<PhysicalProofOracleKind>,
}

impl IntegrityCloseoutHarnessSummary {
    pub const fn acceptance_suite(&self) -> PhysicalIntegrityAcceptanceSuite {
        self.acceptance_suite
    }

    pub const fn evidence_family(&self) -> IntegrityCloseoutEvidenceFamily {
        self.evidence_family
    }

    pub const fn transcript_identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.transcript_identity
    }

    pub const fn lane_family(&self) -> RoadmapLaneFamily {
        self.lane_family
    }

    pub fn driver_families(&self) -> &[PhysicalScenarioDriverKind] {
        &self.driver_families
    }

    pub fn observer_families(&self) -> &[PhysicalScenarioObserverKind] {
        &self.observer_families
    }

    pub fn oracle_families(&self) -> &[PhysicalProofOracleKind] {
        &self.oracle_families
    }
}

impl PhysicalIntegrityCloseoutReport {
    pub(crate) fn from_closeout(
        suite: &PhysicalIntegrityCloseoutSuite,
        recovery_handoff_identity: StableDigest,
        recovery_counters: IntegrityHandoffCounters,
        no_raw_bytes_crossed: bool,
        recovery_claimed: bool,
        later_sequence_semantic_claimed: bool,
    ) -> Self {
        Self {
            acceptance_suite_count: suite.evidence().len(),
            evidence_family_count: IntegrityCloseoutEvidenceFamily::ALL
                .into_iter()
                .filter(|family| suite.contains_evidence_family(*family))
                .count(),
            harness_lane: RoadmapLaneFamily::Integrity,
            suite_harnesses: suite_harnesses(suite),
            recovery_handoff_identity,
            recovery_counters,
            no_raw_bytes_crossed,
            recovery_claimed,
            later_sequence_semantic_claimed,
        }
    }

    pub const fn acceptance_suite_count(&self) -> usize {
        self.acceptance_suite_count
    }

    pub const fn evidence_family_count(&self) -> usize {
        self.evidence_family_count
    }

    pub const fn harness_lane(&self) -> RoadmapLaneFamily {
        self.harness_lane
    }

    pub fn suite_harnesses(&self) -> &[IntegrityCloseoutHarnessSummary] {
        &self.suite_harnesses
    }

    pub const fn recovery_handoff_identity(&self) -> &StableDigest {
        &self.recovery_handoff_identity
    }

    pub const fn recovery_counters(&self) -> IntegrityHandoffCounters {
        self.recovery_counters
    }

    pub const fn proves_no_raw_bytes_crossed(&self) -> bool {
        self.no_raw_bytes_crossed
    }

    pub const fn reserves_recovery_physics(&self) -> bool {
        !self.recovery_claimed && !self.later_sequence_semantic_claimed
    }

    pub const fn proves_physical_integrity_closeout(&self) -> bool {
        self.acceptance_suite_count == PhysicalIntegrityAcceptanceSuite::ALL.len()
            && self.evidence_family_count == IntegrityCloseoutEvidenceFamily::ALL.len()
            && matches!(self.harness_lane, RoadmapLaneFamily::Integrity)
            && self.suite_harnesses.len() == PhysicalIntegrityAcceptanceSuite::ALL.len()
            && self.no_raw_bytes_crossed
            && !self.recovery_claimed
            && !self.later_sequence_semantic_claimed
    }
}

fn suite_harnesses(suite: &PhysicalIntegrityCloseoutSuite) -> Vec<IntegrityCloseoutHarnessSummary> {
    suite
        .evidence()
        .iter()
        .map(|evidence| {
            let harness = evidence.harness();
            IntegrityCloseoutHarnessSummary {
                acceptance_suite: evidence.acceptance_suite(),
                evidence_family: evidence.evidence_family(),
                transcript_identity: harness.transcript_identity().clone(),
                lane_family: harness.lane_family(),
                driver_families: harness.driver_families().to_vec(),
                observer_families: harness.observer_families().to_vec(),
                oracle_families: harness.oracle_families().to_vec(),
            }
        })
        .collect()
}
