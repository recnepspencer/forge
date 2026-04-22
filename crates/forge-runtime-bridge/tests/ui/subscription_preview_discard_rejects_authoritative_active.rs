use forge_runtime_bridge::facade::{
    BridgeActiveSubscription, BridgeSubscriptionPreviewResidueScopeIndex, RuntimeBridge,
};

fn discard_authoritative_active(
    runtime: &RuntimeBridge,
    active: BridgeActiveSubscription,
    residue_index: BridgeSubscriptionPreviewResidueScopeIndex,
) {
    let _ = runtime.discard_preview_subscription(active, residue_index);
}

fn main() {}
