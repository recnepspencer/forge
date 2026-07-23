use worth_ui::facade::{
    WorthUiExecutionPlanInput,
    WorthUiOrdinaryFrameTarget,
    runtime::WorthUiRuntime,
};

fn main() {
    let host: WorthUiRuntime = todo!();
    let input: WorthUiExecutionPlanInput = todo!();
    let _ = host.execute_ordinary_lane_frame(&input, WorthUiOrdinaryFrameTarget::root_shell());
}
