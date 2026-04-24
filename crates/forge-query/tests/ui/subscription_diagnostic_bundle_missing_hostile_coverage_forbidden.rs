use forge_query::subscription::{
    QuerySubscriptionDiagnosticCounters, QuerySubscriptionDiagnosticSemanticLabels,
    QuerySubscriptionDiagnosticTrace, QuerySubscriptionDeniedDiagnosticBundle,
};

fn main() {
    let _ = QuerySubscriptionDeniedDiagnosticBundle {
        trace: unsafe { std::mem::zeroed::<QuerySubscriptionDiagnosticTrace>() },
        semantic_labels: unsafe { std::mem::zeroed::<QuerySubscriptionDiagnosticSemanticLabels>() },
        failure_digest: String::from("forged"),
        omitted_stages: vec![String::from("hostile_coverage")],
        counter_snapshot: String::from("forged"),
        bundle_digest: String::from("forged"),
        counters: QuerySubscriptionDiagnosticCounters::default(),
    };
}
