use sha2::{Digest, Sha256};
use worth_store_physical_backend::CompletedArtifactRangeWrite;
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use crate::physical_runtime::record_serving::RecordAppendDenial;

pub(in crate::physical_runtime::record_serving) struct CandidateFramePhysicalWrite {
    receipt: CompletedArtifactRangeWrite,
    settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,
}

pub(in crate::physical_runtime::record_serving) struct CandidateFrameResidencySettlement {
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

    pub(in crate::physical_runtime::record_serving) const fn settlement(
        &self,
    ) -> crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement {
        self.settlement
    }

    pub(in crate::physical_runtime::record_serving) fn settle_residency(
        self,
        store: StableStoreIdentity,
        coordinate: super::CandidateFrameCoordinate,
        bytes: &[u8],
    ) -> Result<CandidateFrameResidencySettlement, CandidateFrameContractViolation> {
        let length = u32::try_from(bytes.len())
            .map_err(|_| CandidateFrameContractViolation::PhysicalWriteMismatch)?;
        let coordinate =
            RecordFrameCoordinate::new(coordinate.artifact(), coordinate.offset(), length)
                .ok_or(CandidateFrameContractViolation::PhysicalWriteMismatch)?;
        if !completed_write_matches(&self.receipt, store, coordinate, bytes) {
            return Err(CandidateFrameContractViolation::PhysicalWriteMismatch);
        }
        Ok(CandidateFrameResidencySettlement {
            settlement: self.settlement,
        })
    }
}

pub(super) fn completed_write_matches(
    receipt: &CompletedArtifactRangeWrite,
    store: StableStoreIdentity,
    coordinate: RecordFrameCoordinate,
    bytes: &[u8],
) -> bool {
    let digest: [u8; 32] = Sha256::digest(bytes).into();
    receipt.store() == store
        && receipt.coordinate() == coordinate
        && receipt.completed_bytes() == bytes.len() as u64
        && receipt.payload_digest() == digest
}

impl CandidateFrameResidencySettlement {
    pub(in crate::physical_runtime::record_serving) const fn settlement(
        self,
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
    PhysicalWriteMismatch,
    CatalogResidencyInvalidationFailed,
    IncompleteFrameSet,
}

#[derive(Debug)]
pub(in crate::physical_runtime::record_serving) enum CandidateFrameWriteFailure<EffectFailure> {
    Contract(CandidateFrameContractViolation),
    Effect(EffectFailure),
    Residency(RecordAppendDenial),
}
