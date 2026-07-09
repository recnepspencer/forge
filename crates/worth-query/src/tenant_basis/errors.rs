use super::TenantBasisCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TenantBasisAdmissionFailureClass {
    DerivedBindingDeferred,
    AmbiguousTenantContext,
    HiddenTenantFilter,
    MissingTenantTruthBasis,
    MissingTenantSchemaBasis,
    TenantSchemaMismatch,
    GlobalSchemaFallbackForbidden,
}

impl TenantBasisAdmissionFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DerivedBindingDeferred => "derived_binding_deferred",
            Self::AmbiguousTenantContext => "ambiguous_tenant_context",
            Self::HiddenTenantFilter => "hidden_tenant_filter",
            Self::MissingTenantTruthBasis => "missing_tenant_truth_basis",
            Self::MissingTenantSchemaBasis => "missing_tenant_schema_basis",
            Self::TenantSchemaMismatch => "tenant_schema_mismatch",
            Self::GlobalSchemaFallbackForbidden => "global_schema_fallback_forbidden",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TenantBasisAdmissionError {
    failure_class: TenantBasisAdmissionFailureClass,
    message: &'static str,
    counters: TenantBasisCounters,
}

impl TenantBasisAdmissionError {
    pub(crate) fn new(
        failure_class: TenantBasisAdmissionFailureClass,
        message: &'static str,
        counters: TenantBasisCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> TenantBasisAdmissionFailureClass {
        self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &TenantBasisCounters {
        &self.counters
    }
}
