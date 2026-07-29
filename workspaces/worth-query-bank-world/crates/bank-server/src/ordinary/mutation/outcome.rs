use worth_query_host::facade::domain::WorthQueryApplicationOperationInstallationDenialKind;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationCommitDenialKind, WorthQueryApplicationCommitDenialStage,
    WorthQueryEntityResolutionDenialKind, WorthQueryInvariantProjectionWork,
    WorthQueryOperationAuthorizationDenialKind,
};

use bank_domain::proposals::BankProposalDenial;

use crate::{BankCommitPreparationDenial, BankCommitReceipt, BankOperationProposalError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankMutationDenial {
    Scope(WorthQueryEntityResolutionDenialKind),
    Installation(WorthQueryApplicationOperationInstallationDenialKind),
    Authorization(WorthQueryOperationAuthorizationDenialKind),
    Proposal(BankOperationProposalError),
    Preparation(BankCommitPreparationDenial),
    Commit {
        kind: WorthQueryApplicationCommitDenialKind,
        stage: WorthQueryApplicationCommitDenialStage,
    },
    IdempotencyIntentDrift,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankMutationStatus {
    Committed(BankCommitReceipt),
    AlreadyCommitted(BankCommitReceipt),
    Stale { stale_fact_count: usize },
    Cancelled,
    DeadlineExceeded,
    Denied(BankMutationDenial),
    InvariantViolated(BankProposalDenial),
    Aborted,
    PartialEffect,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BankMutationMetadata {
    projection_work: Option<WorthQueryInvariantProjectionWork>,
}

impl BankMutationMetadata {
    pub const fn projection_work(self) -> Option<WorthQueryInvariantProjectionWork> {
        self.projection_work
    }

    pub const fn provider_work_units(self) -> usize {
        match self.projection_work {
            Some(work) => work.provider_work_units(),
            None => 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankMutationOutcome {
    status: BankMutationStatus,
    metadata: BankMutationMetadata,
}

impl BankMutationOutcome {
    pub(super) fn new(
        status: BankMutationStatus,
        projection_work: Option<WorthQueryInvariantProjectionWork>,
    ) -> Self {
        let status = normalize_status(status);
        Self {
            status,
            metadata: BankMutationMetadata { projection_work },
        }
    }

    pub const fn status(&self) -> &BankMutationStatus {
        &self.status
    }

    pub const fn metadata(&self) -> BankMutationMetadata {
        self.metadata
    }

    pub fn into_status(self) -> BankMutationStatus {
        self.status
    }
}

impl BankMutationStatus {
    pub(super) const fn is_authoritatively_committed(&self) -> bool {
        matches!(self, Self::Committed(_) | Self::AlreadyCommitted(_))
    }
}

fn normalize_status(status: BankMutationStatus) -> BankMutationStatus {
    match status {
        BankMutationStatus::Denied(BankMutationDenial::Proposal(
            BankOperationProposalError::Invariant(violation),
        )) => BankMutationStatus::InvariantViolated(violation),
        other => other,
    }
}
