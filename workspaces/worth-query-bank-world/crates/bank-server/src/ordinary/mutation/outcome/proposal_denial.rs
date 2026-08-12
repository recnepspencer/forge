//! Bank-owned description of proposal preparation denial.

use bank_domain::proposals::BankProposalDenial;
use worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyResolutionDenialKind;

use super::BankAuthorizationDenial;
use crate::BankOperationProposalError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankIdempotencyResolutionDenialKind {
    Authorization,
    ForeignAdmission,
    ProviderUnavailable,
}

impl BankIdempotencyResolutionDenialKind {
    pub(crate) const fn from_query(
        kind: WorthQueryApplicationIdempotencyResolutionDenialKind,
    ) -> Self {
        match kind {
            WorthQueryApplicationIdempotencyResolutionDenialKind::Authorization => {
                Self::Authorization
            }
            WorthQueryApplicationIdempotencyResolutionDenialKind::ForeignAdmission => {
                Self::ForeignAdmission
            }
            WorthQueryApplicationIdempotencyResolutionDenialKind::ProviderUnavailable => {
                Self::ProviderUnavailable
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankMutationProposalDenial {
    Authorization(BankAuthorizationDenial),
    AuthorizationLineageUnavailable(BankAuthorizationDenial),
    ProjectionWorkBudgetExceeded,
    ProjectionDenied,
    Invariant(BankProposalDenial),
    Idempotency(BankIdempotencyResolutionDenialKind),
}

impl BankMutationProposalDenial {
    pub(super) fn from_query(error: BankOperationProposalError) -> Self {
        match error {
            BankOperationProposalError::Authorization(denial) => Self::Authorization(denial),
            BankOperationProposalError::AuthorizationLineageUnavailable(denial) => {
                Self::AuthorizationLineageUnavailable(denial)
            }
            BankOperationProposalError::ProjectionWorkBudgetExceeded => {
                Self::ProjectionWorkBudgetExceeded
            }
            BankOperationProposalError::Projection(_) => Self::ProjectionDenied,
            BankOperationProposalError::Invariant(denial) => Self::Invariant(denial),
            BankOperationProposalError::Idempotency(kind) => Self::Idempotency(kind),
        }
    }
}
