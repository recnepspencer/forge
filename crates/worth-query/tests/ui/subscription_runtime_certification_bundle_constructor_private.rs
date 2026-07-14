use worth_query::facade::certification::{QuerySubscriptionRuntimeCertificationBundle, QuerySubscriptionRuntimeCertificationCounters};

fn main() {
    let _ = QuerySubscriptionRuntimeCertificationBundle {
        query_digest: String::new(),
        subscription_family_digest: String::new(),
        subscription_declaration_identity: todo!(),
        bridge_declaration_identity: todo!(),
        signal_strategy_identity: todo!(),
        support_report_identity: todo!(),
        bridge_parity_identity: todo!(),
        diagnostic_bundle_identity: todo!(),
        lifecycle_certification_identity: todo!(),
        hostile_coverage_identity: todo!(),
        family_coverage_digest: String::new(),
        runtime_certification_bundle_identity: todo!(),
        counter_identity: todo!(),
        counters: QuerySubscriptionRuntimeCertificationCounters::default(),
    };
}
