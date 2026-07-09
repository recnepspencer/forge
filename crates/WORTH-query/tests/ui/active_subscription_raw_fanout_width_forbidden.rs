use worth_query::facade::{ActiveAllocationScopeWidth, ActiveSubscriptionAllocationPolicy, ActiveSubscriptionWorkBudget, ActiveRegistryLookupWidth};

fn main() {
    let _ = ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        2,
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPolicy::LifecycleArena,
    );
}
