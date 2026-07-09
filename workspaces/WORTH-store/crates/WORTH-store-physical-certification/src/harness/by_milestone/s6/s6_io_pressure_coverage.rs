use crate::{
    CoverageRowDimension, CoverageSurfaceKind, PhysicalCoverageMatrixRow, Roadmap2HarnessSequence,
    S6IoPressureHarnessEvidence,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6ExecutedIoPressureCoverageRows {
    replay_identity: [u8; 32],
    rows: [PhysicalCoverageMatrixRow; 6],
}

impl S6IoPressureHarnessEvidence {
    pub fn executed_replay_coverage_rows(&self) -> S6ExecutedIoPressureCoverageRows {
        let replay_identity = *self.replay_identity();
        let dimensions = coverage_dimensions(self);
        let rows = [
            generated_row(CoverageSurfaceKind::Scenario, replay_identity, &dimensions),
            generated_row(CoverageSurfaceKind::Actor, replay_identity, &dimensions),
            generated_row(CoverageSurfaceKind::Driver, replay_identity, &dimensions),
            generated_row(CoverageSurfaceKind::Counter, replay_identity, &dimensions),
            generated_row(CoverageSurfaceKind::Oracle, replay_identity, &dimensions),
            generated_row(
                CoverageSurfaceKind::Transcript,
                replay_identity,
                &dimensions,
            ),
        ];
        S6ExecutedIoPressureCoverageRows {
            replay_identity,
            rows,
        }
    }
}

impl S6ExecutedIoPressureCoverageRows {
    pub const fn replay_identity(&self) -> &[u8; 32] {
        &self.replay_identity
    }

    pub const fn rows(&self) -> &[PhysicalCoverageMatrixRow; 6] {
        &self.rows
    }

    pub fn iter(&self) -> impl Iterator<Item = &PhysicalCoverageMatrixRow> {
        self.rows.iter()
    }
}

fn coverage_dimensions(evidence: &S6IoPressureHarnessEvidence) -> [CoverageRowDimension; 10] {
    [
        CoverageRowDimension::ResourceEnvelopeProfile(evidence.replay_profile()),
        CoverageRowDimension::BackgroundInterference(evidence.driver()),
        CoverageRowDimension::FaultPhase(evidence.fault_phase()),
        CoverageRowDimension::S6BackendTarget(evidence.scenario().backend_profile()),
        CoverageRowDimension::S6ForegroundLane(evidence.scenario().foreground_lane()),
        CoverageRowDimension::S6BackgroundPressure(evidence.scenario().background_pressure()),
        CoverageRowDimension::S6SecureIoPosture(evidence.scenario().secure_io_posture()),
        CoverageRowDimension::S6IoPressureFaultKind(evidence.scenario().fault_kind()),
        CoverageRowDimension::S6FaultEvidenceClass(evidence.scenario().fault_evidence_class()),
        CoverageRowDimension::S6EvidenceMaturity(evidence.maturity()),
    ]
}

fn generated_row(
    surface: CoverageSurfaceKind,
    source_identity: [u8; 32],
    dimensions: &[CoverageRowDimension],
) -> PhysicalCoverageMatrixRow {
    PhysicalCoverageMatrixRow::generated(
        Roadmap2HarnessSequence::S45,
        surface,
        source_identity,
        dimensions.iter().cloned(),
    )
}
