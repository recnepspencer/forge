use worth_query::facade::runtime::{QuerySubscriptionDeclarationCounters, QuerySubscriptionScaleCounterSnapshot, QuerySubscriptionScaleFixtureSize};

fn main() {
    let counters = QuerySubscriptionDeclarationCounters::default();
    let _ = QuerySubscriptionScaleCounterSnapshot::from_counters(
        QuerySubscriptionScaleFixtureSize::Small,
        1,
        &counters,
    );
}
