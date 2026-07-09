use worth_signal::facade::{
    DeniedResourceRejection, RejectedResourceRequest, ResourceBoundaryPerformanceEnvelope,
    ResourceLifecycleSummary, ResourceLifecycleTransition, ResourceRejectionReport,
};

fn WORTHd_rejected() -> RejectedResourceRequest {
    loop {}
}

fn WORTHd_denial() -> DeniedResourceRejection {
    loop {}
}

fn WORTHd_lifecycle() -> ResourceLifecycleSummary {
    loop {}
}

fn WORTHd_transition() -> ResourceLifecycleTransition {
    loop {}
}

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _ = ResourceRejectionReport {
        rejected_request: Some(WORTHd_rejected()),
        denied_rejection: Some(WORTHd_denial()),
        lifecycle: Some(WORTHd_lifecycle()),
        transition: Some(WORTHd_transition()),
        performance: WORTHd_performance(),
    };
}
