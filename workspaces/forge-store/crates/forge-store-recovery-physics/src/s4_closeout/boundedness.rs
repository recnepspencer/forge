use crate::{
    BoundedRecoveryReceipt, CheckpointIntervalContract, RecoveryCounterSnapshot,
    WalTailReplayBudget,
};

use super::RecoveryPhysicsCloseoutDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryBoundednessEvidence {
    checkpoint_interval_frames: usize,
    wal_tail_frame_limit: usize,
    observed_wal_tail_frames: usize,
    max_scanned_segments: usize,
    max_page_redos: usize,
    counters: RecoveryCounterSnapshot,
}

impl RecoveryBoundednessEvidence {
    pub fn from_recovery_receipt(
        receipt: &BoundedRecoveryReceipt,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        Self::from_receipt_bounds(receipt)
    }

    pub fn from_admitted_budget(
        checkpoint_interval: CheckpointIntervalContract,
        wal_tail_budget: WalTailReplayBudget,
        receipt: &BoundedRecoveryReceipt,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        let bounds = receipt.work_bounds();
        if bounds.checkpoint_interval_frames() != checkpoint_interval.max_tail_frame_count()
            || bounds.wal_tail_frame_limit() != wal_tail_budget.max_frame_count()
            || bounds.max_scanned_segments() != wal_tail_budget.max_scanned_segments()
            || bounds.max_page_redos() != wal_tail_budget.max_page_redos()
        {
            return Err(RecoveryPhysicsCloseoutDenial::BoundednessAuthorityMismatch);
        }
        Self::from_receipt_bounds(receipt)
    }

    fn from_receipt_bounds(
        receipt: &BoundedRecoveryReceipt,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        let counters = receipt.counters();
        let bounds = receipt.work_bounds();
        if counters.forbidden_full_store_scans() != 0 {
            return Err(RecoveryPhysicsCloseoutDenial::UnboundedRecoveryPlan);
        }
        if counters.validated_checkpoints() == 0 {
            return Err(RecoveryPhysicsCloseoutDenial::MissingBoundedRecoveryCounters);
        }
        let observed_wal_tail_frames = counters.replayed_frames() + counters.skipped_frames();
        if observed_wal_tail_frames == 0 {
            return Err(RecoveryPhysicsCloseoutDenial::MissingBoundedRecoveryCounters);
        }
        let checkpoint_interval_frames = bounds.checkpoint_interval_frames();
        let wal_tail_frame_limit = bounds.wal_tail_frame_limit();
        if observed_wal_tail_frames > checkpoint_interval_frames
            || observed_wal_tail_frames > wal_tail_frame_limit
            || counters.scanned_segments() > bounds.max_scanned_segments()
            || counters.page_redos() > bounds.max_page_redos()
        {
            return Err(RecoveryPhysicsCloseoutDenial::UnboundedRecoveryPlan);
        }
        Ok(Self {
            checkpoint_interval_frames,
            wal_tail_frame_limit,
            observed_wal_tail_frames,
            max_scanned_segments: bounds.max_scanned_segments(),
            max_page_redos: bounds.max_page_redos(),
            counters,
        })
    }

    pub const fn work_bound(self) -> RecoveryWorkBound {
        RecoveryWorkBound::CheckpointIntervalAndWalTail {
            checkpoint_interval_frames: self.checkpoint_interval_frames,
            wal_tail_frame_limit: self.wal_tail_frame_limit,
            observed_wal_tail_frames: self.observed_wal_tail_frames,
        }
    }

    pub const fn counters(self) -> RecoveryCounterSnapshot {
        self.counters
    }

    pub const fn checkpoint_interval_frames(self) -> usize {
        self.checkpoint_interval_frames
    }

    pub const fn wal_tail_frame_limit(self) -> usize {
        self.wal_tail_frame_limit
    }

    pub const fn max_scanned_segments(self) -> usize {
        self.max_scanned_segments
    }

    pub const fn max_page_redos(self) -> usize {
        self.max_page_redos
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryWorkBound {
    CheckpointIntervalAndWalTail {
        checkpoint_interval_frames: usize,
        wal_tail_frame_limit: usize,
        observed_wal_tail_frames: usize,
    },
}

impl RecoveryWorkBound {
    pub const fn is_bounded_by_checkpoint_interval_and_wal_tail(self) -> bool {
        matches!(self, Self::CheckpointIntervalAndWalTail { .. })
    }

    pub const fn wal_tail_frames(self) -> usize {
        match self {
            Self::CheckpointIntervalAndWalTail {
                observed_wal_tail_frames,
                ..
            } => observed_wal_tail_frames,
        }
    }

    pub const fn checkpoint_interval_frames(self) -> usize {
        match self {
            Self::CheckpointIntervalAndWalTail {
                checkpoint_interval_frames,
                ..
            } => checkpoint_interval_frames,
        }
    }

    pub const fn wal_tail_frame_limit(self) -> usize {
        match self {
            Self::CheckpointIntervalAndWalTail {
                wal_tail_frame_limit,
                ..
            } => wal_tail_frame_limit,
        }
    }
}
