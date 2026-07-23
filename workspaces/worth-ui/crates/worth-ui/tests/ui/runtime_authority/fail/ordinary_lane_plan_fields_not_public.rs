use worth_ui::facade::WorthUiOrdinaryLanePlan;

fn main() {
    let _plan = WorthUiOrdinaryLanePlan {
        handle_receipt: uninitialized_field(),
        support_digest: 0,
        ordinary_plan_digest: 0,
        region_store: uninitialized_field(),
        root_shell_slots: uninitialized_field(),
        counters: Default::default(),
    };
}

fn uninitialized_field<T>() -> T {
    panic!("compile-fail fixture never runs")
}
