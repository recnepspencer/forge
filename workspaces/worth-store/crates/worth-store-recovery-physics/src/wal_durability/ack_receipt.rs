use std::marker::PhantomData;

use worth_store_physical_backend::{
    BackendDurabilityProfile, BackendDurabilityProfileId, WalDurabilityBarrierSet,
};

use crate::{WalLsnRange, WalSegmentGeneration, WalSegmentId};

use super::{AcknowledgmentPrecondition, WalFrameDigest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableAckBasis {
    profile_id: BackendDurabilityProfileId,
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
    frame_digest: WalFrameDigest,
    required_barriers: WalDurabilityBarrierSet,
    completed_barriers: WalDurabilityBarrierSet,
}

impl DurableAckBasis {
    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.profile_id
    }

    pub const fn segment_id(&self) -> WalSegmentId {
        self.segment_id
    }

    pub const fn generation(&self) -> WalSegmentGeneration {
        self.generation
    }

    pub const fn lsn_range(&self) -> WalLsnRange {
        self.lsn_range
    }

    pub fn frame_digest(&self) -> &WalFrameDigest {
        &self.frame_digest
    }

    pub const fn required_barriers(&self) -> WalDurabilityBarrierSet {
        self.required_barriers
    }

    pub const fn completed_barriers(&self) -> WalDurabilityBarrierSet {
        self.completed_barriers
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableAckReceipt<P: BackendDurabilityProfile> {
    profile: PhantomData<P>,
    basis: DurableAckBasis,
}

impl<P: BackendDurabilityProfile> DurableAckReceipt<P> {
    pub fn acknowledge(precondition: AcknowledgmentPrecondition<P>) -> Self {
        let receipt = precondition.into_receipt();
        Self {
            profile: PhantomData,
            basis: DurableAckBasis {
                profile_id: P::ID,
                segment_id: receipt.segment_id(),
                generation: receipt.generation(),
                lsn_range: receipt.lsn_range(),
                frame_digest: receipt.frame_digest().clone(),
                required_barriers: receipt.required_barriers(),
                completed_barriers: receipt.completed_barriers(),
            },
        }
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.basis.profile_id
    }

    pub const fn ack_basis(&self) -> &DurableAckBasis {
        &self.basis
    }
}
