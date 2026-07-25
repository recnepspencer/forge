use worth_store_physical_backend::CompletedArtifactRangeWrite;

use crate::physical_runtime::record_serving::RecordAppendDenial;

pub(in crate::physical_runtime::record_serving) struct CandidateFramePhysicalWrite {
    receipt: Option<CompletedArtifactRangeWrite>,
    settlement: Option<crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement>,
}

impl CandidateFramePhysicalWrite {
    pub(in crate::physical_runtime::record_serving) fn completed(
        receipt: CompletedArtifactRangeWrite,
    ) -> Self {
        Self {
            receipt: Some(receipt),
            settlement: None,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn bind_settlement(
        mut self,
        settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,
    ) -> Self {
        self.settlement = Some(settlement);
        self
    }

    pub(in crate::physical_runtime::record_serving) fn receipt(
        &self,
    ) -> Option<&CompletedArtifactRangeWrite> {
        self.receipt.as_ref()
    }

    pub(in crate::physical_runtime::record_serving) const fn work(
        &self,
    ) -> Option<crate::physical_runtime::PhysicalWorkIdentity> {
        match self.settlement {
            Some(settlement) => Some(settlement.identity()),
            None => None,
        }
    }

    pub(in crate::physical_runtime::record_serving) const fn settlement(
        &self,
    ) -> Option<crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement> {
        self.settlement
    }

    #[cfg(test)]
    pub(super) fn for_contract_test() -> Self {
        Self {
            receipt: None,
            settlement: None,
        }
    }
}

#[derive(Debug)]
pub(in crate::physical_runtime::record_serving) struct CandidateFrameWriteCompletion {
    frame_bytes: u64,
    reusable_bytes: Option<Vec<u8>>,
}

impl CandidateFrameWriteCompletion {
    pub(in crate::physical_runtime::record_serving) fn retained(frame_bytes: u64) -> Self {
        Self {
            frame_bytes,
            reusable_bytes: None,
        }
    }

    pub(in crate::physical_runtime::record_serving) const fn frame_bytes(&self) -> u64 {
        self.frame_bytes
    }

    pub(in crate::physical_runtime::record_serving) fn into_reusable_bytes(
        self,
    ) -> Option<Vec<u8>> {
        self.reusable_bytes
    }

    #[cfg(test)]
    pub(super) fn for_contract_test(frame_bytes: u64) -> Self {
        Self {
            frame_bytes,
            reusable_bytes: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateFrameContractViolation {
    FrameCountExceedsDeclaration,
    FrameBytesExceedDeclaration,
    RetainedFrameMismatch,
    FrameCompletionMismatch,
    CoordinateRoleMismatch,
    UnexpectedFrame,
    RetainedFrameBytesChanged,
    CatalogResidencyInvalidationFailed,
    IncompleteFrameSet,
}

#[derive(Debug)]
pub(in crate::physical_runtime::record_serving) enum CandidateFrameWriteFailure<EffectFailure> {
    Contract(CandidateFrameContractViolation),
    Effect(EffectFailure),
    Residency(RecordAppendDenial),
}
