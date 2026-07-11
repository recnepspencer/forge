use super::Roadmap2CoverageRegistry;
use crate::SimulationReplayBundle;

use super::super::{
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, PhysicalCoverageMatrixRow,
};

impl Roadmap2CoverageRegistry {
    pub fn register_transcript(
        mut self,
        replay: &SimulationReplayBundle,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Transcript)?;
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Transcript,
                })?;
        if replay.plan().identity() != plan.identity() {
            return Err(CoverageGapDenial::TranscriptPlanMismatch);
        }
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Transcript,
            *replay.transcript_identity().digest_bytes(),
            [
                CoverageRowDimension::TranscriptOutput,
                CoverageRowDimension::OfflineVerifier(replay.trace().observer()),
            ],
        ));
        Ok(self)
    }

    pub fn register_transcript_surface_from_plan(mut self) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Transcript)?;
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Transcript,
                })?;
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Transcript,
            *plan.identity().digest_bytes(),
            std::iter::once(CoverageRowDimension::TranscriptOutput).chain(
                plan.observers()
                    .iter()
                    .map(CoverageRowDimension::OfflineVerifier),
            ),
        ));
        Ok(self)
    }
}
