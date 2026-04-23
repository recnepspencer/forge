use crate::identity::hash_parts;

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
    digest: String,
}

impl QuerySubscriptionAdmissionDiagnostics {
    pub(super) fn new(
        stage: QuerySubscriptionAdmissionDiagnosticStage,
        outcome: QuerySubscriptionAdmissionDiagnosticOutcome,
        reason: impl Into<String>,
        source_digest: &str,
    ) -> Self {
        let reason = reason.into();
        let digest = hash_parts(&[
            "query_subscription_admission_diagnostics_v1".to_string(),
            stage.as_str().to_string(),
            outcome.as_str().to_string(),
            reason.clone(),
            source_digest.to_string(),
        ]);
        Self {
            stage,
            outcome,
            reason,
            digest,
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

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
