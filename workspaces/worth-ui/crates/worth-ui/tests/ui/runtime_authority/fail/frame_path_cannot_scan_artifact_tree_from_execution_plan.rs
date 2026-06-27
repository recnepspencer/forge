use worth_ui::facade::WorthUiExecutionPlan;

fn frame_traversal(plan: &WorthUiExecutionPlan) {
    let _ = plan.scan_artifact_tree_for_children("component:root");
}

fn main() {}
