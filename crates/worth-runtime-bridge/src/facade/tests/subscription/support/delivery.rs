use super::*;

pub(crate) fn sealed_window(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
) -> crate::facade::BridgeSubscriptionDeliveryWindowSealed {
    sealed_window_with_member(
        runtime,
        active,
        family_kind,
        0,
        BridgeSubscriptionDeliveryMemberInput::delivery_content_digest(
            "slice:entity-1/profile/name",
            "routing:fixture",
            BridgeSubscriptionDeliveryMemberClass::Update,
            BridgeSubscriptionDeliveryContentDigest::admit_bridge_owned("content:fixture"),
        ),
    )
}

pub(crate) fn sealed_window_with_member(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
    delivery_window_sequence: u64,
    member: BridgeSubscriptionDeliveryMemberInput,
) -> crate::facade::BridgeSubscriptionDeliveryWindowSealed {
    let open =
        runtime.open_subscription_delivery_window(active, family_kind, delivery_window_sequence);
    runtime
        .seal_subscription_delivery_window(open, vec![member])
        .expect("delivery window should seal")
}

pub(crate) fn sealed_window_with_members(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
    delivery_window_sequence: u64,
    members: Vec<BridgeSubscriptionDeliveryMemberInput>,
) -> crate::facade::BridgeSubscriptionDeliveryWindowSealed {
    let open =
        runtime.open_subscription_delivery_window(active, family_kind, delivery_window_sequence);
    runtime
        .seal_subscription_delivery_window(open, members)
        .expect("delivery window should seal")
}

pub(crate) fn fixture_members(count: usize) -> Vec<BridgeSubscriptionDeliveryMemberInput> {
    (0..count)
        .map(|index| {
            BridgeSubscriptionDeliveryMemberInput::delivery_content_digest(
                "slice:entity-1/profile/name",
                format!("routing:fixture:{index}"),
                BridgeSubscriptionDeliveryMemberClass::Update,
                BridgeSubscriptionDeliveryContentDigest::admit_bridge_owned(format!(
                    "content:fixture:{index}"
                )),
            )
        })
        .collect()
}

pub(crate) fn checkpoint_from_sealed(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    sealed: &crate::facade::BridgeSubscriptionDeliveryWindowSealed,
    acknowledged_sequence: usize,
    duplicate_replay_policy_kind: crate::facade::BridgeSubscriptionDuplicateReplayPolicyKind,
) -> crate::facade::BridgeSubscriptionCheckpoint {
    let acknowledged = &sealed.members()[acknowledged_sequence];
    let frontier = runtime
        .admit_subscription_acknowledgement_frontier(sealed, acknowledged_sequence, acknowledged)
        .expect("frontier should admit");
    runtime
        .publish_subscription_checkpoint(
            runtime.prepare_subscription_checkpoint(frontier),
            active,
            duplicate_replay_policy_kind,
        )
        .expect("checkpoint should publish")
}

pub(crate) fn fanout_checkpoint_from_sealed(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    sealed: &crate::facade::BridgeSubscriptionDeliveryWindowSealed,
    fanout_layout: &crate::facade::BridgeSubscriptionFanoutLayout,
    acknowledged_sequence: usize,
    duplicate_replay_policy_kind: crate::facade::BridgeSubscriptionDuplicateReplayPolicyKind,
) -> crate::facade::BridgeSubscriptionCheckpoint {
    let acknowledged = &sealed.members()[acknowledged_sequence];
    let frontier = runtime
        .admit_subscription_acknowledgement_frontier(sealed, acknowledged_sequence, acknowledged)
        .expect("frontier should admit");
    runtime
        .publish_subscription_fanout_checkpoint(
            runtime.prepare_subscription_checkpoint(frontier),
            active,
            fanout_layout,
            duplicate_replay_policy_kind,
        )
        .expect("fanout checkpoint should publish")
}
