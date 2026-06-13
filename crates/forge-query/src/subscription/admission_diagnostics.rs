use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionAdmissionDiagnosticStage {
    RuntimeBackedAdmission,
    AdmissionBudget,
    DurableReloadOverclaim,
    ActiveLifecycleAllocation,
    ActivationReadiness,
}

impl QuerySubscriptionAdmissionDiagnosticStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeBackedAdmission => "runtime_backed_admission",
            Self::AdmissionBudget => "admission_budget",
            Self::DurableReloadOverclaim => "durable_reload_overclaim",
            Self::ActiveLifecycleAllocation => "active_lifecycle_allocation",
            Self::ActivationReadiness => "activation_readiness",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionAdmissionDiagnosticOutcome {
    Admitted,
    Denied,
}

impl QuerySubscriptionAdmissionDiagnosticOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionAdmissionDiagnostics {
    stage: QuerySubscriptionAdmissionDiagnosticStage,
    outcome: QuerySubscriptionAdmissionDiagnosticOutcome,
    reason: String,
    diagnostics_identity: ForgeQueryEvidenceIdentity,
}

impl QuerySubscriptionAdmissionDiagnostics {
    pub(super) fn new(
        stage: QuerySubscriptionAdmissionDiagnosticStage,
        outcome: QuerySubscriptionAdmissionDiagnosticOutcome,
        reason: impl Into<String>,
        source_for_reporting: &str,
    ) -> Self {
        let reason = reason.into();
        let diagnostics_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_admission_diagnostics_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("stage"), stage.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("outcome"), outcome.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("reason"), &reason)
        .field_shape(ForgeQueryEvidenceTag::new("source"), source_for_reporting)
        .seal();
        Self {
            stage,
            outcome,
            reason,
            diagnostics_identity,
        }
    }

    pub fn stage(&self) -> &QuerySubscriptionAdmissionDiagnosticStage {
        &self.stage
    }

    pub fn outcome(&self) -> &QuerySubscriptionAdmissionDiagnosticOutcome {
        &self.outcome
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn diagnostics_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.diagnostics_identity
    }

    pub fn digest(&self) -> &str {
        self.diagnostics_identity.as_str()
    }
}
