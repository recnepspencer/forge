use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::preview::{
    admit_authoritative_preview_comparison_candidate, admit_preview_promotion_parity_comparison,
    admit_preview_workflow_foundation, admit_preview_workflow_foundation_request,
    bind_preflight_to_preview_session, execute_promotion_eligible_preview_session_plan,
    execute_read_only_preview_session_plan, PreviewEvaluationClass, PreviewSessionQueryContext,
    PreviewWorkflowFoundationFailureClass, PreviewWorkflowFoundationRequest,
};
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context,
    foundation::{workflow_context_basis_identity, workflow_context_query_identity},
    workflow_canonical_query_digest_evidence, workflow_validated_query_digest_evidence,
    WorkflowAdmissionFailureClass, WorkflowAuthorityTargetFamily, WorkflowBasisFamily,
    WorkflowBindingSource, WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily,
    WorkflowDeclarationRequest, WorkflowFreshnessPolicy, WorkflowPredictionDriftOutcome,
    WorkflowPreviewEvaluationClass,
};
#[test]
fn runtime_preflight_binding_preserves_query_and_basis_digests() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime preflight should bind");

    assert_eq!(
        binding.query_for_reporting(),
        workflow_context_query_identity(
            &workflow_canonical_query_digest_evidence(
                preflight.plan().query().canonical_query_digest(),
            ),
        )
        .as_str()
    );
    assert_eq!(
        binding.basis_for_reporting(),
        workflow_context_basis_identity(
            &WorkflowBasisFamily::RuntimePreflight,
            preflight.basis().proof().identity(),
        )
        .as_str()
    );
    assert_eq!(binding.counters().workflow_basis_binding_count(), 1);
    assert_eq!(binding.counters().workflow_executor_rediscovery_count(), 0);
}

#[test]
fn preview_workflow_foundation_binding_preserves_preview_identity() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("workflow-binding");
    let binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("preview binding should succeed");
    let foundation = admit_preview_workflow_foundation(&binding)
        .expect("preview workflow foundation should admit");

    let workflow_binding =
        bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
            .expect("workflow binding should reuse preview foundation");

    assert_eq!(
        workflow_binding.preview_session_identity(),
        Some(foundation.preview_session_identity())
    );
    assert_eq!(
        workflow_binding.query_for_reporting(),
        workflow_context_query_identity(&workflow_validated_query_digest_evidence(
            foundation.validated_query_digest(),
        ))
            .as_str()
    );
    assert_eq!(
        workflow_binding.preview_evaluation_class(),
        Some(&WorkflowPreviewEvaluationClass::PromotionEligible)
    );
    assert_eq!(
        workflow_binding
            .counters()
            .workflow_executor_rediscovery_count(),
        0
    );
}

#[test]
fn preview_promotion_comparison_basis_can_author_inspection_and_merge_when_permitted() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("workflow-promotion-comparison");
    let binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("promotion-eligible binding should succeed");
    let execution = execute_promotion_eligible_preview_session_plan(
        &crate::preview::admit_promotion_eligible_preview_session_plan_binding(binding)
            .expect("promotion binding should admit"),
    )
    .expect("promotion execution should succeed");
    let candidate_execution = crate::execution::execute_preflight_bundle(&preflight)
        .expect("candidate execution should succeed");
    let candidate =
        admit_authoritative_preview_comparison_candidate(&preflight, &candidate_execution)
            .expect("candidate should admit");
    let comparison = admit_preview_promotion_parity_comparison(&execution, &candidate)
        .expect("promotion parity should admit");
    let workflow_binding = bind_workflow_context(
        WorkflowBindingSource::PreviewPromotionComparison(&comparison),
    )
    .expect("promotion comparison should bind");

    assert_eq!(
        workflow_binding.query_for_reporting(),
        workflow_context_query_identity(&workflow_validated_query_digest_evidence(
            comparison.validated_query_digest(),
        ))
            .as_str()
    );
    let inspection = admit_query_workflow_declaration(
        &workflow_binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::ConflictInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection,
            WorkflowCostClass::InspectionNarrow,
            WorkflowBudgetClass::InspectionBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("inspection declaration should admit");
    assert_eq!(
        inspection.report().authority_target_family(),
        &WorkflowAuthorityTargetFamily::QueryInspection
    );

    let merge = admit_query_workflow_declaration(
        &workflow_binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("merge declaration should admit");
    assert_eq!(
        merge.report().authority_target_family(),
        &WorkflowAuthorityTargetFamily::RelationalMerge
    );
}

#[test]
fn denied_paths_increment_exact_counters_without_rediscovery() {
    let store_preflight = execution_preflights::store_detail_preflight();
    let invalid_basis =
        bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&store_preflight))
            .expect_err("store-backed workflow binding should deny");
    assert_eq!(
        invalid_basis.failure_class(),
        &WorkflowAdmissionFailureClass::InvalidBasisPairing
    );
    assert_eq!(invalid_basis.counters().workflow_denial_count(), 1);
    assert_eq!(
        invalid_basis
            .counters()
            .workflow_executor_rediscovery_count(),
        0
    );

    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let broadening = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::CrossBoundaryExpansion,
            WorkflowFreshnessPolicy::AllowExplicitRebind,
        ),
    )
    .expect_err("cross-boundary expansion should deny");
    assert_eq!(
        broadening.failure_class(),
        &WorkflowAdmissionFailureClass::ForbiddenWorkflowBroadening
    );
    assert_eq!(
        broadening.drift_outcome(),
        &WorkflowPredictionDriftOutcome::ExplicitBroadeningDenied
    );
    assert_eq!(broadening.counters().workflow_broadening_denial_count(), 1);
    assert_eq!(
        broadening.counters().workflow_executor_rediscovery_count(),
        0
    );
}

#[test]
fn read_only_preview_foundation_denies_authority_requests() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("workflow-read-only");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("read-only preview binding should succeed");
    let admitted_binding =
        crate::preview::admit_read_only_preview_session_plan_binding(preview_binding.clone())
            .expect("read-only binding should admit");
    let _execution = execute_read_only_preview_session_plan(&admitted_binding)
        .expect("read-only execution should succeed");
    let foundation = admit_preview_workflow_foundation(&preview_binding)
        .expect("read-only workflow foundation should admit");
    let workflow_binding =
        bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
            .expect("read-only foundation should bind");

    let error = admit_query_workflow_declaration(
        &workflow_binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect_err("read-only preview should deny merge intent");
    assert_eq!(
        error.failure_class(),
        &WorkflowAdmissionFailureClass::PreviewReadOnlyAuthorityRequestForbidden
    );
}

#[test]
fn read_only_preview_cannot_request_deferred_writeback_foundation() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("workflow-read-only-writeback-foundation-denied");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("read-only preview binding should succeed");

    let error = admit_preview_workflow_foundation_request(
        &preview_binding,
        PreviewWorkflowFoundationRequest::deferred_mutation_writeback(),
    )
    .expect_err("read-only preview should not admit deferred writeback foundations");

    assert_eq!(
        error.failure_class(),
        &PreviewWorkflowFoundationFailureClass::ReadOnlyPreviewWritebackFoundationForbidden
    );
}
