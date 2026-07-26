use worth_query_execution::facade::provider_session::WorthQueryGraphProviderReceipt;
use worth_query_host::facade::convergence_epoch::WorthQueryPendingDirectConvergenceIteration;

fn bypass(
    pending: WorthQueryPendingDirectConvergenceIteration,
    receipt: WorthQueryGraphProviderReceipt,
) {
    let _ = pending.admit_completion(receipt);
}

fn main() {}
