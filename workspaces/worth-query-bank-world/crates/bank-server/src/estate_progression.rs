mod approval;
mod close;
mod freeze_account;
mod lifecycle_facts;
mod request;
mod review;

pub use freeze_account::BankEstateFreezeProjectionDenial;

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
            Self::Idempotency(denial) => denial.fmt(formatter),
            Self::LifecycleProjection(denial) => denial.fmt(formatter),
            Self::Attempt(denial) => denial.fmt(formatter),
        }
    }
}

impl std::error::Error for BankEstateProgressionDenial {}

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
