use forge_query::facade::{ActiveSubscriptionAllocationPosture, QueryDeliveryWindow};

fn main() {
    let _window = QueryDeliveryWindow {
        delivery_window_identity: todo!(),
        active_lane_digest: todo!(),
        attachment_digest: todo!(),
        next_sequence: todo!(),
        delivery_window_width: 1,
        patch_group_width: 1,
        maintenance_delta_width: 1,
        allocation_scope_width: 1,
        allocation_posture: ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
    };
}
