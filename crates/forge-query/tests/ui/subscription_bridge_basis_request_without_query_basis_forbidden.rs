use forge_query::facade::{
    lower_query_subscription_to_bridge, QuerySubscriptionBasisBindingRequestKind,
    QuerySubscriptionBridgeLoweringBudget,
};

fn main() {
    let raw_basis = QuerySubscriptionBasisBindingRequestKind::CurrentHead;
    let budget = QuerySubscriptionBridgeLoweringBudget::admitted(1, 1, 1, 1, 1);
    let _plan = lower_query_subscription_to_bridge(raw_basis, budget);
}
