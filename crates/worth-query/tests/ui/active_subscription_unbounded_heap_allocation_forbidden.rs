use worth_query::facade::runtime::{ActiveFanoutWidth, ActiveRegistryLookupWidth, ActiveSubscriptionAllocationPolicy, ActiveSubscriptionWorkBudget};

fn main() {
    let _ = ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(1),
        "unbounded-heap",
        ActiveSubscriptionAllocationPolicy::LifecycleArena,
    );
}
