use forge_query::facade::ActiveSubscriptionLifecycleError;

fn main() {
    let error: ActiveSubscriptionLifecycleError = todo!();
    let _ = error.source_digest();
}
