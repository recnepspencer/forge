use worth_query_execution::facade::provider_session::WorthQueryViolatedInvariantReceipt;

fn progress(violated: WorthQueryViolatedInvariantReceipt) {
    let _ = violated.admit_blocking_progression();
}

fn main() {}
