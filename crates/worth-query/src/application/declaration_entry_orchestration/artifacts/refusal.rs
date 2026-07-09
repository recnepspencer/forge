use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::super::sequencing::{
    WorthQueryDeclarationEntryOrchestrationAutomationBoundary,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusal,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::step_record::WorthQueryDeclarationEntryOrchestrationStage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationRefusalClass {
    UnsupportedAutomation,
    ExplicitIntentRequired,
    StrongerProofRequired,
    AuthorityTransitionRequired,
    ExpensiveWorkNotAdmittedByDefault,
    PreparedButNotExecutedContinuation,
}

impl WorthQueryDeclarationEntryOrchestrationRefusalClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedAutomation => "unsupported_automation",
            Self::ExplicitIntentRequired => "explicit_intent_required",
            Self::StrongerProofRequired => "stronger_proof_required",
            Self::AuthorityTransitionRequired => "authority_transition_required",
            Self::ExpensiveWorkNotAdmittedByDefault => "expensive_work_not_admitted_by_default",
            Self::PreparedButNotExecutedContinuation => "prepared_but_not_executed_continuation",
        }
    }
}

pub struct WorthQueryDeclarationEntryOrchestrationRefusal<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    refusal_class: WorthQueryDeclarationEntryOrchestrationRefusalClass,
    stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
    reason: &'static str,
    automation_refusal: WorthQueryDeclarationEntryOrchestrationAutomationRefusal,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEntryOrchestrationRefusal<D, I>
{
    pub(crate) fn from_automation(
        automation_refusal: WorthQueryDeclarationEntryOrchestrationAutomationRefusal,
        stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
    ) -> Self {
        let refusal_class = automation_refusal.refusal_class().broad_refusal_class();
        Self {
            declaration_family_key: automation_refusal.declaration_family_key(),
            refusal_class,
            stop_stage,
            reason: automation_refusal.reason(),
            automation_refusal,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn refusal_class(&self) -> WorthQueryDeclarationEntryOrchestrationRefusalClass {
        self.refusal_class
    }

    pub fn stop_stage(&self) -> WorthQueryDeclarationEntryOrchestrationStage {
        self.stop_stage
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn automation_refusal_class(
        &self,
    ) -> WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass {
        self.automation_refusal.refusal_class()
    }

    pub fn automation_boundary(&self) -> WorthQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.automation_refusal.automation_boundary()
    }

    pub fn retained_digest(&self) -> Option<&str> {
        self.automation_refusal.retained_digest()
    }

    pub fn orchestration_identity_digest(&self) -> &str {
        self.automation_refusal.orchestration_identity_digest()
    }

    pub fn automation_refusal(&self) -> &WorthQueryDeclarationEntryOrchestrationAutomationRefusal {
        &self.automation_refusal
    }
}
