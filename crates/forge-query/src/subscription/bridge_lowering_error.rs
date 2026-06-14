use super::counters::QuerySubscriptionDeclarationCounters;
use super::diagnostic::{QuerySubscriptionDiagnosticEvidence, QuerySubscriptionDiagnosticStage};
use super::evidence_identities::diagnostic_source_projection_identity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionBridgeLoweringDenialKind {
    BridgeFamilyUnsupported,
    BridgeSliceUnsupported,
    BridgeFallbackUnsupported,
    BasisBindingUnsupported,
    LoweringBudgetExceeded,
}

impl QuerySubscriptionBridgeLoweringDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BridgeFamilyUnsupported => "bridge_family_unsupported",
            Self::BridgeSliceUnsupported => "bridge_slice_unsupported",
            Self::BridgeFallbackUnsupported => "bridge_fallback_unsupported",
            Self::BasisBindingUnsupported => "basis_binding_unsupported",
            Self::LoweringBudgetExceeded => "lowering_budget_exceeded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionBridgeLoweringError {
    denial_kind: QuerySubscriptionBridgeLoweringDenialKind,
    message: String,
    diagnostic: QuerySubscriptionDiagnosticEvidence,
    counters: QuerySubscriptionDeclarationCounters,
}

impl QuerySubscriptionBridgeLoweringError {
    pub(super) fn new(
        denial_kind: QuerySubscriptionBridgeLoweringDenialKind,
        message: impl Into<String>,
        diagnostic_stage: QuerySubscriptionDiagnosticStage,
        source_digest: impl Into<String>,
        counters: QuerySubscriptionDeclarationCounters,
    ) -> Self {
        let message = message.into();
        let source_label = source_digest.into();
        let source_identity = diagnostic_source_projection_identity(&source_label);
        let diagnostic = QuerySubscriptionDiagnosticEvidence::denied(
            diagnostic_stage,
            message.clone(),
            &source_identity,
            &counters.evidence_identity(),
        );
        Self {
            denial_kind,
            message,
            diagnostic,
            counters,
        }
    }

    pub fn denial_kind(&self) -> &QuerySubscriptionBridgeLoweringDenialKind {
        &self.denial_kind
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
