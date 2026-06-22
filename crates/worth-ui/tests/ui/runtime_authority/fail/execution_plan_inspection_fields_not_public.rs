use worth_ui::facade::WorthUiExecutionPlanInspection;

fn uninhabited<T>() -> T {
    panic!("compile-fail fixture never runs")
}

fn main() {
    let _inspection = WorthUiExecutionPlanInspection {
        plan_digest: uninhabited(),
        nodes: Vec::new(),
        lanes: Vec::new(),
        provenance: Vec::new(),
        counters: uninhabited(),
    };
}
