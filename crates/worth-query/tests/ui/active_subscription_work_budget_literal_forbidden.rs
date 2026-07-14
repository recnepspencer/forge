use worth_query::facade::runtime::{ActiveAllocationScopeWidth, ActiveFanoutWidth, ActiveLaneLookupClass, ActiveRegistryLookupWidth, ActiveSubscriptionAllocationPolicy, ActiveSubscriptionWorkBudget};

fn main() {
    let _budget = ActiveSubscriptionWorkBudget {
        registry_lookup_width: ActiveRegistryLookupWidth::measured(1),
        fanout_width: ActiveFanoutWidth::measured(1),
        allocation_scope_width: ActiveAllocationScopeWidth::measured(1),
        lookup_class: ActiveLaneLookupClass::EquivalenceIndex,
        allocation_policy: ActiveSubscriptionAllocationPolicy::LifecycleArena,
        durable_checkpoint_requested: false,
        store_backed_restart_requested: false,
    };
}
