use super::admission_diagnostics::QuerySubscriptionAdmissionDiagnostics;
use super::counters::QuerySubscriptionDeclarationCounters;
use super::diagnostic::QuerySubscriptionDiagnosticEvidence;
use super::support::QuerySubscriptionSupportProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionAdmissionDenialKind {
    AdmissionBudgetExceeded,
    DurableReloadOverclaim,
    ActiveLifecycleAllocationForbidden,
}

impl QuerySubscriptionAdmissionDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdmissionBudgetExceeded => "admission_budget_exceeded",
            Self::DurableReloadOverclaim => "durable_reload_overclaim",
            Self::ActiveLifecycleAllocationForbidden => "active_lifecycle_allocation_forbidden",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionAdmissionError {
    denial_kind: QuerySubscriptionAdmissionDenialKind,
    message: String,
    diagnostics: QuerySubscriptionAdmissionDiagnostics,
    pipeline_diagnostic: QuerySubscriptionDiagnosticEvidence,
    support_profile: QuerySubscriptionSupportProfile,
    counters: QuerySubscriptionDeclarationCounters,
}

impl QuerySubscriptionAdmissionError {
    pub(super) fn new(
        denial_kind: QuerySubscriptionAdmissionDenialKind,
        message: impl Into<String>,
        diagnostics: QuerySubscriptionAdmissionDiagnostics,
        pipeline_diagnostic: QuerySubscriptionDiagnosticEvidence,
        support_profile: QuerySubscriptionSupportProfile,
        counters: QuerySubscriptionDeclarationCounters,
    ) -> Self {
        Self {
            denial_kind,
            message: message.into(),
            diagnostics,
            pipeline_diagnostic,
            support_profile,
            counters,
        }
    }

    pub fn denial_kind(&self) -> &QuerySubscriptionAdmissionDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn diagnostics(&self) -> &QuerySubscriptionAdmissionDiagnostics {
        &self.diagnostics
    }

    pub fn pipeline_diagnostic(&self) -> &QuerySubscriptionDiagnosticEvidence {
        &self.pipeline_diagnostic
    }

    pub fn support_profile(&self) -> &QuerySubscriptionSupportProfile {
        &self.support_profile
    }

    pub fn counters(&self) -> &QuerySubscriptionDeclarationCounters {
        &self.counters
    }
}
