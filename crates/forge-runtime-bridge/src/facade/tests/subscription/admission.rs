use super::support::*;

#[test]
fn runtime_admits_detail_exact_subscription_against_current_snapshot_basis() {
    let (_runtime, ready) = activation_ready_detail_subscription();
    let admitted = ready.admitted();

    assert_eq!(
        admitted.declaration().requested_family_kind(),
        BridgeSubscriptionDeclarationFamilyKind::DetailExact
    );
    assert_eq!(
        admitted.basis_binding().basis_kind(),
        BridgeSubscriptionBasisKind::Snapshot
    );
    assert_eq!(admitted.counters().admitted_subscription_count(), 1);
}

#[test]
fn runtime_admits_collection_membership_subscription_against_snapshot_basis() {
    let (_runtime, ready) = activation_ready_collection_subscription();
    let admitted = ready.admitted();

    assert_eq!(
        admitted.declaration().requested_family_kind(),
        BridgeSubscriptionDeclarationFamilyKind::CollectionMembership
    );
    assert_eq!(
        admitted.basis_binding().basis_kind(),
        BridgeSubscriptionBasisKind::Snapshot
    );
    assert_eq!(ready.counters().lifecycle_record_count(), 1);
    assert_eq!(
        ready.lifecycle_record().admitted_subscription_identity(),
        admitted.admitted_subscription_identity()
    );
}

#[test]
fn runtime_rejects_subscription_admission_when_snapshot_basis_cannot_bind() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "name",
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("declaration should succeed");

    let rejection = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("missing")),
        )
        .expect_err("unknown snapshot should reject admission");

    assert_eq!(
        rejection.rejection_kind(),
        BridgeSubscriptionAdmissionRejectionKind::BasisResolutionFailure
    );
    assert_eq!(rejection.counters().basis_rejection_count(), 1);
}

#[test]
fn runtime_prepares_and_inspects_activation_ready_subscription() {
    let (runtime, ready) = activation_ready_detail_subscription();

    let explanation = runtime.inspect_activation_ready_subscription(&ready);

    assert_eq!(
        explanation.admitted_subscription_identity(),
        Some(ready.admitted().admitted_subscription_identity())
    );
    assert_eq!(ready.counters().lifecycle_record_count(), 1);
    assert_eq!(explanation.counters().diagnostics_bundle_count(), 1);
}

#[test]
fn runtime_deactivates_and_replays_retained_subscription_bundle() {
    let (runtime, ready) = activation_ready_detail_subscription();
    let registry_identity = runtime.subscription_family_registry_identity().clone();

    let deactivated = runtime.deactivate_subscription(ready);
    let replay = runtime
        .replay_subscription(deactivated.retained_bundle())
        .expect("replay should succeed");

    assert_eq!(
        replay.retained_bundle().registry_identity(),
        &registry_identity
    );
    assert_eq!(replay.counters().replay_reconstruction_count(), 1);
}
