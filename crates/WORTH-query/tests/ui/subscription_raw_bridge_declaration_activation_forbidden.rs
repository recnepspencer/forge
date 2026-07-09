use worth_query::facade::{
    prepare_subscription_activation, BridgeSubscriptionDeclarationFamilyKind,
};

fn main() {
    let raw_bridge = BridgeSubscriptionDeclarationFamilyKind::DetailExact;
    let _activation = prepare_subscription_activation(raw_bridge);
}
