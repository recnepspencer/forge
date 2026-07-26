#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TenantBasisCounters {
    direct_tenant_binding_admitted_count: usize,
    cached_tenant_binding_admitted_count: usize,
    derived_tenant_binding_denial_count: usize,
    ambiguous_tenant_denial_count: usize,
    hidden_tenant_filter_denial_count: usize,
    missing_truth_basis_denial_count: usize,
    missing_schema_basis_denial_count: usize,
    global_schema_fallback_denial_count: usize,
}

impl TenantBasisCounters {
    pub(crate) fn direct_admitted() -> Self {
        Self {
            direct_tenant_binding_admitted_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn cached_admitted() -> Self {
        Self {
            cached_tenant_binding_admitted_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_derived() -> Self {
        Self {
            derived_tenant_binding_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_ambiguous() -> Self {
        Self {
            ambiguous_tenant_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_hidden_filter() -> Self {
        Self {
            hidden_tenant_filter_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_missing_truth() -> Self {
        Self {
            missing_truth_basis_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_missing_schema() -> Self {
        Self {
            missing_schema_basis_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_global_fallback() -> Self {
        Self {
            global_schema_fallback_denial_count: 1,
            ..Self::default()
        }
    }

    pub fn direct_tenant_binding_admitted_count(&self) -> usize {
        self.direct_tenant_binding_admitted_count
    }

    pub fn cached_tenant_binding_admitted_count(&self) -> usize {
        self.cached_tenant_binding_admitted_count
    }

    pub fn derived_tenant_binding_denial_count(&self) -> usize {
        self.derived_tenant_binding_denial_count
    }

    pub fn ambiguous_tenant_denial_count(&self) -> usize {
        self.ambiguous_tenant_denial_count
    }

    pub fn hidden_tenant_filter_denial_count(&self) -> usize {
        self.hidden_tenant_filter_denial_count
    }

    pub fn missing_truth_basis_denial_count(&self) -> usize {
        self.missing_truth_basis_denial_count
    }

    pub fn missing_schema_basis_denial_count(&self) -> usize {
        self.missing_schema_basis_denial_count
    }

    pub fn global_schema_fallback_denial_count(&self) -> usize {
        self.global_schema_fallback_denial_count
    }
}
