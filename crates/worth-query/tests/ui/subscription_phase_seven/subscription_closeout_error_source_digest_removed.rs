use worth_query::facade::runtime::SubscriptionLifecycleCloseError;

fn main() {
    let error: SubscriptionLifecycleCloseError = todo!();
    let _ = error.source_digest();
}
