use worth_query::facade::{
    QuerySubscriptionAdmittedDiagnosticBundle, QuerySubscriptionDiagnosticCounters,
    QuerySubscriptionDiagnosticSemanticLabels, QuerySubscriptionDiagnosticTrace,
};

fn main() {
    let trace: QuerySubscriptionDiagnosticTrace = todo!();
    let labels: QuerySubscriptionDiagnosticSemanticLabels = todo!();
    let _ = QuerySubscriptionAdmittedDiagnosticBundle {
        trace,
        semantic_labels: labels,
        support_report_digest: String::new(),
        lifecycle_certification_digest: String::new(),
        continuation_digest: None,
        preview_isolation_digest: None,
        lifecycle_closeout_digest: None,
        counter_snapshot: String::new(),
        bundle_digest: String::new(),
        counters: QuerySubscriptionDiagnosticCounters::default(),
    };
}
