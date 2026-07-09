use worth_query::facade::{build_active_delivery_work_packet, ActiveSubscriptionRuntime};

fn main() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let _ = build_active_delivery_work_packet(
        &mut runtime,
        todo!(),
        todo!(),
        todo!(),
        "dense-refresh",
        todo!(),
        todo!(),
        todo!(),
        todo!(),
        todo!(),
        todo!(),
        todo!(),
    );
}
