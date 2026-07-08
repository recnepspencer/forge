use crate::materialization::{S8LayoutCoverageWitness, S8MaterializationDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8DegradedExactScan {
    coverage: S8LayoutCoverageWitness,
}

impl S8DegradedExactScan {
    pub(crate) fn new(coverage: S8LayoutCoverageWitness) -> Result<Self, S8MaterializationDenial> {
        Ok(Self {
            coverage: coverage.require_exact()?,
        })
    }

    pub const fn coverage(self) -> S8LayoutCoverageWitness {
        self.coverage
    }
}
