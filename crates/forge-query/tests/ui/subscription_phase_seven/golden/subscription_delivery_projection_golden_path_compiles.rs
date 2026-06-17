use forge_query::facade::{QueryDeliveryWindow, SubscriptionConsumerAttachment};

fn delivery_projection_golden_path(
    window: &QueryDeliveryWindow,
    attachment: &SubscriptionConsumerAttachment,
) {
    let _ = window.delivery_window_projection().label();
    let _ = window.active_lane_projection().label();
    let _ = window.attachment_projection().label();
    let _ = attachment.lane_projection().label();
    let _ = attachment.attachment_projection().label();
}

fn main() {}
