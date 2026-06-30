use super::durable_state_reconciliation_test_support::{
    ambiguous_plan_with_inventory, custom_inventory_for_policy, drop_create_inputs,
};
use super::identity_match_graph_test_support::identity_match_app;
use super::identity_state_query_certification_test_support::{
    ambiguous_plan_for_same_active, preserved_query_rebind_plan,
    query_runtime_state_and_rebind_inputs, single_active_state_lifecycle_inputs,
    ui_local_drift_rebind_plan,
};
use super::query_binding_comparison_test_support::standard_query_app;
use crate::runtime::{
    WorthUiDurableStateReconciliationOutcome, WorthUiDurableStateReconciliationPlan,
    WorthUiDurableStateReplacementPolicy, WorthUiIdentityStateQueryCertificationDenialReason,
    WorthUiIdentityStateQueryCertificationScenario, WorthUiQueryBindingDriftDenialKind,
};

#[test]
fn ambiguous_identity_storm_never_preserves_durable_state() {
    let (runtime, plan, inventory) = ambiguous_plan_with_inventory();
    let app = identity_match_app();
    let denial = runtime
        .reconcile_durable_state(&plan, &inventory)
        .expect_err("ambiguous identity denies reconciliation");

    let certification = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("ambiguous identity")
                .with_state_reconciliation_denial("ambiguous preserve attempt", plan, denial),
            app.capabilities(),
        )
        .expect("ambiguous denial certifies no carry-forward");

    assert_eq!(
        certification.counters().ambiguous_identity_denial_count(),
        1
    );
    assert_eq!(certification.counters().state_carry_forward_count(), 0);
    assert!(certification.carry_forward_receipts().is_empty());
    assert!(certification.residue_scan().is_clean());
}

#[test]
fn state_replacement_and_drop_receipts_match_actual_runtime_state() {
    let (runtime, structural_plan, _) = single_active_state_lifecycle_inputs();
    let app = identity_match_app();
    let structural_inventory = custom_inventory_for_policy(
        &runtime,
        &structural_plan,
        WorthUiDurableStateReplacementPolicy::ReplaceOnReplacement,
    );
    let structural_reconciliation = runtime
        .reconcile_durable_state(&structural_plan, &structural_inventory)
        .expect("structural replacement reconciles");
    let certification = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("state lifecycle")
                .with_state_reconciliation_plan(
                    "structural replacement",
                    structural_plan,
                    structural_reconciliation,
                ),
            app.capabilities(),
        )
        .expect("state lifecycle certifies");

    let outcomes = certification
        .state_receipts()
        .iter()
        .map(|receipt| receipt.outcome())
        .collect::<Vec<_>>();
    assert!(outcomes.contains(&WorthUiDurableStateReconciliationOutcome::Replace));
    assert!(outcomes.contains(&WorthUiDurableStateReconciliationOutcome::Drop));
    assert!(outcomes.contains(&WorthUiDurableStateReconciliationOutcome::Recreate));
    assert_eq!(
        certification.counters().state_receipt_count(),
        certification.state_receipts().len()
    );
    assert!(certification
        .state_receipts()
        .iter()
        .all(|receipt| receipt.transition()
            == receipt
                .source_receipt()
                .replacement()
                .map_or(receipt.transition(), |replacement| replacement.transition())));
}

#[test]
fn query_drift_certification_rejects_ui_local_loading_or_subscription_model() {
    let (runtime, preserve_plan) = preserved_query_rebind_plan();
    let app = standard_query_app();
    let denial = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("ui local pseudo status")
                .with_ui_local_query_status_probe("ui local status", preserve_plan),
            app.capabilities(),
        )
        .expect_err("UI-local query status cannot substitute Query recovery");

    match denial.reason() {
        WorthUiIdentityStateQueryCertificationDenialReason::UiLocalQueryStatusResidue { label } => {
            assert_eq!(label, "ui local status")
        }
        other => panic!("unexpected denial: {other:?}"),
    }
    assert_eq!(denial.counters().ui_local_probe_count(), 1);
}

#[test]
fn certification_rejects_mismatched_capability_snapshot() {
    let (runtime, structural_plan, structural_inventory) = single_active_state_lifecycle_inputs();
    let wrong_snapshot_app = standard_query_app();
    let ambiguous_plan = ambiguous_plan_for_same_active(&structural_plan);
    let ambiguous_denial = runtime
        .reconcile_durable_state(&ambiguous_plan, &structural_inventory)
        .expect_err("ambiguous plan denies");

    let denial = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("wrong snapshot")
                .with_state_reconciliation_denial(
                    "non-executed step just to cross empty scenario",
                    ambiguous_plan,
                    ambiguous_denial,
                ),
            wrong_snapshot_app.capabilities(),
        )
        .expect_err("wrong capability snapshot denies before certification");

    match denial.reason() {
        WorthUiIdentityStateQueryCertificationDenialReason::SnapshotDigestMismatch {
            active_snapshot_digest,
            provided_snapshot_digest,
        } => assert_ne!(active_snapshot_digest, provided_snapshot_digest),
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn query_drift_certification_uses_query_stop_classes_not_messages() {
    let (runtime, rebind_plan) = ui_local_drift_rebind_plan();
    let app = standard_query_app();

    let certification = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("typed query stop")
                .with_query_rebind_plan_expecting_denial(
                    "typed ui local denial",
                    rebind_plan,
                    WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery,
                ),
            app.capabilities(),
        )
        .expect("typed query denial certifies");

    assert_eq!(certification.query_drift().typed_denials().len(), 1);
    assert_eq!(
        certification.query_drift().typed_denial_kinds(),
        vec![
            WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery
        ]
    );
}

#[test]
fn query_drift_certification_rejects_wrong_typed_stop_expectation() {
    let (runtime, rebind_plan) = ui_local_drift_rebind_plan();
    let app = standard_query_app();

    let denial = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("wrong typed query stop")
                .with_query_rebind_plan_expecting_denial(
                    "wrong denial family",
                    rebind_plan,
                    WorthUiQueryBindingDriftDenialKind::QuerySupportPostureNotAdmitted,
                ),
            app.capabilities(),
        )
        .expect_err("wrong typed Query stop denies");

    match denial.reason() {
        WorthUiIdentityStateQueryCertificationDenialReason::UnexpectedTypedQueryDriftDenial {
            label,
            expected,
        } => {
            assert_eq!(label, "wrong denial family");
            assert_eq!(
                *expected,
                WorthUiQueryBindingDriftDenialKind::QuerySupportPostureNotAdmitted
            );
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn query_drift_certification_requires_declared_typed_stop_expectation() {
    let (runtime, rebind_plan) = ui_local_drift_rebind_plan();
    let app = standard_query_app();

    let denial = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("undeclared query stop")
                .with_query_rebind_plan("undeclared denied rebind", rebind_plan),
            app.capabilities(),
        )
        .expect_err("denied query plan must declare expected typed stop");

    match denial.reason() {
        WorthUiIdentityStateQueryCertificationDenialReason::MissingTypedQueryDriftDenial {
            label,
        } => assert_eq!(label, "undeclared denied rebind"),
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn state_certification_rejects_reconciliation_receipt_digest_drift() {
    let (runtime, state_plan, state_inventory) = single_active_state_lifecycle_inputs();
    let app = identity_match_app();
    let reconciliation = runtime
        .reconcile_durable_state(&state_plan, &state_inventory)
        .expect("reconciliation succeeds");
    let drifted_reconciliation = WorthUiDurableStateReconciliationPlan::new(
        reconciliation.active_artifact_digest(),
        reconciliation.candidate_artifact_digest() + 1,
        reconciliation.receipts().to_vec(),
        reconciliation.counters(),
    );

    let denial = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("drifted reconciliation")
                .with_state_reconciliation_plan(
                    "candidate digest drift",
                    state_plan,
                    drifted_reconciliation,
                ),
            app.capabilities(),
        )
        .expect_err("drifted reconciliation digest denies");

    match denial.reason() {
        WorthUiIdentityStateQueryCertificationDenialReason::StatePlanDigestMismatch {
            plan_candidate_artifact_digest,
            reconciliation_candidate_artifact_digest,
            ..
        } => assert_ne!(
            plan_candidate_artifact_digest,
            reconciliation_candidate_artifact_digest
        ),
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn state_certification_rejects_receipts_from_another_active_runtime() {
    let (runtime, _, _) = single_active_state_lifecycle_inputs();
    let app = identity_match_app();
    let (foreign_runtime, foreign_plan, foreign_inventory) = drop_create_inputs();
    let foreign_reconciliation = foreign_runtime
        .reconcile_durable_state(&foreign_plan, &foreign_inventory)
        .expect("foreign reconciliation succeeds");

    let denial = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("foreign state plan")
                .with_state_reconciliation_plan(
                    "foreign state receipt",
                    foreign_plan,
                    foreign_reconciliation,
                ),
            app.capabilities(),
        )
        .expect_err("foreign active state plan denies");

    match denial.reason() {
        WorthUiIdentityStateQueryCertificationDenialReason::StatePlanActiveRuntimeMismatch {
            label,
            active_runtime_artifact_digest,
            plan_active_artifact_digest,
        } => {
            assert_eq!(label, "foreign state receipt");
            assert_ne!(active_runtime_artifact_digest, plan_active_artifact_digest);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn query_certification_rejects_rebind_plan_from_another_active_runtime() {
    let (runtime, _, _) = single_active_state_lifecycle_inputs();
    let app = identity_match_app();
    let (_, foreign_rebind_plan) = ui_local_drift_rebind_plan();

    let denial = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("foreign query plan")
                .with_query_rebind_plan_expecting_denial(
                    "foreign query rebind",
                    foreign_rebind_plan,
                    WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery,
                ),
            app.capabilities(),
        )
        .expect_err("foreign active query plan denies");

    match denial.reason() {
        WorthUiIdentityStateQueryCertificationDenialReason::QueryPlanActiveRuntimeMismatch {
            label,
            active_runtime_artifact_digest,
            plan_active_artifact_digest,
        } => {
            assert_eq!(label, "foreign query rebind");
            assert_ne!(active_runtime_artifact_digest, plan_active_artifact_digest);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn state_and_query_residue_scan_clean_after_failed_and_successful_reload_mix() {
    let (runtime, state_plan, state_inventory, query_rebind_plan) =
        query_runtime_state_and_rebind_inputs();
    let app = standard_query_app();
    let ambiguous_plan = ambiguous_plan_for_same_active(&state_plan);
    let ambiguous_denial = runtime
        .reconcile_durable_state(&ambiguous_plan, &state_inventory)
        .expect_err("ambiguous identity denies");
    let state_reconciliation = runtime
        .reconcile_durable_state(&state_plan, &state_inventory)
        .expect("state reconciliation succeeds");

    let certification = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("mixed reload storm")
                .with_state_reconciliation_denial(
                    "ambiguous failed reload",
                    ambiguous_plan,
                    ambiguous_denial,
                )
                .with_state_reconciliation_plan(
                    "successful state reload",
                    state_plan,
                    state_reconciliation,
                )
                .with_query_rebind_plan_expecting_denial(
                    "typed query recovery stop",
                    query_rebind_plan,
                    WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery,
                )
                .with_strict_residue_scan(),
            app.capabilities(),
        )
        .expect("mixed failed/successful certification has clean residue");

    assert!(certification.residue_scan().is_clean());
    assert_eq!(
        certification.counters().ambiguous_identity_denial_count(),
        1
    );
    assert_eq!(
        certification.query_drift().ui_local_recovery_denial_count(),
        1
    );
    assert!(certification.residue_scan().scanned_state_receipts() > 0);
    assert!(certification.residue_scan().scanned_query_bindings() > 0);
}
