use forge_query::facade::SubscriptionLifecycleCloseError;

fn main() {
    let error: SubscriptionLifecycleCloseError = todo!();
    let _ = error.source_digest();
}
