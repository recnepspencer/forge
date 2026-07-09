use worth_query::facade::{
    ActiveAllocationScopeWidth, ActiveSubscriptionAllocationPosture, DeliveryBackpressurePolicy,
    MaintenanceDeltaWidth, PatchGroupWidth, QueryDeliveryWindowBudget,
};

fn main() {
    let _ = QueryDeliveryWindowBudget::admitted(
        3,
        PatchGroupWidth::measured(1),
        MaintenanceDeltaWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    );
}
