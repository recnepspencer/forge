use std::collections::BTreeSet;

use crate::{CoverageRowDimension, CoverageSurfaceKind};

use super::{
    PhysicalSimulationHarnessCloseoutDenial, S45AcceptanceSuiteEvidenceSource,
    S45AcceptanceSuiteName, S45ExecutedAcceptanceSuiteEvidence, S45HarnessDogfoodEvidence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S45AcceptanceEvidenceLane {
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
pub struct S45AcceptanceSuiteReceipt {
    suite: S45AcceptanceSuiteName,
    source: S45AcceptanceSuiteEvidenceSource,
    lanes: Vec<S45AcceptanceEvidenceLane>,
    slice_scenario_digests: [[u8; 32]; 3],
    slice_transcript_digests: [[u8; 32]; 3],
    execution_basis_digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45AcceptanceSuiteEvidence {
    executed: S45ExecutedAcceptanceSuiteEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45AcceptanceSuiteReceiptSet {
    receipts: Vec<S45AcceptanceSuiteReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45AcceptanceSuiteMap {
    suites: Vec<S45AcceptanceSuiteCoverage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45AcceptanceSuiteCoverage {
    suite: S45AcceptanceSuiteName,
    source: S45AcceptanceSuiteEvidenceSource,
    lanes: Vec<S45AcceptanceEvidenceLane>,
    execution_basis_digest: [u8; 32],
}

impl S45AcceptanceSuiteEvidence {
    pub fn from_executed_suite(executed: S45ExecutedAcceptanceSuiteEvidence) -> Self {
        Self { executed }
    }

    pub const fn suite(&self) -> S45AcceptanceSuiteName {
        self.executed.suite()
    }

    pub const fn source(&self) -> S45AcceptanceSuiteEvidenceSource {
        self.executed.source()
    }
}

impl S45AcceptanceSuiteReceipt {
    pub fn from_suite_evidence(
        evidence: S45AcceptanceSuiteEvidence,
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
        receipt.require_all_required_s45_lanes()?;
        Ok(receipt)
    }

    pub const fn suite(&self) -> S45AcceptanceSuiteName {
        self.suite
    }

    pub fn lanes(&self) -> &[S45AcceptanceEvidenceLane] {
        &self.lanes
    }

    pub const fn source(&self) -> S45AcceptanceSuiteEvidenceSource {
        self.source
    }

    pub const fn execution_basis_digest(&self) -> &[u8; 32] {
        &self.execution_basis_digest
    }

    fn contains(&self, lane: S45AcceptanceEvidenceLane) -> bool {
        self.lanes.contains(&lane)
    }

    fn is_bound_to(&self, dogfood: &S45HarnessDogfoodEvidence) -> bool {
        self.slice_scenario_digests == dogfood_slice_scenario_digests(dogfood)
            && self.slice_transcript_digests == dogfood_slice_transcript_digests(dogfood)
    }

    fn require_all_required_s45_lanes(
        &self,
    ) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        for lane in required_s45_lanes() {
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

impl S45AcceptanceSuiteReceiptSet {
    pub fn from_receipts(
        receipts: Vec<S45AcceptanceSuiteReceipt>,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        let set = Self { receipts };
        set.require_no_duplicate_suites()?;
        set.require_no_duplicate_execution_basis()?;
        set.require_all_required_suites()?;
        set.require_all_receipt_lanes()?;
        Ok(set)
    }

    pub fn receipts(&self) -> &[S45AcceptanceSuiteReceipt] {
        &self.receipts
    }

    pub(crate) fn into_suite_map_bound_to(
        self,
        dogfood: &S45HarnessDogfoodEvidence,
    ) -> Result<S45AcceptanceSuiteMap, PhysicalSimulationHarnessCloseoutDenial> {
        self.require_receipts_bound_to(dogfood)?;
        let suites = self
            .receipts
            .into_iter()
            .map(|receipt| S45AcceptanceSuiteCoverage {
                suite: receipt.suite,
                source: receipt.source,
                lanes: receipt.lanes,
                execution_basis_digest: receipt.execution_basis_digest,
            })
            .collect();
        Ok(S45AcceptanceSuiteMap { suites })
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
        for suite in S45AcceptanceSuiteName::required_s45() {
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
            receipt.require_all_required_s45_lanes()?;
        }
        Ok(())
    }

    fn require_receipts_bound_to(
        &self,
        dogfood: &S45HarnessDogfoodEvidence,
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

impl S45AcceptanceSuiteMap {
    pub fn suites(&self) -> &[S45AcceptanceSuiteCoverage] {
        &self.suites
    }

    pub fn all_required_s45_lanes_are_satisfied(&self) -> bool {
        self.suites.iter().all(|suite| {
            required_s45_lanes()
                .into_iter()
                .all(|lane| suite.contains(lane))
        })
    }
}

impl S45AcceptanceSuiteCoverage {
    pub const fn suite(&self) -> S45AcceptanceSuiteName {
        self.suite
    }

    pub fn lanes(&self) -> &[S45AcceptanceEvidenceLane] {
        &self.lanes
    }

    pub const fn source(&self) -> S45AcceptanceSuiteEvidenceSource {
        self.source
    }

    pub const fn execution_basis_digest(&self) -> &[u8; 32] {
        &self.execution_basis_digest
    }

    pub fn contains(&self, lane: S45AcceptanceEvidenceLane) -> bool {
        self.lanes.contains(&lane)
    }
}

pub(crate) fn lanes_from_closeout_evidence(
    coverage: &super::S45CloseoutCoverageReport,
) -> Vec<S45AcceptanceEvidenceLane> {
    let mut lanes = BTreeSet::new();
    for matrix in coverage.matrices() {
        for row in matrix.rows() {
            match row.surface() {
                CoverageSurfaceKind::Scenario => {
                    lanes.insert(S45AcceptanceEvidenceLane::Scenario);
                    lanes.insert(S45AcceptanceEvidenceLane::Positive);
                }
                CoverageSurfaceKind::Plan => {
                    lanes.insert(S45AcceptanceEvidenceLane::Plan);
                }
                CoverageSurfaceKind::YieldpointSchedule => {
                    lanes.insert(S45AcceptanceEvidenceLane::Schedule);
                }
                CoverageSurfaceKind::Actor => {
                    lanes.insert(S45AcceptanceEvidenceLane::Actors);
                }
                CoverageSurfaceKind::Driver => {
                    lanes.insert(S45AcceptanceEvidenceLane::Drivers);
                    lanes.insert(S45AcceptanceEvidenceLane::Hostile);
                }
                CoverageSurfaceKind::Oracle => {
                    lanes.insert(S45AcceptanceEvidenceLane::Oracles);
                }
                CoverageSurfaceKind::Counter => {
                    lanes.insert(S45AcceptanceEvidenceLane::Counters);
                }
                CoverageSurfaceKind::Transcript => {
                    lanes.insert(S45AcceptanceEvidenceLane::Transcripts);
                    lanes.insert(S45AcceptanceEvidenceLane::Replay);
                }
                CoverageSurfaceKind::MutationResult => {
                    lanes.insert(S45AcceptanceEvidenceLane::Mutation);
                    lanes.insert(S45AcceptanceEvidenceLane::Shortcut);
                }
            }
            if row
                .dimensions()
                .iter()
                .any(|dimension| matches!(dimension, CoverageRowDimension::OfflineVerifier(_)))
            {
                lanes.insert(S45AcceptanceEvidenceLane::Observers);
            }
        }
    }
    lanes.into_iter().collect()
}

pub(crate) fn required_s45_lanes() -> [S45AcceptanceEvidenceLane; 14] {
    [
        S45AcceptanceEvidenceLane::Scenario,
        S45AcceptanceEvidenceLane::Plan,
        S45AcceptanceEvidenceLane::Schedule,
        S45AcceptanceEvidenceLane::Actors,
        S45AcceptanceEvidenceLane::Drivers,
        S45AcceptanceEvidenceLane::Observers,
        S45AcceptanceEvidenceLane::Oracles,
        S45AcceptanceEvidenceLane::Transcripts,
        S45AcceptanceEvidenceLane::Counters,
        S45AcceptanceEvidenceLane::Positive,
        S45AcceptanceEvidenceLane::Hostile,
        S45AcceptanceEvidenceLane::Shortcut,
        S45AcceptanceEvidenceLane::Replay,
        S45AcceptanceEvidenceLane::Mutation,
    ]
}

fn dogfood_slice_scenario_digests(dogfood: &S45HarnessDogfoodEvidence) -> [[u8; 32]; 3] {
    [
        *dogfood
            .s4_recovery()
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
            .s5_readiness_shape_probe()
            .scenario()
            .scenario()
            .identity()
            .digest_bytes(),
    ]
}

fn dogfood_slice_transcript_digests(dogfood: &S45HarnessDogfoodEvidence) -> [[u8; 32]; 3] {
    [
        *dogfood
            .s4_recovery()
            .evidence()
            .primary()
            .transcript_digest(),
        *dogfood
            .shortcut_rejection()
            .evidence()
            .primary()
            .transcript_digest(),
        *dogfood
            .s5_readiness_shape_probe()
            .evidence()
            .primary()
            .transcript_digest(),
    ]
}
