use crate::facade::{
    BridgeSubscriptionDeliveryContentDigest, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryMemberClass, BridgeSubscriptionDeliveryMemberInput,
    BridgeSubscriptionDuplicateReplayPolicyKind, RuntimeBridge,
};

pub(crate) fn fixture_members(count: usize) -> Vec<BridgeSubscriptionDeliveryMemberInput> {
    (0..count)
        .map(|index| {
            BridgeSubscriptionDeliveryMemberInput::delivery_content_digest(
                "slice:entity-1/profile/name",
                format!("routing:harness:{index}"),
                BridgeSubscriptionDeliveryMemberClass::Update,
                BridgeSubscriptionDeliveryContentDigest::new(format!("content:harness:{index}")),
            )
        })
        .collect()
}

pub(crate) fn sealed_window_with_members(
    runtime: &RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
    sequence: u64,
    members: Vec<BridgeSubscriptionDeliveryMemberInput>,
) -> crate::facade::BridgeSubscriptionDeliveryWindowSealed {
    let open = runtime.open_subscription_delivery_window(active, family_kind, sequence);
    runtime
        .seal_subscription_delivery_window(open, members)
        .expect("delivery window should seal")
}

pub(crate) fn checkpoint_from_sealed(
    runtime: &RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    sealed: &crate::facade::BridgeSubscriptionDeliveryWindowSealed,
    acknowledged_sequence: usize,
    duplicate_policy: BridgeSubscriptionDuplicateReplayPolicyKind,
) -> crate::facade::BridgeSubscriptionCheckpoint {
    let acknowledged = &sealed.members()[acknowledged_sequence];
    let frontier = runtime
        .admit_subscription_acknowledgement_frontier(sealed, acknowledged_sequence, acknowledged)
        .expect("acknowledgement frontier should admit");
    let ready = runtime.prepare_subscription_checkpoint(frontier);
    runtime
        .publish_subscription_checkpoint(ready, active, duplicate_policy)
        .expect("checkpoint should publish")
}
