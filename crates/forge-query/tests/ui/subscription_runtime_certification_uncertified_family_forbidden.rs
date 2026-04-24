use forge_query::subscription::{
    QuerySubscriptionRuntimeCertificationBundle, QuerySubscriptionRuntimeCertificationCounters,
};

fn main() {
    let _ = QuerySubscriptionRuntimeCertificationBundle {
        query_digest: String::from("forged"),
        subscription_family_digest: String::from("forged"),
        subscription_declaration_digest: String::from("forged"),
        bridge_declaration_digest: String::from("forged"),
        signal_strategy_digest: String::from("forged"),
        support_report_digest: String::from("forged"),
        bridge_parity_digest: String::from("forged"),
        diagnostic_bundle_digest: String::from("forged"),
        lifecycle_certification_digest: String::from("forged"),
        hostile_coverage_digest: String::from("forged"),
        family_coverage_digest: String::from("forged"),
        runtime_certification_bundle_digest: String::from("forged"),
        counter_snapshot: String::from("forged"),
        counters: QuerySubscriptionRuntimeCertificationCounters::default(),
    };
}
