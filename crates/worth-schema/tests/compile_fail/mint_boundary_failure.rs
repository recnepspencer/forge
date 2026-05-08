use schema::facade::{
    BoundaryFailure, DecisionTrace, IntegrityMarkers, PerformanceAccounting,
};

fn main() {
    let _forged = BoundaryFailure {
        error: "nope",
        warnings: Vec::new(),
        decision_trace: DecisionTrace::default(),
        integrity_markers: IntegrityMarkers::default(),
        performance_accounting: PerformanceAccounting::default(),
    };
}
