use worth_query::facade::live::WorthQueryManagedLiveContinuation;

fn author_maintenance(continuation: &mut WorthQueryManagedLiveContinuation) {
    continuation.advance_maintenance();
}

fn main() {}
