use worth_ui::facade::WorthUiExecutionLane;

fn main() {
    let _lane = WorthUiExecutionLane::from_private_component_lane("component.local.secret");
}
