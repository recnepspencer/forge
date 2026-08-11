use crate::workflow::WorkflowCounters;

use super::admission_reporting::{
    WorkflowAdmissionError, WorkflowAdmissionFailureClass, WorkflowPredictionDriftOutcome,
};
use super::declaration_model::{WorkflowAuthorityTargetFamily, WorkflowDeclarationFamily};

pub(super) fn validate_target_for_family(
    family: &WorkflowDeclarationFamily,
    target: &WorkflowAuthorityTargetFamily,
) -> Result<(), WorkflowAdmissionError> {
    let supported = matches!(
        (family, target),
        (
            WorkflowDeclarationFamily::ConflictInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection
        ) | (
            WorkflowDeclarationFamily::MutationLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMutation
        ) | (
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge
        ) | (
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback
        ) | (
            WorkflowDeclarationFamily::PostMergeInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection
        )
    );
    if supported {
        Ok(())
    } else {
        Err(WorkflowAdmissionError::new(
            WorkflowAdmissionFailureClass::UnsupportedAuthorityTargetFamily,
            "workflow declaration family and authority target family must match exactly",
            WorkflowPredictionDriftOutcome::WithinBudget,
            WorkflowCounters {
                workflow_declaration_count: 1,
                workflow_basis_binding_count: 1,
                workflow_basis_binding_width: 1,
                workflow_authority_target_check_count: 1,
                workflow_denial_count: 1,
                workflow_executor_rediscovery_count: 0,
                ..WorkflowCounters::default()
            },
        ))
    }
}
