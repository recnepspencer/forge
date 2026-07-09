use worth_query::facade::{emit_query_delivery_batch, QueryDeliveryWindow, QuerySubscriptionMaintenanceDelta};

fn main() {
    let window: QueryDeliveryWindow = todo!();
    let delta: QuerySubscriptionMaintenanceDelta = todo!();
    let _ = emit_query_delivery_batch(window, delta);
}
