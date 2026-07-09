use worth_query::facade::{
    ActiveAllocationScopeWidth, ActiveSubscriptionAllocationPosture, DeliveryBackpressurePolicy,
    DeliveryWindowWidth, MaintenanceDeltaWidth, PatchGroupWidth, QueryDeliveryWindowBudget,
};

fn main() {
    let _ = QueryDeliveryWindowBudget {
        delivery_window_width: DeliveryWindowWidth::measured(0),
        patch_group_width: PatchGroupWidth::measured(1),
        maintenance_delta_width: MaintenanceDeltaWidth::measured(1),
        allocation_scope_width: ActiveAllocationScopeWidth::measured(1),
        allocation_posture: ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
        backpressure_policy: DeliveryBackpressurePolicy::RetainWithinWindow,
    };
}
