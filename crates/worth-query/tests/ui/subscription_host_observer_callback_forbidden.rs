use worth_query::facade::{select_query_subscription_family, QuerySubscriptionWorkBudget};

fn main() {
    let observer_callback = || "host-local-observer-state";
    let budget = QuerySubscriptionWorkBudget::scratch_buffer_only(1, 1, 1, 1, 1);
    let _selection = select_query_subscription_family(observer_callback, budget);
}
