use worth_query::subscription::{
    QuerySubscriptionRuntimeCertificationBundle, QuerySubscriptionRuntimeCertificationCounters,
};

fn main() {
    let _ = QuerySubscriptionRuntimeCertificationBundle {
        query_digest: String::from("Worthd"),
        subscription_family_digest: String::from("Worthd"),
        subscription_declaration_digest: String::from("Worthd"),
        bridge_declaration_digest: String::from("Worthd"),
        signal_strategy_digest: String::from("Worthd"),
        support_report_digest: String::from("Worthd"),
        bridge_parity_digest: String::from("Worthd"),
        diagnostic_bundle_digest: String::from("Worthd"),
        lifecycle_certification_digest: String::from("Worthd"),
        hostile_coverage_digest: String::from("Worthd"),
        family_coverage_digest: String::from("Worthd"),
        runtime_certification_bundle_digest: String::from("Worthd"),
        counter_snapshot: String::from("Worthd"),
        counters: QuerySubscriptionRuntimeCertificationCounters::default(),
    };
}
