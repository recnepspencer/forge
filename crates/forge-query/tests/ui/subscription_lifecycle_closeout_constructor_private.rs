use forge_query::facade::{
    ActiveSubscriptionCounters, ActiveSubscriptionLaneDigest, QuerySubscriptionSupportProfile,
    SubscriptionLifecycleCloseout, SubscriptionLifecycleCloseoutKind, SubscriptionPerformanceReceipt,
    SubscriptionConsumerAttachmentDigest,
};

fn main() {
    let _ = SubscriptionLifecycleCloseout {
        active_lane_digest: todo!() as ActiveSubscriptionLaneDigest,
        attachment_digest: todo!() as SubscriptionConsumerAttachmentDigest,
        closeout_kind: SubscriptionLifecycleCloseoutKind::ConsumerTerminated,
        lane_terminal: true,
        support_profile: todo!() as QuerySubscriptionSupportProfile,
        performance_receipt: todo!() as SubscriptionPerformanceReceipt,
        counters: ActiveSubscriptionCounters::default(),
        source_identity: todo!(),
        closeout_identity: todo!(),
    };
}
