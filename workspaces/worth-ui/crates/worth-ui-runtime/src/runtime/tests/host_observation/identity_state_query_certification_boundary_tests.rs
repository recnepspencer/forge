use super::durable_state_reconciliation_test_support::{
    ambiguous_plan_with_inventory, custom_inventory_for_policy, drop_create_inputs,
};
use super::identity_match_graph_test_support::identity_match_app;
use super::identity_state_query_certification_test_support::{
    ambiguous_plan_for_same_active, single_active_state_lifecycle_inputs,
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
                    WorthUiQueryBindingDriftDenialKind::MissingCandidateUiRequirementsForRebind,
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
fn strict_query_residue_scan_reads_the_active_plan_and_binding_stores() {
    let (runtime, rebind_plan) = ui_local_drift_rebind_plan();
    let app = standard_query_app();
    let certification = runtime
        .certify_identity_state_and_query_drift_against_snapshot(
            WorthUiIdentityStateQueryCertificationScenario::named("active Query residue")
                .with_query_rebind_plan("typed UI requirement drift", rebind_plan)
                .with_strict_residue_scan(),
            app.capabilities(),
        )
        .expect("the actual active Query stores are coherent");

    let scan = certification.residue_scan();
    assert_eq!(scan.scanned_query_bindings(), 2);
    assert_eq!(scan.scanned_plan_query_links(), 1);
    assert_eq!(scan.scanned_settled_snapshots(), 0);
    assert_eq!(scan.scanned_live_resources(), 0);
    assert!(scan.is_clean());
}
