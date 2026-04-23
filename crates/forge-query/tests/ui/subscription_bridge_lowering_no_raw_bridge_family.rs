use forge_query::facade::{
    lower_query_subscription_to_bridge, BridgeSubscriptionDeclarationFamilyKind,
    QuerySubscriptionBridgeLoweringBudget,
};

fn main() {
    let raw_family = BridgeSubscriptionDeclarationFamilyKind::DetailExact;
    let budget = QuerySubscriptionBridgeLoweringBudget::admitted(1, 1, 1, 1, 1);
    let _plan = lower_query_subscription_to_bridge(raw_family, budget);
}
