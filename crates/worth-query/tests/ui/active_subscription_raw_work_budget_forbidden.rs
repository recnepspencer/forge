use worth_query::facade::runtime::{ActiveLaneLookupClass, ActiveSubscriptionAllocationPolicy, ActiveSubscriptionWorkBudget};

fn main() {
    let _budget = ActiveSubscriptionWorkBudget {
        registry_lookup_width: 1,
        fanout_width: 1,
        allocation_scope_width: 1,
        lookup_class: ActiveLaneLookupClass::EquivalenceIndex,
        allocation_policy: ActiveSubscriptionAllocationPolicy::LifecycleArena,
        durable_checkpoint_requested: false,
        store_backed_restart_requested: false,
    };
}
