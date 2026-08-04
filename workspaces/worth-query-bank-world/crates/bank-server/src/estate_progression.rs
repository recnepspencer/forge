mod request;

use worth_query_host::facade::domain::{
    WorthQueryApplicationCapabilityInstallationDenial,
    WorthQueryApplicationOperationInstallationDenial,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationAttemptDenial, WorthQueryInvariantDecisionPlanDenial,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationProjectionDenial,
};

#[derive(Debug)]
pub enum BankEstateProgressionDenial {
    CapabilityInstallation(WorthQueryApplicationCapabilityInstallationDenial),
    OperationInstallation(WorthQueryApplicationOperationInstallationDenial),
    Authorization(WorthQueryOperationAuthorizationDenial),
    Projection(WorthQueryOperationProjectionDenial),
    DecisionProjection(WorthQueryInvariantDecisionPlanDenial),
    Attempt(WorthQueryApplicationAttemptDenial),
}

impl std::fmt::Display for BankEstateProgressionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CapabilityInstallation(denial) => denial.fmt(formatter),
            Self::OperationInstallation(denial) => denial.fmt(formatter),
            Self::Authorization(denial) => denial.fmt(formatter),
            Self::Projection(denial) => denial.fmt(formatter),
            Self::DecisionProjection(denial) => denial.fmt(formatter),
            Self::Attempt(denial) => denial.fmt(formatter),
        }
    }
}

impl std::error::Error for BankEstateProgressionDenial {}
