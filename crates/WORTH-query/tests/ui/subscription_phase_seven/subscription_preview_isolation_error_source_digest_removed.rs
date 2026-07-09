use worth_query::facade::PreviewSubscriptionIsolationError;

fn main() {
    let error: PreviewSubscriptionIsolationError = todo!();
    let _ = error.source_digest();
}
