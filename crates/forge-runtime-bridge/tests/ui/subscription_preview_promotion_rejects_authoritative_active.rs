use forge_runtime_bridge::facade::{
    BridgeActiveSubscription, BridgePreviewPromotionRecord, BridgeSubscriptionActivationReady,
    BridgeSubscriptionPreviewWorkTrace, RuntimeBridge,
};

fn promote_authoritative_active(
    runtime: &RuntimeBridge,
    active: BridgeActiveSubscription,
    work_trace: &BridgeSubscriptionPreviewWorkTrace,
    promotion_record: &BridgePreviewPromotionRecord,
    promoted_ready: &BridgeSubscriptionActivationReady,
) {
    let _ = runtime.promote_preview_subscription(active, work_trace, promotion_record, promoted_ready);
}

fn main() {}
