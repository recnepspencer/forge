use super::*;

pub(crate) fn shared_delivery_bundle(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
) -> crate::facade::BridgeSharedConsumerDeliveryBundleSealed {
    shared_delivery_bundle_for_consumers(
        runtime,
        active,
        family_kind,
        vec![canonical_consumer_contract(runtime)],
    )
}

pub(crate) fn shared_delivery_bundle_for_consumers(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
    consumers: Vec<crate::facade::BridgeSubscriptionConsumerContract>,
) -> crate::facade::BridgeSharedConsumerDeliveryBundleSealed {
    let fanout_plan = runtime
        .plan_shared_subscription_fanout(active, consumers)
        .expect("fanout plan should admit");
    let fanout_layout = runtime.build_subscription_fanout_layout(fanout_plan, family_kind);
    let truth_patch = committed_patch(
        crate::truth_identity_fixtures::truth_branch_fixture("truth-main"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
    );
    let ordering =
        runtime.order_mixed_causes(&crate::facade::BridgeMixedCauseOrderingRequest::new(
            crate::facade::BridgeMixedCauseOrderingLaneKind::Authoritative,
            vec![crate::facade::BridgeMixedCauseOrderingInput::TruthPatch(
                truth_patch,
            )],
        ));
    let mixed_window = runtime
        .plan_mixed_cause_delivery_window(&ordering, family_kind)
        .expect("mixed cause window should plan");
    let plan = runtime
        .plan_shared_subscription_delivery(active, &mixed_window, &fanout_layout)
        .expect("shared delivery plan should admit");
    let layout = runtime.build_shared_subscription_delivery_layout(&plan);
    let draft = runtime.draft_shared_delivery_bundle(&layout);
    runtime.seal_shared_delivery_bundle(draft)
}
