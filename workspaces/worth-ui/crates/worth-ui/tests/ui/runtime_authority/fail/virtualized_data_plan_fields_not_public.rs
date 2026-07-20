use worth_ui::facade::WorthUiVirtualizedDataPlan;

fn forged<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _plan = WorthUiVirtualizedDataPlan {
        handle_receipt: forged(),
        support_digest: 0,
        data_plan_digest: 0,
        rows: Vec::new(),
        counters: Default::default(),
        row_index: forged(),
    };
}
