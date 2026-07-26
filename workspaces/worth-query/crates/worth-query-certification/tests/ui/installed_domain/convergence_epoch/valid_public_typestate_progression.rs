use worth_query_host::facade::convergence_epoch::{
    WorthQueryAdmittedDirectConvergenceEpoch,
    WorthQueryDirectConvergenceIterationStartRejection,
    WorthQueryDirectConvergenceYieldCleanupOutcome,
    WorthQueryIteratingDirectConvergenceEpoch,
    WorthQueryStartedDirectConvergenceIteration,
    WorthQueryWorkflowConvergenceYieldCleanupOutcome,
    WorthQueryYieldedDirectConvergenceIteration,
    WorthQueryYieldedWorkflowConvergenceIteration,
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

fn cleanup_yielded_direct(
    yielded: WorthQueryYieldedDirectConvergenceIteration,
) -> WorthQueryDirectConvergenceYieldCleanupOutcome {
    yielded.cleanup()
}

fn cleanup_yielded_workflow(
    yielded: WorthQueryYieldedWorkflowConvergenceIteration,
) -> WorthQueryWorkflowConvergenceYieldCleanupOutcome {
    yielded.cleanup()
}

fn main() {}
