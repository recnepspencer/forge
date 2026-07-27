use worth_query_execution::facade::provider_session::WorthQueryPassedInvariantReceipt;

fn progress(passed: WorthQueryPassedInvariantReceipt) {
    let _ = passed.admit_blocking_progression();
}

fn main() {}
