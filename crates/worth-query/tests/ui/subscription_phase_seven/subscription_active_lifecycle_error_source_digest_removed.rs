use worth_query::facade::runtime::ActiveSubscriptionLifecycleError;

fn main() {
    let error: ActiveSubscriptionLifecycleError = todo!();
    let _ = error.source_digest();
}
