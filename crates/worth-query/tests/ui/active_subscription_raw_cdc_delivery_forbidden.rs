use worth_query::facade::{emit_query_delivery_batch, QueryDeliveryWindow};

fn main() {
    let _ = emit_query_delivery_batch(todo::<QueryDeliveryWindow>(), "raw-cdc");
}

fn todo<T>() -> T {
    unimplemented!()
}
