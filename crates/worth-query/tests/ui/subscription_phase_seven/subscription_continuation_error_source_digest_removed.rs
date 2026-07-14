use worth_query::facade::runtime::SubscriptionContinuationError;

fn main() {
    let error: SubscriptionContinuationError = todo!();
    let _ = error.source_digest();
}
