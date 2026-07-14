use worth_query::facade::runtime::PreviewSubscriptionPromotionHandoff;

fn main() {
    let _ = PreviewSubscriptionPromotionHandoff {
        preview_lane_digest: todo!(),
        authoritative_active_lane_digest: todo!(),
        attachment_digest: todo!(),
        preview_epoch_digest: "epoch".to_string(),
        authority_digest: "authority".to_string(),
        counters: todo!(),
        handoff_digest: "handoff".to_string(),
    };
}
