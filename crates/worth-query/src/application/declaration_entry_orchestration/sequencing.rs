use super::artifacts::{
    WorthQueryDeclarationEntryOrchestrationRefusalClass,
    WorthQueryDeclarationEntryOrchestrationStage,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationAutomationBoundary {
    EnvelopeCeiling,
}

impl WorthQueryDeclarationEntryOrchestrationAutomationBoundary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EnvelopeCeiling => "envelope_ceiling",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationAutomationStep {
    AdmittedHandle,
    CanonicalDeclaration,
    Legality,
    Progression,
    FoundationalEvidence,
    RoutePlan,
    Receipt,
    Envelope,
}

impl WorthQueryDeclarationEntryOrchestrationAutomationStep {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedHandle => "admitted_handle",
            Self::CanonicalDeclaration => "canonical_declaration",
            Self::Legality => "legality",
            Self::Progression => "progression",
            Self::FoundationalEvidence => "foundational_evidence",
            Self::RoutePlan => "route_plan",
            Self::Receipt => "receipt",
            Self::Envelope => "envelope",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass {
    ExplicitIntentRequired,
    ExpensiveAutomationForbidden,
    AuthorityTransitionRequired,
    PreparedButNotExecuted,
    UnsupportedAutomation,
    StrongerProofRequired,
}

impl WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitIntentRequired => "explicit_intent_required",
            Self::ExpensiveAutomationForbidden => "expensive_automation_forbidden",
            Self::AuthorityTransitionRequired => "authority_transition_required",
            Self::PreparedButNotExecuted => "prepared_but_not_executed",
            Self::UnsupportedAutomation => "unsupported_automation",
            Self::StrongerProofRequired => "stronger_proof_required",
        }
    }

    pub(crate) fn broad_refusal_class(self) -> WorthQueryDeclarationEntryOrchestrationRefusalClass {
        match self {
            Self::ExplicitIntentRequired => {
                WorthQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired
            }
            Self::ExpensiveAutomationForbidden => {
                WorthQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault
            }
            Self::AuthorityTransitionRequired => {
                WorthQueryDeclarationEntryOrchestrationRefusalClass::AuthorityTransitionRequired
            }
            Self::PreparedButNotExecuted => {
                WorthQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation
            }
            Self::UnsupportedAutomation => {
                WorthQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation
            }
            Self::StrongerProofRequired => {
                WorthQueryDeclarationEntryOrchestrationRefusalClass::StrongerProofRequired
            }
        }
    }
}

pub struct WorthQueryDeclarationEntryOrchestrationAutomationRefusal {
    refusal_class: WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass,
    stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
    reason: &'static str,
    declaration_family_key: &'static str,
    retained_digest: Option<String>,
    orchestration_identity_digest: String,
    automation_boundary: WorthQueryDeclarationEntryOrchestrationAutomationBoundary,
}

impl WorthQueryDeclarationEntryOrchestrationAutomationRefusal {
    pub(crate) fn new(
        refusal_class: WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass,
        stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
        reason: &'static str,
        declaration_family_key: &'static str,
        retained_digest: Option<String>,
        orchestration_identity_digest: &str,
        automation_boundary: WorthQueryDeclarationEntryOrchestrationAutomationBoundary,
    ) -> Self {
        Self {
            refusal_class,
            stop_stage,
            reason,
            declaration_family_key,
            retained_digest,
            orchestration_identity_digest: orchestration_identity_digest.to_string(),
            automation_boundary,
        }
    }

    pub fn refusal_class(&self) -> WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass {
        self.refusal_class
    }

    pub fn stop_stage(&self) -> WorthQueryDeclarationEntryOrchestrationStage {
        self.stop_stage
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn retained_digest(&self) -> Option<&str> {
        self.retained_digest.as_deref()
    }

    pub fn orchestration_identity_digest(&self) -> &str {
        &self.orchestration_identity_digest
    }

    pub fn automation_boundary(&self) -> WorthQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.automation_boundary
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryDeclarationEntryOrchestrationAutomationContext<'a> {
    orchestration_identity_digest: &'a str,
    automation_boundary: WorthQueryDeclarationEntryOrchestrationAutomationBoundary,
}

impl<'a> WorthQueryDeclarationEntryOrchestrationAutomationContext<'a> {
    pub(crate) fn new(
        orchestration_identity_digest: &'a str,
        automation_boundary: WorthQueryDeclarationEntryOrchestrationAutomationBoundary,
    ) -> Self {
        Self {
            orchestration_identity_digest,
            automation_boundary,
        }
    }

    pub(crate) fn orchestration_identity_digest(&self) -> &'a str {
        self.orchestration_identity_digest
    }

    pub(crate) fn automation_boundary(
        &self,
    ) -> WorthQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.automation_boundary
    }
}

#[cfg(test)]
pub(crate) struct WorthQueryDeclarationEntryOrchestrationAutomationParityReceipt {
    explicit_outcome_identity_digest: String,
    orchestrated_outcome_identity_digest: String,
    explicit_stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
    orchestrated_stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
    explicit_farthest_crossed_stage: WorthQueryDeclarationEntryOrchestrationStage,
    orchestrated_farthest_crossed_stage: WorthQueryDeclarationEntryOrchestrationStage,
    parity_holds: bool,
}

#[cfg(test)]
impl WorthQueryDeclarationEntryOrchestrationAutomationParityReceipt {
    pub(crate) fn new(
        explicit_outcome_identity_digest: String,
        orchestrated_outcome_identity_digest: String,
        explicit_stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
        orchestrated_stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
        explicit_farthest_crossed_stage: WorthQueryDeclarationEntryOrchestrationStage,
        orchestrated_farthest_crossed_stage: WorthQueryDeclarationEntryOrchestrationStage,
    ) -> Self {
        let parity_holds = explicit_outcome_identity_digest == orchestrated_outcome_identity_digest
            && explicit_stop_stage == orchestrated_stop_stage
            && explicit_farthest_crossed_stage == orchestrated_farthest_crossed_stage;
        Self {
            explicit_outcome_identity_digest,
            orchestrated_outcome_identity_digest,
            explicit_stop_stage,
            orchestrated_stop_stage,
            explicit_farthest_crossed_stage,
            orchestrated_farthest_crossed_stage,
            parity_holds,
        }
    }

    pub(crate) fn explicit_outcome_identity_digest(&self) -> &str {
        &self.explicit_outcome_identity_digest
    }

    pub(crate) fn orchestrated_outcome_identity_digest(&self) -> &str {
        &self.orchestrated_outcome_identity_digest
    }

    pub(crate) fn explicit_stop_stage(&self) -> WorthQueryDeclarationEntryOrchestrationStage {
        self.explicit_stop_stage
    }

    pub(crate) fn orchestrated_stop_stage(&self) -> WorthQueryDeclarationEntryOrchestrationStage {
        self.orchestrated_stop_stage
    }

    pub(crate) fn explicit_farthest_crossed_stage(
        &self,
    ) -> WorthQueryDeclarationEntryOrchestrationStage {
        self.explicit_farthest_crossed_stage
    }

    pub(crate) fn orchestrated_farthest_crossed_stage(
        &self,
    ) -> WorthQueryDeclarationEntryOrchestrationStage {
        self.orchestrated_farthest_crossed_stage
    }

    pub(crate) fn parity_holds(&self) -> bool {
        self.parity_holds
    }
}

pub(crate) fn envelope_ceiling_automation_steps(
) -> Vec<WorthQueryDeclarationEntryOrchestrationAutomationStep> {
    vec![
        WorthQueryDeclarationEntryOrchestrationAutomationStep::AdmittedHandle,
        WorthQueryDeclarationEntryOrchestrationAutomationStep::CanonicalDeclaration,
        WorthQueryDeclarationEntryOrchestrationAutomationStep::Legality,
        WorthQueryDeclarationEntryOrchestrationAutomationStep::Progression,
        WorthQueryDeclarationEntryOrchestrationAutomationStep::FoundationalEvidence,
        WorthQueryDeclarationEntryOrchestrationAutomationStep::RoutePlan,
        WorthQueryDeclarationEntryOrchestrationAutomationStep::Receipt,
        WorthQueryDeclarationEntryOrchestrationAutomationStep::Envelope,
    ]
}

pub(crate) fn automation_step_for_stage(
    stage: WorthQueryDeclarationEntryOrchestrationStage,
) -> WorthQueryDeclarationEntryOrchestrationAutomationStep {
    match stage {
        WorthQueryDeclarationEntryOrchestrationStage::AdmittedHandle => {
            WorthQueryDeclarationEntryOrchestrationAutomationStep::AdmittedHandle
        }
        WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed => {
            WorthQueryDeclarationEntryOrchestrationAutomationStep::CanonicalDeclaration
        }
        WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished => {
            WorthQueryDeclarationEntryOrchestrationAutomationStep::Legality
        }
        WorthQueryDeclarationEntryOrchestrationStage::ProgressionAdmitted
        | WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved => {
            WorthQueryDeclarationEntryOrchestrationAutomationStep::Progression
        }
        WorthQueryDeclarationEntryOrchestrationStage::FoundationalDescribed => {
            WorthQueryDeclarationEntryOrchestrationAutomationStep::FoundationalEvidence
        }
        WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned => {
            WorthQueryDeclarationEntryOrchestrationAutomationStep::RoutePlan
        }
        WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued => {
            WorthQueryDeclarationEntryOrchestrationAutomationStep::Receipt
        }
        WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed => {
            WorthQueryDeclarationEntryOrchestrationAutomationStep::Envelope
        }
    }
}
