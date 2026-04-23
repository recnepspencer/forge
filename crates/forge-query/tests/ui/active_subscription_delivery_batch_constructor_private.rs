use forge_query::facade::QueryDeliveryBatch;

fn main() {
    let _ = QueryDeliveryBatch {
        delivery_batch_digest: "batch".to_string(),
        delivery_window_digest: "window".to_string(),
        attachment_digest: todo!(),
        sequence: todo!(),
        patch_group: todo!(),
        receipt: todo!(),
        counters: todo!(),
    };
}
