mod account_access;
mod account_creation;
mod business_payment;
mod journal;
mod money_movement;
mod reversal;

use worth_query_host::facade::domain::WorthQueryCanonicalWorkPhases;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
    WorthQueryApplicationCommitDenialKind, WorthQueryApplicationCommitDenialStage,
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationCommitReceipt,
};
use worth_query_host::facade::publication::domain_computation::{
    publish_application_commit, WorthQueryApplicationCommitPublicationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankCommitReceipt {
    application: WorthQueryApplicationCommitPublicationReceipt,
}

impl BankCommitReceipt {
    pub const fn commit_id(&self) -> u64 {
        self.application.terminal().commit_id().0
    }

    pub const fn changed_record_count(&self) -> usize {
        self.application.terminal().changed_record_count()
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.application.terminal().emitted_effect_count()
    }

    pub const fn expected_version_count(&self) -> usize {
        self.application
            .terminal()
            .precondition_comparison()
            .expected_version_count()
    }

    pub const fn expected_fact_count(&self) -> usize {
        self.application
            .terminal()
            .precondition_comparison()
            .expected_fact_count()
    }

    pub const fn decision_fact_count(&self) -> Option<usize> {
        match self.application.terminal().mutation_work() {
            Some(work) => Some(work.decision_fact_count()),
            None => None,
        }
    }

    pub const fn precondition_comparison_identity(&self) -> Option<&[u8; 32]> {
        self.application
            .terminal()
            .precondition_comparison()
            .identity()
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkPhases {
        self.application.terminal().canonical_work()
    }

    pub const fn publication(&self) -> &WorthQueryApplicationCommitPublicationReceipt {
        &self.application
    }

    pub fn is_same_authoritative_commit(&self, other: &Self) -> bool {
        self.application
            .terminal()
            .is_same_authoritative_commit(other.application.terminal())
    }

    pub(crate) const fn application(&self) -> &WorthQueryApplicationCommitReceipt {
        self.application.terminal()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankMutationCommitOutcome {
    Committed(BankCommitReceipt),
    AlreadyCommitted(BankCommitReceipt),
    Stale {
        stale_fact_count: usize,
    },
    Cancelled,
    Denied {
        kind: WorthQueryApplicationCommitDenialKind,
        stage: WorthQueryApplicationCommitDenialStage,
    },
    Aborted,
    PartialEffect,
    Indeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankCommitPreparationDenial {
    Application {
        kind: WorthQueryApplicationAttemptDenialKind,
        subject: String,
    },
    InvalidProposalShape,
    AccountingRevisionOverflow,
}

impl From<WorthQueryApplicationAttemptDenial> for BankCommitPreparationDenial {
    fn from(denial: WorthQueryApplicationAttemptDenial) -> Self {
        Self::Application {
            kind: denial.kind(),
            subject: denial.subject().to_owned(),
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
                kind: denial.kind(),
                stage: denial.stage(),
            },
            WorthQueryApplicationCommitOutcome::Aborted => Self::Aborted,
            WorthQueryApplicationCommitOutcome::PartialEffect => Self::PartialEffect,
            WorthQueryApplicationCommitOutcome::Indeterminate => Self::Indeterminate,
        }
    }
}

pub(crate) fn commit_receipt(
    receipt: worth_query_host::facade::primary_graph::WorthQueryApplicationCommitReceipt,
) -> BankCommitReceipt {
    BankCommitReceipt {
        application: publish_application_commit(receipt).into_receipt(),
    }
}

pub(super) fn application_idempotency(
    proposal: &bank_domain::proposals::BankInvariantApprovedProposal,
) -> worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding {
    worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding::new(
        proposal.idempotency_key_identity().bytes(),
        proposal.idempotency_intent().bytes(),
    )
}

pub(super) fn entity_key<Entity>(
    value: String,
) -> Result<
    worth_query_host::facade::primary_graph::WorthQueryApplicationEntityKey<
        bank_domain::schema::BankSchema,
        Entity,
    >,
    BankCommitPreparationDenial,
> {
    worth_query_host::facade::primary_graph::WorthQueryApplicationEntityKey::new(value)
        .map_err(|_| BankCommitPreparationDenial::InvalidProposalShape)
}

impl std::fmt::Display for BankCommitPreparationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "bank commit preparation denied: {self:?}")
    }
}

impl std::error::Error for BankCommitPreparationDenial {}
