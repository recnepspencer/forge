use crate::aspect_field_authoring::single_aspect_field_patch_from_external_json;
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::preview::{
    admit_preview_workflow_foundation_request, bind_preflight_to_preview_session,
    PreviewEvaluationClass, PreviewSessionQueryContext, PreviewWorkflowFoundationRequest,
};
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, lower_merge_workflow_declaration,
    lower_mutation_intent_declaration, lower_query_writeback_declaration, MergeLoweringInput,
    MutationLoweringInput, QueryWritebackDeclaration, WorkflowAuthorityTargetFamily,
    WorkflowBindingSource, WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily,
    WorkflowDeclarationRequest, WorkflowFreshnessBinding, WorkflowFreshnessPolicy,
    WorkflowLoweringFailureClass, WorkflowStalenessClass, WritebackLoweringInput,
};
use forge_relational::facade::commit_strategies::{
    IntentReconciliationInput, StrategyCallerProvenance, StrategyRequestOrigin,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, PartitionId};
use forge_relational::facade::merge::{MergeExecutionRequest, MergeIntent};
use forge_runtime_bridge::facade::{BridgeRequestKind, BridgeWritebackRequestMode};
use serde_json::json;

#[test]
fn runtime_mutation_lowering_emits_explicit_strategy_request() {
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
    .expect("mutation declaration should admit");

    let authority_binding_identity = binding.basis_identity();
    let lowered = lower_mutation_intent_declaration(
        &declaration,
        &authority_binding_identity,
        MutationLoweringInput::IntentReconciliation {
            entity_id: EntityId::new(PartitionId(1), 41, 0),
            desired_aspect_fields_external_json: json!({"name":"after"}),
        },
    )
    .expect("runtime mutation lowering should succeed");

    assert_eq!(
        lowered.strategy_request().strategy_name().as_str(),
        "strategy.intent.reconcile"
    );
    assert_eq!(
        lowered.freshness_binding(),
        &WorkflowFreshnessBinding::RuntimeBasisExact
    );
    assert_eq!(
        lowered.staleness_class(),
        &WorkflowStalenessClass::ExactBasisPreserved
    );
    assert_eq!(lowered.counters().workflow_lowering_width(), 1);
    assert_eq!(lowered.counters().workflow_mutation_lowering_count(), 1);
    assert_eq!(lowered.counters().workflow_merge_lowering_count(), 0);
    assert_eq!(lowered.counters().workflow_staleness_check_count(), 1);
    assert_eq!(
        lowered
            .counters()
            .workflow_work_avoided_by_query_lowering_count(),
        1
    );

    let control = IntentReconciliationInput {
        entity_id: EntityId::new(PartitionId(1), 41, 0),
        desired_aspect_fields: single_aspect_field_patch_from_external_json(
            "name",
            "name",
            json!("after"),
        )
        .expect("control field patch"),
    }
    .into_native_canonical_request(StrategyCallerProvenance {
        request_origin: StrategyRequestOrigin::Api,
        actor_identity: Some("forge-query".to_string()),
        correlation_id: Some(declaration.report().declaration_digest().to_string()),
    })
    .expect("control strategy request should encode");
    assert_eq!(
        lowered.strategy_request().strategy_name().as_str(),
        control.strategy_name().as_str()
    );
    assert_eq!(
        lowered.strategy_request().input_bytes(),
        control.input_bytes()
    );
    assert_eq!(
        lowered.strategy_request().caller_provenance(),
        control.caller_provenance()
    );
}

#[test]
fn merge_lowering_preserves_explicit_branches_and_requires_authority_validation() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("merge declaration should admit");

    let lowered = lower_merge_workflow_declaration(
        &declaration,
        MergeLoweringInput::reconcile_into_target(
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        ),
    )
    .expect("merge lowering should succeed");

    assert_eq!(lowered.merge_request().target_branch().0, "main");
    assert_eq!(lowered.merge_request().source_branch().0, "candidate");
    assert_eq!(
        lowered.staleness_class(),
        &WorkflowStalenessClass::AuthorityValidationRequired
    );
    assert_eq!(lowered.counters().workflow_executor_rediscovery_count(), 0);
    assert_eq!(lowered.counters().workflow_merge_lowering_count(), 1);
    assert_eq!(lowered.counters().workflow_mutation_lowering_count(), 0);
    assert_eq!(lowered.counters().workflow_staleness_check_count(), 1);

    let control = MergeExecutionRequest {
        target_branch: BranchId("main".to_string()),
        source_branch: BranchId("candidate".to_string()),
        merge_intent: MergeIntent::ReconcileIntoTarget,
    };
    assert_eq!(lowered.merge_request(), &control);
}

#[test]
fn writeback_lowering_requires_authoritative_rebind_outside_runtime_basis() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("workflow-writeback");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("promotion preview binding should succeed");
    let foundation = admit_preview_workflow_foundation_request(
        &preview_binding,
        PreviewWorkflowFoundationRequest::deferred_mutation_writeback(),
    )
    .expect("deferred writeback workflow foundation should admit");
    let workflow_binding =
        bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
            .expect("preview workflow binding should succeed");
    let declaration = admit_query_workflow_declaration(
        &workflow_binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
            WorkflowCostClass::WritebackLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::AllowExplicitRebind,
        ),
    )
    .expect("writeback declaration should admit");

    let error = lower_query_writeback_declaration(
        &declaration,
        WritebackLoweringInput::projected_state_diff(),
    )
    .expect_err("preview writeback should require explicit rebind");

    assert_eq!(
        workflow_binding.preview_request_family(),
        Some(&PreviewWorkflowFoundationRequest::DeferredMutationWriteback)
    );

    assert_eq!(
        error.failure_class(),
        &WorkflowLoweringFailureClass::ExplicitRebindRequired
    );
    assert_eq!(
        error.staleness_class(),
        &WorkflowStalenessClass::ExplicitRebindRequired
    );
    assert_eq!(
        error.counters().workflow_lowering_staleness_denial_count(),
        1
    );
    assert_eq!(
        error.counters().workflow_explicit_rebind_required_count(),
        1
    );
    assert_eq!(error.counters().workflow_writeback_declaration_count(), 0);
    assert_eq!(error.counters().workflow_writeback_denial_count(), 1);
    assert_eq!(error.counters().workflow_budget_cross_count(), 1);
}

#[test]
fn writeback_lowering_stale_denies_exact_basis_preview_requests() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("workflow-writeback-stale");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("promotion preview binding should succeed");
    let foundation = admit_preview_workflow_foundation_request(
        &preview_binding,
        PreviewWorkflowFoundationRequest::deferred_mutation_writeback(),
    )
    .expect("deferred writeback workflow foundation should admit");
    let workflow_binding =
        bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
            .expect("preview workflow binding should succeed");
    let declaration = admit_query_workflow_declaration(
        &workflow_binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
            WorkflowCostClass::WritebackLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("writeback declaration should admit");

    let error = lower_query_writeback_declaration(
        &declaration,
        WritebackLoweringInput::projected_state_diff(),
    )
    .expect_err("preview writeback should stale-deny for exact-basis freshness");

    assert_eq!(
        error.failure_class(),
        &WorkflowLoweringFailureClass::StaleWorkflowDenied
    );
    assert_eq!(
        error.staleness_class(),
        &WorkflowStalenessClass::StaleDenied
    );
    assert_eq!(error.counters().workflow_staleness_check_count(), 1);
    assert_eq!(error.counters().workflow_stale_denial_count(), 1);
    assert_eq!(error.counters().workflow_writeback_declaration_count(), 0);
    assert_eq!(
        error.counters().workflow_explicit_rebind_required_count(),
        0
    );
}

#[test]
fn runtime_writeback_lowering_emits_bridge_declaration_and_causality() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
            WorkflowCostClass::WritebackLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("writeback declaration should admit");

    let lowered: QueryWritebackDeclaration = lower_query_writeback_declaration(
        &declaration,
        WritebackLoweringInput::projected_state_diff(),
    )
    .expect("runtime writeback lowering should succeed");

    assert_eq!(
        lowered.causality_binding().request_kind(),
        BridgeRequestKind::Authoritative
    );
    assert_eq!(
        lowered.bridge_declaration().request_mode(),
        BridgeWritebackRequestMode::WritebackCapable
    );
    assert!(!lowered.causality_binding().causality_for_reporting().is_empty());
    assert_eq!(lowered.counters().workflow_staleness_check_count(), 1);
    assert_eq!(lowered.counters().workflow_writeback_declaration_count(), 1);
    assert_eq!(
        lowered
            .counters()
            .workflow_writeback_causality_binding_count(),
        1
    );
    assert_eq!(
        lowered.bridge_declaration().request_kind(),
        BridgeRequestKind::Authoritative
    );
    assert_eq!(
        lowered.bridge_declaration().family_kind(),
        Some(forge_runtime_bridge::facade::BridgeWritebackFamilyKind::ProjectedStateDiff)
    );
    assert_eq!(
        lowered.bridge_declaration().effect_class(),
        forge_runtime_bridge::facade::BridgeWritebackEffectClass::ProjectedStateDiff
    );
    assert_eq!(
        lowered.bridge_declaration().strategy_class(),
        Some(
            forge_runtime_bridge::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        )
    );
    assert_eq!(
        lowered.bridge_declaration().idempotence_class(),
        forge_runtime_bridge::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression
    );
}

#[test]
fn unsupported_workflow_family_denials_are_typed_during_lowering() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
            WorkflowCostClass::WritebackLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("writeback declaration should admit");

    let error = lower_merge_workflow_declaration(
        &declaration,
        MergeLoweringInput::reconcile_into_target(
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        ),
    )
    .expect_err("writeback declaration should not lower as merge");

    assert_eq!(
        error.failure_class(),
        &WorkflowLoweringFailureClass::UnsupportedMergeFamily
    );
    assert_eq!(error.counters().workflow_lowering_denial_count(), 1);
    assert_eq!(error.counters().workflow_merge_denial_count(), 1);
}

#[test]
fn unsupported_writeback_family_denials_are_typed_during_lowering() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("merge declaration should admit");

    let error = lower_query_writeback_declaration(
        &declaration,
        WritebackLoweringInput::projected_state_diff(),
    )
    .expect_err("merge declaration should not lower as writeback");

    assert_eq!(
        error.failure_class(),
        &WorkflowLoweringFailureClass::UnsupportedWritebackFamily
    );
    assert_eq!(error.counters().workflow_lowering_denial_count(), 1);
    assert_eq!(error.counters().workflow_writeback_denial_count(), 1);
}
