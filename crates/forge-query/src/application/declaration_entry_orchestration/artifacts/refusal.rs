use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::super::sequencing::{
    ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::step_record::ForgeQueryDeclarationEntryOrchestrationStage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationRefusalClass {
    UnsupportedAutomation,
    ExplicitIntentRequired,
    StrongerProofRequired,
    AuthorityTransitionRequired,
    ExpensiveWorkNotAdmittedByDefault,
    PreparedButNotExecutedContinuation,
}

impl ForgeQueryDeclarationEntryOrchestrationRefusalClass {
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

pub struct ForgeQueryDeclarationEntryOrchestrationRefusal<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    refusal_class: ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    reason: &'static str,
    automation_refusal: ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryOrchestrationRefusal<D, I>
{
    pub(crate) fn from_automation(
        automation_refusal: ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
        stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
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

    pub fn refusal_class(&self) -> ForgeQueryDeclarationEntryOrchestrationRefusalClass {
        self.refusal_class
    }

    pub fn stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.stop_stage
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn automation_refusal_class(
        &self,
    ) -> ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass {
        self.automation_refusal.refusal_class()
    }

    pub fn automation_boundary(&self) -> ForgeQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.automation_refusal.automation_boundary()
    }

    pub fn retained_digest(&self) -> Option<&str> {
        self.automation_refusal.retained_digest()
    }

    pub fn orchestration_identity_digest(&self) -> &str {
        self.automation_refusal.orchestration_identity_digest()
    }

    pub fn automation_refusal(&self) -> &ForgeQueryDeclarationEntryOrchestrationAutomationRefusal {
        &self.automation_refusal
    }
}
