use worth_store_physical_backend::CompletedArtifactRangeWrite;

use crate::physical_runtime::record_serving::RecordAppendDenial;

pub(in crate::physical_runtime::record_serving) struct CandidateFramePhysicalWrite {
    receipt: CompletedArtifactRangeWrite,
    settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,
}

impl CandidateFramePhysicalWrite {
    pub(in crate::physical_runtime::record_serving) fn completed(
        receipt: CompletedArtifactRangeWrite,
        settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,
    ) -> Self {
        Self {
            receipt,
            settlement,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn receipt(
        &self,
    ) -> &CompletedArtifactRangeWrite {
        &self.receipt
    }

    pub(in crate::physical_runtime::record_serving) const fn work(
        &self,
    ) -> crate::physical_runtime::PhysicalWorkIdentity {
        self.settlement.identity()
    }

    pub(in crate::physical_runtime::record_serving) const fn settlement(
        &self,
    ) -> crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement {
        self.settlement
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
