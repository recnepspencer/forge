use worth_query::facade::{
    QuerySubscriptionDeclarationCounters, QuerySubscriptionScaleCounterSnapshot,
    QuerySubscriptionScaleFixtureSize,
};

fn main() {
    let _ = QuerySubscriptionScaleCounterSnapshot {
        fixture_size: QuerySubscriptionScaleFixtureSize::Small,
        fixture_row_count: 1,
        activation_digest: String::new(),
        admission_digest: String::new(),
        counter_digest: String::new(),
        counters: QuerySubscriptionDeclarationCounters::default(),
        snapshot_digest: String::new(),
    };
}
