use super::artifacts::{
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationAutomationBoundary {
    EnvelopeCeiling,
}

impl ForgeQueryDeclarationEntryOrchestrationAutomationBoundary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EnvelopeCeiling => "envelope_ceiling",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationAutomationStep {
    AdmittedHandle,
    CanonicalDeclaration,
    Legality,
    Progression,
    FoundationalEvidence,
    RoutePlan,
    Receipt,
    Envelope,
}

impl ForgeQueryDeclarationEntryOrchestrationAutomationStep {
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
pub enum ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass {
    ExplicitIntentRequired,
    ExpensiveAutomationForbidden,
    AuthorityTransitionRequired,
    PreparedButNotExecuted,
    UnsupportedAutomation,
    StrongerProofRequired,
}

impl ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass {
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

    pub(crate) fn broad_refusal_class(self) -> ForgeQueryDeclarationEntryOrchestrationRefusalClass {
        match self {
            Self::ExplicitIntentRequired => {
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired
            }
            Self::ExpensiveAutomationForbidden => {
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault
            }
            Self::AuthorityTransitionRequired => {
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::AuthorityTransitionRequired
            }
            Self::PreparedButNotExecuted => {
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation
            }
            Self::UnsupportedAutomation => {
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation
            }
            Self::StrongerProofRequired => {
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::StrongerProofRequired
            }
        }
    }
}

pub struct ForgeQueryDeclarationEntryOrchestrationAutomationRefusal {
    refusal_class: ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    reason: &'static str,
    declaration_family_key: &'static str,
    retained_digest: Option<String>,
    orchestration_identity_digest: String,
    automation_boundary: ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
}

impl ForgeQueryDeclarationEntryOrchestrationAutomationRefusal {
    pub(crate) fn new(
        refusal_class: ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
        stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        reason: &'static str,
        declaration_family_key: &'static str,
        retained_digest: Option<String>,
        orchestration_identity_digest: &str,
        automation_boundary: ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
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

    pub fn refusal_class(&self) -> ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass {
        self.refusal_class
    }

    pub fn stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
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

    pub fn automation_boundary(&self) -> ForgeQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.automation_boundary
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryDeclarationEntryOrchestrationAutomationContext<'a> {
    orchestration_identity_digest: &'a str,
    automation_boundary: ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
}

impl<'a> ForgeQueryDeclarationEntryOrchestrationAutomationContext<'a> {
    pub(crate) fn new(
        orchestration_identity_digest: &'a str,
        automation_boundary: ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
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
    ) -> ForgeQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.automation_boundary
    }
}

#[cfg(test)]
pub(crate) struct ForgeQueryDeclarationEntryOrchestrationAutomationParityReceipt {
    explicit_outcome_identity_digest: String,
    orchestrated_outcome_identity_digest: String,
    explicit_stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    orchestrated_stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    explicit_farthest_crossed_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    orchestrated_farthest_crossed_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    parity_holds: bool,
}

#[cfg(test)]
impl ForgeQueryDeclarationEntryOrchestrationAutomationParityReceipt {
    pub(crate) fn new(
        explicit_outcome_identity_digest: String,
        orchestrated_outcome_identity_digest: String,
        explicit_stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        orchestrated_stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        explicit_farthest_crossed_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        orchestrated_farthest_crossed_stage: ForgeQueryDeclarationEntryOrchestrationStage,
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

    pub(crate) fn explicit_stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.explicit_stop_stage
    }

    pub(crate) fn orchestrated_stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.orchestrated_stop_stage
    }

    pub(crate) fn explicit_farthest_crossed_stage(
        &self,
    ) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.explicit_farthest_crossed_stage
    }

    pub(crate) fn orchestrated_farthest_crossed_stage(
        &self,
    ) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.orchestrated_farthest_crossed_stage
    }

    pub(crate) fn parity_holds(&self) -> bool {
        self.parity_holds
    }
}

pub(crate) fn envelope_ceiling_automation_steps(
) -> Vec<ForgeQueryDeclarationEntryOrchestrationAutomationStep> {
    vec![
        ForgeQueryDeclarationEntryOrchestrationAutomationStep::AdmittedHandle,
        ForgeQueryDeclarationEntryOrchestrationAutomationStep::CanonicalDeclaration,
        ForgeQueryDeclarationEntryOrchestrationAutomationStep::Legality,
        ForgeQueryDeclarationEntryOrchestrationAutomationStep::Progression,
        ForgeQueryDeclarationEntryOrchestrationAutomationStep::FoundationalEvidence,
        ForgeQueryDeclarationEntryOrchestrationAutomationStep::RoutePlan,
        ForgeQueryDeclarationEntryOrchestrationAutomationStep::Receipt,
        ForgeQueryDeclarationEntryOrchestrationAutomationStep::Envelope,
    ]
}

pub(crate) fn automation_step_for_stage(
    stage: ForgeQueryDeclarationEntryOrchestrationStage,
) -> ForgeQueryDeclarationEntryOrchestrationAutomationStep {
    match stage {
        ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::AdmittedHandle
        }
        ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::CanonicalDeclaration
        }
        ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Legality
        }
        ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Progression
        }
        ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::FoundationalEvidence
        }
        ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::RoutePlan
        }
        ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Receipt
        }
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Envelope
        }
    }
}
