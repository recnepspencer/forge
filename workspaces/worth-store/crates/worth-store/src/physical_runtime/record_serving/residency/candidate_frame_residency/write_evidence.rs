use sha2::{Digest, Sha256};
use worth_store_physical_backend::CompletedArtifactRangeWrite;
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use crate::physical_runtime::record_serving::RecordAppendDenial;
use crate::physical_runtime::{
    PhysicalEffectIdentity, PhysicalWorkEffectFate, PhysicalWorkIdentity,
    PhysicalWorkRecoveryDisposition,
};

pub(in crate::physical_runtime::record_serving) struct CandidateFramePhysicalWrite {
    receipt: CompletedArtifactRangeWrite,
    settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,
}

pub(in crate::physical_runtime::record_serving) struct CandidateFrameResidencySettlement {
    settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum CandidateFrameEffectSource {
    NewArtifact,
    C6Writeback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct CandidateFrameEffectSettlement {
    source: CandidateFrameEffectSource,
    coordinate: RecordFrameCoordinate,
    payload_digest: [u8; 32],
    work: PhysicalWorkIdentity,
    effect: Option<PhysicalEffectIdentity>,
    fate: PhysicalWorkEffectFate,
    recovery: PhysicalWorkRecoveryDisposition,
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

impl CandidateFrameEffectSettlement {
    fn canonical(
        coordinate: RecordFrameCoordinate,
        payload_digest: [u8; 32],
        settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,
    ) -> Self {
        Self {
            source: CandidateFrameEffectSource::NewArtifact,
            coordinate,
            payload_digest,
            work: settlement.identity(),
            effect: settlement.effect(),
            fate: settlement.effect_fate(),
            recovery: settlement.recovery(),
        }
    }

    fn writeback(
        coordinate: RecordFrameCoordinate,
        payload_digest: [u8; 32],
        settlement: crate::physical_runtime::record_serving::residency::dirty::
            PhysicalWritebackSettlement,
    ) -> Self {
        Self {
            source: CandidateFrameEffectSource::C6Writeback,
            coordinate,
            payload_digest,
            work: settlement.identity(),
            effect: settlement.effect(),
            fate: settlement.effect_fate(),
            recovery: settlement.recovery(),
        }
    }

    pub(in crate::physical_runtime) const fn source(self) -> CandidateFrameEffectSource {
        self.source
    }

    pub(in crate::physical_runtime) const fn coordinate(self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub(in crate::physical_runtime) const fn payload_digest(self) -> [u8; 32] {
        self.payload_digest
    }

    pub(in crate::physical_runtime) const fn work(self) -> PhysicalWorkIdentity {
        self.work
    }

    pub(in crate::physical_runtime) const fn effect(self) -> Option<PhysicalEffectIdentity> {
        self.effect
    }

    pub(in crate::physical_runtime) const fn fate(self) -> PhysicalWorkEffectFate {
        self.fate
    }

    pub(in crate::physical_runtime) const fn recovery(self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }
}

#[derive(Debug)]
pub(in crate::physical_runtime::record_serving) struct CandidateFrameWriteCompletion {
    frame_bytes: u64,
    reusable_bytes: Option<Vec<u8>>,
    effect: Option<CandidateFrameEffectSettlement>,
}

impl CandidateFrameWriteCompletion {
    pub(in crate::physical_runtime::record_serving) fn canonical(
        frame_bytes: u64,
        coordinate: RecordFrameCoordinate,
        payload_digest: [u8; 32],
        settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,
    ) -> Self {
        Self {
            frame_bytes,
            reusable_bytes: None,
            effect: Some(CandidateFrameEffectSettlement::canonical(
                coordinate,
                payload_digest,
                settlement,
            )),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn writeback(
        frame_bytes: u64,
        coordinate: RecordFrameCoordinate,
        payload_digest: [u8; 32],
        settlement: crate::physical_runtime::record_serving::residency::dirty::
            PhysicalWritebackSettlement,
    ) -> Self {
        Self {
            frame_bytes,
            reusable_bytes: None,
            effect: Some(CandidateFrameEffectSettlement::writeback(
                coordinate,
                payload_digest,
                settlement,
            )),
        }
    }

    #[cfg(test)]
    pub(in crate::physical_runtime::record_serving) fn retained(frame_bytes: u64) -> Self {
        Self {
            frame_bytes,
            reusable_bytes: None,
            effect: None,
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

    pub(in crate::physical_runtime) const fn effect(
        &self,
    ) -> Option<CandidateFrameEffectSettlement> {
        self.effect
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
