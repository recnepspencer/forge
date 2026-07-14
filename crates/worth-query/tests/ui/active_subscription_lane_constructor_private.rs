use worth_query::facade::runtime::{ActiveLaneLookupClass, ActiveSubscriptionAllocationPolicy, ActiveSubscriptionDeliveryPosture, ActiveSubscriptionLane, ActiveSubscriptionLaneDigest, ActiveSubscriptionLifecyclePosture};

fn main() {
    let _lane = ActiveSubscriptionLane {
        lane_digest: ActiveSubscriptionLaneDigest(String::new()),
        activation_digest: String::new(),
        admission_digest: String::new(),
        query_declaration_digest: String::new(),
        bridge_declaration_digest: String::new(),
        basis_binding_digest: String::new(),
        signal_strategy_digest: String::new(),
        lifecycle_posture: ActiveSubscriptionLifecyclePosture::SingleConsumer,
        delivery_posture: ActiveSubscriptionDeliveryPosture::QueryShapedPatch,
        lookup_class: ActiveLaneLookupClass::EquivalenceIndex,
        allocation_policy: ActiveSubscriptionAllocationPolicy::LifecycleArena,
    };
}
