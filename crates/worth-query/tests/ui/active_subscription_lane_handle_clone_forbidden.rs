use worth_query::facade::runtime::{admit_active_subscription_lane, open_active_subscription_lane, ActiveAllocationScopeWidth, ActiveFanoutWidth, ActiveRegistryLookupWidth, ActiveSubscriptionAllocationPolicy, ActiveSubscriptionRuntime, ActiveSubscriptionWorkBudget, SubscriptionActivationInput};

fn main() {
    fn fabricated_activation() -> SubscriptionActivationInput {
        unimplemented!()
    }

    let budget = ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPolicy::LifecycleArena,
    );
    let admission = admit_active_subscription_lane(fabricated_activation(), budget).unwrap();
    let mut runtime = ActiveSubscriptionRuntime::new();
    let handle = open_active_subscription_lane(&mut runtime, admission).unwrap();
    let _clone = handle.clone();
}
