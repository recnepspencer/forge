use worth_store_physical_backend::{ArtifactTreeFailure, CompletedArtifactRangeWrite};

use crate::physical_runtime::record_serving::RecordAppendDenial;

pub(in crate::physical_runtime::record_serving) struct CandidateFramePhysicalWrite {
    receipt: Option<CompletedArtifactRangeWrite>,
}

impl CandidateFramePhysicalWrite {
    pub(in crate::physical_runtime::record_serving) fn completed(
        receipt: CompletedArtifactRangeWrite,
    ) -> Self {
        Self {
            receipt: Some(receipt),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn receipt(
        &self,
    ) -> Option<&CompletedArtifactRangeWrite> {
        self.receipt.as_ref()
    }

    #[cfg(test)]
    pub(super) fn for_contract_test() -> Self {
        Self { receipt: None }
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
pub(in crate::physical_runtime::record_serving) enum CandidateFrameWriteFailure {
    Contract(CandidateFrameContractViolation),
    Backend(ArtifactTreeFailure),
    Residency(RecordAppendDenial),
}
