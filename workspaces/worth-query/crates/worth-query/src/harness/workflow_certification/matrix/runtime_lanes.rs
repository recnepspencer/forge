use super::super::lane::WorkflowCertificationLane;
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::preview::{
    admit_preview_workflow_foundation, bind_preflight_to_preview_session, PreviewEvaluationClass,
    PreviewSessionQueryContext,
};
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, WorkflowAuthorityTargetFamily,
    WorkflowBindingSource, WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily,
    WorkflowDeclarationRequest, WorkflowFreshnessPolicy,
};

pub(super) fn runtime_conflict_lane() -> WorkflowCertificationLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::ConflictInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection,
            WorkflowCostClass::InspectionNarrow,
            WorkflowBudgetClass::InspectionBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("runtime conflict declaration should admit");
    WorkflowCertificationLane::from_declaration(&declaration)
}

pub(super) fn runtime_merge_lane(budget_class: WorkflowBudgetClass) -> WorkflowCertificationLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            budget_class,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("runtime merge declaration should admit");
    WorkflowCertificationLane::from_declaration(&declaration)
}

pub(super) fn runtime_mutation_lane() -> WorkflowCertificationLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MutationLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMutation,
            WorkflowCostClass::MutationLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("runtime mutation declaration should admit");
    WorkflowCertificationLane::from_declaration(&declaration)
}

pub(super) fn preview_foundation_lane() -> WorkflowCertificationLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("workflow-certification-preview");
    let binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("preview binding should succeed");
    let foundation =
        admit_preview_workflow_foundation(&binding).expect("preview foundation should admit");
    let workflow_binding =
        bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
            .expect("preview workflow binding should admit");
    let declaration = admit_query_workflow_declaration(
        &workflow_binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::ConflictInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection,
            WorkflowCostClass::InspectionNarrow,
            WorkflowBudgetClass::InspectionBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("preview inspection declaration should admit");
    WorkflowCertificationLane::from_declaration(&declaration)
}
