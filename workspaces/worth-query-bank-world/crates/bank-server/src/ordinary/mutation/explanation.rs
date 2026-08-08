use bank_domain::proposals::BankProposalDenial;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationCommitDenialStage, WorthQueryApplicationCommitRecoveryKind,
};

use super::{BankMutationDenial, BankMutationOutcome, BankMutationStatus};
use crate::{BankCommitReceipt, BankOperationProposalError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankMutationExplanationStage {
    Admission,
    Projection,
    Idempotency,
    EffectPreparation,
    ProviderCommit(WorthQueryApplicationCommitDenialStage),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankMutationExplanation<'outcome> {
    Committed {
        receipt: &'outcome BankCommitReceipt,
        recovered: bool,
    },
    Stale {
        stale_fact_count: usize,
    },
    Cancelled,
    DeadlineExceeded,
    Denied {
        stage: BankMutationExplanationStage,
        reason: &'outcome BankMutationDenial,
    },
    InvariantViolated(&'outcome BankProposalDenial),
    Aborted,
    /// Some effect may have landed; `recovery` is Query's own verdict on
    /// which repair the operator owes, not a Bank re-derivation.
    PartialEffect {
        recovery: WorthQueryApplicationCommitRecoveryKind,
    },
    Indeterminate {
        recovery: WorthQueryApplicationCommitRecoveryKind,
    },
}

impl BankMutationOutcome {
    pub fn explanation(&self) -> BankMutationExplanation<'_> {
        match self.status() {
            BankMutationStatus::Committed(receipt) => BankMutationExplanation::Committed {
                receipt,
                recovered: false,
            },
            BankMutationStatus::AlreadyCommitted(receipt) => BankMutationExplanation::Committed {
                receipt,
                recovered: true,
            },
            BankMutationStatus::Stale { stale_fact_count } => BankMutationExplanation::Stale {
                stale_fact_count: *stale_fact_count,
            },
            BankMutationStatus::Cancelled => BankMutationExplanation::Cancelled,
            BankMutationStatus::DeadlineExceeded => BankMutationExplanation::DeadlineExceeded,
            BankMutationStatus::Denied(reason) => BankMutationExplanation::Denied {
                stage: denial_stage(reason),
                reason,
            },
            BankMutationStatus::InvariantViolated(reason) => {
                BankMutationExplanation::InvariantViolated(reason)
            }
            BankMutationStatus::Aborted => BankMutationExplanation::Aborted,
            BankMutationStatus::PartialEffect(evidence) => BankMutationExplanation::PartialEffect {
                recovery: evidence.recovery(),
            },
            BankMutationStatus::Indeterminate(evidence) => BankMutationExplanation::Indeterminate {
                recovery: evidence.recovery(),
            },
        }
    }
}

fn denial_stage(denial: &BankMutationDenial) -> BankMutationExplanationStage {
    match denial {
        BankMutationDenial::Scope(_)
        | BankMutationDenial::Installation(_)
        | BankMutationDenial::Authorization(_) => BankMutationExplanationStage::Admission,
        BankMutationDenial::Proposal(BankOperationProposalError::Idempotency(_))
        | BankMutationDenial::IdempotencyIntentDrift => BankMutationExplanationStage::Idempotency,
        BankMutationDenial::Proposal(_) => BankMutationExplanationStage::Projection,
        BankMutationDenial::Preparation(_) => BankMutationExplanationStage::EffectPreparation,
        BankMutationDenial::Commit { stage, .. } => {
            BankMutationExplanationStage::ProviderCommit(*stage)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_noncommit_terminal_retains_distinct_typed_explanation() {
        assert_explanation(
            BankMutationStatus::Stale {
                stale_fact_count: 3,
            },
            |explanation| {
                matches!(
                    explanation,
                    BankMutationExplanation::Stale {
                        stale_fact_count: 3
                    }
                )
            },
        );
        assert_explanation(BankMutationStatus::Cancelled, |explanation| {
            matches!(explanation, BankMutationExplanation::Cancelled)
        });
        assert_explanation(BankMutationStatus::DeadlineExceeded, |explanation| {
            matches!(explanation, BankMutationExplanation::DeadlineExceeded)
        });
        assert_explanation(
            BankMutationStatus::Denied(BankMutationDenial::IdempotencyIntentDrift),
            |explanation| {
                matches!(
                    explanation,
                    BankMutationExplanation::Denied {
                        stage: BankMutationExplanationStage::Idempotency,
                        ..
                    }
                )
            },
        );
        assert_explanation(
            BankMutationStatus::InvariantViolated(BankProposalDenial::SelfApproval),
            |explanation| {
                matches!(
                    explanation,
                    BankMutationExplanation::InvariantViolated(BankProposalDenial::SelfApproval)
                )
            },
        );
        assert_explanation(BankMutationStatus::Aborted, |explanation| {
            matches!(explanation, BankMutationExplanation::Aborted)
        });
    }

    fn assert_explanation(
        status: BankMutationStatus,
        predicate: impl FnOnce(BankMutationExplanation<'_>) -> bool,
    ) {
        let outcome = BankMutationOutcome::new(status, None);
        assert!(predicate(outcome.explanation()));
    }
}
