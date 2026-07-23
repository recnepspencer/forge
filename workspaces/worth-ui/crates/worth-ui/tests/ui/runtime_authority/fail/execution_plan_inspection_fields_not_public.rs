use worth_ui::facade::WorthUiExecutionPlanInspection;

fn uninhabited<T>() -> T {
    panic!("compile-fail fixture never runs")
}

fn main() {
    let _inspection = WorthUiExecutionPlanInspection {
        active_artifact_digest: 0,
        handle_basis_digest: 0,
        plan_digest: uninhabited(),
        nodes: Vec::new(),
        lanes: Vec::new(),
        provenance: Vec::new(),
        counters: uninhabited(),
    };
}
