use crate::{
    CoverageSurfaceKind, GeneratedCoverageMatrix, PhysicalCertificationEvidenceBundle,
    PhysicalScenarioCanonicalIdentity,
};

use super::{
    PhysicalIsolationReadinessShapeProbeScenario, PhysicalSimulationHarnessCloseoutDenial,
    S4RecoveryDogfoodScenario, ShortcutRejectionDogfoodScenario,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimulationHarnessDogfoodSliceKind {
    S4Recovery,
    ShortcutRejection,
    PhysicalIsolationReadinessShapeProbe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4RecoveryDogfoodSliceEvidence {
    scenario: S4RecoveryDogfoodScenario,
    coverage: GeneratedCoverageMatrix,
    evidence: PhysicalCertificationEvidenceBundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutRejectionDogfoodSliceEvidence {
    scenario: ShortcutRejectionDogfoodScenario,
    coverage: GeneratedCoverageMatrix,
    evidence: PhysicalCertificationEvidenceBundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationReadinessShapeProbeSliceEvidence {
    scenario: PhysicalIsolationReadinessShapeProbeScenario,
    coverage: GeneratedCoverageMatrix,
    evidence: PhysicalCertificationEvidenceBundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessDogfoodEvidence {
    s4_recovery: S4RecoveryDogfoodSliceEvidence,
    shortcut_rejection: ShortcutRejectionDogfoodSliceEvidence,
    physical_isolation_readiness_shape_probe: PhysicalIsolationReadinessShapeProbeSliceEvidence,
}

impl S4RecoveryDogfoodSliceEvidence {
    pub fn from_replay_evidence(
        scenario: S4RecoveryDogfoodScenario,
        coverage: GeneratedCoverageMatrix,
        evidence: PhysicalCertificationEvidenceBundle,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_slice_evidence(
            SimulationHarnessDogfoodSliceKind::S4Recovery,
            scenario.scenario().identity(),
            &coverage,
            &evidence,
        )?;
        Ok(Self {
            scenario,
            coverage,
            evidence,
        })
    }

    pub const fn scenario(&self) -> &S4RecoveryDogfoodScenario {
        &self.scenario
    }

    pub const fn coverage(&self) -> &GeneratedCoverageMatrix {
        &self.coverage
    }

    pub const fn evidence(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.evidence
    }
}

impl ShortcutRejectionDogfoodSliceEvidence {
    pub fn from_replay_evidence(
        scenario: ShortcutRejectionDogfoodScenario,
        coverage: GeneratedCoverageMatrix,
        evidence: PhysicalCertificationEvidenceBundle,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_slice_evidence(
            SimulationHarnessDogfoodSliceKind::ShortcutRejection,
            scenario.scenario().identity(),
            &coverage,
            &evidence,
        )?;
        Ok(Self {
            scenario,
            coverage,
            evidence,
        })
    }

    pub const fn scenario(&self) -> &ShortcutRejectionDogfoodScenario {
        &self.scenario
    }

    pub const fn coverage(&self) -> &GeneratedCoverageMatrix {
        &self.coverage
    }

    pub const fn evidence(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.evidence
    }
}

impl PhysicalIsolationReadinessShapeProbeSliceEvidence {
    pub fn from_replay_evidence(
        scenario: PhysicalIsolationReadinessShapeProbeScenario,
        coverage: GeneratedCoverageMatrix,
        evidence: PhysicalCertificationEvidenceBundle,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_slice_evidence(
            SimulationHarnessDogfoodSliceKind::PhysicalIsolationReadinessShapeProbe,
            scenario.scenario().identity(),
            &coverage,
            &evidence,
        )?;
        Ok(Self {
            scenario,
            coverage,
            evidence,
        })
    }

    pub const fn scenario(&self) -> &PhysicalIsolationReadinessShapeProbeScenario {
        &self.scenario
    }

    pub const fn coverage(&self) -> &GeneratedCoverageMatrix {
        &self.coverage
    }

    pub const fn evidence(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.evidence
    }
}

impl SimulationHarnessDogfoodEvidence {
    pub const fn new(
        s4_recovery: S4RecoveryDogfoodSliceEvidence,
        shortcut_rejection: ShortcutRejectionDogfoodSliceEvidence,
        physical_isolation_readiness_shape_probe: PhysicalIsolationReadinessShapeProbeSliceEvidence,
    ) -> Self {
        Self {
            s4_recovery,
            shortcut_rejection,
            physical_isolation_readiness_shape_probe,
        }
    }

    pub const fn s4_recovery(&self) -> &S4RecoveryDogfoodSliceEvidence {
        &self.s4_recovery
    }

    pub const fn shortcut_rejection(&self) -> &ShortcutRejectionDogfoodSliceEvidence {
        &self.shortcut_rejection
    }

    pub const fn physical_isolation_readiness_shape_probe(
        &self,
    ) -> &PhysicalIsolationReadinessShapeProbeSliceEvidence {
        &self.physical_isolation_readiness_shape_probe
    }
}

fn require_slice_evidence(
    slice: SimulationHarnessDogfoodSliceKind,
    scenario_identity: &PhysicalScenarioCanonicalIdentity,
    coverage: &GeneratedCoverageMatrix,
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    let primary = evidence.primary();
    if primary.scenario_digest() != scenario_identity.digest_bytes() {
        return Err(
            PhysicalSimulationHarnessCloseoutDenial::DogfoodSliceScenarioEvidenceMismatch { slice },
        );
    }
    require_coverage_source(
        slice,
        coverage,
        CoverageSurfaceKind::Scenario,
        scenario_identity.digest_bytes(),
        PhysicalSimulationHarnessCloseoutDenial::DogfoodSliceScenarioCoverageMissing { slice },
    )?;
    require_coverage_source(
        slice,
        coverage,
        CoverageSurfaceKind::Transcript,
        primary.transcript_digest(),
        PhysicalSimulationHarnessCloseoutDenial::DogfoodSliceTranscriptCoverageMissing { slice },
    )
}

fn require_coverage_source(
    _slice: SimulationHarnessDogfoodSliceKind,
    coverage: &GeneratedCoverageMatrix,
    surface: CoverageSurfaceKind,
    source_identity: &[u8; 32],
    denial: PhysicalSimulationHarnessCloseoutDenial,
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    if coverage
        .rows()
        .iter()
        .any(|row| row.surface() == surface && row.source_identity() == source_identity)
    {
        Ok(())
    } else {
        Err(denial)
    }
}
