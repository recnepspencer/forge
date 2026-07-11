mod counter;
mod mutation;
mod oracle;
mod scenario_plan_identity;
mod schedule_actor_driver;
mod transcript;

use super::{
    CoverageGapDenial, CoverageSurfaceKind, GeneratedCoverageMatrix, HarnessCoverageStage,
    PhysicalCoverageMatrix, PhysicalCoverageMatrixRow,
};
use crate::{PhysicalScenarioCanonicalIdentity, PhysicalSimulationPlan};

#[derive(Debug, Clone)]
pub struct PhysicalCoverageRegistry {
    sequence: HarnessCoverageStage,
    scenario_identity: Option<PhysicalScenarioCanonicalIdentity>,
    plan: Option<PhysicalSimulationPlan>,
    rows: Vec<PhysicalCoverageMatrixRow>,
}

impl PhysicalCoverageRegistry {
    pub fn for_sequence(sequence: HarnessCoverageStage) -> Self {
        Self {
            sequence,
            scenario_identity: None,
            plan: None,
            rows: Vec::new(),
        }
    }

    pub fn generate_matrix(self) -> Result<GeneratedCoverageMatrix, CoverageGapDenial> {
        let matrix = PhysicalCoverageMatrix::generated(self.sequence, self.rows)?;
        Ok(GeneratedCoverageMatrix::from_matrix(matrix))
    }

    fn require_surface_not_registered(
        &self,
        surface: CoverageSurfaceKind,
    ) -> Result<(), CoverageGapDenial> {
        if self.rows.iter().any(|row| row.surface() == surface) {
            Err(CoverageGapDenial::DuplicateRegistrationEvidence { surface })
        } else {
            Ok(())
        }
    }
}
