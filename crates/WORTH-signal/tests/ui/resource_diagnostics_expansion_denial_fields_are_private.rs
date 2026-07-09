use worth_signal::facade::{
    ResourceBoundaryPerformanceEnvelope, ResourceDiagnosticsExpansionBudget,
    ResourceDiagnosticsExpansionDenial, ResourceDiagnosticsExpansionDenialClass,
};

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _WORTHd = ResourceDiagnosticsExpansionDenial {
        class: ResourceDiagnosticsExpansionDenialClass::ColdReconstructionDisabled,
        budget: ResourceDiagnosticsExpansionBudget::retained_summary_only(),
        replay_reconstruction_width: 1,
        performance: WORTHd_performance(),
    };
}
