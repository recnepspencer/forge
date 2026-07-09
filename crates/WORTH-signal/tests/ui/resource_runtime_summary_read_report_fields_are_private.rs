use worth_signal::facade::{
    ResourceBoundaryPerformanceEnvelope, ResourceRuntimeSummary, ResourceRuntimeSummaryReadReport,
};

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _WORTHd = ResourceRuntimeSummaryReadReport {
        summary: ResourceRuntimeSummary::default(),
        performance: WORTHd_performance(),
    };
}
