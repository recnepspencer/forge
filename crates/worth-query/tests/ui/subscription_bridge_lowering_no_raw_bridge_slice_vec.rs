use worth_query::facade::runtime::{lower_query_subscription_to_bridge, BridgeSubscriptionSliceKind, QuerySubscriptionBridgeLoweringBudget};

fn main() {
    let raw_slices = vec![BridgeSubscriptionSliceKind::ProjectedField];
    let budget = QuerySubscriptionBridgeLoweringBudget::admitted(1, 1, 1, 1, 1);
    let _plan = lower_query_subscription_to_bridge(raw_slices, budget);
}
