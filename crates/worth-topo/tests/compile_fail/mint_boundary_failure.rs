use worth_schema::facade::{
    WorthBoundaryFailure, WorthDecisionTrace, WorthIntegrityMarkers, WorthPerformanceAccounting,
};

fn main() {
    let _forged = WorthBoundaryFailure {
        error: "nope",
        warnings: Vec::new(),
        decision_trace: WorthDecisionTrace::default(),
        integrity_markers: WorthIntegrityMarkers::default(),
        performance_accounting: WorthPerformanceAccounting::default(),
    };
}
