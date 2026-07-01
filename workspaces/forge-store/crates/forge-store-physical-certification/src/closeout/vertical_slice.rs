use crate::{
    CoverageSurfaceKind, GeneratedCoverageMatrix, PhysicalCertificationEvidenceBundle,
    PhysicalScenarioCanonicalIdentity,
};

use super::{
    PhysicalSimulationHarnessCloseoutDenial, S4RecoveryDogfoodScenario,
    S5ReadinessShapeProbeScenario, ShortcutRejectionDogfoodScenario,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S45DogfoodSliceKind {
    S4Recovery,
    ShortcutRejection,
    S5ReadinessShapeProbe,
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
pub struct S5ReadinessShapeProbeSliceEvidence {
    scenario: S5ReadinessShapeProbeScenario,
    coverage: GeneratedCoverageMatrix,
    evidence: PhysicalCertificationEvidenceBundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45HarnessDogfoodEvidence {
    s4_recovery: S4RecoveryDogfoodSliceEvidence,
    shortcut_rejection: ShortcutRejectionDogfoodSliceEvidence,
    s5_readiness_shape_probe: S5ReadinessShapeProbeSliceEvidence,
}

impl S4RecoveryDogfoodSliceEvidence {
    pub fn from_replay_evidence(
        scenario: S4RecoveryDogfoodScenario,
        coverage: GeneratedCoverageMatrix,
        evidence: PhysicalCertificationEvidenceBundle,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_slice_evidence(
            S45DogfoodSliceKind::S4Recovery,
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
            S45DogfoodSliceKind::ShortcutRejection,
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

impl S5ReadinessShapeProbeSliceEvidence {
    pub fn from_replay_evidence(
        scenario: S5ReadinessShapeProbeScenario,
        coverage: GeneratedCoverageMatrix,
        evidence: PhysicalCertificationEvidenceBundle,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_slice_evidence(
            S45DogfoodSliceKind::S5ReadinessShapeProbe,
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

    pub const fn scenario(&self) -> &S5ReadinessShapeProbeScenario {
        &self.scenario
    }

    pub const fn coverage(&self) -> &GeneratedCoverageMatrix {
        &self.coverage
    }

    pub const fn evidence(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.evidence
    }
}

impl S45HarnessDogfoodEvidence {
    pub const fn new(
        s4_recovery: S4RecoveryDogfoodSliceEvidence,
        shortcut_rejection: ShortcutRejectionDogfoodSliceEvidence,
        s5_readiness_shape_probe: S5ReadinessShapeProbeSliceEvidence,
    ) -> Self {
        Self {
            s4_recovery,
            shortcut_rejection,
            s5_readiness_shape_probe,
        }
    }

    pub const fn s4_recovery(&self) -> &S4RecoveryDogfoodSliceEvidence {
        &self.s4_recovery
    }

    pub const fn shortcut_rejection(&self) -> &ShortcutRejectionDogfoodSliceEvidence {
        &self.shortcut_rejection
    }

    pub const fn s5_readiness_shape_probe(&self) -> &S5ReadinessShapeProbeSliceEvidence {
        &self.s5_readiness_shape_probe
    }
}

fn require_slice_evidence(
    slice: S45DogfoodSliceKind,
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
    _slice: S45DogfoodSliceKind,
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
