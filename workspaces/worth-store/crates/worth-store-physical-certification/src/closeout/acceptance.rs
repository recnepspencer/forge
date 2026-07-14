use std::collections::BTreeSet;

use crate::{CoverageRowDimension, CoverageSurfaceKind};

use super::{
    ExecutedSimulationHarnessAcceptanceSuiteEvidence, PhysicalSimulationHarnessCloseoutDenial,
    SimulationHarnessAcceptanceSuiteEvidenceSource, SimulationHarnessAcceptanceSuiteName,
    SimulationHarnessDogfoodEvidence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimulationHarnessAcceptanceEvidenceLane {
    Scenario,
    Plan,
    Schedule,
    Actors,
    Drivers,
    Observers,
    Oracles,
    Transcripts,
    Counters,
    Positive,
    Hostile,
    Shortcut,
    Replay,
    Mutation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessAcceptanceSuiteReceipt {
    suite: SimulationHarnessAcceptanceSuiteName,
    source: SimulationHarnessAcceptanceSuiteEvidenceSource,
    lanes: Vec<SimulationHarnessAcceptanceEvidenceLane>,
    slice_scenario_digests: [[u8; 32]; 3],
    slice_transcript_digests: [[u8; 32]; 3],
    execution_basis_digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessAcceptanceSuiteEvidence {
    executed: ExecutedSimulationHarnessAcceptanceSuiteEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessAcceptanceSuiteReceiptSet {
    receipts: Vec<SimulationHarnessAcceptanceSuiteReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessAcceptanceSuiteMap {
    suites: Vec<SimulationHarnessAcceptanceSuiteCoverage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessAcceptanceSuiteCoverage {
    suite: SimulationHarnessAcceptanceSuiteName,
    source: SimulationHarnessAcceptanceSuiteEvidenceSource,
    lanes: Vec<SimulationHarnessAcceptanceEvidenceLane>,
    execution_basis_digest: [u8; 32],
}

impl SimulationHarnessAcceptanceSuiteEvidence {
    pub fn from_executed_suite(executed: ExecutedSimulationHarnessAcceptanceSuiteEvidence) -> Self {
        Self { executed }
    }

    pub const fn suite(&self) -> SimulationHarnessAcceptanceSuiteName {
        self.executed.suite()
    }

    pub const fn source(&self) -> SimulationHarnessAcceptanceSuiteEvidenceSource {
        self.executed.source()
    }
}

impl SimulationHarnessAcceptanceSuiteReceipt {
    pub fn from_suite_evidence(
        evidence: SimulationHarnessAcceptanceSuiteEvidence,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        let slice_scenario_digests = *evidence.executed.slice_scenario_digests();
        let slice_transcript_digests = *evidence.executed.slice_transcript_digests();
        let execution_basis_digest = *evidence.executed.execution_basis_digest();
        let receipt = Self {
            suite: evidence.suite(),
            source: evidence.source(),
            lanes: evidence.executed.into_lanes(),
            slice_scenario_digests,
            slice_transcript_digests,
            execution_basis_digest,
        };
        receipt.require_all_required_simulation_harness_lanes()?;
        Ok(receipt)
    }

    pub const fn suite(&self) -> SimulationHarnessAcceptanceSuiteName {
        self.suite
    }

    pub fn lanes(&self) -> &[SimulationHarnessAcceptanceEvidenceLane] {
        &self.lanes
    }

    pub const fn source(&self) -> SimulationHarnessAcceptanceSuiteEvidenceSource {
        self.source
    }

    pub const fn execution_basis_digest(&self) -> &[u8; 32] {
        &self.execution_basis_digest
    }

    fn contains(&self, lane: SimulationHarnessAcceptanceEvidenceLane) -> bool {
        self.lanes.contains(&lane)
    }

    fn is_bound_to(&self, dogfood: &SimulationHarnessDogfoodEvidence) -> bool {
        self.slice_scenario_digests == dogfood_slice_scenario_digests(dogfood)
            && self.slice_transcript_digests == dogfood_slice_transcript_digests(dogfood)
    }

    fn require_all_required_simulation_harness_lanes(
        &self,
    ) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        for lane in required_simulation_harness_lanes() {
            if !self.contains(lane) {
                return Err(
                    PhysicalSimulationHarnessCloseoutDenial::MissingAcceptanceSuiteLane {
                        suite: self.suite,
                        lane,
                    },
                );
            }
        }
        Ok(())
    }
}

impl SimulationHarnessAcceptanceSuiteReceiptSet {
    pub fn from_receipts(
        receipts: Vec<SimulationHarnessAcceptanceSuiteReceipt>,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        let set = Self { receipts };
        set.require_no_duplicate_suites()?;
        set.require_no_duplicate_execution_basis()?;
        set.require_all_required_suites()?;
        set.require_all_receipt_lanes()?;
        Ok(set)
    }

    pub fn receipts(&self) -> &[SimulationHarnessAcceptanceSuiteReceipt] {
        &self.receipts
    }

    pub(crate) fn into_suite_map_bound_to(
        self,
        dogfood: &SimulationHarnessDogfoodEvidence,
    ) -> Result<SimulationHarnessAcceptanceSuiteMap, PhysicalSimulationHarnessCloseoutDenial> {
        self.require_receipts_bound_to(dogfood)?;
        let suites = self
            .receipts
            .into_iter()
            .map(|receipt| SimulationHarnessAcceptanceSuiteCoverage {
                suite: receipt.suite,
                source: receipt.source,
                lanes: receipt.lanes,
                execution_basis_digest: receipt.execution_basis_digest,
            })
            .collect();
        Ok(SimulationHarnessAcceptanceSuiteMap { suites })
    }

    fn require_no_duplicate_suites(&self) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        let mut seen = BTreeSet::new();
        for receipt in self.receipts.iter() {
            if !seen.insert(receipt.suite()) {
                return Err(
                    PhysicalSimulationHarnessCloseoutDenial::DuplicateAcceptanceSuiteReceipt {
                        suite: receipt.suite(),
                    },
                );
            }
        }
        Ok(())
    }

    fn require_no_duplicate_execution_basis(
        &self,
    ) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        let mut seen = BTreeSet::new();
        for receipt in self.receipts.iter() {
            if !seen.insert(*receipt.execution_basis_digest()) {
                return Err(
                    PhysicalSimulationHarnessCloseoutDenial::DuplicateAcceptanceSuiteReceipt {
                        suite: receipt.suite(),
                    },
                );
            }
        }
        Ok(())
    }

    fn require_all_required_suites(&self) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        for suite in SimulationHarnessAcceptanceSuiteName::required_simulation_harness() {
            if !self.receipts.iter().any(|receipt| receipt.suite() == suite) {
                return Err(
                    PhysicalSimulationHarnessCloseoutDenial::MissingAcceptanceSuiteReceipt {
                        suite,
                    },
                );
            }
        }
        Ok(())
    }

    fn require_all_receipt_lanes(&self) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        for receipt in self.receipts.iter() {
            receipt.require_all_required_simulation_harness_lanes()?;
        }
        Ok(())
    }

    fn require_receipts_bound_to(
        &self,
        dogfood: &SimulationHarnessDogfoodEvidence,
    ) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        for receipt in self.receipts.iter() {
            if !receipt.is_bound_to(dogfood) {
                return Err(
                    PhysicalSimulationHarnessCloseoutDenial::StaleAcceptanceSuiteReceipt {
                        suite: receipt.suite(),
                    },
                );
            }
        }
        Ok(())
    }
}

impl SimulationHarnessAcceptanceSuiteMap {
    pub fn suites(&self) -> &[SimulationHarnessAcceptanceSuiteCoverage] {
        &self.suites
    }

    pub fn all_required_simulation_harness_lanes_are_satisfied(&self) -> bool {
        self.suites.iter().all(|suite| {
            required_simulation_harness_lanes()
                .into_iter()
                .all(|lane| suite.contains(lane))
        })
    }
}

impl SimulationHarnessAcceptanceSuiteCoverage {
    pub const fn suite(&self) -> SimulationHarnessAcceptanceSuiteName {
        self.suite
    }

    pub fn lanes(&self) -> &[SimulationHarnessAcceptanceEvidenceLane] {
        &self.lanes
    }

    pub const fn source(&self) -> SimulationHarnessAcceptanceSuiteEvidenceSource {
        self.source
    }

    pub const fn execution_basis_digest(&self) -> &[u8; 32] {
        &self.execution_basis_digest
    }

    pub fn contains(&self, lane: SimulationHarnessAcceptanceEvidenceLane) -> bool {
        self.lanes.contains(&lane)
    }
}

pub(crate) fn lanes_from_closeout_evidence(
    coverage: &super::SimulationHarnessCloseoutCoverageReport,
) -> Vec<SimulationHarnessAcceptanceEvidenceLane> {
    let mut lanes = BTreeSet::new();
    for matrix in coverage.matrices() {
        for row in matrix.rows() {
            match row.surface() {
                CoverageSurfaceKind::Scenario => {
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Scenario);
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Positive);
                }
                CoverageSurfaceKind::Plan => {
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Plan);
                }
                CoverageSurfaceKind::YieldpointSchedule => {
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Schedule);
                }
                CoverageSurfaceKind::Actor => {
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Actors);
                }
                CoverageSurfaceKind::Driver => {
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Drivers);
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Hostile);
                }
                CoverageSurfaceKind::Oracle => {
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Oracles);
                }
                CoverageSurfaceKind::Counter => {
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Counters);
                }
                CoverageSurfaceKind::Transcript => {
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Transcripts);
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Replay);
                }
                CoverageSurfaceKind::MutationResult => {
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Mutation);
                    lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Shortcut);
                }
            }
            if row
                .dimensions()
                .iter()
                .any(|dimension| matches!(dimension, CoverageRowDimension::OfflineVerifier(_)))
            {
                lanes.insert(SimulationHarnessAcceptanceEvidenceLane::Observers);
            }
        }
    }
    lanes.into_iter().collect()
}

pub(crate) fn required_simulation_harness_lanes() -> [SimulationHarnessAcceptanceEvidenceLane; 14] {
    [
        SimulationHarnessAcceptanceEvidenceLane::Scenario,
        SimulationHarnessAcceptanceEvidenceLane::Plan,
        SimulationHarnessAcceptanceEvidenceLane::Schedule,
        SimulationHarnessAcceptanceEvidenceLane::Actors,
        SimulationHarnessAcceptanceEvidenceLane::Drivers,
        SimulationHarnessAcceptanceEvidenceLane::Observers,
        SimulationHarnessAcceptanceEvidenceLane::Oracles,
        SimulationHarnessAcceptanceEvidenceLane::Transcripts,
        SimulationHarnessAcceptanceEvidenceLane::Counters,
        SimulationHarnessAcceptanceEvidenceLane::Positive,
        SimulationHarnessAcceptanceEvidenceLane::Hostile,
        SimulationHarnessAcceptanceEvidenceLane::Shortcut,
        SimulationHarnessAcceptanceEvidenceLane::Replay,
        SimulationHarnessAcceptanceEvidenceLane::Mutation,
    ]
}

fn dogfood_slice_scenario_digests(dogfood: &SimulationHarnessDogfoodEvidence) -> [[u8; 32]; 3] {
    [
        *dogfood
            .recovery()
            .scenario()
            .scenario()
            .identity()
            .digest_bytes(),
        *dogfood
            .shortcut_rejection()
            .scenario()
            .scenario()
            .identity()
            .digest_bytes(),
        *dogfood
            .physical_isolation_readiness_shape_probe()
            .scenario()
            .scenario()
            .identity()
            .digest_bytes(),
    ]
}

fn dogfood_slice_transcript_digests(dogfood: &SimulationHarnessDogfoodEvidence) -> [[u8; 32]; 3] {
    [
        *dogfood.recovery().evidence().primary().transcript_digest(),
        *dogfood
            .shortcut_rejection()
            .evidence()
            .primary()
            .transcript_digest(),
        *dogfood
            .physical_isolation_readiness_shape_probe()
            .evidence()
            .primary()
            .transcript_digest(),
    ]
}
