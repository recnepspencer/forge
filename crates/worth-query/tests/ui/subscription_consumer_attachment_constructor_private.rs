use worth_query::facade::runtime::SubscriptionConsumerAttachment;

fn main() {
    let _attachment = SubscriptionConsumerAttachment {
        attachment_digest: todo!(),
        lane_digest: todo!(),
        consumer_digest: String::new(),
        delivery_cursor_digest: String::new(),
        attachment_index: 0,
        acknowledgement_frontier: todo!(),
        next_delivery_sequence: todo!(),
        backpressure_policy: todo!(),
        fanout_report: todo!(),
    };
}
