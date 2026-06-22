use worth_ui::facade::WorthUiExecutionPlanInput;

fn value<T>() -> T {
    unreachable!()
}

fn main() {
    let _ = WorthUiExecutionPlanInput {
        basis: value(),
        context: value(),
        node_inputs: Vec::new(),
        counters: value(),
    };
}
