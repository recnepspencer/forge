use std::marker::PhantomData;

use worth_store_physical_backend::{
    BackendDurabilityProfile, BackendDurabilityProfileId, WalDurabilityBarrier,
    WalDurabilityBarrierSet,
};

use crate::{WalLsnRange, WalSegmentGeneration, WalSegmentId};

use super::IllegalAcknowledgmentDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalFrameDigest {
    value: String,
}

impl WalFrameDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, IllegalAcknowledgmentDenial> {
        let value = value.into();
        if value.is_empty() {
            return Err(IllegalAcknowledgmentDenial::new(
                super::IllegalAcknowledgmentDenialKind::EmptyFrameDigest,
            ));
        }
        Ok(Self { value })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WalDurabilityFailure {
    BarrierFailed(WalDurabilityBarrier),
    DelayedFlush(WalDurabilityBarrier),
    LostFlush,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalAppendReceipt<P: BackendDurabilityProfile> {
    profile: PhantomData<P>,
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
    frame_digest: WalFrameDigest,
    expected_bytes: u64,
    observed_bytes: u64,
    completed_barriers: WalDurabilityBarrierSet,
    failure: Option<WalDurabilityFailure>,
}

impl<P: BackendDurabilityProfile> WalAppendReceipt<P> {
    pub(crate) fn new(
        segment_id: WalSegmentId,
        generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
        frame_digest: WalFrameDigest,
        expected_bytes: u64,
        observed_bytes: u64,
        completed_barriers: WalDurabilityBarrierSet,
        failure: Option<WalDurabilityFailure>,
    ) -> Self {
        Self {
            profile: PhantomData,
            segment_id,
            generation,
            lsn_range,
            frame_digest,
            expected_bytes,
            observed_bytes,
            completed_barriers,
            failure,
        }
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        P::ID
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

    pub const fn expected_bytes(&self) -> u64 {
        self.expected_bytes
    }

    pub const fn observed_bytes(&self) -> u64 {
        self.observed_bytes
    }

    pub fn frame_digest(&self) -> &WalFrameDigest {
        &self.frame_digest
    }

    pub const fn required_barriers(&self) -> WalDurabilityBarrierSet {
        P::REQUIRED_BARRIERS
    }

    pub const fn completed_barriers(&self) -> WalDurabilityBarrierSet {
        self.completed_barriers
    }

    pub(crate) const fn failure(&self) -> Option<WalDurabilityFailure> {
        self.failure
    }
}
