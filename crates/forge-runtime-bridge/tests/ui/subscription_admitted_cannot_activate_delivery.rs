use forge_runtime_bridge::facade::{
    AdmittedBridgeSubscription, BridgeSubscriptionConsumerContract,
    BridgeSubscriptionDeliveryCostProfile, RuntimeBridge,
};

fn cannot_activate_delivery_from_raw_admitted_subscription(
    runtime: &RuntimeBridge,
    admitted: AdmittedBridgeSubscription,
    cost_profile: BridgeSubscriptionDeliveryCostProfile,
    consumer: BridgeSubscriptionConsumerContract,
) {
    let _ = runtime.activate_subscription_delivery(admitted, cost_profile, consumer);
}

fn main() {}
