use super::{counters::BlobRetentionReclaimCounterSnapshot, holds::BlobRetentionHoldKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobRetentionReclaimDenial {
    MissingS6ReclaimPosture {
        counters: BlobRetentionReclaimCounterSnapshot,
    },
    S6ReclaimPostureScopeMismatch {
        counters: BlobRetentionReclaimCounterSnapshot,
    },
    ReclaimBlockedByRetentionHold {
        kind: BlobRetentionHoldKind,
        counters: BlobRetentionReclaimCounterSnapshot,
    },
    ReachabilityReclaimDenied {
        counters: BlobRetentionReclaimCounterSnapshot,
    },
    ReclaimCandidateIdentityMismatch {
        counters: BlobRetentionReclaimCounterSnapshot,
    },
    BackendResidueRejected {
        counters: BlobRetentionReclaimCounterSnapshot,
    },
    CopiedReceiptRejected {
        counters: BlobRetentionReclaimCounterSnapshot,
    },
    CopiedCounterRejected {
        counters: BlobRetentionReclaimCounterSnapshot,
    },
    TerminalProjectionRejected {
        counters: BlobRetentionReclaimCounterSnapshot,
    },
    S6HandoffAloneRejected {
        counters: BlobRetentionReclaimCounterSnapshot,
    },
}

pub fn reject_backend_residue_as_retention_reclaim_authority() -> BlobRetentionReclaimDenial {
    BlobRetentionReclaimDenial::BackendResidueRejected {
        counters: BlobRetentionReclaimCounterSnapshot::start().record_copied_or_weak_denial(),
    }
}

pub fn reject_copied_receipt_as_retention_reclaim_authority() -> BlobRetentionReclaimDenial {
    BlobRetentionReclaimDenial::CopiedReceiptRejected {
        counters: BlobRetentionReclaimCounterSnapshot::start().record_copied_or_weak_denial(),
    }
}

pub fn reject_copied_counter_as_retention_reclaim_authority() -> BlobRetentionReclaimDenial {
    BlobRetentionReclaimDenial::CopiedCounterRejected {
        counters: BlobRetentionReclaimCounterSnapshot::start().record_copied_or_weak_denial(),
    }
}

pub fn reject_terminal_projection_as_retention_reclaim_authority() -> BlobRetentionReclaimDenial {
    BlobRetentionReclaimDenial::TerminalProjectionRejected {
        counters: BlobRetentionReclaimCounterSnapshot::start().record_copied_or_weak_denial(),
    }
}

pub fn reject_s6_reclaim_handoff_as_retention_reclaim_authority() -> BlobRetentionReclaimDenial {
    BlobRetentionReclaimDenial::S6HandoffAloneRejected {
        counters: BlobRetentionReclaimCounterSnapshot::start().record_s6_posture_denial(),
    }
}
