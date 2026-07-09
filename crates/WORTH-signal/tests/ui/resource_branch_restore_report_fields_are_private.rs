use worth_signal::facade::{
    ResourceBoundaryPerformanceEnvelope, ResourceBranchRestoreReport,
};

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _WORTHd = ResourceBranchRestoreReport {
        restored_in_flight_width: 1,
        retained_summary_width: 1,
        broad_rebuild_denial_count: 1,
        performance: WORTHd_performance(),
    };
}
