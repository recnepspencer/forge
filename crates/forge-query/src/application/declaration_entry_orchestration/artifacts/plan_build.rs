use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use super::product::ForgeQueryDeclarationEntryOrchestrationProduct;
use super::step_record::ForgeQueryDeclarationEntryOrchestrationStage;
use crate::application::declaration_entry_orchestration::materialization::{
    ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
};
use crate::application::declaration_entry_orchestration::sequencing::{
    envelope_ceiling_automation_steps, ForgeQueryDeclarationEntryOrchestrationAutomationStep,
};

pub(super) fn step_plan_for(
    product: ForgeQueryDeclarationEntryOrchestrationProduct,
    starting_artifact_stage: ForgeQueryDeclarationEntryOrchestrationStage,
) -> Vec<ForgeQueryDeclarationEntryOrchestrationStage> {
    let full = match product {
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan => vec![
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        ],
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt => vec![
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
        ],
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope => {
            envelope_step_plan(starting_artifact_stage)
        }
    };
    full.into_iter()
        .skip_while(|stage| *stage != starting_artifact_stage)
        .collect()
}

pub(super) fn automation_steps_for(
    step_plan: &[ForgeQueryDeclarationEntryOrchestrationStage],
) -> Vec<ForgeQueryDeclarationEntryOrchestrationAutomationStep> {
    let full = envelope_ceiling_automation_steps();
    let first_step = step_plan.first().map(stage_to_automation_step);
    full.into_iter()
        .skip_while(|step| Some(*step) != first_step)
        .collect()
}

pub(super) fn explicit_caller_handoff_steps_for(
    product: ForgeQueryDeclarationEntryOrchestrationProduct,
) -> Vec<ForgeQueryDeclarationEntryOrchestrationAutomationStep> {
    match product {
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan => vec![
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Receipt,
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Envelope,
        ],
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt => {
            vec![ForgeQueryDeclarationEntryOrchestrationAutomationStep::Envelope]
        }
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope => Vec::new(),
    }
}

pub(super) fn materialization_tier_for_product(
    policy: &ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy,
    product: ForgeQueryDeclarationEntryOrchestrationProduct,
) -> ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
    match product {
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan => {
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::from(
                policy.foundational_evidence_profile(),
            )
        }
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt => policy.receipt_tier(),
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope => policy.envelope_tier(),
    }
}

impl From<FoundationalBoundaryEvidenceMaterializationProfile>
    for ForgeQueryDeclarationEntryOrchestrationMaterializationTier
{
    fn from(value: FoundationalBoundaryEvidenceMaterializationProfile) -> Self {
        match value {
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics => {
                Self::OperationalLean
            }
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics => {
                Self::SupportReady
            }
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness => {
                Self::FullDescriptive
            }
        }
    }
}

fn envelope_step_plan(
    starting_artifact_stage: ForgeQueryDeclarationEntryOrchestrationStage,
) -> Vec<ForgeQueryDeclarationEntryOrchestrationStage> {
    if starting_artifact_stage == ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle {
        vec![
            ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        ]
    } else {
        vec![
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        ]
    }
}

fn stage_to_automation_step(
    stage: &ForgeQueryDeclarationEntryOrchestrationStage,
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
