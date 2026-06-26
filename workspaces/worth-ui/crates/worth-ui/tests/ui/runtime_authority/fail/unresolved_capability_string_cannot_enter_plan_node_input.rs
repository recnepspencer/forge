use worth_ui::facade::WorthUiPlanNodeInput;

fn main() {
    let _ = WorthUiPlanNodeInput::from_unresolved_capability("query.workspace.selection");
}
