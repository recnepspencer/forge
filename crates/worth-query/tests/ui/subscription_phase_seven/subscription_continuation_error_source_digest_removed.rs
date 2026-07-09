use worth_query::facade::SubscriptionContinuationError;

fn main() {
    let error: SubscriptionContinuationError = todo!();
    let _ = error.source_digest();
}
