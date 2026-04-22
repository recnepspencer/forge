use forge_runtime_bridge::facade::{
    BridgeActiveSubscription, BridgeSubscriptionDeliveryWindowSealed, RuntimeBridge,
};

fn cannot_project_from_raw_active_subscription(
    runtime: &RuntimeBridge,
    active: &BridgeActiveSubscription,
    sealed: &BridgeSubscriptionDeliveryWindowSealed,
) {
    let _ = runtime.project_subscription_delivery_to_fanout(active, sealed);
}

fn main() {}
