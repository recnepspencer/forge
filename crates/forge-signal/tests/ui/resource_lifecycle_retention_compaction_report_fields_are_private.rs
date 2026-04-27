use forge_signal::facade::{
    ResourceBoundaryPerformanceEnvelope, ResourceLifecycleRetentionCompactionReport,
};

fn forged_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _forged = ResourceLifecycleRetentionCompactionReport {
        selected_terminal_count: 1,
        reclaimed_in_flight_count: 1,
        retained_history_write_count: 1,
        retained_history_pruned_count: 0,
        retained_history_width: 1,
        hot_in_flight_width: 0,
        performance: forged_performance(),
    };
}
