mod account_access;
mod account_creation;
mod business_payment;
mod journal;
mod money_movement;
mod reversal;

pub(crate) use journal::{lower_journal, resolve_journal_accounts};

use worth_query_host::facade::domain::WorthQueryCanonicalWorkPhases;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
    WorthQueryApplicationCommitDenialKind, WorthQueryApplicationCommitDenialStage,
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationCommitReceipt,
    WorthQueryApplicationCommitRecoveryKind, WorthQueryApplicationUnresolvedCommitEvidence,
};
use worth_query_host::facade::publication::application_aftermath::{
    WorthQueryPublishedApplicationAftermath, WorthQueryPublishedExternalEffectPosture,
};
use worth_query_host::facade::publication::domain_computation::{
    publish_application_commit, WorthQueryApplicationCommitPublicationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankCommitReceipt {
    application: WorthQueryApplicationCommitPublicationReceipt,
    execution: WorthQueryApplicationCommitReceipt,
}

impl BankCommitReceipt {
    pub const fn outcome_identity(&self) -> Option<u64> {
        match self.execution.outcome_identity() {
            Some(identity) => Some(identity.get()),
            None => None,
        }
    }

    pub const fn commit_id(&self) -> u64 {
        self.execution.commit_id().0
    }

    pub const fn changed_record_count(&self) -> usize {
        self.execution.changed_record_count()
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.execution.emitted_effect_count()
    }

    pub const fn expected_version_count(&self) -> usize {
        self.execution
            .precondition_comparison()
            .expected_version_count()
    }

    pub const fn expected_fact_count(&self) -> usize {
        self.execution
            .precondition_comparison()
            .expected_fact_count()
    }

    pub fn decision_fact_count(&self) -> Option<usize> {
        self.execution
            .mutation_work()
            .map(|work| work.decision_fact_count())
    }

    pub const fn precondition_comparison_identity(&self) -> Option<&[u8; 32]> {
        self.execution.precondition_comparison().identity()
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkPhases {
        self.execution.canonical_work()
    }

    pub const fn publication(&self) -> &WorthQueryApplicationCommitPublicationReceipt {
        &self.application
    }

    pub const fn aftermath(&self) -> &WorthQueryPublishedApplicationAftermath {
        self.application.aftermath()
    }

    pub fn is_same_authoritative_commit(&self, other: &Self) -> bool {
        self.execution
            .is_same_authoritative_commit(&other.execution)
    }

    /// What production dispatch observed for a declared external effect.
    ///
    /// `None` for every operation that declares no external effect, which is
    /// the ordinary money-movement case.
    pub const fn external_dispatch_posture(
        &self,
    ) -> Option<WorthQueryPublishedExternalEffectPosture> {
        match self.aftermath().external_effect() {
            WorthQueryPublishedExternalEffectPosture::NotDeclared
            | WorthQueryPublishedExternalEffectPosture::PendingDispatch => None,
            posture => Some(posture),
        }
    }

    /// True when this commit durably co-committed a dispatch outbox record.
    pub const fn co_committed_dispatch_outbox(&self) -> bool {
        self.execution.dispatch_outbox().is_some()
    }

    /// Installed operation identity retained on the commit receipt (R8.62 / C1).
    pub const fn installed_operation(&self) -> &[u8; 32] {
        self.execution.installed_operation()
    }

    /// Admitted principal scope retained on the commit receipt (R8.62 / C1).
    pub const fn principal_scope(
        &self,
    ) -> &worth_query_host::facade::primary_graph::WorthQueryOperationScopeBinding {
        self.execution.principal_scope()
    }

    /// Idempotency binding retained on the commit receipt (R8.62 / C1).
    pub const fn idempotency_binding(
        &self,
    ) -> worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding {
        self.execution.idempotency_binding()
    }

    /// Whether the commit retained an inverse pre-image slice (R8.2).
    pub const fn retained_preimage(&self) -> bool {
        self.execution.retained_preimage().is_some()
    }

    pub(crate) const fn application(&self) -> &WorthQueryApplicationCommitReceipt {
        &self.execution
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
    /// Some effect may have landed. Query's correlation evidence is retained
    /// so recovery can distinguish commit-path from abort-path repair.
    PartialEffect(WorthQueryApplicationUnresolvedCommitEvidence),
    /// The commit's fate is unknown. The same retained Query evidence names
    /// which recovery the operator owes.
    Indeterminate(WorthQueryApplicationUnresolvedCommitEvidence),
}

impl BankMutationCommitOutcome {
    /// Query's retained evidence when the commit did not resolve.
    ///
    /// Bank does not re-derive this: it is exactly what the provider session
    /// observed, including whether commit or abort recovery is owed.
    pub const fn unresolved_evidence(
        &self,
    ) -> Option<&WorthQueryApplicationUnresolvedCommitEvidence> {
        match self {
            Self::PartialEffect(evidence) | Self::Indeterminate(evidence) => Some(evidence),
            _ => None,
        }
    }

    /// Which recovery an unresolved outcome owes, straight from Query.
    pub const fn commit_recovery_kind(&self) -> Option<WorthQueryApplicationCommitRecoveryKind> {
        match self.unresolved_evidence() {
            Some(evidence) => Some(evidence.recovery()),
            None => None,
        }
    }
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
            WorthQueryApplicationCommitOutcome::PartialEffect(evidence) => {
                Self::PartialEffect(evidence)
            }
            WorthQueryApplicationCommitOutcome::Indeterminate(evidence) => {
                Self::Indeterminate(evidence)
            }
        }
    }
}

pub(crate) fn commit_receipt(
    receipt: worth_query_host::facade::primary_graph::WorthQueryApplicationCommitReceipt,
) -> BankCommitReceipt {
    let application = publish_application_commit(receipt.clone()).into_receipt();
    BankCommitReceipt {
        application,
        execution: receipt,
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
