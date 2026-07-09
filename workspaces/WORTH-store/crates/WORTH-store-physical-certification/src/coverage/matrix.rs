use std::collections::BTreeSet;

use super::{
    CoverageGapDenial, CoverageSurfaceKind, HarnessMaturityEvidence, PhysicalCoverageMatrixRow,
    Roadmap2HarnessSequence,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roadmap2PhysicalCoverageMatrix {
    sequence: Roadmap2HarnessSequence,
    rows: Vec<PhysicalCoverageMatrixRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedCoverageMatrix {
    matrix: Roadmap2PhysicalCoverageMatrix,
}

impl Roadmap2PhysicalCoverageMatrix {
    pub(crate) fn generated(
        sequence: Roadmap2HarnessSequence,
        mut rows: Vec<PhysicalCoverageMatrixRow>,
    ) -> Result<Self, CoverageGapDenial> {
        rows.sort_by_key(|row| (row.subsystem(), row.surface(), *row.source_identity()));
        let matrix = Self { sequence, rows };
        matrix.require_required_s45_surfaces()?;
        Ok(matrix)
    }

    pub const fn sequence(&self) -> Roadmap2HarnessSequence {
        self.sequence
    }

    pub fn rows(&self) -> &[PhysicalCoverageMatrixRow] {
        &self.rows
    }

    fn require_required_s45_surfaces(&self) -> Result<(), CoverageGapDenial> {
        if self.sequence != Roadmap2HarnessSequence::S45 {
            return Ok(());
        }
        let covered = self.rows.iter().map(|row| row.surface()).collect();
        for surface in required_s45_surfaces() {
            require_surface(&covered, surface)?;
        }
        Ok(())
    }
}

impl GeneratedCoverageMatrix {
    pub(crate) const fn from_matrix(matrix: Roadmap2PhysicalCoverageMatrix) -> Self {
        Self { matrix }
    }

    pub const fn sequence(&self) -> Roadmap2HarnessSequence {
        self.matrix.sequence()
    }

    pub const fn matrix(&self) -> &Roadmap2PhysicalCoverageMatrix {
        &self.matrix
    }

    pub fn rows(&self) -> &[PhysicalCoverageMatrixRow] {
        self.matrix.rows()
    }

    pub fn derive_maturity(&self) -> HarnessMaturityEvidence {
        HarnessMaturityEvidence::from_generated_matrix(self)
    }
}

fn require_surface(
    covered: &BTreeSet<CoverageSurfaceKind>,
    surface: CoverageSurfaceKind,
) -> Result<(), CoverageGapDenial> {
    if covered.contains(&surface) {
        Ok(())
    } else {
        Err(CoverageGapDenial::MissingRegistrationEvidence { surface })
    }
}

fn required_s45_surfaces() -> [CoverageSurfaceKind; 9] {
    [
        CoverageSurfaceKind::Scenario,
        CoverageSurfaceKind::Plan,
        CoverageSurfaceKind::YieldpointSchedule,
        CoverageSurfaceKind::Actor,
        CoverageSurfaceKind::Driver,
        CoverageSurfaceKind::Oracle,
        CoverageSurfaceKind::Counter,
        CoverageSurfaceKind::Transcript,
        CoverageSurfaceKind::MutationResult,
    ]
}
