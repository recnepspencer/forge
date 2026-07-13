use worth_query::facade::runtime::QuerySubscriptionScaleSlopeReport;

fn main() {
    let _ = QuerySubscriptionScaleSlopeReport {
        digest: String::new(),
        activation_digest: String::new(),
        admission_digest: String::new(),
        small_snapshot_digest: String::new(),
        medium_snapshot_digest: String::new(),
        large_snapshot_digest: String::new(),
        small_row_count: 1,
        medium_row_count: 2,
        large_row_count: 3,
        structural_counter_digest: String::new(),
    };
}
