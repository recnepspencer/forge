use super::activation_staging_test_support::activation_staging_inputs;
use super::durable_state_reconciliation_test_support::{
    deterministic_reconciliation_inputs, stale_inventory_for,
};
use super::query_binding_comparison_test_support::{
    denial_presentation_drift_query_app, phase11_pipeline, query_artifact, standard_query_app,
};
use super::reload_failure_test_support::{
    assert_failure_preserves_active_runtime, assert_preserved_counters,
    missing_artifact_candidate_denial, missing_dependency_candidate_denial,
    missing_lowering_basis_candidate_denial, stale_dependency_candidate_denial,
};
use crate::runtime::{
    WorthUiNodeReplacementPlan, WorthUiQueryLiveRebindOutcome, WorthUiReloadCheckedStopPosture,
    WorthUiReloadFailureStage, WorthUiReplacementCandidateDenial, WorthUiRuntimeLifecycle,
};
use std::collections::BTreeSet;

#[test]
fn invalid_candidate_reload_preserves_previous_active_plan() {
    let fixture = activation_staging_inputs();
    let active_before = fixture.runtime.inspect_active();
    let last_valid_before = fixture.runtime.last_valid();
    let denial = missing_artifact_candidate_denial();

    let failure = fixture.runtime.preserve_invalid_candidate_reload(denial);

    assert_eq!(fixture.runtime.inspect_active(), active_before);
    assert_eq!(fixture.runtime.last_valid(), last_valid_before);
    assert_failure_preserves_active_runtime(
        failure,
        active_before,
        last_valid_before,
        WorthUiReloadFailureStage::InvalidCandidate,
    );
    assert_eq!(
        denial,
        WorthUiReplacementCandidateDenial::MissingArtifactDigest
    );
    assert!(failure.denial().upstream_evidence_digest().is_some());
    assert_eq!(
        failure.preservation_receipt().active_lifecycle(),
        WorthUiRuntimeLifecycle::Active
    );
}

#[test]
fn failed_reconciliation_preserves_prior_valid_runtime_state() {
    let (runtime, plan, inventory) = deterministic_reconciliation_inputs();
    let active_before_failure = runtime.inspect_active();
    let last_valid_before_failure = runtime.last_valid();
    let stale_inventory = stale_inventory_for(&inventory);
    let denial = runtime
        .reconcile_durable_state(&plan, &stale_inventory)
        .expect_err("stale inventory denies state reconciliation");

    let failure = runtime.preserve_failed_reconciliation(&denial);

    assert_eq!(runtime.inspect_active(), active_before_failure);
    assert_eq!(runtime.last_valid(), last_valid_before_failure);
    assert_failure_preserves_active_runtime(
        failure,
        active_before_failure,
        last_valid_before_failure,
        WorthUiReloadFailureStage::DurableStateReconciliation,
    );
    assert!(failure.denial().upstream_evidence_digest().is_some());
}

#[test]
fn failed_plan_lowering_preserves_prior_valid_runtime_state() {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    runtime.advance_frame_epoch_for_test();
    let active_before_failure = runtime.inspect_active();
    let last_valid_before_failure = runtime.last_valid();
    let denial = runtime
        .prepare_execution_plan_input(&pending)
        .expect_err("stale pending activation must deny plan lowering");

    let failure = runtime.preserve_failed_plan_lowering(&denial);

    assert_eq!(runtime.inspect_active(), active_before_failure);
    assert_eq!(runtime.last_valid(), last_valid_before_failure);
    assert_failure_preserves_active_runtime(
        failure,
        active_before_failure,
        last_valid_before_failure,
        WorthUiReloadFailureStage::PlanLowering,
    );
    assert!(failure.denial().upstream_evidence_digest().is_some());
}

#[test]
fn activation_staging_denial_preservation_keeps_prior_valid_state() {
    let inputs = activation_staging_inputs();
    let runtime = inputs.runtime;
    let active_before_failure = runtime.inspect_active();
    let last_valid_before_failure = runtime.last_valid();
    let denial = runtime
        .stage_replacement_activation(
            inputs.admitted,
            &inputs.impact,
            &inputs.narrowing,
            &inputs.node_plan,
            crate::runtime::WorthUiActivationStagingPlans::new(
                None,
                Some(&inputs.query_rebind_plan),
                Some(&inputs.pending_execution_plan_lowering_input),
            ),
        )
        .expect_err("missing reconciliation denies activation staging");

    let failure = runtime.preserve_failed_activation_staging(&denial);

    assert_eq!(runtime.inspect_active(), active_before_failure);
    assert_eq!(runtime.last_valid(), last_valid_before_failure);
    assert_failure_preserves_active_runtime(
        failure,
        active_before_failure,
        last_valid_before_failure,
        WorthUiReloadFailureStage::ActivationStaging,
    );
    assert!(failure.denial().upstream_evidence_digest().is_some());
}

#[test]
fn query_live_rebind_denial_preserves_active_and_checked_stop_posture() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);
    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let stale_plan = WorthUiNodeReplacementPlan::new(
        plan.active_artifact_digest(),
        plan.candidate_artifact_digest() + 1,
        plan.classifications().to_vec(),
        plan.counters(),
    );
    let active_before_failure = runtime.inspect_active();
    let last_valid_before_failure = runtime.last_valid();
    let denial = runtime
        .plan_query_live_rebinds(&comparison, &stale_plan, &narrowing, &admitted)
        .expect_err("stale plan denies live Query rebind planning");

    let failure = runtime.preserve_failed_query_live_rebind(&denial);

    assert_eq!(runtime.inspect_active(), active_before_failure);
    assert_eq!(runtime.last_valid(), last_valid_before_failure);
    assert_failure_preserves_active_runtime(
        failure,
        active_before_failure,
        last_valid_before_failure,
        WorthUiReloadFailureStage::QueryLiveRebind,
    );
    assert_eq!(
        failure.denial().checked_stop_posture(),
        WorthUiReloadCheckedStopPosture::QuerySupportDenied
    );
    assert!(failure
        .denial()
        .checked_stop_posture()
        .is_query_checked_stop());
}

#[test]
fn query_recovery_checked_stop_preserves_active_and_recovery_posture() {
    let active_app = standard_query_app();
    let candidate_app = denial_presentation_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);
    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let active_before_failure = runtime.inspect_active();
    let last_valid_before_failure = runtime.last_valid();
    let rebind_plan = runtime
        .plan_query_live_rebinds(&comparison, &plan, &narrowing, &admitted)
        .expect("live Query rebind planning records entry-level denial");
    let entry = rebind_plan
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding planned");
    let WorthUiQueryLiveRebindOutcome::Deny(denial) = entry.outcome() else {
        panic!("denial-presentation drift must preserve Query recovery by checked stop");
    };

    let failure = runtime.preserve_query_recovery_checked_stop(denial);

    assert_eq!(runtime.inspect_active(), active_before_failure);
    assert_eq!(runtime.last_valid(), last_valid_before_failure);
    assert_failure_preserves_active_runtime(
        failure,
        active_before_failure,
        last_valid_before_failure,
        WorthUiReloadFailureStage::QueryLiveRebind,
    );
    assert_eq!(
        failure.denial().checked_stop_posture(),
        WorthUiReloadCheckedStopPosture::QueryRecoveryPreserved
    );
    assert!(failure
        .denial()
        .checked_stop_posture()
        .is_query_checked_stop());
}

#[test]
fn repeated_invalid_reloads_do_not_accumulate_runtime_residue() {
    let fixture = activation_staging_inputs();
    let active_before = fixture.runtime.inspect_active();
    let last_valid_before = fixture.runtime.last_valid();
    let failures = [
        fixture
            .runtime
            .preserve_invalid_candidate_reload(missing_artifact_candidate_denial()),
        fixture
            .runtime
            .preserve_invalid_candidate_reload(missing_dependency_candidate_denial()),
        fixture
            .runtime
            .preserve_invalid_candidate_reload(missing_lowering_basis_candidate_denial()),
        fixture
            .runtime
            .preserve_invalid_candidate_reload(stale_dependency_candidate_denial()),
    ];

    assert_eq!(fixture.runtime.inspect_active(), active_before);
    assert_eq!(fixture.runtime.last_valid(), last_valid_before);

    let mut evidence_digests = BTreeSet::new();
    for failure in failures {
        assert_eq!(
            failure.denial().stage(),
            WorthUiReloadFailureStage::InvalidCandidate
        );
        evidence_digests.insert(
            failure
                .denial()
                .upstream_evidence_digest()
                .expect("invalid candidate failure carries upstream evidence"),
        );
        assert_eq!(
            failure.preservation_receipt().active_artifact_digest(),
            active_before.artifact_digest()
        );
        assert_eq!(
            failure.preservation_receipt().active_plan_digest(),
            active_before.active_plan_digest()
        );
        assert_preserved_counters(failure.counters());
    }
    assert_eq!(evidence_digests.len(), 4);
}

#[test]
fn activation_gate_denial_preservation_keeps_active_meaning_unchanged() {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    let (snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "reload.activation-gate",
        );
    let admitted = snapshot
        .admit_allocation_catalog_basis_set(vec![first, second])
        .expect("graph admits complete reload catalog");
    let active_before_failure = runtime.inspect_active();
    let last_valid_before_failure = runtime.last_valid();
    let boundary = runtime.traversal_frame_boundary_for_test();
    let denial = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect_err("unsafe frame boundary denies activation");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("post-mint denial carries canonical evidence")
    };
    let crate::runtime::UiCommittedAllocationActivationDenialReason::FrameBoundary(gate) =
        denial.reason()
    else {
        panic!("canonical denial retains frame reason")
    };
    let failure = runtime.preserve_failed_activation_gate(gate);

    assert_eq!(runtime.inspect_active(), active_before_failure);
    assert_eq!(runtime.last_valid(), last_valid_before_failure);
    assert_failure_preserves_active_runtime(
        failure,
        active_before_failure,
        last_valid_before_failure,
        WorthUiReloadFailureStage::ActivationGate,
    );
}
