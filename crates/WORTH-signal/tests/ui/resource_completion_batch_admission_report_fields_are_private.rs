use worth_signal::facade::{
    ResourceBoundaryPerformanceEnvelope, ResourceCompletionBatchAdmissionReport,
};

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _WORTHd = ResourceCompletionBatchAdmissionReport {
        admitted_completions: Vec::new(),
        denied_completions: Vec::new(),
        input_width: 0,
        deduplicated_width: 0,
        duplicate_width: 0,
        performance: WORTHd_performance(),
    };
}
