use super::WorthQueryWorkflowStageMaterialParts;

pub(super) enum WorthQueryWorkflowAdvanceStep {
    Advanced,
    Deferred(Vec<crate::domain_installation::WorthQueryConditionalProvenance>),
}

pub(super) enum WorthQueryStageConditionAdmission {
    Admitted(Vec<crate::domain_installation::WorthQueryConditionalProvenance>),
    Deferred(Vec<crate::domain_installation::WorthQueryConditionalProvenance>),
}

pub(super) struct WorthQueryExecutedWorkflowStage {
    pub(super) predecessor_receipt_identities: Vec<String>,
    pub(super) material: WorthQueryWorkflowStageMaterialParts,
    pub(super) effect_workflow_binding: crate::workflow::WorkflowContextBinding,
}
