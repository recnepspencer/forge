use worth_query::subscription::{
    QuerySubscriptionSupportCounters, QuerySubscriptionSupportMatrix, QuerySubscriptionSupportPosture,
    QuerySubscriptionSupportReport, QuerySubscriptionSupportSubject,
};

fn main() {
    let _ = QuerySubscriptionSupportReport {
        support_subject: unsafe { std::mem::zeroed::<QuerySubscriptionSupportSubject>() },
        support_posture: QuerySubscriptionSupportPosture::RuntimeBackedCertified,
        support_matrix: unsafe { std::mem::zeroed::<QuerySubscriptionSupportMatrix>() },
        source_digest: String::from("Worthd"),
        counter_snapshot: String::from("Worthd"),
        lookup_receipt_digest: String::from("Worthd"),
        report_digest: String::from("Worthd"),
        counters: QuerySubscriptionSupportCounters::default(),
    };
}
