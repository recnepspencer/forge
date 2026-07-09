use worth_signal::facade::{
    DeniedResourceRetry, ResourceBoundaryPerformanceEnvelope, ResourceRetryScheduleReport,
    ScheduledResourceRetry,
};

fn WORTHd_retry() -> ScheduledResourceRetry {
    loop {}
}

fn WORTHd_denial() -> DeniedResourceRetry {
    loop {}
}

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _ = ResourceRetryScheduleReport {
        scheduled_retry: Some(WORTHd_retry()),
        denied_retry: Some(WORTHd_denial()),
        performance: WORTHd_performance(),
    };
}
