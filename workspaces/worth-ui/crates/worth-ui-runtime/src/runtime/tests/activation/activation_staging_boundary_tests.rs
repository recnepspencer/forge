use super::activation_staging_test_support::{activation_staging_inputs, ActivationStagingInputs};
use crate::runtime::{
    WorthUiActivationStagingDenialReason, WorthUiDurableStateReconciliationPlan,
    WorthUiNodeReplacementPlan, WorthUiQueryLiveRebindPlan, WorthUiRuntimeFrameEpoch,
    WorthUiRuntimeLaunchDenial,
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
        0
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
            crate::runtime::WorthUiActivationStagingPlans::new(Some(&reconciliation_plan), None),
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
