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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankCommitReceipt {
    application: WorthQueryApplicationCommitReceipt,
}

impl BankCommitReceipt {
    pub fn branch_id(&self) -> &str {
        &self.application.branch_id().0
    }

    pub const fn commit_id(&self) -> u64 {
        self.application.commit_id().0
    }

    pub const fn changed_record_count(&self) -> usize {
        self.application.changed_record_count()
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.application.emitted_effect_count()
    }

    pub const fn expected_version_count(&self) -> usize {
        self.application
            .precondition_comparison()
            .expected_version_count()
    }

    pub const fn expected_fact_count(&self) -> usize {
        self.application
            .precondition_comparison()
            .expected_fact_count()
    }

    pub const fn precondition_comparison_identity(&self) -> Option<&[u8; 32]> {
        self.application.precondition_comparison().identity()
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkPhases {
        self.application.canonical_work()
    }

    pub fn graph_work_session_identity(&self) -> Option<&[u8; 32]> {
        self.application
            .graph_work()
            .map(|receipt| receipt.session_identity().bytes())
    }

    pub fn graph_work_session_identity_hex(&self) -> Option<String> {
        self.application
            .graph_work()
            .map(|receipt| receipt.session_identity().render_hex())
    }

    pub fn graph_work_provider_session_identity(&self) -> Option<&str> {
        self.application
            .graph_work()
            .map(|receipt| receipt.provider_session_identity())
    }

    pub fn graph_work_plan_identity(&self) -> Option<&[u8; 32]> {
        self.application
            .graph_work()
            .map(|receipt| receipt.plan_identity().bytes())
    }

    pub fn graph_work_obligation_identity(&self) -> Option<&[u8; 32]> {
        self.application
            .graph_work()
            .map(|receipt| receipt.obligation_identity().bytes())
    }

    pub fn graph_work_branch_id(&self) -> Option<&str> {
        self.application
            .graph_work()
            .map(|receipt| receipt.branch_id().0.as_str())
    }

    pub fn graph_work_required_obligation_count(&self) -> Option<usize> {
        self.application
            .graph_work()
            .map(|receipt| receipt.required_obligation_count())
    }

    pub fn graph_work_released_reservation_count(&self) -> Option<usize> {
        self.application
            .graph_work()
            .map(|receipt| receipt.released_reservation_count())
    }

    pub fn graph_work_basis_released(&self) -> Option<bool> {
        self.application
            .graph_work()
            .map(|receipt| receipt.basis_released())
    }

    pub(crate) const fn application(&self) -> &WorthQueryApplicationCommitReceipt {
        &self.application
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
        application: receipt,
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
