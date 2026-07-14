use crate::{
    CoverageRowDimension, CoverageSurfaceKind, HarnessCoverageStage, IoPressureHarnessEvidence,
    PhysicalCoverageMatrixRow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedIoPressureCoverageRows {
    replay_identity: [u8; 32],
    rows: [PhysicalCoverageMatrixRow; 6],
}

impl IoPressureHarnessEvidence {
    pub fn executed_replay_coverage_rows(&self) -> ExecutedIoPressureCoverageRows {
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
        ExecutedIoPressureCoverageRows {
            replay_identity,
            rows,
        }
    }
}

impl ExecutedIoPressureCoverageRows {
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

fn coverage_dimensions(evidence: &IoPressureHarnessEvidence) -> [CoverageRowDimension; 10] {
    [
        CoverageRowDimension::ResourceEnvelopeProfile(evidence.replay_profile()),
        CoverageRowDimension::BackgroundInterference(evidence.driver()),
        CoverageRowDimension::FaultPhase(evidence.fault_phase()),
        CoverageRowDimension::IoPressureBackendTarget(evidence.scenario().backend_profile()),
        CoverageRowDimension::IoPressureForegroundLane(evidence.scenario().foreground_lane()),
        CoverageRowDimension::IoPressureBackgroundPressure(
            evidence.scenario().background_pressure(),
        ),
        CoverageRowDimension::SecureIoPosture(evidence.scenario().secure_io_posture()),
        CoverageRowDimension::IoPressureFaultKind(evidence.scenario().fault_kind()),
        CoverageRowDimension::IoPressureFaultEvidenceClass(
            evidence.scenario().fault_evidence_class(),
        ),
        CoverageRowDimension::IoPressureEvidenceMaturity(evidence.maturity()),
    ]
}

fn generated_row(
    surface: CoverageSurfaceKind,
    source_identity: [u8; 32],
    dimensions: &[CoverageRowDimension],
) -> PhysicalCoverageMatrixRow {
    PhysicalCoverageMatrixRow::generated(
        HarnessCoverageStage::SimulationAdmission,
        surface,
        source_identity,
        dimensions.iter().cloned(),
    )
}
