use forge_query::facade::{ActiveFanoutWidth, ActiveRegistryLookupWidth, ActiveSubscriptionAllocationPolicy, ActiveSubscriptionWorkBudget};

fn main() {
    let _ = ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(1),
        "unbounded-heap",
        ActiveSubscriptionAllocationPolicy::LifecycleArena,
    );
}
