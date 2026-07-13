use worth_query::facade::runtime::QuerySubscriptionDiagnosticTrace;

fn projection_golden_path(trace: &QuerySubscriptionDiagnosticTrace) {
    let _ = trace.trace_projection().label();
}

fn main() {}
