use forge_query::subscription::{
    QuerySubscriptionSupportCounters, QuerySubscriptionSupportMatrix, QuerySubscriptionSupportPosture,
    QuerySubscriptionSupportReport, QuerySubscriptionSupportSubject,
};

fn main() {
    let _ = QuerySubscriptionSupportReport {
        support_subject: unsafe { std::mem::zeroed::<QuerySubscriptionSupportSubject>() },
        support_posture: QuerySubscriptionSupportPosture::RuntimeBackedCertified,
        support_matrix: unsafe { std::mem::zeroed::<QuerySubscriptionSupportMatrix>() },
        source_digest: String::from("forged"),
        counter_snapshot: String::from("forged"),
        lookup_receipt_digest: String::from("forged"),
        report_digest: String::from("forged"),
        counters: QuerySubscriptionSupportCounters::default(),
    };
}
