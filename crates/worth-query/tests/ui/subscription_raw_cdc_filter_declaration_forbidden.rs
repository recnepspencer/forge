use worth_query::facade::runtime::{declare_query_subscription, QuerySubscriptionSliceBudget};

struct RawCdcFilter {
    table: &'static str,
}

fn main() {
    let raw_cdc = RawCdcFilter { table: "employees" };
    let budget = QuerySubscriptionSliceBudget::scratch_buffer_only(1, 1, 1, 1, 1, 1, 1, 1);
    let _declaration = declare_query_subscription(raw_cdc, budget);
}
