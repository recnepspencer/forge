use worth_query::facade::runtime::SubscriptionConsumerAttachmentError;

fn main() {
    let error: SubscriptionConsumerAttachmentError = todo!();
    let _ = error.source_digest();
}
