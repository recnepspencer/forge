use worth_signal::facade::{
    AdmittedResourceCompletion, DeniedResourceCompletion, ResourceBoundaryPerformanceEnvelope,
    ResourceCompletionAdmissionReport,
};

fn WORTHd_admitted() -> AdmittedResourceCompletion {
    loop {}
}

fn WORTHd_denied() -> DeniedResourceCompletion {
    loop {}
}

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _ = ResourceCompletionAdmissionReport {
        admitted_completion: Some(WORTHd_admitted()),
        denied_completion: Some(WORTHd_denied()),
        performance: WORTHd_performance(),
    };
}
