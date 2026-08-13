use std::sync::Arc;

use crate::facade::{
    BridgeManagedClockInstallationParts, BridgeManagedClockObservationOutcome,
    BridgeManagedClockObservationParts, BridgeManagedTemporalDenialKind,
    BridgeManagedTemporalIntentIdentity, BridgeManagedTemporalIntentLifecycle,
    BridgeManagedTemporalIntentReconciliation, BridgeManagedTemporalIntentReconciliationParts,
};

use super::{always_eligible_contract, install};

fn installed_clock(
    maximum_active_intents: usize,
    maximum_due_wakes_per_observation: usize,
) -> (
    crate::facade::BridgeOwnedSignalRuntime,
    crate::facade::BridgeManagedClockBinding,
) {
    let (mut owner, _) = install(always_eligible_contract("query:one"), "managed-time");
    let binding = owner
        .install_managed_clock(BridgeManagedClockInstallationParts {
            binding_identity: Arc::from("query:clock:billing"),
            source_identity: Arc::from("host:clock:billing"),
            timeline_identity: Arc::from("timeline:billing:v1"),
            maximum_active_intents,
            maximum_due_wakes_per_observation,
        })
        .expect("exact managed clock installs");
    (owner, binding)
}

fn active<'a>(
    binding: &'a crate::facade::BridgeManagedClockBinding,
    identity: &str,
    revision: u64,
    due_coordinate: u64,
) -> BridgeManagedTemporalIntentReconciliationParts<'a> {
    BridgeManagedTemporalIntentReconciliationParts {
        binding,
        identity: BridgeManagedTemporalIntentIdentity::declare(Arc::from(identity)).unwrap(),
        revision,
        due_coordinate,
        idempotency_identity: Arc::from(format!("idempotency:{identity}")),
        lifecycle: BridgeManagedTemporalIntentLifecycle::Active,
    }
}

#[test]
fn intent_reconciliation_is_revisioned_capacity_bounded_and_effect_safe() {
    let (mut owner, binding) = installed_clock(2, 2);
    assert_eq!(
        owner
            .reconcile_managed_temporal_intent(active(&binding, "intent:a", 1, 5))
            .unwrap(),
        BridgeManagedTemporalIntentReconciliation::Installed
    );
    assert_eq!(
        owner
            .reconcile_managed_temporal_intent(active(&binding, "intent:a", 1, 5))
            .unwrap(),
        BridgeManagedTemporalIntentReconciliation::Duplicate
    );
    let conflict = owner
        .reconcile_managed_temporal_intent(active(&binding, "intent:a", 1, 6))
        .expect_err("same revision cannot change due meaning");
    assert_eq!(
        conflict.kind(),
        BridgeManagedTemporalDenialKind::IntentRevisionConflict
    );
    assert_eq!(
        owner
            .reconcile_managed_temporal_intent(active(&binding, "intent:a", 2, 7))
            .unwrap(),
        BridgeManagedTemporalIntentReconciliation::Superseded
    );
    owner
        .reconcile_managed_temporal_intent(active(&binding, "intent:b", 1, 9))
        .unwrap();
    let saturated = owner
        .reconcile_managed_temporal_intent(active(&binding, "intent:c", 1, 11))
        .expect_err("capacity rejects before Signal scheduling");
    assert_eq!(
        saturated.kind(),
        BridgeManagedTemporalDenialKind::IntentCapacityExhausted
    );

    let closure = owner.close_managed_clock(binding).unwrap();
    assert_eq!(closure.active_intents(), 2);
    assert_eq!(closure.scheduled_wakes(), 2);
    assert_eq!(closure.ready_wakes(), 0);
}

#[test]
fn duplicate_observation_drains_only_the_remaining_bounded_due_frontier() {
    let (mut owner, binding) = installed_clock(3, 1);
    owner
        .reconcile_managed_temporal_intent(active(&binding, "intent:a", 1, 5))
        .unwrap();
    owner
        .reconcile_managed_temporal_intent(active(&binding, "intent:b", 1, 5))
        .unwrap();

    let observation = || BridgeManagedClockObservationParts {
        binding: &binding,
        source_identity: "host:clock:billing",
        timeline_identity: "timeline:billing:v1",
        sequence: 1,
        observed_coordinate: 5,
    };
    let BridgeManagedClockObservationOutcome::Accepted(first) =
        owner.observe_managed_clock(observation()).unwrap()
    else {
        panic!("first observation must advance the exact clock")
    };
    assert_eq!(first.due().wakes().len(), 1);
    assert!(first.due().due_work_remaining());
    assert_eq!(first.signal_advance_ordinal(), Some(1));

    let BridgeManagedClockObservationOutcome::Duplicate(second) =
        owner.observe_managed_clock(observation()).unwrap()
    else {
        panic!("exact repeated reading must be typed duplicate")
    };
    assert_eq!(second.due().wakes().len(), 1);
    assert!(!second.due().due_work_remaining());
    assert_eq!(second.signal_advance_ordinal(), None);
    assert_ne!(
        first.due().wakes()[0].intent_identity(),
        second.due().wakes()[0].intent_identity()
    );
}

#[test]
fn observation_affinity_and_ordering_fail_without_due_progress() {
    let (mut owner, binding) = installed_clock(1, 1);
    owner
        .reconcile_managed_temporal_intent(active(&binding, "intent:a", 1, 5))
        .unwrap();
    let accepted = owner
        .observe_managed_clock(BridgeManagedClockObservationParts {
            binding: &binding,
            source_identity: "host:clock:billing",
            timeline_identity: "timeline:billing:v1",
            sequence: 4,
            observed_coordinate: 3,
        })
        .unwrap();
    assert!(matches!(
        accepted,
        BridgeManagedClockObservationOutcome::Accepted(_)
    ));
    let stale = owner
        .observe_managed_clock(BridgeManagedClockObservationParts {
            binding: &binding,
            source_identity: "host:clock:billing",
            timeline_identity: "timeline:billing:v1",
            sequence: 3,
            observed_coordinate: 5,
        })
        .unwrap();
    assert!(matches!(stale, BridgeManagedClockObservationOutcome::Stale));
    let reordered = owner
        .observe_managed_clock(BridgeManagedClockObservationParts {
            binding: &binding,
            source_identity: "host:clock:billing",
            timeline_identity: "timeline:billing:v1",
            sequence: 5,
            observed_coordinate: 2,
        })
        .unwrap();
    assert!(matches!(
        reordered,
        BridgeManagedClockObservationOutcome::Reordered
    ));
    let foreign = match owner.observe_managed_clock(BridgeManagedClockObservationParts {
        binding: &binding,
        source_identity: "host:clock:foreign",
        timeline_identity: "timeline:billing:v1",
        sequence: 5,
        observed_coordinate: 5,
    }) {
        Ok(_) => panic!("foreign source cannot advance or promote"),
        Err(denial) => denial,
    };
    assert_eq!(
        foreign.kind(),
        BridgeManagedTemporalDenialKind::ForeignClockSource
    );
}

#[test]
fn successor_runtime_requires_managed_clock_rebinding() {
    let (owner, binding) = installed_clock(1, 1);
    let mut successor = owner.successor_installation_runtime().unwrap();
    let denial = match successor.observe_managed_clock(BridgeManagedClockObservationParts {
        binding: &binding,
        source_identity: "host:clock:billing",
        timeline_identity: "timeline:billing:v1",
        sequence: 1,
        observed_coordinate: 1,
    }) {
        Ok(_) => panic!("successor cannot reuse predecessor clock authority"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial.kind(),
        BridgeManagedTemporalDenialKind::ForeignClockBinding
    );
}
