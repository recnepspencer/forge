use forge_store_physical_backend::{
    BackendDurabilityProfile, BackendDurabilityProfileId, WalDurabilityBarrierSet,
};

use crate::{
    DurableAckBasis, LogSequenceNumber, WalAppendReceipt, WalFrameDigest, WalLsnRange,
    WalSegmentGeneration, WalSegmentId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnacknowledgedDurableWal {
    profile_id: BackendDurabilityProfileId,
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
    frame_digest: WalFrameDigest,
    required_barriers: WalDurabilityBarrierSet,
    completed_barriers: WalDurabilityBarrierSet,
}

impl UnacknowledgedDurableWal {
    pub fn from_append_receipt<P: BackendDurabilityProfile>(receipt: WalAppendReceipt<P>) -> Self {
        Self {
            profile_id: receipt.profile_id(),
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
pub enum PartialPublicationCrashEdge {
    BeforeWalAppend {
        operation_digest: String,
    },
    AfterWalAppendBeforeDurability {
        wal_range: WalLsnRange,
        operation_digest: String,
    },
    AfterDurabilityBeforeAck {
        durable_wal: UnacknowledgedDurableWal,
    },
    AfterAckBeforePageFlush {
        ack_basis: DurableAckBasis,
    },
    DuringCheckpointCutover {
        checkpoint_digest: String,
    },
}

impl PartialPublicationCrashEdge {
    pub fn before_wal_append(operation_digest: impl Into<String>) -> Self {
        Self::BeforeWalAppend {
            operation_digest: operation_digest.into(),
        }
    }

    pub fn after_wal_append_before_durability(
        wal_range: WalLsnRange,
        operation_digest: impl Into<String>,
    ) -> Self {
        Self::AfterWalAppendBeforeDurability {
            wal_range,
            operation_digest: operation_digest.into(),
        }
    }

    pub fn after_durability_before_ack<P: BackendDurabilityProfile>(
        receipt: WalAppendReceipt<P>,
    ) -> Self {
        Self::AfterDurabilityBeforeAck {
            durable_wal: UnacknowledgedDurableWal::from_append_receipt(receipt),
        }
    }

    pub fn after_ack_before_page_flush(ack_basis: DurableAckBasis) -> Self {
        Self::AfterAckBeforePageFlush { ack_basis }
    }

    pub fn during_checkpoint_cutover(checkpoint_digest: impl Into<String>) -> Self {
        Self::DuringCheckpointCutover {
            checkpoint_digest: checkpoint_digest.into(),
        }
    }

    pub const fn first_lsn(&self) -> Option<LogSequenceNumber> {
        match self {
            Self::BeforeWalAppend { .. } | Self::DuringCheckpointCutover { .. } => None,
            Self::AfterWalAppendBeforeDurability { wal_range, .. } => Some(wal_range.start()),
            Self::AfterDurabilityBeforeAck { durable_wal } => Some(durable_wal.lsn_range().start()),
            Self::AfterAckBeforePageFlush { ack_basis } => Some(ack_basis.lsn_range().start()),
        }
    }
}
