use crate::workflow::WorkflowCounters;

use super::admission_reporting::{
    QueryWorkflowDeclaration, WorkflowAdmissionError, WorkflowAdmissionFailureClass,
    WorkflowAdmissionReport, WorkflowPredictionDriftOutcome,
};
use super::context_admission::validate_binding_for_request;
use super::context_binding::{WorkflowBindingSource, WorkflowContextBinding};
use super::context_identity::workflow_declaration_identity;
use super::declaration_model::{WorkflowBudgetClass, WorkflowDeclarationRequest};
use super::family_admission::validate_target_for_family;
use super::preview_binding::{bind_preview_foundation, bind_preview_promotion_comparison};
use super::runtime_binding::bind_runtime_preflight;

pub(crate) fn bind_workflow_context(
    source: WorkflowBindingSource<'_>,
) -> Result<WorkflowContextBinding, WorkflowAdmissionError> {
    match source {
        WorkflowBindingSource::RuntimePreflight(preflight) => bind_runtime_preflight(preflight),
        WorkflowBindingSource::PreviewFoundation(foundation) => bind_preview_foundation(foundation),
        WorkflowBindingSource::PreviewPromotionComparison(comparison) => {
            bind_preview_promotion_comparison(comparison)
        }
        WorkflowBindingSource::CorrespondenceHistorical(_historical) => {
            Err(WorkflowAdmissionError::new(
                WorkflowAdmissionFailureClass::UnsupportedBasisFamily,
                "correspondence/historical workflow binding remains explicitly denied in phase 1",
                WorkflowPredictionDriftOutcome::WithinBudget,
                WorkflowCounters {
                    workflow_basis_binding_count: 1,
                    workflow_basis_binding_width: 1,
                    workflow_denial_count: 1,
                    ..WorkflowCounters::default()
                },
            ))
        }
    }
}

pub(crate) fn admit_query_workflow_declaration(
    binding: &WorkflowContextBinding,
    request: WorkflowDeclarationRequest,
) -> Result<QueryWorkflowDeclaration, WorkflowAdmissionError> {
    if request.budget_class() == &WorkflowBudgetClass::CrossBoundaryExpansion {
        return Err(WorkflowAdmissionError::new(
            WorkflowAdmissionFailureClass::ForbiddenWorkflowBroadening,
            "workflow declarations that require cross-boundary expansion must deny in phase 1",
            WorkflowPredictionDriftOutcome::ExplicitBroadeningDenied,
            WorkflowCounters {
                workflow_declaration_count: 1,
                workflow_basis_binding_count: 1,
                workflow_basis_binding_width: 1,
                workflow_authority_target_check_count: 1,
                workflow_denial_count: 1,
                workflow_broadening_denial_count: 1,
                workflow_executor_rediscovery_count: 0,
            },
        ));
    }

    validate_target_for_family(
        request.declaration_family(),
        request.authority_target_family(),
    )?;
    validate_binding_for_request(binding, &request)?;

    let counters = WorkflowCounters {
        workflow_declaration_count: 1,
        workflow_basis_binding_count: binding.counters().workflow_basis_binding_count(),
        workflow_basis_binding_width: binding.counters().workflow_basis_binding_width(),
        workflow_authority_target_check_count: 1,
        workflow_denial_count: 0,
        workflow_broadening_denial_count: 0,
        workflow_executor_rediscovery_count: 0,
    };
    let declaration_identity = workflow_declaration_identity(binding.binding_identity(), &request);

    Ok(QueryWorkflowDeclaration {
        binding: binding.clone(),
        request: request.clone(),
        report: WorkflowAdmissionReport {
            binding_identity: binding.binding_identity().clone(),
            declaration_identity,
            declaration_family: request.declaration_family().clone(),
            basis_family: binding.basis_family().clone(),
            authority_target_family: request.authority_target_family().clone(),
            cost_class: request.cost_class().clone(),
            budget_class: request.budget_class().clone(),
            freshness_policy: request.freshness_policy().clone(),
            drift_outcome: WorkflowPredictionDriftOutcome::WithinBudget,
            counters,
        },
    })
}
