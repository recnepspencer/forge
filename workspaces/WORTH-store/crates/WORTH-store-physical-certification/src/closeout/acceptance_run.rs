use std::collections::BTreeSet;

use crate::{CoverageSurfaceKind, GeneratedCoverageMatrix};

use super::{
    lanes_from_closeout_evidence, required_s45_lanes, PhysicalSimulationHarnessCloseoutDenial,
    S45AcceptanceEvidenceLane, S45AcceptanceSuiteEvidenceSource, S45AcceptanceSuiteName,
    S45CloseoutCoverageReport, S45HarnessDogfoodEvidence,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45AcceptanceSuiteExecutionProof {
    source: S45AcceptanceSuiteEvidenceSource,
    lanes: Vec<S45AcceptanceEvidenceLane>,
    slice_scenario_digests: [[u8; 32]; 3],
    slice_transcript_digests: [[u8; 32]; 3],
    execution_basis_digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45ExecutedAcceptanceSuiteEvidence {
    proof: S45AcceptanceSuiteExecutionProof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45ExecutedAcceptanceSuiteEvidenceSet {
    executed: Vec<S45ExecutedAcceptanceSuiteEvidence>,
}

impl S45AcceptanceSuiteExecutionProof {
    pub(crate) fn from_closeout_suite_run(
        source: S45AcceptanceSuiteEvidenceSource,
        dogfood: &S45HarnessDogfoodEvidence,
        coverage: &S45CloseoutCoverageReport,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        let slice_scenario_digests = dogfood_slice_scenario_digests(dogfood);
        let slice_transcript_digests = dogfood_slice_transcript_digests(dogfood);
        require_slice_coverage_bindings(
            source.suite(),
            coverage,
            slice_scenario_digests,
            slice_transcript_digests,
        )?;
        let lanes = lanes_from_closeout_evidence(coverage);
        let evidence = Self {
            source,
            execution_basis_digest: execution_basis_digest(
                source,
                &lanes,
                &slice_scenario_digests,
                &slice_transcript_digests,
            ),
            lanes,
            slice_scenario_digests,
            slice_transcript_digests,
        };
        evidence.require_all_required_s45_lanes()?;
        Ok(evidence)
    }

    pub const fn suite(&self) -> S45AcceptanceSuiteName {
        self.source.suite()
    }

    pub const fn source(&self) -> S45AcceptanceSuiteEvidenceSource {
        self.source
    }

    pub fn lanes(&self) -> &[S45AcceptanceEvidenceLane] {
        &self.lanes
    }

    pub const fn slice_scenario_digests(&self) -> &[[u8; 32]; 3] {
        &self.slice_scenario_digests
    }

    pub const fn slice_transcript_digests(&self) -> &[[u8; 32]; 3] {
        &self.slice_transcript_digests
    }

    pub const fn execution_basis_digest(&self) -> &[u8; 32] {
        &self.execution_basis_digest
    }

    fn contains(&self, lane: S45AcceptanceEvidenceLane) -> bool {
        self.lanes.contains(&lane)
    }

    fn require_all_required_s45_lanes(
        &self,
    ) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        for lane in required_s45_lanes() {
            if !self.contains(lane) {
                return Err(
                    PhysicalSimulationHarnessCloseoutDenial::MissingAcceptanceSuiteLane {
                        suite: self.suite(),
                        lane,
                    },
                );
            }
        }
        Ok(())
    }
}

impl S45ExecutedAcceptanceSuiteEvidence {
    pub fn from_execution_proof(proof: S45AcceptanceSuiteExecutionProof) -> Self {
        Self { proof }
    }

    pub const fn suite(&self) -> S45AcceptanceSuiteName {
        self.proof.suite()
    }

    pub const fn source(&self) -> S45AcceptanceSuiteEvidenceSource {
        self.proof.source()
    }

    pub fn lanes(&self) -> &[S45AcceptanceEvidenceLane] {
        self.proof.lanes()
    }

    pub const fn slice_scenario_digests(&self) -> &[[u8; 32]; 3] {
        self.proof.slice_scenario_digests()
    }

    pub const fn slice_transcript_digests(&self) -> &[[u8; 32]; 3] {
        self.proof.slice_transcript_digests()
    }

    pub const fn execution_basis_digest(&self) -> &[u8; 32] {
        self.proof.execution_basis_digest()
    }

    pub(crate) fn into_lanes(self) -> Vec<S45AcceptanceEvidenceLane> {
        self.proof.lanes
    }
}

impl S45ExecutedAcceptanceSuiteEvidenceSet {
    pub fn from_executed_suites(
        executed: Vec<S45ExecutedAcceptanceSuiteEvidence>,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        let set = Self { executed };
        set.require_no_duplicate_suites()?;
        set.require_no_duplicate_execution_basis()?;
        set.require_all_required_suites()?;
        Ok(set)
    }

    pub fn executed(&self) -> &[S45ExecutedAcceptanceSuiteEvidence] {
        &self.executed
    }

    pub(crate) fn into_executed(self) -> Vec<S45ExecutedAcceptanceSuiteEvidence> {
        self.executed
    }

    fn require_no_duplicate_suites(&self) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        let mut seen = BTreeSet::new();
        for executed in self.executed.iter() {
            if !seen.insert(executed.suite()) {
                return Err(
                    PhysicalSimulationHarnessCloseoutDenial::DuplicateAcceptanceSuiteExecution {
                        suite: executed.suite(),
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
        for executed in self.executed.iter() {
            if !seen.insert(*executed.execution_basis_digest()) {
                return Err(
                    PhysicalSimulationHarnessCloseoutDenial::DuplicateAcceptanceSuiteExecution {
                        suite: executed.suite(),
                    },
                );
            }
        }
        Ok(())
    }

    fn require_all_required_suites(&self) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
        for suite in S45AcceptanceSuiteName::required_s45() {
            if !self
                .executed
                .iter()
                .any(|executed| executed.suite() == suite)
            {
                return Err(
                    PhysicalSimulationHarnessCloseoutDenial::MissingAcceptanceSuiteExecution {
                        suite,
                    },
                );
            }
        }
        Ok(())
    }
}

fn execution_basis_digest(
    source: S45AcceptanceSuiteEvidenceSource,
    lanes: &[S45AcceptanceEvidenceLane],
    scenario_digests: &[[u8; 32]; 3],
    transcript_digests: &[[u8; 32]; 3],
) -> [u8; 32] {
    let mut digest = [0_u8; 32];
    mix_byte(&mut digest, source.suite().ordinal());
    for lane in lanes {
        mix_byte(&mut digest, lane_ordinal(*lane));
    }
    for source_digest in scenario_digests.iter().chain(transcript_digests.iter()) {
        mix_bytes(&mut digest, source_digest);
    }
    digest
}

fn mix_bytes(digest: &mut [u8; 32], bytes: &[u8]) {
    for byte in bytes {
        mix_byte(digest, *byte);
    }
}

fn mix_byte(digest: &mut [u8; 32], byte: u8) {
    let carry = digest[31];
    for index in (1..digest.len()).rev() {
        digest[index] = digest[index - 1].rotate_left(1);
    }
    digest[0] = carry.rotate_left(1);
    digest[(byte as usize) % digest.len()] ^= byte.wrapping_mul(31).wrapping_add(17);
}

fn lane_ordinal(lane: S45AcceptanceEvidenceLane) -> u8 {
    match lane {
        S45AcceptanceEvidenceLane::Scenario => 0,
        S45AcceptanceEvidenceLane::Plan => 1,
        S45AcceptanceEvidenceLane::Schedule => 2,
        S45AcceptanceEvidenceLane::Actors => 3,
        S45AcceptanceEvidenceLane::Drivers => 4,
        S45AcceptanceEvidenceLane::Observers => 5,
        S45AcceptanceEvidenceLane::Oracles => 6,
        S45AcceptanceEvidenceLane::Transcripts => 7,
        S45AcceptanceEvidenceLane::Counters => 8,
        S45AcceptanceEvidenceLane::Positive => 9,
        S45AcceptanceEvidenceLane::Hostile => 10,
        S45AcceptanceEvidenceLane::Shortcut => 11,
        S45AcceptanceEvidenceLane::Replay => 12,
        S45AcceptanceEvidenceLane::Mutation => 13,
    }
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

fn require_slice_coverage_bindings(
    suite: S45AcceptanceSuiteName,
    coverage: &S45CloseoutCoverageReport,
    scenario_digests: [[u8; 32]; 3],
    transcript_digests: [[u8; 32]; 3],
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    let matrices = coverage.matrices();
    for index in 0..matrices.len() {
        require_matrix_source(
            suite,
            matrices[index],
            CoverageSurfaceKind::Scenario,
            &scenario_digests[index],
        )?;
        require_matrix_source(
            suite,
            matrices[index],
            CoverageSurfaceKind::Transcript,
            &transcript_digests[index],
        )?;
    }
    Ok(())
}

fn require_matrix_source(
    suite: S45AcceptanceSuiteName,
    matrix: &GeneratedCoverageMatrix,
    surface: CoverageSurfaceKind,
    source_identity: &[u8; 32],
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    if matrix
        .rows()
        .iter()
        .any(|row| row.surface() == surface && row.source_identity() == source_identity)
    {
        Ok(())
    } else {
        Err(
            PhysicalSimulationHarnessCloseoutDenial::MissingAcceptanceSuiteLane {
                suite,
                lane: S45AcceptanceEvidenceLane::Transcripts,
            },
        )
    }
}
