use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use worth_proof::TransitionOutcome;
use worth_query::facade::domain;

use super::conditional_node_contract::{
    conditional_node_result, distance_dependency, threshold, GeometryCondition, Millimeters,
};
use super::conditional_threshold_execution::ThresholdCompute;
use super::dependency_impact::bind_direct;
use super::installed_operation_fixture::{
    conditional_installation_with_change, conditional_installation_with_repeated_value_changes,
    conditional_public_observe_workspace_with_invalidation, DirectConditionalCompute,
    GeometryDomain, ReadExecutionInput, ReadFamily, ReadVertex,
};
use super::operation_invalidation_conditional::settle;
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn condition_only_suppression_has_zero_compute_delivery_and_delta() {
    let dependency = distance_dependency();
    let node = conditional_node_result(
        "invalidation-condition-only-suppressed",
        dependency.clone(),
        domain::WorthQueryConditionalEvaluationCondition::delta_threshold(
            dependency,
            threshold::<Millimeters>(),
        ),
        domain::WorthQueryConditionalTrigger::DependencyChange,
        domain::WorthQueryMaintenancePosture::LazyUntilObserved,
    )
    .unwrap();
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (installation, requests, snapshots) =
        conditional_installation_with_repeated_value_changes(&node);
    let [changed_request, repeated_request] = requests;
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let contacts = Arc::new(AtomicUsize::new(0));
    let mut workspace = conditional_public_observe_workspace_with_invalidation(
        "invalidation-condition-only-suppressed",
        node,
        installation,
        ThresholdCompute(Arc::clone(&contacts)),
        &harness,
        domain::WorthQueryConsumerSupportPosture::Deferred,
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    assert!(matches!(
        bind_direct(&workspace, &installed).execute(ReadExecutionInput::default(), &mut workspace),
        TransitionOutcome::Deferred(_)
    ));
    let TransitionOutcome::Success(_) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                changed_request,
            ),
        )
        .unwrap()
    else {
        panic!("first threshold change did not stage")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let live = match settle(&mut workspace)
        .into_lifecycle()
        .promote(&mut workspace)
    {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("condition-only projection did not promote"),
    };
    let lease = match live.into_managed_lease(&mut workspace) {
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Admitted(lease) => lease,
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Stopped(stop) => {
            panic!("condition-only lease stopped: {}", stop.detail())
        }
    };
    let before = contacts.load(Ordering::SeqCst);
    let TransitionOutcome::Success(owner_delivery) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                repeated_request,
            ),
        )
        .unwrap()
    else {
        panic!("repeated threshold change did not stage")
    };
    harness.set_relational_snapshot(snapshots[2].snapshot_id(), snapshots[2].version_id());
    let delivery = match lease.drain_conditional_owner_delivery(&owner_delivery, &mut workspace) {
        Ok(delivery) => delivery,
        Err(_) => panic!("condition-only suppression did not complete"),
    };

    assert_eq!(contacts.load(Ordering::SeqCst), before);
    assert_eq!(delivery.counters().conditional_compute_contacts, 0);
    assert_eq!(
        delivery.impact().class(),
        domain::WorthQueryImpactClass::UnaffectedOrSuppressed
    );
    assert_eq!(delivery.counters().this_lease_semantic_delivery, 0);
    let stop = match lease.consumer_invalidation_delta(delivery) {
        Err(stop) => stop,
        Ok(_) => panic!("condition-only suppression minted an invalidation delta"),
    };
    assert_eq!(
        stop.kind(),
        domain::WorthQueryConsumerInvalidationDeltaStopKind::NoSemanticDelivery
    );
}

#[test]
fn deferred_owner_condition_returns_a_retained_typed_stop() {
    let dependency = distance_dependency();
    let node = conditional_node_result(
        "invalidation-owner-deferred",
        dependency,
        domain::WorthQueryConditionalEvaluationCondition::domain_specific::<GeometryCondition>([])
            .unwrap(),
        domain::WorthQueryConditionalTrigger::DependencyChange,
        domain::WorthQueryMaintenancePosture::LazyUntilObserved,
    )
    .unwrap();
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (mut installation, request, snapshots) = conditional_installation_with_change(&node);
    let eligible = Arc::new(AtomicBool::new(true));
    installation.providers = worth_runtime_bridge::facade::BridgeConditionalProviderSet::new()
        .condition(ToggleCondition(Arc::clone(&eligible)));
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let mut workspace = conditional_public_observe_workspace_with_invalidation(
        "invalidation-owner-deferred",
        node,
        installation,
        DirectConditionalCompute,
        &harness,
        domain::WorthQueryConsumerSupportPosture::Supported,
    )
    .unwrap();
    let live = match settle(&mut workspace)
        .into_lifecycle()
        .promote(&mut workspace)
    {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("eligible owner did not promote"),
    };
    let lease = match live.into_managed_lease(&mut workspace) {
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Admitted(lease) => lease,
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Stopped(stop) => {
            panic!("eligible owner lease stopped: {}", stop.detail())
        }
    };
    eligible.store(false, Ordering::SeqCst);
    let TransitionOutcome::Success(owner_delivery) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                request,
            ),
        )
        .unwrap()
    else {
        panic!("owner change did not stage")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let stopped = match lease.drain_conditional_owner_delivery(&owner_delivery, &mut workspace) {
        Err(stopped) => stopped,
        Ok(delivery) => panic!(
            "deferred owner condition emitted {:?} after {} compute contacts",
            delivery
                .conditional_decision()
                .map(|decision| decision.class()),
            delivery.counters().conditional_compute_contacts
        ),
    };
    assert!(stopped.owner_delivery_retained());
    assert_eq!(stopped.counters().workspace_capability_checks, 1);
    assert_eq!(stopped.counters().owner_index_lookups, 1);
    assert_eq!(stopped.counters().lease_index_lookups, 1);
    assert_eq!(stopped.counters().sharing_readmission_checks, 1);
    assert_eq!(stopped.counters().epoch_compilations, 0);
    assert_eq!(stopped.counters().unrelated_owner_scans, 0);
    let refresh = stopped
        .refresh_error()
        .expect("condition deferral is a typed refresh stop");
    assert_eq!(
        refresh.work().conditional_compute_contacts(),
        0,
        "deferred condition must stop before domain compute"
    );
}

struct ToggleCondition(Arc<AtomicBool>);

impl worth_runtime_bridge::facade::BridgeConditionalProviderSemantics for ToggleCondition {
    type SemanticContract = ();

    fn semantic_contract(&self) -> Self::SemanticContract {}
}

impl worth_runtime_bridge::facade::BridgeConditionalConditionProvider for ToggleCondition {
    fn resolve(
        &self,
        _declaration: &domain::WorthQueryPortableConditionalNodeDeclaration,
        _context: worth_runtime_bridge::facade::BridgeConditionalResolverContext,
    ) -> Result<worth_signal::facade::InstalledSignalConditionDecision, String> {
        Ok(if self.0.load(Ordering::SeqCst) {
            worth_signal::facade::InstalledSignalConditionDecision::Eligible
        } else {
            worth_signal::facade::InstalledSignalConditionDecision::Deferred
        })
    }
}
