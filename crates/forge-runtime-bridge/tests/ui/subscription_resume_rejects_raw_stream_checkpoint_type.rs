use forge_runtime_bridge::facade::{BridgeActiveSubscription, RuntimeBridge};

fn admit_raw_stream_checkpoint(
    runtime: &RuntimeBridge,
    active_subscription: &BridgeActiveSubscription,
    raw_stream_checkpoint: &str,
) {
    let _ = runtime.admit_subscription_resume(active_subscription, raw_stream_checkpoint);
}

fn main() {}
