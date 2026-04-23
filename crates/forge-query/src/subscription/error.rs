use super::counters::QuerySubscriptionDeclarationCounters;
use super::diagnostic::{QuerySubscriptionDiagnosticEvidence, QuerySubscriptionDiagnosticStage};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionFamilySelectionFailureClass {
    ViewFamilyLiveFamilyMismatch,
    InvalidAdmissionDimensions,
    WorkBudgetExceeded,
    AllocationBudgetExceeded,
    UnknownSubscriptionCost,
    UnsupportedViewFamily,
    HostObserverInferenceForbidden,
    RawCdcFallbackForbidden,
    GenericSubscriptionKindForbidden,
    RelationshipProofAdmissionDrift,
}

impl QuerySubscriptionFamilySelectionFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ViewFamilyLiveFamilyMismatch => "view_family_live_family_mismatch",
            Self::InvalidAdmissionDimensions => "invalid_admission_dimensions",
            Self::WorkBudgetExceeded => "work_budget_exceeded",
            Self::AllocationBudgetExceeded => "allocation_budget_exceeded",
            Self::UnknownSubscriptionCost => "unknown_subscription_cost",
            Self::UnsupportedViewFamily => "unsupported_view_family",
            Self::HostObserverInferenceForbidden => "host_observer_inference_forbidden",
            Self::RawCdcFallbackForbidden => "raw_cdc_fallback_forbidden",
            Self::GenericSubscriptionKindForbidden => "generic_subscription_kind_forbidden",
            Self::RelationshipProofAdmissionDrift => "relationship_proof_admission_drift",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionFamilySelectionError {
    failure_class: QuerySubscriptionFamilySelectionFailureClass,
    message: String,
    diagnostic: QuerySubscriptionDiagnosticEvidence,
    counters: QuerySubscriptionDeclarationCounters,
}

impl QuerySubscriptionFamilySelectionError {
    pub(super) fn new(
        failure_class: QuerySubscriptionFamilySelectionFailureClass,
        message: impl Into<String>,
        diagnostic_stage: QuerySubscriptionDiagnosticStage,
        source_digest: impl Into<String>,
        counters: QuerySubscriptionDeclarationCounters,
    ) -> Self {
        let message = message.into();
        let diagnostic = QuerySubscriptionDiagnosticEvidence::denied(
            diagnostic_stage,
            message.clone(),
            source_digest,
            counters.digest(),
        );
        Self {
            failure_class,
            message,
            diagnostic,
            counters,
        }
    }

    pub fn failure_class(&self) -> &QuerySubscriptionFamilySelectionFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn diagnostic(&self) -> &QuerySubscriptionDiagnosticEvidence {
        &self.diagnostic
    }

    pub fn counters(&self) -> &QuerySubscriptionDeclarationCounters {
        &self.counters
    }
}
