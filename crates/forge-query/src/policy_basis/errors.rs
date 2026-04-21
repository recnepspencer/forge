use super::PolicyTenantAdmissionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyTenantAdmissionFailureClass {
    PolicyQueryFamilyDenied,
    BranchAccessDenied,
    TenantAdmissionDenied,
    UnsupportedExecutionMode,
    RawMiddlewarePolicySourceForbidden,
    PolicyWorkBudgetDenied,
    SavedQueryPolicyTenantBypassForbidden,
}

impl PolicyTenantAdmissionFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PolicyQueryFamilyDenied => "policy_query_family_denied",
            Self::BranchAccessDenied => "branch_access_denied",
            Self::TenantAdmissionDenied => "tenant_admission_denied",
            Self::UnsupportedExecutionMode => "unsupported_execution_mode",
            Self::RawMiddlewarePolicySourceForbidden => "raw_middleware_policy_source_forbidden",
            Self::PolicyWorkBudgetDenied => "policy_work_budget_denied",
            Self::SavedQueryPolicyTenantBypassForbidden => {
                "saved_query_policy_tenant_bypass_forbidden"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyTenantAdmissionError {
    failure_class: PolicyTenantAdmissionFailureClass,
    message: &'static str,
    counters: PolicyTenantAdmissionCounters,
}

impl PolicyTenantAdmissionError {
    pub(crate) fn new(
        failure_class: PolicyTenantAdmissionFailureClass,
        message: &'static str,
        counters: PolicyTenantAdmissionCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> PolicyTenantAdmissionFailureClass {
        self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &PolicyTenantAdmissionCounters {
        &self.counters
    }
}
