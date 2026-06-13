use worth_ui::facade::WorthUiExecutionPlan;

fn main() {
    let _plan = WorthUiExecutionPlan {
        handle_receipt: uninitialized_field(),
        topology: uninitialized_field(),
        lane_partitions: Vec::new(),
        lookup_index: uninitialized_field(),
        counters: uninitialized_field(),
    };
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
