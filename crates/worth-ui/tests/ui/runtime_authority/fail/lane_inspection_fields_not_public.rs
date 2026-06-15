use worth_ui::facade::WorthUiLaneInspection;

fn uninhabited<T>() -> T {
    panic!("compile-fail fixture never runs")
}

fn main() {
    let _lane = WorthUiLaneInspection {
        lane: uninhabited(),
        plan_indexes: Vec::new(),
        node_count: 0,
    };
}
