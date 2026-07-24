use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, read};

use super::conditional_node_contract::{
    conditional_node_result, distance_dependency, node, threshold, Millimeters,
};
use super::conditional_threshold_execution::ThresholdCompute;
use super::dependency_impact::bind_direct;
use super::installed_operation_fixture::{
    conditional_installation_with_change, conditional_installation_with_repeated_value_changes,
    conditional_public_workspace_with, DirectConditionalCompute, GeometryDomain,
    ReadExecutionInput, ReadFamily, ReadVertex,
};
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn exact_current_conditional_decision_and_owner_receipt_mint_one_classified_delta() {
    let node = node(
        "invalidation-conditional-owner",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let expected_node = node.clone();
    let (installation, request, snapshots) = conditional_installation_with_change(&node);
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let mut workspace = conditional_public_workspace_with(
        "invalidation-conditional-owner",
        node,
        installation,
        DirectConditionalCompute,
        &harness,
    )
    .unwrap();
    let subject = settle(&mut workspace);
    let live = match subject.into_lifecycle().promote(&mut workspace) {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("conditional invalidation subject did not promote"),
    };
    let subject = match live.into_managed_lease(&mut workspace) {
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Admitted(lease) => lease,
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Stopped(stop) => {
            panic!("conditional invalidation lease stopped: {}", stop.detail())
        }
    };
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
        panic!("conditional owner change did not reach the shared owner")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());

    let subject_delivery =
        match subject.drain_conditional_owner_delivery(&owner_delivery, &mut workspace) {
            Ok(delivery) => delivery,
            Err(_) => panic!("exact conditional owner delivery did not readmit"),
        };
    assert_eq!(
        subject_delivery.impact().class(),
        domain::WorthQueryImpactClass::ValuePatch
    );
    assert_eq!(subject_delivery.counters().conditional_compute_contacts, 1);
    assert_eq!(subject_delivery.counters().impact_classifications, 1);
    assert_eq!(
        subject_delivery
            .drain_counters()
            .workspace_capability_checks,
        1
    );
    assert_eq!(
        subject_delivery
            .drain_counters()
            .abandoned_owner_index_lookups,
        1
    );
    assert_eq!(subject_delivery.drain_counters().owner_index_lookups, 1);
    assert_eq!(subject_delivery.drain_counters().lease_index_lookups, 2);
    assert_eq!(
        subject_delivery.drain_counters().sharing_readmission_checks,
        1
    );
    assert_eq!(subject_delivery.drain_counters().epoch_compilations, 1);
    assert_eq!(subject_delivery.drain_counters().unrelated_owner_scans, 0);
    assert!(subject_delivery.conditional_decision().is_some());
    assert!(!subject_delivery.conditional_provenance().is_empty());

    let subject_delta = subject
        .consumer_invalidation_delta(subject_delivery)
        .unwrap();
    let semantic = subject_delta.semantic_projection();
    assert_eq!(semantic.conditional_path().len(), 1);
    let semantic_decision = semantic
        .conditional_decision()
        .expect("semantic delta retains the current conditional decision");
    assert_eq!(semantic_decision.location(), &location);
    assert_eq!(semantic_decision.declaration(), &expected_node);
    assert_eq!(
        semantic_decision.outcome(),
        domain::WorthQueryConditionalOutcomeClass::ComputedChanged
    );
    assert_eq!(
        semantic_decision.observations().len(),
        subject_delta
            .conditional_decision()
            .unwrap()
            .semantic_observation_count()
    );
    assert_eq!(subject_delta.epoch_counters().capability_index_lookups, 1);
    assert_eq!(
        subject_delta.conditional_decision().unwrap().location(),
        &location
    );
    assert_eq!(
        subject_delta.conditional_decision().unwrap().class(),
        domain::WorthQueryConditionalOutcomeClass::ComputedChanged
    );
    assert_eq!(
        subject_delta.disposition(),
        domain::WorthQueryConsumerInvalidationDisposition::Unsupported
    );
    assert!(matches!(
        subject_delta.cause(),
        domain::WorthQueryConsumerInvalidationCause::NativeNarrowingUnavailable(_)
    ));
    let admitted = match subject.admit_consumer_invalidation_delta(subject_delta, &workspace) {
        Ok(admitted) => admitted,
        Err(_) => panic!("current unsupported delta did not readmit for consequence policy"),
    };
    let downgrade = match admitted.attach_consumer_authored_consequence(
        &workspace,
        domain::WorthQueryConsumerInvalidationDisposition::LocalPatch,
        "patch-anyway",
    ) {
        Err(stop) => stop,
        Ok(_) => panic!("unsupported Query meaning was downgraded to a local patch"),
    };
    assert_eq!(
        downgrade.required_disposition(),
        domain::WorthQueryConsumerInvalidationDisposition::Unsupported
    );
    assert_eq!(
        downgrade.requested_disposition(),
        domain::WorthQueryConsumerInvalidationDisposition::LocalPatch
    );
    assert_eq!(downgrade.into_consumer_authored(), "patch-anyway");
}

#[test]
fn suppressed_condition_omits_its_consequence_but_preserves_a_direct_projection_change() {
    let dependency = distance_dependency();
    let node = conditional_node_result(
        "invalidation-conditional-suppressed",
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
    let mut workspace = conditional_public_workspace_with(
        "invalidation-conditional-suppressed",
        node,
        installation,
        ThresholdCompute(Arc::clone(&contacts)),
        &harness,
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    assert!(matches!(
        bind_direct(&workspace, &installed)
            .admit_execution_resources(
                ReadExecutionInput::default(),
                crate::suite::installed_operation_fixture::execution_resource_request(),
                &workspace
            )
            .unwrap()
            .execute(&mut workspace),
        TransitionOutcome::Deferred(_)
    ));
    let TransitionOutcome::Success(_first_delivery) = workspace
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
        _ => panic!("threshold projection did not promote"),
    };
    let lease = match live.into_managed_lease(&mut workspace) {
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Admitted(lease) => lease,
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Stopped(stop) => {
            panic!("threshold lease stopped: {}", stop.detail())
        }
    };
    let before = contacts.load(Ordering::SeqCst);
    let TransitionOutcome::Success(repeated_delivery) = workspace
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
    let delivery = match lease.drain_conditional_owner_delivery(&repeated_delivery, &mut workspace)
    {
        Ok(delivery) => delivery,
        Err(_) => panic!("suppressed owner delivery did not complete"),
    };
    assert_eq!(contacts.load(Ordering::SeqCst), before);
    assert_eq!(delivery.counters().conditional_compute_contacts, 0);
    assert_eq!(
        delivery.conditional_decision().unwrap().class(),
        domain::WorthQueryConditionalOutcomeClass::Suppressed
    );
    assert_eq!(
        delivery.impact().affected_roles(),
        [domain::WorthQuerySemanticDependencyRole::ProjectedValue]
    );
    assert_eq!(
        delivery.impact().class(),
        domain::WorthQueryImpactClass::ValuePatch
    );
    assert_eq!(delivery.counters().this_lease_semantic_delivery, 1);
    let delta = lease.consumer_invalidation_delta(delivery).unwrap();
    assert_eq!(
        delta.disposition(),
        domain::WorthQueryConsumerInvalidationDisposition::Unsupported
    );
}

pub(super) fn settle(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
) -> domain::WorthQuerySettledDomainProjection<
    GeometryDomain,
    ReadVertex,
    ReadFamily,
    worth_query::facade::foundation::ObservationLaneWitness,
> {
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = bind_direct(workspace, &installed);
    let consumer = bound.consumer_projection_contract().unwrap();
    bound
        .admit_execution_resources(
            ReadExecutionInput::default(),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &*workspace,
        )
        .unwrap()
        .execute(workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap()
}
