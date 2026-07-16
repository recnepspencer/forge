use super::activation_staging_test_support::{activation_staging_inputs, ActivationStagingInputs};
use crate::runtime::{
    WorthUiActivationStagingDenialReason, WorthUiDurableStateReconciliationPlan,
    WorthUiNodeReplacementPlan, WorthUiPendingExecutionPlanLoweringInput,
    WorthUiQueryLiveRebindPlan, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLaunchDenial,
};

#[test]
fn equivalent_replacement_inputs_produce_equivalent_pending_activation() {
    let first = activation_staging_inputs();
    let second = activation_staging_inputs();

    let first_pending = first.stage();
    let second_pending = second.stage();

    assert_eq!(
        first_pending.staged_replacement().active_artifact_digest(),
        second_pending.staged_replacement().active_artifact_digest()
    );
    assert_eq!(
        first_pending
            .staged_replacement()
            .candidate_artifact_digest(),
        second_pending
            .staged_replacement()
            .candidate_artifact_digest()
    );
    assert_eq!(first_pending.readiness(), second_pending.readiness());
    assert!(first_pending
        .readiness()
        .is_ready_for_execution_plan_input());
    assert_eq!(
        first_pending.staging_report().counters(),
        second_pending.staging_report().counters()
    );
    assert_eq!(
        first_pending
            .staging_report()
            .counters()
            .staged_query_binding_count(),
        1
    );
    assert!(
        first_pending
            .staging_report()
            .counters()
            .staged_reconciliation_receipt_count()
            > 0
    );
}

#[test]
fn partial_replacement_bundle_cannot_be_activated() {
    let without_state = activation_staging_inputs();
    let state_denial = without_state.stage_without_reconciliation();
    assert_eq!(
        state_denial.reason(),
        WorthUiActivationStagingDenialReason::MissingDurableStateReconciliation
    );
    assert_eq!(state_denial.counters().rejected_missing_input_count(), 1);

    let without_query = activation_staging_inputs();
    let query_denial = without_query.stage_without_query_rebind();
    assert_eq!(
        query_denial.reason(),
        WorthUiActivationStagingDenialReason::MissingQueryLiveRebindPlan
    );
    assert_eq!(query_denial.counters().rejected_missing_input_count(), 1);

    let without_lowering_input = activation_staging_inputs();
    let lowering_denial = without_lowering_input.stage_without_plan_lowering_input();
    assert_eq!(
        lowering_denial.reason(),
        WorthUiActivationStagingDenialReason::MissingExecutionPlanLoweringInput
    );
    assert_eq!(lowering_denial.counters().rejected_missing_input_count(), 1);
}

#[test]
fn staging_does_not_mutate_active_runtime_state() {
    let ActivationStagingInputs {
        runtime,
        admitted,
        impact,
        narrowing,
        node_plan,
        reconciliation_plan,
        pending_execution_plan_lowering_input,
        ..
    } = activation_staging_inputs();
    let active_before = runtime.inspect_active();
    let last_valid_before = runtime.last_valid();

    let denial = runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            Some(&reconciliation_plan),
            None,
            Some(&pending_execution_plan_lowering_input),
        )
        .expect_err("missing query rebind denies");

    assert_eq!(
        denial.reason(),
        WorthUiActivationStagingDenialReason::MissingQueryLiveRebindPlan
    );
    assert_eq!(active_before, runtime.inspect_active());
    assert_eq!(last_valid_before, runtime.last_valid());
}

#[test]
fn frame_epoch_mismatch_rejects_real_pending_activation() {
    let mut inputs = activation_staging_inputs();
    let pending = activation_staging_inputs().stage();
    inputs.runtime.advance_frame_epoch_for_test();

    assert_eq!(
        inputs
            .runtime
            .reject_if_pending_activation_is_stale(pending),
        Err(WorthUiRuntimeLaunchDenial::StalePendingActivation {
            pending_epoch: WorthUiRuntimeFrameEpoch::initial(),
            active_epoch: inputs.runtime.frame_epoch(),
        })
    );
}

#[test]
fn staging_rejects_reconciliation_plan_from_different_candidate() {
    let inputs = activation_staging_inputs();
    let stale_reconciliation = WorthUiDurableStateReconciliationPlan::new(
        inputs.reconciliation_plan.active_artifact_digest(),
        inputs.reconciliation_plan.candidate_artifact_digest() + 1,
        inputs.reconciliation_plan.receipts().to_vec(),
        inputs.reconciliation_plan.counters(),
    );

    let denial = inputs.stage_with_reconciliation(&stale_reconciliation);

    assert_eq!(
        denial.reason(),
        WorthUiActivationStagingDenialReason::CandidateArtifactDigestMismatch
    );
    assert_eq!(denial.counters().rejected_mismatched_input_count(), 1);
}

#[test]
fn staging_rejects_query_rebind_plan_from_different_candidate() {
    let inputs = activation_staging_inputs();
    let stale_query_rebind = WorthUiQueryLiveRebindPlan::new(
        inputs.query_rebind_plan.active_artifact_digest(),
        inputs.query_rebind_plan.candidate_artifact_digest() + 1,
        inputs.query_rebind_plan.entries().to_vec(),
    );

    let denial = inputs.stage_with_query_rebind(&stale_query_rebind);

    assert_eq!(
        denial.reason(),
        WorthUiActivationStagingDenialReason::CandidateArtifactDigestMismatch
    );
    assert_eq!(denial.counters().rejected_mismatched_input_count(), 1);
}

#[test]
fn staging_rejects_node_plan_from_different_active_runtime() {
    let inputs = activation_staging_inputs();
    let stale_node_plan = WorthUiNodeReplacementPlan::new(
        inputs.node_plan.active_artifact_digest() + 1,
        inputs.node_plan.candidate_artifact_digest(),
        inputs.node_plan.classifications().to_vec(),
        inputs.node_plan.counters(),
    );

    let denial = inputs.stage_with_node_plan(&stale_node_plan);

    assert_eq!(
        denial.reason(),
        WorthUiActivationStagingDenialReason::ActiveArtifactDigestMismatch
    );
    assert_eq!(denial.counters().rejected_mismatched_input_count(), 1);
}

#[test]
fn staging_rejects_plan_lowering_input_from_different_candidate() {
    let inputs = activation_staging_inputs();
    let stale_input = WorthUiPendingExecutionPlanLoweringInput::from_staged_plans(
        &WorthUiNodeReplacementPlan::new(
            inputs.node_plan.active_artifact_digest(),
            inputs.node_plan.candidate_artifact_digest() + 1,
            inputs.node_plan.classifications().to_vec(),
            inputs.node_plan.counters(),
        ),
        &inputs.reconciliation_plan,
        &inputs.query_rebind_plan,
    );

    let denial = inputs.stage_with_plan_lowering_input(&stale_input);

    assert_eq!(
        denial.reason(),
        WorthUiActivationStagingDenialReason::CandidateArtifactDigestMismatch
    );
    assert_eq!(denial.counters().rejected_mismatched_input_count(), 1);
}

#[test]
fn staging_rejects_plan_lowering_input_with_same_digest_but_stale_query_width() {
    let inputs = activation_staging_inputs();
    let stale_same_digest_query = WorthUiQueryLiveRebindPlan::new(
        inputs.query_rebind_plan.active_artifact_digest(),
        inputs.query_rebind_plan.candidate_artifact_digest(),
        Vec::new(),
    );
    let stale_input = WorthUiPendingExecutionPlanLoweringInput::from_staged_plans(
        &inputs.node_plan,
        &inputs.reconciliation_plan,
        &stale_same_digest_query,
    );

    let denial = inputs.stage_with_plan_lowering_input(&stale_input);

    assert_eq!(
        denial.reason(),
        WorthUiActivationStagingDenialReason::ExecutionPlanLoweringInputMismatch
    );
    assert_eq!(denial.counters().rejected_mismatched_input_count(), 1);
}

#[test]
fn staging_rejects_plan_lowering_input_with_same_digest_but_stale_node_width() {
    let inputs = activation_staging_inputs();
    let stale_same_digest_node_plan = WorthUiNodeReplacementPlan::new(
        inputs.node_plan.active_artifact_digest(),
        inputs.node_plan.candidate_artifact_digest(),
        Vec::new(),
        inputs.node_plan.counters(),
    );
    let stale_input = WorthUiPendingExecutionPlanLoweringInput::from_staged_plans(
        &stale_same_digest_node_plan,
        &inputs.reconciliation_plan,
        &inputs.query_rebind_plan,
    );

    let denial = inputs.stage_with_plan_lowering_input(&stale_input);

    assert_eq!(
        denial.reason(),
        WorthUiActivationStagingDenialReason::ExecutionPlanLoweringInputMismatch
    );
    assert_eq!(denial.counters().rejected_mismatched_input_count(), 1);
}

#[test]
fn staging_rejects_plan_lowering_input_with_same_digest_but_stale_reconciliation_width() {
    let inputs = activation_staging_inputs();
    let stale_same_digest_reconciliation = WorthUiDurableStateReconciliationPlan::new(
        inputs.reconciliation_plan.active_artifact_digest(),
        inputs.reconciliation_plan.candidate_artifact_digest(),
        Vec::new(),
        inputs.reconciliation_plan.counters(),
    );
    let stale_input = WorthUiPendingExecutionPlanLoweringInput::from_staged_plans(
        &inputs.node_plan,
        &stale_same_digest_reconciliation,
        &inputs.query_rebind_plan,
    );

    let denial = inputs.stage_with_plan_lowering_input(&stale_input);

    assert_eq!(
        denial.reason(),
        WorthUiActivationStagingDenialReason::ExecutionPlanLoweringInputMismatch
    );
    assert_eq!(denial.counters().rejected_mismatched_input_count(), 1);
}

#[test]
fn changed_admitted_query_support_contract_cannot_enter_staging() {
    let mut inputs = activation_staging_inputs();
    inputs.admitted = inputs
        .admitted
        .with_admitted_query_contract_for_test("stale-activation-contract");

    let denial = inputs.stage_denial();

    assert_eq!(
        denial.reason(),
        WorthUiActivationStagingDenialReason::AdmittedQuerySupportContractChanged
    );
    assert_eq!(denial.counters().receipt_verification_count(), 1);
    assert_eq!(denial.counters().verified_input_count(), 0);
}
