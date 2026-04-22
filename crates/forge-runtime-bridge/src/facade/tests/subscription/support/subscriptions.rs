use super::*;

pub(crate) fn activation_ready_detail_subscription() -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeSubscriptionActivationReady,
) {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let ready = activation_ready_detail_subscription_in_runtime(&runtime);
    (runtime, ready)
}

pub(crate) fn activation_ready_detail_subscription_in_runtime(
    runtime: &crate::facade::RuntimeBridge,
) -> crate::facade::BridgeSubscriptionActivationReady {
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
    let admitted = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("admission should succeed");
    runtime.prepare_subscription_activation(&admitted)
}

pub(crate) fn activation_ready_collection_subscription() -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeSubscriptionActivationReady,
) {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west",
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("declaration should succeed");
    let admitted = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("admission should succeed");
    let ready = runtime.prepare_subscription_activation(&admitted);
    (runtime, ready)
}

pub(crate) fn active_detail_subscription(
    posture: BridgeSubscriptionDeliveryDensityPosture,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    active_detail_subscription_with_fanout(posture, 1)
}

pub(crate) fn active_detail_subscription_with_fanout(
    posture: BridgeSubscriptionDeliveryDensityPosture,
    max_fanout_width: usize,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    let (runtime, ready) = activation_ready_detail_subscription();
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(posture, 4, 4, max_fanout_width)
        .expect("cost profile should admit");
    let consumer = canonical_consumer_contract(&runtime);
    let active = runtime.activate_subscription_delivery(ready, cost_profile, consumer);
    (runtime, active)
}

pub(crate) fn active_detail_subscription_with_member_limit(
    posture: BridgeSubscriptionDeliveryDensityPosture,
    max_member_count: usize,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    let (runtime, ready) = activation_ready_detail_subscription();
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(posture, max_member_count, max_member_count, 1)
        .expect("cost profile should admit");
    let consumer = canonical_consumer_contract(&runtime);
    let active = runtime.activate_subscription_delivery(ready, cost_profile, consumer);
    (runtime, active)
}

pub(crate) fn active_detail_subscription_with_consumer(
    posture: BridgeSubscriptionDeliveryDensityPosture,
    max_fanout_width: usize,
    consumer: crate::facade::BridgeSubscriptionConsumerContract,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    let (runtime, ready) = activation_ready_detail_subscription();
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(posture, 4, 4, max_fanout_width)
        .expect("cost profile should admit");
    let active = runtime.activate_subscription_delivery(ready, cost_profile, consumer);
    (runtime, active)
}

pub(crate) fn active_collection_subscription(
    posture: BridgeSubscriptionDeliveryDensityPosture,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    active_collection_subscription_with_fanout(posture, 1)
}

pub(crate) fn active_collection_subscription_with_fanout(
    posture: BridgeSubscriptionDeliveryDensityPosture,
    max_fanout_width: usize,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west",
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("declaration should succeed");
    let admitted = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("admission should succeed");
    let ready = runtime.prepare_subscription_activation(&admitted);
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(posture, 4, 4, max_fanout_width)
        .expect("cost profile should admit");
    let consumer = canonical_consumer_contract(&runtime);
    let active = runtime.activate_subscription_delivery(ready, cost_profile, consumer);
    (runtime, active)
}

pub(crate) fn canonical_consumer_contract(
    runtime: &crate::facade::RuntimeBridge,
) -> crate::facade::BridgeSubscriptionConsumerContract {
    runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("consumer contract should admit")
}
