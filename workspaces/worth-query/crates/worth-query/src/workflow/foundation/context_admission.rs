use crate::workflow::WorkflowCounters;

use super::admission_reporting::{
    WorkflowAdmissionError, WorkflowAdmissionFailureClass, WorkflowPredictionDriftOutcome,
};
use super::context_binding::{
    WorkflowBasisFamily, WorkflowContextBinding, WorkflowPreviewEvaluationClass,
};
use super::declaration_model::{
    WorkflowAuthorityTargetFamily, WorkflowDeclarationFamily, WorkflowDeclarationRequest,
};
use crate::preview::PreviewWorkflowFoundationRequest;

pub(super) fn validate_binding_for_request(
    binding: &WorkflowContextBinding,
    request: &WorkflowDeclarationRequest,
) -> Result<(), WorkflowAdmissionError> {
    match binding.basis_family() {
        WorkflowBasisFamily::RuntimePreflight => Ok(()),
        WorkflowBasisFamily::PreviewFoundation => {
            if request.declaration_family() == &WorkflowDeclarationFamily::PostMergeInspectionNarrow
            {
                return Err(WorkflowAdmissionError::new(
                    WorkflowAdmissionFailureClass::UnsupportedWorkflowFamily,
                    "post-merge inspection declarations require authoritative workflow basis, not preview foundation context",
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
                ));
            }
            if binding.preview_evaluation_class() == Some(&WorkflowPreviewEvaluationClass::ReadOnly)
                && !read_only_preview_request_allows_requested_authority(binding, request)
            {
                return Err(WorkflowAdmissionError::new(
                    WorkflowAdmissionFailureClass::PreviewReadOnlyAuthorityRequestForbidden,
                    "read-only preview workflow contexts may only author inspection declarations",
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
                ));
            }
            Ok(())
        }
        WorkflowBasisFamily::PreviewPromotionComparison => {
            if request.declaration_family() == &WorkflowDeclarationFamily::PostMergeInspectionNarrow
            {
                return Err(WorkflowAdmissionError::new(
                    WorkflowAdmissionFailureClass::UnsupportedWorkflowFamily,
                    "post-merge inspection declarations require authoritative workflow basis, not preview comparison context",
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
                ));
            }
            if matches!(
                request.authority_target_family(),
                WorkflowAuthorityTargetFamily::RelationalMutation
                    | WorkflowAuthorityTargetFamily::BridgeWriteback
            ) {
                return Err(WorkflowAdmissionError::new(
                    WorkflowAdmissionFailureClass::ExplicitRebindRequired,
                    "preview promotion comparison contexts require explicit rebind before mutation or writeback intent",
                    WorkflowPredictionDriftOutcome::ExplicitRebindRequired,
                    WorkflowCounters {
                        workflow_declaration_count: 1,
                        workflow_basis_binding_count: 1,
                        workflow_basis_binding_width: 1,
                        workflow_authority_target_check_count: 1,
                        workflow_denial_count: 1,
                        workflow_executor_rediscovery_count: 0,
                        ..WorkflowCounters::default()
                    },
                ));
            }
            Ok(())
        }
        WorkflowBasisFamily::CorrespondenceHistorical => Err(WorkflowAdmissionError::new(
            WorkflowAdmissionFailureClass::UnsupportedBasisFamily,
            "correspondence/historical workflow declarations remain denied in phase 1",
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
        )),
    }
}

fn read_only_preview_request_allows_requested_authority(
    binding: &WorkflowContextBinding,
    request: &WorkflowDeclarationRequest,
) -> bool {
    if request.authority_target_family() == &WorkflowAuthorityTargetFamily::QueryInspection {
        return true;
    }

    request.authority_target_family() == &WorkflowAuthorityTargetFamily::BridgeWriteback
        && request.declaration_family() == &WorkflowDeclarationFamily::WritebackLoweringNarrow
        && binding.preview_request_family()
            == Some(&PreviewWorkflowFoundationRequest::DeferredMutationWriteback)
}
