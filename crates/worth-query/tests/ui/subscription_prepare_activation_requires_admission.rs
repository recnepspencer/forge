use worth_query::facade::{prepare_subscription_activation, BridgeSubscriptionLoweringPlan};

fn main() {
    let lowering: Option<BridgeSubscriptionLoweringPlan> = None;
    let _activation = prepare_subscription_activation(lowering.unwrap());
}
