use forge_query::facade::{
    QuerySubscriptionRuntimeCertificationBundle, QuerySubscriptionRuntimeCertificationCounters,
};

fn main() {
    let _ = QuerySubscriptionRuntimeCertificationBundle {
        query_digest: String::new(),
        subscription_family_digest: String::new(),
        subscription_declaration_digest: String::new(),
        bridge_declaration_digest: String::new(),
        signal_strategy_digest: String::new(),
        support_report_digest: String::new(),
        bridge_parity_digest: String::new(),
        diagnostic_bundle_digest: String::new(),
        lifecycle_certification_digest: String::new(),
        hostile_coverage_digest: String::new(),
        family_coverage_digest: String::new(),
        runtime_certification_bundle_digest: String::new(),
        counter_snapshot: String::new(),
        counters: QuerySubscriptionRuntimeCertificationCounters::default(),
    };
}
