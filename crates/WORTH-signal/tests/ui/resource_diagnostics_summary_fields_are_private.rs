use worth_signal::facade::{
    ResourceBoundaryPerformanceEnvelope, ResourceDiagnosticsSummary,
    ResourceDiagnosticsExpansionBudget, ResourceReplayReconstructionReport, ResourceRuntimeSummary,
};

fn WORTHd_replay() -> ResourceReplayReconstructionReport {
    loop {}
}

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _WORTHd = ResourceDiagnosticsSummary {
        schema_version: String::new(),
        runtime_summary: ResourceRuntimeSummary::default(),
        latest_branch_restore_report: None,
        replay_reconstruction: WORTHd_replay(),
        expansion_budget: ResourceDiagnosticsExpansionBudget::retained_summary_only(),
        performance: WORTHd_performance(),
        provenance_digest: String::new(),
    };
}
