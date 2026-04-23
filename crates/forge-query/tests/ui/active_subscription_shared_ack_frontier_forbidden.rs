use forge_query::facade::SubscriptionAcknowledgementFrontier;

fn main() {
    let _ = SubscriptionAcknowledgementFrontier {
        attachment_digest: todo!(),
        acknowledged_sequence: todo!(),
        frontier_digest: "shared-frontier".to_string(),
    };
}
