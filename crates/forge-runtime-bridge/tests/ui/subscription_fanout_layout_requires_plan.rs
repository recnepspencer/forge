use forge_runtime_bridge::facade::{
    BridgeSubscriptionConsumerContract, BridgeSubscriptionDeliveryFamilyKind, RuntimeBridge,
};

fn cannot_build_layout_from_raw_consumers(
    runtime: &RuntimeBridge,
    consumers: Vec<BridgeSubscriptionConsumerContract>,
) {
    let _ = runtime
        .build_subscription_fanout_layout(consumers, BridgeSubscriptionDeliveryFamilyKind::CanonicalMember);
}

fn main() {}
