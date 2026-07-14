use worth_query::facade::runtime::{QueryDeliveryError, SubscriptionConsumerAttachmentError, SubscriptionContinuationError};

fn error_source_projection_golden_path(
    delivery: &QueryDeliveryError,
    attachment: &SubscriptionConsumerAttachmentError,
    continuation: &SubscriptionContinuationError,
) {
    let _ = delivery.source_projection().label();
    let _ = attachment.source_projection().label();
    let _ = continuation.source_projection().label();
}

fn main() {}
