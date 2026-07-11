mod counter;
mod mutation;
mod oracle;
mod scenario_plan_identity;
mod schedule_actor_driver;
mod transcript;

use super::{
    CoverageGapDenial, CoverageSurfaceKind, GeneratedCoverageMatrix, PhysicalCoverageMatrixRow,
    Roadmap2HarnessSequence, Roadmap2PhysicalCoverageMatrix,
};
use crate::{PhysicalScenarioCanonicalIdentity, PhysicalSimulationPlan};

#[derive(Debug, Clone)]
pub struct Roadmap2CoverageRegistry {
    sequence: Roadmap2HarnessSequence,
    scenario_identity: Option<PhysicalScenarioCanonicalIdentity>,
    plan: Option<PhysicalSimulationPlan>,
    rows: Vec<PhysicalCoverageMatrixRow>,
}

impl Roadmap2CoverageRegistry {
    pub fn for_sequence(sequence: Roadmap2HarnessSequence) -> Self {
        Self {
            sequence,
            scenario_identity: None,
            plan: None,
            rows: Vec::new(),
        }
    }

    pub fn generate_matrix(self) -> Result<GeneratedCoverageMatrix, CoverageGapDenial> {
        let matrix = Roadmap2PhysicalCoverageMatrix::generated(self.sequence, self.rows)?;
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
