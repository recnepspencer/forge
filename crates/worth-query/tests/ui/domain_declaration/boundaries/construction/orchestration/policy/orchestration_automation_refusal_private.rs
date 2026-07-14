use worth_query::facade::foundation::{WorthQueryDeclarationEntryOrchestrationAutomationBoundary, WorthQueryDeclarationEntryOrchestrationAutomationRefusal, WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass, WorthQueryDeclarationEntryOrchestrationStage};

fn main() {
    let _ = WorthQueryDeclarationEntryOrchestrationAutomationRefusal {
        refusal_class:
            WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
        stop_stage: WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        reason: "Worthd",
        declaration_family_key: "Worthd.family",
        retained_digest: None,
        orchestration_identity_digest: "Worthd".to_string(),
        automation_boundary:
            WorthQueryDeclarationEntryOrchestrationAutomationBoundary::EnvelopeCeiling,
    };
}
