use worth_ui::facade::{
    WorthUiExecutionPlanInput, WorthUiOrdinaryFrameTarget, WorthUiRuntimeHost,
};

fn main() {
    let host: WorthUiRuntimeHost = todo!();
    let input: WorthUiExecutionPlanInput = todo!();
    let _ = host.execute_ordinary_lane_frame(&input, WorthUiOrdinaryFrameTarget::root_shell());
}
