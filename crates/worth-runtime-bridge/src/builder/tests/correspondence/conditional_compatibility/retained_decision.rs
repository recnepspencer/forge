use super::super::semantic_dependencies::always_eligible_contract;
use super::install;
use crate::facade::{BridgeConditionalDecisionReentryRequest, BridgeConditionalDenialKind};

#[test]
fn retained_decision_denies_a_foreign_bridge_runtime() {
    let (mut owner, lowering) = install(always_eligible_contract("query:one"), "bridge-main");
    let evidence = execute(&mut owner, &lowering, "snapshot-a");
    let seed = evidence.retain_for_reentry();
    let (foreign, _) = install(always_eligible_contract("query:one"), "bridge-main");

    let Err(denial) =
        foreign.reenter_retained_conditional_decision(reentry(&seed, &lowering, "snapshot-a"))
    else {
        panic!("another Bridge runtime cannot issue evidence from this seed")
    };
    assert_eq!(denial.kind(), BridgeConditionalDenialKind::StaleLowering);
    let counters = denial.reentry_counters();
    assert_eq!(counters.runtime_key_checks, 1);
    assert_eq!(counters.lowering_identity_checks, 0);
    assert_eq!(counters.installed_lowering_lookups, 0);
    assert_eq!(counters.signal_graph_checks, 0);
    assert_eq!(counters.snapshot_identity_checks, 0);
    assert_eq!(counters.query_continuation_rebindings, 0);
}

#[test]
fn retained_decision_denies_a_nonidentical_installed_lowering() {
    let (mut owner, lowering) = install(always_eligible_contract("query:one"), "bridge-main");
    let evidence = execute(&mut owner, &lowering, "snapshot-a");
    let seed = evidence.retain_for_reentry();
    let (_candidate_owner, candidate) =
        install(always_eligible_contract("query:one"), "bridge-main");

    let Err(denial) =
        owner.reenter_retained_conditional_decision(reentry(&seed, &candidate, "snapshot-a"))
    else {
        panic!("semantic equivalence cannot replace exact lowering identity")
    };
    assert_eq!(denial.kind(), BridgeConditionalDenialKind::StaleLowering);
    let counters = denial.reentry_counters();
    assert_eq!(counters.runtime_key_checks, 1);
    assert_eq!(counters.lowering_identity_checks, 1);
    assert_eq!(counters.installed_lowering_lookups, 1);
    assert_eq!(counters.signal_graph_checks, 0);
    assert_eq!(counters.unrelated_lowering_scans, 0);
}

#[test]
fn retained_decision_denies_snapshot_drift() {
    let (mut owner, lowering) = install(always_eligible_contract("query:one"), "bridge-main");
    let evidence = execute(&mut owner, &lowering, "snapshot-a");
    let seed = evidence.retain_for_reentry();

    let Err(denial) =
        owner.reenter_retained_conditional_decision(reentry(&seed, &lowering, "snapshot-b"))
    else {
        panic!("a retained decision cannot be relabeled for another snapshot")
    };
    assert_eq!(
        denial.kind(),
        BridgeConditionalDenialKind::SnapshotAdmission
    );
    let counters = denial.reentry_counters();
    assert_eq!(counters.runtime_key_checks, 1);
    assert_eq!(counters.installed_lowering_lookups, 1);
    assert_eq!(counters.signal_graph_checks, 1);
    assert_eq!(counters.signal_contract_checks, 1);
    assert_eq!(counters.snapshot_identity_checks, 2);
    assert_eq!(counters.query_continuation_rebindings, 0);
}

#[test]
fn copied_projections_cannot_change_query_continuation_authority() {
    let (mut owner, lowering) = install(always_eligible_contract("query:one"), "bridge-main");
    let evidence = execute(&mut owner, &lowering, "snapshot-a");
    let copied_signal_projection = evidence.signal().projection().label().to_string();
    let copied_lowering_projection = lowering.projection().label().to_string();

    assert!(!copied_signal_projection.is_empty());
    assert!(!copied_lowering_projection.is_empty());
    let counters = evidence.bridge_execution_counters();
    assert_eq!(counters.signal_graph_checks, 1);
    assert_eq!(counters.snapshot_admission_attempts, 0);
    assert_eq!(counters.compute_provider_checks, 1);
    assert_eq!(counters.signal_execution_contacts, 1);
    assert_eq!(counters.decisions_retained, 1);
    assert_eq!(counters.unrelated_lowering_scans, 0);
    assert_eq!(evidence.reentry_counters(), Default::default());
    let seed = evidence.retain_for_reentry();
    let reentered = owner
        .reenter_retained_conditional_decision(reentry(&seed, &lowering, "snapshot-a"))
        .unwrap();
    let counters = reentered.reentry_counters();
    assert_eq!(counters.runtime_key_checks, 1);
    assert_eq!(counters.lowering_identity_checks, 1);
    assert_eq!(counters.installed_lowering_lookups, 1);
    assert_eq!(counters.signal_graph_checks, 1);
    assert_eq!(counters.signal_contract_checks, 1);
    assert_eq!(counters.snapshot_identity_checks, 2);
    assert_eq!(counters.query_continuation_rebindings, 1);
    assert_eq!(counters.unrelated_lowering_scans, 0);
    assert!(evidence.admits_query_continuation(continuation(
        &lowering,
        "query-binding-a",
        "owner-delivery-a"
    )));
    assert!(!evidence.admits_query_continuation(continuation(
        &lowering,
        "query-binding-b",
        "owner-delivery-a"
    )));
    assert!(!evidence.admits_query_continuation(continuation(
        &lowering,
        "query-binding-a",
        "owner-delivery-b"
    )));
}

#[test]
fn stale_lowering_denial_reports_zero_downstream_bridge_and_signal_work() {
    let (_owner, lowering) = install(always_eligible_contract("query:one"), "bridge-main");
    let (mut foreign, _) = install(always_eligible_contract("query:one"), "bridge-main");
    let denial = match foreign.execute(
        crate::facade::BridgeConditionalExecutionRequest {
            lowering: &lowering,
            query_binding_identity: "query-binding-a",
            query_capability_identity: 1,
            snapshot_identity: "snapshot-a",
            truth_branch_identity: None,
            bridge_snapshot_identity: None,
            execution_identity: "owner-delivery-a",
            attempt: 1,
        },
        &mut (),
    ) {
        Err(denial) => denial,
        Ok(_) => panic!("foreign Signal graph admitted the lowering"),
    };
    let counters = denial.bridge_execution_counters();
    assert_eq!(counters.signal_graph_checks, 1);
    assert_eq!(counters.snapshot_admission_attempts, 0);
    assert_eq!(counters.compute_provider_checks, 0);
    assert_eq!(counters.signal_execution_contacts, 0);
    assert_eq!(counters.observation_baseline_writes, 0);
    assert_eq!(counters.decisions_retained, 0);
    assert_eq!(denial.signal_counters(), Default::default());
}

fn execute(
    owner: &mut crate::facade::BridgeOwnedSignalRuntime,
    lowering: &std::sync::Arc<crate::facade::BridgeInstalledConditionalLowering>,
    snapshot: &str,
) -> crate::facade::BridgeConditionalDecisionEvidence {
    owner
        .execute(
            crate::facade::BridgeConditionalExecutionRequest {
                lowering,
                query_binding_identity: "query-binding-a",
                query_capability_identity: 1,
                snapshot_identity: snapshot,
                truth_branch_identity: None,
                bridge_snapshot_identity: None,
                execution_identity: "owner-delivery-a",
                attempt: 1,
            },
            &mut (),
        )
        .unwrap()
}

fn reentry<'a>(
    seed: &'a crate::facade::BridgeRetainedConditionalDecisionSeed,
    lowering: &'a std::sync::Arc<crate::facade::BridgeInstalledConditionalLowering>,
    snapshot: &'a str,
) -> BridgeConditionalDecisionReentryRequest<'a> {
    BridgeConditionalDecisionReentryRequest {
        seed,
        lowering,
        query_binding_identity: "query-binding-b",
        query_capability_identity: 2,
        snapshot_identity: snapshot,
        bridge_snapshot_identity: None,
    }
}

fn continuation<'a>(
    lowering: &'a std::sync::Arc<crate::facade::BridgeInstalledConditionalLowering>,
    query_binding_identity: &'a str,
    execution_identity: &'a str,
) -> crate::facade::BridgeConditionalQueryContinuationAdmission<'a> {
    crate::facade::BridgeConditionalQueryContinuationAdmission {
        lowering,
        query_binding_identity,
        query_capability_identity: 1,
        signal_snapshot_projection: "snapshot-a",
        bridge_snapshot_identity: None,
        signal_execution_projection: execution_identity,
        attempt: 1,
    }
}
