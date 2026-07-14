use worth_query::facade::live::WorthQueryManagedLiveContinuation;

fn duplicate(continuation: WorthQueryManagedLiveContinuation) {
    let _copy = continuation.clone();
}

fn main() {}
