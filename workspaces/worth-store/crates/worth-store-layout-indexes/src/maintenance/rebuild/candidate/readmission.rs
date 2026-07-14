use super::super::{DerivedIndexParityBasis, DerivedIndexRebuildReceipt};

use super::DerivedIndexCandidateDeclaration;

/// Candidate data bound to an actual owner-issued rebuild execution.
///
/// This receipt is intentionally weaker than parity. It proves only that the
/// candidate was read for the execution whose authoritative source remains in
/// the enclosed rebuild receipt.
#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexCandidateReadmissionReceipt {
    execution: DerivedIndexRebuildReceipt,
    candidate: DerivedIndexParityBasis,
}

impl DerivedIndexCandidateReadmissionReceipt {
    pub const fn execution(&self) -> &DerivedIndexRebuildReceipt {
        &self.execution
    }

    pub const fn candidate(&self) -> &DerivedIndexParityBasis {
        &self.candidate
    }

    pub(in crate::maintenance::rebuild) fn into_parts(
        self,
    ) -> (DerivedIndexRebuildReceipt, DerivedIndexParityBasis) {
        (self.execution, self.candidate)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutRebuildCandidateReadmission;

pub const fn layout_rebuild_candidate_readmission() -> LayoutRebuildCandidateReadmission {
    LayoutRebuildCandidateReadmission
}

impl LayoutRebuildCandidateReadmission {
    pub fn readmit(
        self,
        execution: DerivedIndexRebuildReceipt,
        declaration: DerivedIndexCandidateDeclaration,
    ) -> DerivedIndexCandidateReadmissionReceipt {
        DerivedIndexCandidateReadmissionReceipt {
            execution,
            candidate: declaration.basis,
        }
    }
}
