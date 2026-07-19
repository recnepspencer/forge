use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use super::product::WorthQueryDeclarationEntryOrchestrationProduct;
use super::step_record::WorthQueryDeclarationEntryOrchestrationStage;
use crate::application::declaration_entry_orchestration::materialization::{
    WorthQueryDeclarationEntryOrchestrationMaterializationPolicy,
    WorthQueryDeclarationEntryOrchestrationMaterializationTier,
};
use crate::application::declaration_entry_orchestration::sequencing::{
    envelope_ceiling_automation_steps, WorthQueryDeclarationEntryOrchestrationAutomationStep,
};

pub(super) fn step_plan_for(
    product: WorthQueryDeclarationEntryOrchestrationProduct,
    starting_artifact_stage: WorthQueryDeclarationEntryOrchestrationStage,
) -> Vec<WorthQueryDeclarationEntryOrchestrationStage> {
    let full = match product {
        WorthQueryDeclarationEntryOrchestrationProduct::RoutePlan => vec![
            progression_starting_stage(starting_artifact_stage),
            WorthQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        ],
        WorthQueryDeclarationEntryOrchestrationProduct::Receipt => vec![
            progression_starting_stage(starting_artifact_stage),
            WorthQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
            WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
        ],
        WorthQueryDeclarationEntryOrchestrationProduct::Envelope => {
            envelope_step_plan(starting_artifact_stage)
        }
    };
    full.into_iter()
        .skip_while(|stage| *stage != starting_artifact_stage)
        .collect()
}

pub(super) fn automation_steps_for(
    step_plan: &[WorthQueryDeclarationEntryOrchestrationStage],
) -> Vec<WorthQueryDeclarationEntryOrchestrationAutomationStep> {
    let full = envelope_ceiling_automation_steps();
    let first_step = step_plan.first().map(stage_to_automation_step);
    full.into_iter()
        .skip_while(|step| Some(*step) != first_step)
        .collect()
}

pub(super) fn explicit_caller_handoff_steps_for(
    product: WorthQueryDeclarationEntryOrchestrationProduct,
) -> Vec<WorthQueryDeclarationEntryOrchestrationAutomationStep> {
    match product {
        WorthQueryDeclarationEntryOrchestrationProduct::RoutePlan => vec![
            WorthQueryDeclarationEntryOrchestrationAutomationStep::Receipt,
            WorthQueryDeclarationEntryOrchestrationAutomationStep::Envelope,
        ],
        WorthQueryDeclarationEntryOrchestrationProduct::Receipt => {
            vec![WorthQueryDeclarationEntryOrchestrationAutomationStep::Envelope]
        }
        WorthQueryDeclarationEntryOrchestrationProduct::Envelope => Vec::new(),
    }
}

pub(super) fn materialization_tier_for_product(
    policy: &WorthQueryDeclarationEntryOrchestrationMaterializationPolicy,
    product: WorthQueryDeclarationEntryOrchestrationProduct,
) -> WorthQueryDeclarationEntryOrchestrationMaterializationTier {
    match product {
        WorthQueryDeclarationEntryOrchestrationProduct::RoutePlan => {
            WorthQueryDeclarationEntryOrchestrationMaterializationTier::from(
                policy.foundational_evidence_profile(),
            )
        }
        WorthQueryDeclarationEntryOrchestrationProduct::Receipt => policy.receipt_tier(),
        WorthQueryDeclarationEntryOrchestrationProduct::Envelope => policy.envelope_tier(),
    }
}

impl From<FoundationalBoundaryEvidenceMaterializationProfile>
    for WorthQueryDeclarationEntryOrchestrationMaterializationTier
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
    starting_artifact_stage: WorthQueryDeclarationEntryOrchestrationStage,
) -> Vec<WorthQueryDeclarationEntryOrchestrationStage> {
    if starting_artifact_stage == WorthQueryDeclarationEntryOrchestrationStage::AdmittedHandle {
        vec![
            WorthQueryDeclarationEntryOrchestrationStage::AdmittedHandle,
            WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
            WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            WorthQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
            WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
            WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        ]
    } else {
        vec![
            progression_starting_stage(starting_artifact_stage),
            WorthQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
            WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
            WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        ]
    }
}

fn stage_to_automation_step(
    stage: &WorthQueryDeclarationEntryOrchestrationStage,
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

fn progression_starting_stage(
    starting_artifact_stage: WorthQueryDeclarationEntryOrchestrationStage,
) -> WorthQueryDeclarationEntryOrchestrationStage {
    match starting_artifact_stage {
        WorthQueryDeclarationEntryOrchestrationStage::ProgressionAdmitted => {
            WorthQueryDeclarationEntryOrchestrationStage::ProgressionAdmitted
        }
        _ => WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
    }
}
