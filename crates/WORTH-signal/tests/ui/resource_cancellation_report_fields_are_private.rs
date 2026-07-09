use worth_signal::facade::{
    CancelledResourceRequest, ResourceBoundaryPerformanceEnvelope, ResourceCancellationReport,
    ResourceDependentCancellationPropagation, ResourceLifecycleSummary,
    ResourceLifecycleTransition,
};

fn WORTHd_cancelled() -> CancelledResourceRequest {
    loop {}
}

fn WORTHd_lifecycle() -> ResourceLifecycleSummary {
    loop {}
}

fn WORTHd_propagation() -> ResourceDependentCancellationPropagation {
    loop {}
}

fn WORTHd_transition() -> ResourceLifecycleTransition {
    loop {}
}

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _ = ResourceCancellationReport {
        cancelled_request: Some(WORTHd_cancelled()),
        dependent_propagation: Some(WORTHd_propagation()),
        denied_cancellation: None,
        lifecycle: Some(WORTHd_lifecycle()),
        transition: Some(WORTHd_transition()),
        performance: WORTHd_performance(),
    };
}
