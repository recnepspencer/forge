use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::super::evidence_identities::diagnostic_evidence_identity;

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
    source_for_reporting: String,
    counter_for_reporting: String,
    evidence_identity: ForgeQueryEvidenceIdentity,
}

impl QuerySubscriptionDiagnosticEvidence {
    pub(crate) fn admitted(
        stage: QuerySubscriptionDiagnosticStage,
        reason: impl Into<String>,
        source_identity: &ForgeQueryEvidenceIdentity,
        counter_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::new(
            stage,
            QuerySubscriptionDiagnosticOutcome::Admitted,
            reason,
            source_identity,
            counter_identity,
        )
    }

    pub(crate) fn denied(
        stage: QuerySubscriptionDiagnosticStage,
        reason: impl Into<String>,
        source_identity: &ForgeQueryEvidenceIdentity,
        counter_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::new(
            stage,
            QuerySubscriptionDiagnosticOutcome::Denied,
            reason,
            source_identity,
            counter_identity,
        )
    }

    fn new(
        stage: QuerySubscriptionDiagnosticStage,
        outcome: QuerySubscriptionDiagnosticOutcome,
        reason: impl Into<String>,
        source_identity: &ForgeQueryEvidenceIdentity,
        counter_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        let reason = reason.into();
        let evidence_identity = diagnostic_evidence_identity(
            stage.as_str(),
            outcome.as_str(),
            &reason,
            source_identity,
            counter_identity,
        );
        Self {
            stage,
            outcome,
            reason,
            source_for_reporting: source_identity.as_str().to_string(),
            counter_for_reporting: counter_identity.as_str().to_string(),
            evidence_identity,
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
        &self.source_for_reporting
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_for_reporting
    }

    pub fn digest(&self) -> &str {
        self.evidence_for_reporting()
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.evidence_identity
    }

    pub fn evidence_for_reporting(&self) -> &str {
        self.evidence_identity.as_str()
    }
}
