use super::super::tenant_basis::TenantBasisCounters;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PolicyBasisCounters {
    policy_basis_admitted_count: usize,
    policy_basis_denial_count: usize,
    branch_access_denial_count: usize,
    unsupported_execution_mode_denial_count: usize,
    raw_middleware_source_denial_count: usize,
    policy_work_budget_denial_count: usize,
}

impl PolicyBasisCounters {
    pub(crate) fn admitted() -> Self {
        Self {
            policy_basis_admitted_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_policy() -> Self {
        Self {
            policy_basis_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_branch() -> Self {
        Self {
            branch_access_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_mode() -> Self {
        Self {
            unsupported_execution_mode_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_middleware() -> Self {
        Self {
            raw_middleware_source_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_work_budget() -> Self {
        Self {
            policy_work_budget_denial_count: 1,
            ..Self::default()
        }
    }

    pub fn policy_basis_admitted_count(&self) -> usize {
        self.policy_basis_admitted_count
    }

    pub fn policy_basis_denial_count(&self) -> usize {
        self.policy_basis_denial_count
    }

    pub fn branch_access_denial_count(&self) -> usize {
        self.branch_access_denial_count
    }

    pub fn unsupported_execution_mode_denial_count(&self) -> usize {
        self.unsupported_execution_mode_denial_count
    }

    pub fn raw_middleware_source_denial_count(&self) -> usize {
        self.raw_middleware_source_denial_count
    }

    pub fn policy_work_budget_denial_count(&self) -> usize {
        self.policy_work_budget_denial_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyTenantAdmissionCounters {
    policy: PolicyBasisCounters,
    tenant: TenantBasisCounters,
    admission_bundle_count: usize,
    saved_query_reuse_classification_count: usize,
}

impl PolicyTenantAdmissionCounters {
    pub(crate) fn admitted(policy: PolicyBasisCounters, tenant: TenantBasisCounters) -> Self {
        Self {
            policy,
            tenant,
            admission_bundle_count: 1,
            saved_query_reuse_classification_count: 0,
        }
    }

    pub fn saved_query_reuse_classified() -> Self {
        Self {
            policy: PolicyBasisCounters::default(),
            tenant: TenantBasisCounters::default(),
            admission_bundle_count: 0,
            saved_query_reuse_classification_count: 1,
        }
    }

    pub fn policy(&self) -> &PolicyBasisCounters {
        &self.policy
    }

    pub fn tenant(&self) -> &TenantBasisCounters {
        &self.tenant
    }

    pub fn admission_bundle_count(&self) -> usize {
        self.admission_bundle_count
    }

    pub fn saved_query_reuse_classification_count(&self) -> usize {
        self.saved_query_reuse_classification_count
    }
}
