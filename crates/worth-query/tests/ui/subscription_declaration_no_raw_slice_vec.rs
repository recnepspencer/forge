use worth_query::facade::runtime::{declare_query_subscription, QuerySubscriptionSliceBudget};

fn main() {
    let raw_slices = vec!["authorized_projection:0".to_string()];
    let budget = QuerySubscriptionSliceBudget::scratch_buffer_only(1, 1, 1, 1, 1, 1, 1, 1);
    let _declaration = declare_query_subscription(raw_slices, budget);
}
