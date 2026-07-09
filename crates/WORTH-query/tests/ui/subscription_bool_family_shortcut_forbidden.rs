use worth_query::facade::{select_query_subscription_family, QuerySubscriptionWorkBudget};

fn main() {
    let use_collection_subscription = true;
    let budget = QuerySubscriptionWorkBudget::scratch_buffer_only(1, 1, 1, 1, 1);
    let _selection = select_query_subscription_family(use_collection_subscription, budget);
}
