//! Bank-owned terminal classification of one Query application commit.

use worth_query_host::facade::primary_graph::WorthQueryApplicationCommitOutcome;

use super::commit_denial::{denial_kind, denial_stage};
use super::{
    commit_receipt, BankCommitDenialKind, BankCommitDenialStage, BankCommitReceipt,
    BankCommitRecoveryKind, BankUnresolvedCommitEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankMutationCommitOutcome {
    Committed(BankCommitReceipt),
    AlreadyCommitted(BankCommitReceipt),
    Stale {
        stale_fact_count: usize,
    },
    Cancelled,
    Denied {
        kind: BankCommitDenialKind,
        stage: BankCommitDenialStage,
    },
    Aborted,
    /// Some effect may have landed. Query's correlation evidence is retained
    /// so recovery can distinguish commit-path from abort-path repair.
    PartialEffect(BankUnresolvedCommitEvidence),
    /// The commit's fate is unknown. The same retained Query evidence names
    /// which recovery the operator owes.
    Indeterminate(BankUnresolvedCommitEvidence),
}

impl BankMutationCommitOutcome {
    pub const fn unresolved_evidence(&self) -> Option<&BankUnresolvedCommitEvidence> {
        match self {
            Self::PartialEffect(evidence) | Self::Indeterminate(evidence) => Some(evidence),
            _ => None,
        }
    }

    pub const fn commit_recovery_kind(&self) -> Option<BankCommitRecoveryKind> {
        match self.unresolved_evidence() {
            Some(evidence) => Some(evidence.recovery_kind()),
            None => None,
        }
    }
}

impl From<WorthQueryApplicationCommitOutcome> for BankMutationCommitOutcome {
    fn from(outcome: WorthQueryApplicationCommitOutcome) -> Self {
        match outcome {
            WorthQueryApplicationCommitOutcome::Committed(receipt) => {
                Self::Committed(commit_receipt(receipt))
            }
            WorthQueryApplicationCommitOutcome::AlreadyCommitted(receipt) => {
                Self::AlreadyCommitted(commit_receipt(receipt))
            }
            WorthQueryApplicationCommitOutcome::Stale(stale) => Self::Stale {
                stale_fact_count: stale.stale_fact_count(),
            },
            WorthQueryApplicationCommitOutcome::Cancelled => Self::Cancelled,
            WorthQueryApplicationCommitOutcome::Denied(denial) => Self::Denied {
                kind: denial_kind(denial.kind()),
                stage: denial_stage(denial.stage()),
            },
            WorthQueryApplicationCommitOutcome::Aborted => Self::Aborted,
            WorthQueryApplicationCommitOutcome::PartialEffect(evidence) => {
                Self::PartialEffect(BankUnresolvedCommitEvidence::from_execution(evidence))
            }
            WorthQueryApplicationCommitOutcome::Indeterminate(evidence) => {
                Self::Indeterminate(BankUnresolvedCommitEvidence::from_execution(evidence))
            }
        }
    }
}
