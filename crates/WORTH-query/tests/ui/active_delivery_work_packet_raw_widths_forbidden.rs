use worth_query::facade::{
    build_active_delivery_work_packet, ActiveDeliveryDensityPosture,
    ActiveSubscriptionAllocationPosture, ActiveSubscriptionRuntime,
};

fn main() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    build_active_delivery_work_packet(
        &mut runtime,
        todo!(),
        todo!(),
        todo!(),
        ActiveDeliveryDensityPosture::SparseDelta,
        1,
        1,
        1,
        0,
        0,
        1,
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap();
}
