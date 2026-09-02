use worth_relational::facade::history::RelationalCommitReceipt;
use worth_relational::facade::mvcc::{
    PerformedRelationalCommit, PreparedRelationalCommitCandidate,
};
use worth_relational::facade::publication::DeferredPublicationSettlement;
use worth_signal::facade::branch::{SignalBranchAdvanceOutcome, SignalBranchForkOutcome};

/// Exact Relational owner progress. A generic ordinal cannot say which owner
/// evidence or settlement obligation is alive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalAttemptProgressPosture {
    Untouched,
    Prepared,
    Performed,
    SettlementPending,
    Settled,
}

#[derive(Debug)]
pub(super) enum RelationalProgressEvidence {
    Prepared(PreparedRelationalCommitCandidate),
    Performed(PerformedRelationalCommit),
    SettlementPending {
        performed: PerformedRelationalCommit,
        settlement: DeferredPublicationSettlement,
    },
    Settled {
        receipt: RelationalCommitReceipt,
    },
}

#[derive(Debug)]
pub struct RelationalAttemptProgress {
    posture: RelationalAttemptProgressPosture,
    evidence: Option<RelationalProgressEvidence>,
}

impl RelationalAttemptProgress {
    pub(crate) fn untouched() -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Untouched,
            evidence: None,
        }
    }

    pub(crate) fn prepared(candidate: PreparedRelationalCommitCandidate) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Prepared,
            evidence: Some(RelationalProgressEvidence::Prepared(candidate)),
        }
    }

    pub(crate) fn performed_settlement_pending(
        performed: PerformedRelationalCommit,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::SettlementPending,
            evidence: Some(RelationalProgressEvidence::SettlementPending {
                performed,
                settlement,
            }),
        }
    }

    pub(crate) fn performed(performed: PerformedRelationalCommit) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Performed,
            evidence: Some(RelationalProgressEvidence::Performed(performed)),
        }
    }

    pub(crate) fn settled(receipt: RelationalCommitReceipt) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Settled,
            evidence: Some(RelationalProgressEvidence::Settled { receipt }),
        }
    }

    pub const fn posture(&self) -> RelationalAttemptProgressPosture {
        self.posture
    }

    pub(super) fn into_evidence(self) -> Option<RelationalProgressEvidence> {
        self.evidence
    }
}

/// Exact Signal owner progress. Signal has no Relational settlement state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalAttemptProgressPosture {
    Untouched,
    PreparedForExecution,
    Performed,
}

#[derive(Debug)]
pub(super) enum SignalProgressEvidence {
    Prepared,
    Advanced(SignalBranchAdvanceOutcome),
    Forked(SignalBranchForkOutcome),
}

#[derive(Debug)]
pub struct SignalAttemptProgress {
    posture: SignalAttemptProgressPosture,
    evidence: Option<SignalProgressEvidence>,
}

impl SignalAttemptProgress {
    pub(crate) fn untouched() -> Self {
        Self {
            posture: SignalAttemptProgressPosture::Untouched,
            evidence: None,
        }
    }

    pub(crate) fn prepared_for_execution() -> Self {
        Self {
            posture: SignalAttemptProgressPosture::PreparedForExecution,
            evidence: Some(SignalProgressEvidence::Prepared),
        }
    }

    pub(crate) fn advanced(outcome: SignalBranchAdvanceOutcome) -> Self {
        Self {
            posture: SignalAttemptProgressPosture::Performed,
            evidence: Some(SignalProgressEvidence::Advanced(outcome)),
        }
    }

    pub(crate) fn forked(outcome: SignalBranchForkOutcome) -> Self {
        Self {
            posture: SignalAttemptProgressPosture::Performed,
            evidence: Some(SignalProgressEvidence::Forked(outcome)),
        }
    }

    pub const fn posture(&self) -> SignalAttemptProgressPosture {
        self.posture
    }

    pub(super) fn into_evidence(self) -> Option<SignalProgressEvidence> {
        self.evidence
    }
}

#[derive(Debug)]
pub struct CompositeAttemptProgress {
    relational: RelationalAttemptProgress,
    signal: SignalAttemptProgress,
}

impl CompositeAttemptProgress {
    pub(crate) fn untouched() -> Self {
        Self {
            relational: RelationalAttemptProgress::untouched(),
            signal: SignalAttemptProgress::untouched(),
        }
    }

    pub(crate) fn new(
        relational: RelationalAttemptProgress,
        signal: SignalAttemptProgress,
    ) -> Self {
        Self { relational, signal }
    }

    pub const fn relational_posture(&self) -> RelationalAttemptProgressPosture {
        self.relational.posture()
    }

    pub const fn signal_posture(&self) -> SignalAttemptProgressPosture {
        self.signal.posture()
    }

    pub fn relational(&self) -> &RelationalAttemptProgress {
        &self.relational
    }

    pub fn signal(&self) -> &SignalAttemptProgress {
        &self.signal
    }

    pub(crate) fn into_parts(self) -> (RelationalAttemptProgress, SignalAttemptProgress) {
        (self.relational, self.signal)
    }
}
