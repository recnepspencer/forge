use std::marker::PhantomData;

use worth_store_physical_backend::{
    BackendDurabilityProfile, WalDurabilityBarrier, WalDurabilityBarrierReceipt,
    WalDurabilityBarrierSet,
};

use crate::{WalLsnRange, WalSegmentGeneration, WalSegmentId};

use super::{
    IllegalAcknowledgmentDenial, IllegalAcknowledgmentDenialKind, WalAppendReceipt,
    WalDurabilityFailure, WalFrameDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalAppendPlan<P: BackendDurabilityProfile> {
    profile: PhantomData<P>,
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
    frame_digest: WalFrameDigest,
    expected_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalAppendDurabilityScope {
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
    frame_digest: WalFrameDigest,
    expected_bytes: u64,
}

impl WalAppendDurabilityScope {
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

    pub const fn expected_bytes(&self) -> u64 {
        self.expected_bytes
    }
}

impl<P: BackendDurabilityProfile> WalAppendPlan<P> {
    pub fn new(
        segment_id: WalSegmentId,
        generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
        frame_digest: impl Into<String>,
        expected_bytes: u64,
    ) -> Result<Self, IllegalAcknowledgmentDenial> {
        if expected_bytes == 0 {
            return Err(IllegalAcknowledgmentDenial::new(
                IllegalAcknowledgmentDenialKind::EmptyFrameWrite,
            ));
        }
        Ok(Self {
            profile: PhantomData,
            segment_id,
            generation,
            lsn_range,
            frame_digest: WalFrameDigest::new(frame_digest)?,
            expected_bytes,
        })
    }

    pub fn record_written_bytes(self, observed_bytes: u64) -> WalAppendProgress<P> {
        WalAppendProgress {
            plan: self,
            observed_bytes,
            completed_barriers: WalDurabilityBarrierSet::EMPTY,
            failure: None,
        }
    }

    fn durability_scope(&self) -> WalAppendDurabilityScope {
        WalAppendDurabilityScope {
            segment_id: self.segment_id,
            generation: self.generation,
            lsn_range: self.lsn_range,
            frame_digest: self.frame_digest.clone(),
            expected_bytes: self.expected_bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalAppendProgress<P: BackendDurabilityProfile> {
    plan: WalAppendPlan<P>,
    observed_bytes: u64,
    completed_barriers: WalDurabilityBarrierSet,
    failure: Option<WalDurabilityFailure>,
}

impl<P: BackendDurabilityProfile> WalAppendProgress<P> {
    pub fn durability_scope(&self) -> WalAppendDurabilityScope {
        self.plan.durability_scope()
    }

    pub fn complete_barrier(
        mut self,
        receipt: WalDurabilityBarrierReceipt<P, WalAppendDurabilityScope>,
    ) -> Result<Self, IllegalAcknowledgmentDenial> {
        if receipt.scope() != &self.plan.durability_scope() {
            return Err(IllegalAcknowledgmentDenial::barrier_receipt_scope_mismatch(
                P::ID,
                self.plan.segment_id,
                self.plan.generation,
                self.plan.lsn_range,
                receipt.barrier(),
            ));
        }
        self.completed_barriers = self.completed_barriers.insert(receipt.barrier());
        Ok(self)
    }

    pub fn fail_barrier(mut self, barrier: WalDurabilityBarrier) -> Self {
        self.failure = Some(WalDurabilityFailure::BarrierFailed(barrier));
        self
    }

    pub fn delay_flush(mut self, barrier: WalDurabilityBarrier) -> Self {
        self.failure = Some(WalDurabilityFailure::DelayedFlush(barrier));
        self
    }

    pub fn lose_flush(mut self) -> Self {
        self.failure = Some(WalDurabilityFailure::LostFlush);
        self
    }

    pub fn finish(self) -> Result<WalAppendReceipt<P>, IllegalAcknowledgmentDenial> {
        if self.observed_bytes == 0 {
            return Err(IllegalAcknowledgmentDenial::append_not_completed());
        }
        Ok(WalAppendReceipt::new(
            self.plan.segment_id,
            self.plan.generation,
            self.plan.lsn_range,
            self.plan.frame_digest,
            self.plan.expected_bytes,
            self.observed_bytes,
            self.completed_barriers,
            self.failure,
        ))
    }
}
