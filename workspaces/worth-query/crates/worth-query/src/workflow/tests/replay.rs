use crate::harness::fixtures::execution_preflights;
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, build_workflow_replay_bundle,
    lower_query_writeback_declaration, shape_writeback_authority_outcome,
    WorkflowAuthorityTargetFamily, WorkflowBindingSource, WorkflowBudgetClass, WorkflowCostClass,
    WorkflowDeclarationFamily, WorkflowDeclarationRequest, WorkflowFreshnessPolicy,
    WritebackLoweringInput,
};

#[test]
fn replay_bundle_is_built_from_authority_outcome_artifact() {
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
    let lowered = lower_query_writeback_declaration(
        &declaration,
        WritebackLoweringInput::projected_state_diff(),
    )
    .expect("writeback lowering should succeed");
    let outcome = shape_writeback_authority_outcome(&lowered);
    let replay = build_workflow_replay_bundle(&outcome);

    assert_eq!(
        replay.query_digest(),
        declaration.binding().query_for_reporting()
    );
    assert_eq!(
        replay.authority_target_family(),
        &WorkflowAuthorityTargetFamily::BridgeWriteback
    );
    assert!(!replay.bundle_digest().is_empty());
    assert_eq!(replay.counters().workflow_replay_bundle_count(), 1);
    assert_eq!(replay.counters().workflow_writeback_declaration_count(), 1);
    assert_eq!(
        replay
            .counters()
            .workflow_writeback_causality_binding_count(),
        1
    );
    assert_eq!(replay.counters().workflow_executor_rediscovery_count(), 0);
}
