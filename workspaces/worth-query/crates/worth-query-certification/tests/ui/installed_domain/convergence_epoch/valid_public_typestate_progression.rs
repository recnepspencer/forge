use worth_query_host::facade::convergence_epoch::{
    WorthQueryAdmittedDirectConvergenceEpoch,
    WorthQueryDirectConvergenceIterationStartRejection,
    WorthQueryIteratingDirectConvergenceEpoch,
    WorthQueryStartedDirectConvergenceIteration,
};
use worth_query_host::facade::installed::domain_computation::WorthQueryManagedGraphCallRequest;

fn start(
    admitted: WorthQueryAdmittedDirectConvergenceEpoch,
) -> WorthQueryIteratingDirectConvergenceEpoch {
    admitted.start()
}

fn begin(
    iterating: WorthQueryIteratingDirectConvergenceEpoch,
    request: WorthQueryManagedGraphCallRequest,
) -> Result<
    WorthQueryStartedDirectConvergenceIteration,
    WorthQueryDirectConvergenceIterationStartRejection,
> {
    iterating.begin_iteration(request)
}

fn main() {}
