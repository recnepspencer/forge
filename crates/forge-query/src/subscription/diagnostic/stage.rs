use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionDiagnosticStage {
    FamilySelection,
    Declaration,
    BridgeFamilyLowering,
    BridgeSliceLowering,
    BasisBinding,
    AdmissionBudget,
    DurableReloadOverclaim,
    ActiveLifecycleAllocation,
    RuntimeBackedAdmission,
    ActivationReadiness,
    SupportReporting,
    Continuation,
    PreviewIsolation,
    LifecycleCloseout,
    ViewMismatch,
    RelationshipProofDrift,
    DeliveryIntent,
    Certification,
}

impl QuerySubscriptionDiagnosticStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FamilySelection => "family_selection",
            Self::Declaration => "declaration",
            Self::BridgeFamilyLowering => "bridge_family_lowering",
            Self::BridgeSliceLowering => "bridge_slice_lowering",
            Self::BasisBinding => "basis_binding",
            Self::AdmissionBudget => "admission_budget",
            Self::DurableReloadOverclaim => "durable_reload_overclaim",
            Self::ActiveLifecycleAllocation => "active_lifecycle_allocation",
            Self::RuntimeBackedAdmission => "runtime_backed_admission",
            Self::ActivationReadiness => "activation_readiness",
            Self::SupportReporting => "support_reporting",
            Self::Continuation => "continuation",
            Self::PreviewIsolation => "preview_isolation",
            Self::LifecycleCloseout => "lifecycle_closeout",
            Self::ViewMismatch => "view_mismatch",
            Self::RelationshipProofDrift => "relationship_proof_drift",
            Self::DeliveryIntent => "delivery_intent",
            Self::Certification => "certification",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionDiagnosticOutcome {
    Admitted,
    Denied,
}

impl QuerySubscriptionDiagnosticOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticEvidence {
    stage: QuerySubscriptionDiagnosticStage,
    outcome: QuerySubscriptionDiagnosticOutcome,
    reason: String,
    source_digest: String,
    counter_digest: String,
    digest: String,
}

impl QuerySubscriptionDiagnosticEvidence {
    pub(crate) fn admitted(
        stage: QuerySubscriptionDiagnosticStage,
        reason: impl Into<String>,
        source_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Self {
        Self::new(
            stage,
            QuerySubscriptionDiagnosticOutcome::Admitted,
            reason,
            source_digest,
            counter_digest,
        )
    }

    pub(crate) fn denied(
        stage: QuerySubscriptionDiagnosticStage,
        reason: impl Into<String>,
        source_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Self {
        Self::new(
            stage,
            QuerySubscriptionDiagnosticOutcome::Denied,
            reason,
            source_digest,
            counter_digest,
        )
    }

    fn new(
        stage: QuerySubscriptionDiagnosticStage,
        outcome: QuerySubscriptionDiagnosticOutcome,
        reason: impl Into<String>,
        source_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        let source_digest = source_digest.into();
        let counter_digest = counter_digest.into();
        let digest = hash_parts(&[
            "query_subscription_diagnostic_evidence_v1".to_string(),
            format!("stage:{}", stage.as_str()),
            format!("outcome:{}", outcome.as_str()),
            format!("reason:{reason}"),
            format!("source:{source_digest}"),
            format!("counters:{counter_digest}"),
        ]);
        Self {
            stage,
            outcome,
            reason,
            source_digest,
            counter_digest,
            digest,
        }
    }

    pub fn stage(&self) -> &QuerySubscriptionDiagnosticStage {
        &self.stage
    }

    pub fn outcome(&self) -> &QuerySubscriptionDiagnosticOutcome {
        &self.outcome
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
