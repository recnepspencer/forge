use worth_signal::facade::{
    DeniedResourceTimeout, ResourceBoundaryPerformanceEnvelope, ResourceLifecycleSummary,
    ResourceLifecycleTransition, ResourceTimeoutReport, TimedOutResourceRequest,
};

fn WORTHd_timeout() -> TimedOutResourceRequest {
    loop {}
}

fn WORTHd_denial() -> DeniedResourceTimeout {
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
    let _ = ResourceTimeoutReport {
        timed_out_request: Some(WORTHd_timeout()),
        denied_timeout: Some(WORTHd_denial()),
        lifecycle: Some(WORTHd_lifecycle()),
        transition: Some(WORTHd_transition()),
        performance: WORTHd_performance(),
    };
}
