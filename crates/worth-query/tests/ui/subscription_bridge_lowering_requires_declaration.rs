use worth_query::facade::runtime::{lower_query_subscription_to_bridge, QuerySubscriptionBridgeLoweringBudget, QuerySubscriptionFamilySelection};

fn main() {
    let selection = Option::<QuerySubscriptionFamilySelection>::None.unwrap();
    let budget = QuerySubscriptionBridgeLoweringBudget::admitted(1, 1, 1, 1, 1);
    let _plan = lower_query_subscription_to_bridge(selection, budget);
}
