use worth_query::facade::runtime::{ActiveSubscriptionAllocationPosture, DeliveryBackpressurePolicy, QueryDeliveryWindowBudget};

fn main() {
    let _budget = QueryDeliveryWindowBudget::admitted(
        2,
        1,
        1,
        1,
        ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    );
}
