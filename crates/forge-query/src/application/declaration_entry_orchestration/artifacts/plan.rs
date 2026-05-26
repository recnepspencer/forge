use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::identity::hash_parts;

use super::super::sequencing::{
    envelope_ceiling_automation_steps, ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    ForgeQueryDeclarationEntryOrchestrationAutomationStep,
};
use super::input::ForgeQueryDeclarationEntryOrchestrationInput;
use super::step_record::ForgeQueryDeclarationEntryOrchestrationStage;

pub struct ForgeQueryDeclarationEntryOrchestrationPlan<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    input: ForgeQueryDeclarationEntryOrchestrationInput<D, I>,
    ceiling_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    step_plan: Vec<ForgeQueryDeclarationEntryOrchestrationStage>,
    automation_boundary: ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    automation_steps: Vec<ForgeQueryDeclarationEntryOrchestrationAutomationStep>,
    explicit_caller_handoff_steps: Vec<ForgeQueryDeclarationEntryOrchestrationAutomationStep>,
    orchestration_identity_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryOrchestrationPlan<D, I>
{
    pub(crate) fn new(input: ForgeQueryDeclarationEntryOrchestrationInput<D, I>) -> Self {
        let automation_boundary =
            ForgeQueryDeclarationEntryOrchestrationAutomationBoundary::EnvelopeCeiling;
        let automation_steps = envelope_ceiling_automation_steps();
        let explicit_caller_handoff_steps = Vec::new();
        let step_plan = vec![
            ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        ];
        let orchestration_identity_digest = hash_parts(&[
            format!("family:{}", input.declaration_family_key()),
            format!("handle:{}", input.handle_identity_digest()),
            format!(
                "operating_context:{}",
                input.operating_context_identity_digest()
            ),
            "ceiling:envelope_constructed".to_string(),
            format!(
                "steps:{}",
                step_plan
                    .iter()
                    .map(|stage| stage.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            format!("automation_boundary:{}", automation_boundary.as_str()),
            format!(
                "automation_steps:{}",
                automation_steps
                    .iter()
                    .map(|step| step.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            ),
        ]);
        Self {
            input,
            ceiling_stage: ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            step_plan,
            automation_boundary,
            automation_steps,
            explicit_caller_handoff_steps,
            orchestration_identity_digest,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.input.declaration_family_key()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.input.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.input.operating_context_identity_digest()
    }

    pub fn exposure_level(
        &self,
    ) -> super::exposure::ForgeQueryDeclarationEntryOrchestrationExposureLevel {
        self.input.exposure_level()
    }

    pub fn artifact_policy(
        &self,
    ) -> super::policy::ForgeQueryDeclarationEntryOrchestrationArtifactPolicy {
        self.input.artifact_policy()
    }

    pub fn ceiling_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.ceiling_stage
    }

    pub fn step_plan(&self) -> &[ForgeQueryDeclarationEntryOrchestrationStage] {
        &self.step_plan
    }

    pub fn automation_boundary(&self) -> ForgeQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.automation_boundary
    }

    pub fn automation_steps(&self) -> &[ForgeQueryDeclarationEntryOrchestrationAutomationStep] {
        &self.automation_steps
    }

    pub fn explicit_caller_handoff_steps(
        &self,
    ) -> &[ForgeQueryDeclarationEntryOrchestrationAutomationStep] {
        &self.explicit_caller_handoff_steps
    }

    pub fn orchestration_identity_digest(&self) -> &str {
        &self.orchestration_identity_digest
    }
}
