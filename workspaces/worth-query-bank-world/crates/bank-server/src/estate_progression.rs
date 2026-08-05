mod approval;
mod close;
mod delegation;
mod disburse_estate;
mod freeze_account;
mod idempotency;
mod lifecycle_facts;
mod notify_death;
mod open_estate_case;
mod recognize_executor;
mod release_estate;
mod request;
mod review;

pub use delegation::{
    BankCapabilityDelegationProjectionDenial, BankCapabilityRevocationProjectionDenial,
};
pub use disburse_estate::BankEstateDisbursementProjectionDenial;
pub use freeze_account::BankEstateFreezeProjectionDenial;
pub use notify_death::BankDeathNotificationProjectionDenial;
pub use open_estate_case::BankEstateCaseOpeningProjectionDenial;
pub use recognize_executor::BankExecutorRecognitionProjectionDenial;
pub use release_estate::BankEstateReleaseProjectionDenial;

use worth_query_host::facade::domain::{
    WorthQueryApplicationCapabilityInstallationDenial,
    WorthQueryApplicationOperationInstallationDenial,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationIdempotencyResolutionDenial,
    WorthQueryElevationApprovalAuthorizationDenial, WorthQueryElevationCloseAuthorizationDenial,
    WorthQueryEntityResolutionDenial, WorthQueryInvariantDecisionPlanDenial,
    WorthQueryInvariantProjectionTraversalDenial, WorthQueryMandatoryReviewAuthorizationDenial,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationProjectionDenial,
};

#[derive(Debug)]
pub enum BankEstateLifecycleProjectionDenial {
    ReceiptIdentity(&'static str),
    RelationCardinality {
        relation: &'static str,
        expected: usize,
        observed: usize,
    },
    RelationTargetMismatch {
        relation: &'static str,
    },
    EntityResolution(WorthQueryEntityResolutionDenial),
    DecisionPlan(WorthQueryInvariantDecisionPlanDenial),
    Traversal(WorthQueryInvariantProjectionTraversalDenial),
}

#[derive(Debug)]
pub enum BankEstateProgressionDenial {
    CapabilityInstallation(WorthQueryApplicationCapabilityInstallationDenial),
    OperationInstallation(WorthQueryApplicationOperationInstallationDenial),
    Authorization(WorthQueryOperationAuthorizationDenial),
    ApprovalAuthorization(Box<WorthQueryElevationApprovalAuthorizationDenial>),
    CloseAuthorization(Box<WorthQueryElevationCloseAuthorizationDenial>),
    ReviewAuthorization(Box<WorthQueryMandatoryReviewAuthorizationDenial>),
    CommandInput(&'static str),
    Projection(WorthQueryOperationProjectionDenial),
    DecisionProjection(WorthQueryInvariantDecisionPlanDenial),
    FreezeProjection(BankEstateFreezeProjectionDenial),
    DeathNotificationProjection(BankDeathNotificationProjectionDenial),
    CaseOpeningProjection(BankEstateCaseOpeningProjectionDenial),
    ExecutorRecognitionProjection(BankExecutorRecognitionProjectionDenial),
    EstateReleaseProjection(BankEstateReleaseProjectionDenial),
    EstateDisbursementProjection(BankEstateDisbursementProjectionDenial),
    Proposal(bank_domain::proposals::BankProposalDenial),
    CommitPreparation(crate::operation_commit::BankCommitPreparationDenial),
    CapabilityDelegationProjection(BankCapabilityDelegationProjectionDenial),
    CapabilityRevocationProjection(BankCapabilityRevocationProjectionDenial),
    Idempotency(WorthQueryApplicationIdempotencyResolutionDenial),
    LifecycleProjection(BankEstateLifecycleProjectionDenial),
    Attempt(WorthQueryApplicationAttemptDenial),
}

impl std::fmt::Display for BankEstateProgressionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CapabilityInstallation(denial) => denial.fmt(formatter),
            Self::OperationInstallation(denial) => denial.fmt(formatter),
            Self::Authorization(denial) => denial.fmt(formatter),
            Self::ApprovalAuthorization(denial) => denial.denial().fmt(formatter),
            Self::CloseAuthorization(denial) => denial.denial().fmt(formatter),
            Self::ReviewAuthorization(denial) => denial.denial().fmt(formatter),
            Self::CommandInput(operation) => {
                write!(formatter, "invalid estate lifecycle input for {operation}")
            }
            Self::Projection(denial) => denial.fmt(formatter),
            Self::DecisionProjection(denial) => denial.fmt(formatter),
            Self::FreezeProjection(denial) => denial.fmt(formatter),
            Self::DeathNotificationProjection(denial) => denial.fmt(formatter),
            Self::CaseOpeningProjection(denial) => denial.fmt(formatter),
            Self::ExecutorRecognitionProjection(denial) => denial.fmt(formatter),
            Self::EstateReleaseProjection(denial) => denial.fmt(formatter),
            Self::EstateDisbursementProjection(denial) => denial.fmt(formatter),
            Self::Proposal(denial) => denial.fmt(formatter),
            Self::CommitPreparation(denial) => denial.fmt(formatter),
            Self::CapabilityDelegationProjection(denial) => denial.fmt(formatter),
            Self::CapabilityRevocationProjection(denial) => denial.fmt(formatter),
            Self::Idempotency(denial) => denial.fmt(formatter),
            Self::LifecycleProjection(denial) => denial.fmt(formatter),
            Self::Attempt(denial) => denial.fmt(formatter),
        }
    }
}

impl std::error::Error for BankEstateProgressionDenial {}

impl From<WorthQueryApplicationAttemptDenial> for BankEstateProgressionDenial {
    fn from(denial: WorthQueryApplicationAttemptDenial) -> Self {
        Self::Attempt(denial)
    }
}

impl std::fmt::Display for BankEstateLifecycleProjectionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReceiptIdentity(subject) => {
                write!(formatter, "invalid elevation receipt identity: {subject}")
            }
            Self::RelationCardinality {
                relation,
                expected,
                observed,
            } => write!(
                formatter,
                "lifecycle relation {relation} expected {expected} target, observed {observed}"
            ),
            Self::RelationTargetMismatch { relation } => {
                write!(
                    formatter,
                    "lifecycle relation {relation} targets the wrong estate"
                )
            }
            Self::EntityResolution(denial) => denial.fmt(formatter),
            Self::DecisionPlan(denial) => denial.fmt(formatter),
            Self::Traversal(denial) => denial.fmt(formatter),
        }
    }
}

impl std::error::Error for BankEstateLifecycleProjectionDenial {}

impl From<WorthQueryEntityResolutionDenial> for BankEstateLifecycleProjectionDenial {
    fn from(denial: WorthQueryEntityResolutionDenial) -> Self {
        Self::EntityResolution(denial)
    }
}

impl From<WorthQueryInvariantDecisionPlanDenial> for BankEstateLifecycleProjectionDenial {
    fn from(denial: WorthQueryInvariantDecisionPlanDenial) -> Self {
        Self::DecisionPlan(denial)
    }
}

impl From<WorthQueryInvariantProjectionTraversalDenial> for BankEstateLifecycleProjectionDenial {
    fn from(denial: WorthQueryInvariantProjectionTraversalDenial) -> Self {
        Self::Traversal(denial)
    }
}
