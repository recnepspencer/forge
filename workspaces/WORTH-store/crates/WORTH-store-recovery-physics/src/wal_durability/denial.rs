use worth_store_physical_backend::{
    BackendDurabilityProfileId, WalDurabilityBarrier, WalDurabilityBarrierSet,
};

use crate::{WalLsnRange, WalSegmentGeneration, WalSegmentId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IllegalAcknowledgmentDenialKind {
    EmptyFrameDigest,
    EmptyFrameWrite,
    AppendNotCompleted,
    ShortWrite,
    RequiredBarrierMissing,
    BarrierFailed,
    BarrierReceiptScopeMismatch,
    DirectorySyncFailure,
    DelayedFlush,
    LostFlush,
    UnsupportedDurabilityCapability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IllegalAcknowledgmentDenial {
    kind: IllegalAcknowledgmentDenialKind,
    profile_id: Option<BackendDurabilityProfileId>,
    segment_id: Option<WalSegmentId>,
    generation: Option<WalSegmentGeneration>,
    lsn_range: Option<WalLsnRange>,
    expected_bytes: Option<u64>,
    observed_bytes: Option<u64>,
    required_barriers: Option<WalDurabilityBarrierSet>,
    completed_barriers: Option<WalDurabilityBarrierSet>,
    barrier: Option<WalDurabilityBarrier>,
}

impl IllegalAcknowledgmentDenial {
    pub(crate) const fn new(kind: IllegalAcknowledgmentDenialKind) -> Self {
        Self {
            kind,
            profile_id: None,
            segment_id: None,
            generation: None,
            lsn_range: None,
            expected_bytes: None,
            observed_bytes: None,
            required_barriers: None,
            completed_barriers: None,
            barrier: None,
        }
    }

    pub(crate) const fn append_not_completed() -> Self {
        Self::new(IllegalAcknowledgmentDenialKind::AppendNotCompleted)
    }

    pub(crate) const fn short_write(
        profile_id: BackendDurabilityProfileId,
        segment_id: WalSegmentId,
        generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
        expected_bytes: u64,
        observed_bytes: u64,
    ) -> Self {
        Self {
            kind: IllegalAcknowledgmentDenialKind::ShortWrite,
            profile_id: Some(profile_id),
            segment_id: Some(segment_id),
            generation: Some(generation),
            lsn_range: Some(lsn_range),
            expected_bytes: Some(expected_bytes),
            observed_bytes: Some(observed_bytes),
            required_barriers: None,
            completed_barriers: None,
            barrier: None,
        }
    }

    pub(crate) const fn unsupported_profile(profile_id: BackendDurabilityProfileId) -> Self {
        Self {
            profile_id: Some(profile_id),
            ..Self::new(IllegalAcknowledgmentDenialKind::UnsupportedDurabilityCapability)
        }
    }

    pub(crate) const fn lost_flush(profile_id: BackendDurabilityProfileId) -> Self {
        Self {
            profile_id: Some(profile_id),
            ..Self::new(IllegalAcknowledgmentDenialKind::LostFlush)
        }
    }

    pub(crate) const fn delayed_flush(
        profile_id: BackendDurabilityProfileId,
        barrier: WalDurabilityBarrier,
    ) -> Self {
        Self {
            profile_id: Some(profile_id),
            barrier: Some(barrier),
            ..Self::new(IllegalAcknowledgmentDenialKind::DelayedFlush)
        }
    }

    pub(crate) const fn barrier_failed(
        profile_id: BackendDurabilityProfileId,
        barrier: WalDurabilityBarrier,
    ) -> Self {
        let kind = match barrier {
            WalDurabilityBarrier::WalDirectoryFsync
            | WalDurabilityBarrier::WindowsDirectorySync => {
                IllegalAcknowledgmentDenialKind::DirectorySyncFailure
            }
            _ => IllegalAcknowledgmentDenialKind::BarrierFailed,
        };
        Self {
            profile_id: Some(profile_id),
            barrier: Some(barrier),
            ..Self::new(kind)
        }
    }

    pub(crate) const fn missing_barrier(
        profile_id: BackendDurabilityProfileId,
        required_barriers: WalDurabilityBarrierSet,
        completed_barriers: WalDurabilityBarrierSet,
        barrier: WalDurabilityBarrier,
    ) -> Self {
        Self {
            kind: IllegalAcknowledgmentDenialKind::RequiredBarrierMissing,
            profile_id: Some(profile_id),
            segment_id: None,
            generation: None,
            lsn_range: None,
            expected_bytes: None,
            observed_bytes: None,
            required_barriers: Some(required_barriers),
            completed_barriers: Some(completed_barriers),
            barrier: Some(barrier),
        }
    }

    pub(crate) const fn barrier_receipt_scope_mismatch(
        profile_id: BackendDurabilityProfileId,
        segment_id: WalSegmentId,
        generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
        barrier: WalDurabilityBarrier,
    ) -> Self {
        Self {
            kind: IllegalAcknowledgmentDenialKind::BarrierReceiptScopeMismatch,
            profile_id: Some(profile_id),
            segment_id: Some(segment_id),
            generation: Some(generation),
            lsn_range: Some(lsn_range),
            expected_bytes: None,
            observed_bytes: None,
            required_barriers: None,
            completed_barriers: None,
            barrier: Some(barrier),
        }
    }

    pub const fn kind(&self) -> IllegalAcknowledgmentDenialKind {
        self.kind
    }

    pub const fn profile_id(&self) -> Option<BackendDurabilityProfileId> {
        self.profile_id
    }

    pub const fn segment_id(&self) -> Option<WalSegmentId> {
        self.segment_id
    }

    pub const fn generation(&self) -> Option<WalSegmentGeneration> {
        self.generation
    }

    pub const fn lsn_range(&self) -> Option<WalLsnRange> {
        self.lsn_range
    }

    pub const fn expected_bytes(&self) -> Option<u64> {
        self.expected_bytes
    }

    pub const fn observed_bytes(&self) -> Option<u64> {
        self.observed_bytes
    }

    pub const fn required_barriers(&self) -> Option<WalDurabilityBarrierSet> {
        self.required_barriers
    }

    pub const fn completed_barriers(&self) -> Option<WalDurabilityBarrierSet> {
        self.completed_barriers
    }

    pub const fn barrier(&self) -> Option<WalDurabilityBarrier> {
        self.barrier
    }
}
