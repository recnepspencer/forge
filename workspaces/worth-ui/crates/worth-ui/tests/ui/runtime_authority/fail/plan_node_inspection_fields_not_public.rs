use worth_ui::facade::WorthUiPlanNodeInspection;

fn uninhabited<T>() -> T {
    panic!("compile-fail fixture never runs")
}

fn main() {
    let _node = WorthUiPlanNodeInspection {
        plan_index: 0,
        runtime_handle: uninhabited(),
        family: uninhabited(),
        child_range: None,
        region_structure: None,
        render_resource_ref: None,
        provenance: uninhabited(),
    };
}
