use crate::{LogSequenceNumber, RecoverySourceDecisionKind, WalLsnRange};
use worth_store_budgets::BudgetAdmissionDecision;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryBudgetDenial {
    kind: RecoveryBudgetDenialKind,
    decision: BudgetAdmissionDecision,
    redo_execution_attempts: u64,
}

impl RecoveryBudgetDenial {
    pub(crate) const fn new(kind: RecoveryBudgetDenialKind) -> Self {
        Self {
            kind,
            decision: BudgetAdmissionDecision::Deny,
            redo_execution_attempts: 0,
        }
    }

    pub const fn kind(&self) -> &RecoveryBudgetDenialKind {
        &self.kind
    }

    pub const fn decision(&self) -> BudgetAdmissionDecision {
        self.decision
    }

    pub const fn redo_execution_attempts(&self) -> u64 {
        self.redo_execution_attempts
    }

    pub const fn execution_started(&self) -> bool {
        self.redo_execution_attempts != 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryBudgetDenialKind {
    CheckpointIntervalMismatch {
        checkpoint_end: LogSequenceNumber,
        tail_start: LogSequenceNumber,
    },
    WalTailFrameBudgetExceeded {
        planned: usize,
        max: usize,
    },
    WalTailSourceMismatch {
        declared_tail_range: WalLsnRange,
        planned_source_range: WalLsnRange,
    },
    WalTailSegmentBudgetExceeded {
        scanned: usize,
        max: usize,
    },
    PageRedoBudgetExceeded {
        planned: usize,
        max: usize,
    },
    CheckpointDiscoveryBudgetExceeded {
        discovered: usize,
        max: usize,
    },
    ForbiddenFullStoreScan {
        attempted_pages: u64,
        checkpoint_interval_frames: usize,
        wal_tail_frame_limit: usize,
    },
    RecoverySourceAdmissionMismatch {
        admitted_kind: RecoverySourceDecisionKind,
        planned_kind: RecoverySourceDecisionKind,
        admitted_candidates: usize,
        planned_candidates: usize,
    },
    MissingCheckpointBaseForBoundedRecovery {
        source_kind: RecoverySourceDecisionKind,
    },
    MissingWalTailForBoundedRecovery {
        source_kind: RecoverySourceDecisionKind,
    },
    MemoryEnvelopeBudgetExceeded {
        admitted_bytes: u64,
        max_bytes: u64,
    },
    AllocationBudgetExceeded {
        allocated_bytes: u64,
        max_bytes: u64,
    },
}
