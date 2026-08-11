mod admission_denial;
mod projection_work;
mod proposal_denial;

pub use admission_denial::{
    BankAuthorizationDenial, BankAuthorizationDenialKind, BankEntityResolutionDenial,
    BankEntityResolutionDenialKind, BankOperationInstallationDenial,
    BankOperationInstallationDenialKind,
};
pub use projection_work::BankMutationProjectionWork;
pub use proposal_denial::{BankIdempotencyResolutionDenialKind, BankMutationProposalDenial};

use bank_domain::proposals::BankProposalDenial;

use crate::{
    BankCommitDenialKind, BankCommitDenialStage, BankCommitPreparationDenial, BankCommitReceipt,
    BankOperationProposalError, BankUnresolvedCommitEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankMutationDenial {
    Scope(BankEntityResolutionDenial),
    Installation(BankOperationInstallationDenial),
    Authorization(BankAuthorizationDenial),
    Proposal(BankMutationProposalDenial),
    Preparation(BankCommitPreparationDenial),
    Commit {
        kind: BankCommitDenialKind,
        stage: BankCommitDenialStage,
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
    PartialEffect(BankUnresolvedCommitEvidence),
    Indeterminate(BankUnresolvedCommitEvidence),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BankMutationMetadata {
    projection_work: Option<BankMutationProjectionWork>,
}

impl BankMutationMetadata {
    pub const fn projection_work(self) -> Option<BankMutationProjectionWork> {
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
        projection_work: Option<BankMutationProjectionWork>,
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

    pub const fn unresolved_evidence(&self) -> Option<&BankUnresolvedCommitEvidence> {
        match &self.status {
            BankMutationStatus::PartialEffect(evidence)
            | BankMutationStatus::Indeterminate(evidence) => Some(evidence),
            _ => None,
        }
    }
}

impl BankMutationStatus {
    pub(super) const fn is_authoritatively_committed(&self) -> bool {
        matches!(self, Self::Committed(_) | Self::AlreadyCommitted(_))
    }
}

impl BankMutationDenial {
    pub(super) fn from_proposal(error: BankOperationProposalError) -> Self {
        Self::Proposal(BankMutationProposalDenial::from_query(error))
    }
}

fn normalize_status(status: BankMutationStatus) -> BankMutationStatus {
    match status {
        BankMutationStatus::Denied(BankMutationDenial::Proposal(
            BankMutationProposalDenial::Invariant(violation),
        )) => BankMutationStatus::InvariantViolated(violation),
        other => other,
    }
}
