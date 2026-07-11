use super::coverage::S8LayoutCoverageWitness;
use super::denial::S8MaterializationDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AbsenceAuthorityClass {
    ExactIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PhysicalAbsenceProof {
    coverage: S8LayoutCoverageWitness,
    authority_class: S8AbsenceAuthorityClass,
}

impl S8PhysicalAbsenceProof {
    pub(crate) fn exact_index(
        coverage: S8LayoutCoverageWitness,
    ) -> Result<Self, S8MaterializationDenial> {
        Ok(Self {
            coverage: coverage.require_exact()?,
            authority_class: S8AbsenceAuthorityClass::ExactIndex,
        })
    }

    pub const fn coverage(self) -> S8LayoutCoverageWitness {
        self.coverage
    }

    pub const fn authority_class(self) -> S8AbsenceAuthorityClass {
        self.authority_class
    }
}
