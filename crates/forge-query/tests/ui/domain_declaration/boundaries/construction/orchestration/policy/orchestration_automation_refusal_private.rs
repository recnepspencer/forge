use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage,
};

fn main() {
    let _ = ForgeQueryDeclarationEntryOrchestrationAutomationRefusal {
        refusal_class:
            ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
        stop_stage: ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        reason: "forged",
        declaration_family_key: "forged.family",
        retained_digest: None,
        orchestration_identity_digest: "forged".to_string(),
        automation_boundary:
            ForgeQueryDeclarationEntryOrchestrationAutomationBoundary::EnvelopeCeiling,
    };
}
