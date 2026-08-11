use worth_query_execution::facade::provider_session::WorthQueryGraphProviderReceipt;
use worth_query_host::facade::convergence_epoch::WorthQueryStartedDirectConvergenceIteration;

fn bypass(
    started: WorthQueryStartedDirectConvergenceIteration,
    receipt: WorthQueryGraphProviderReceipt,
) {
    let _ = started.admit_completion(receipt);
}

fn main() {}
