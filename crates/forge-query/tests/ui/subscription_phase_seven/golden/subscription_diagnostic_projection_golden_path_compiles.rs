use forge_query::facade::QuerySubscriptionDiagnosticTrace;

fn projection_golden_path(trace: &QuerySubscriptionDiagnosticTrace) {
    let _ = trace.trace_projection().label();
}

fn main() {}
