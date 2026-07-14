use std::marker::PhantomData;

use worth_store_physical_backend::{
    BackendDurabilityProfile, BackendDurabilityProfileId, WalDurabilityBarrierSet,
};

use crate::{WalLsnRange, WalSegmentGeneration, WalSegmentId};

use super::{ReopenedWalDurabilityCrashRecord, WalAppendReceipt, WalFrameDigest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalDurabilityCrashBasis {
    profile_id: BackendDurabilityProfileId,
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
    frame_digest: WalFrameDigest,
    required_barriers: WalDurabilityBarrierSet,
    completed_barriers: WalDurabilityBarrierSet,
}

impl WalDurabilityCrashBasis {
    pub(crate) fn from_append_receipt<P: BackendDurabilityProfile>(
        receipt: WalAppendReceipt<P>,
    ) -> Self {
        Self {
            profile_id: P::ID,
            segment_id: receipt.segment_id(),
            generation: receipt.generation(),
            lsn_range: receipt.lsn_range(),
            frame_digest: receipt.frame_digest().clone(),
            required_barriers: receipt.required_barriers(),
            completed_barriers: receipt.completed_barriers(),
        }
    }

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
pub enum WalDurabilityCrashPosture<P: BackendDurabilityProfile> {
    UnacknowledgedCompleted {
        profile: PhantomData<P>,
        basis: WalDurabilityCrashBasis,
    },
}

impl<P: BackendDurabilityProfile> WalDurabilityCrashPosture<P> {
    pub fn from_reopened_durability_record(record: ReopenedWalDurabilityCrashRecord<P>) -> Self {
        Self::UnacknowledgedCompleted {
            profile: PhantomData,
            basis: record.into_basis(),
        }
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.crash_basis().profile_id()
    }

    pub const fn crash_basis(&self) -> &WalDurabilityCrashBasis {
        match self {
            Self::UnacknowledgedCompleted { basis, .. } => basis,
        }
    }

    pub const fn is_replayable_after_crash(&self) -> bool {
        matches!(self, Self::UnacknowledgedCompleted { .. })
    }

    pub const fn is_acknowledged(&self) -> bool {
        false
    }
}
