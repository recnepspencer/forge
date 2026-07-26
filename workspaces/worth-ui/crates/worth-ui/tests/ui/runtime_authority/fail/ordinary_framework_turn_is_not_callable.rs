use worth_ui::facade::app::{WorthUiActiveApplicationSession, WorthUiMountedFrameExecutionStop};

fn enter_midpoint(session: &mut WorthUiActiveApplicationSession) {
    let _ = session.execute_framework_turn(|_| {});
}

fn recover_midpoint(stop: WorthUiMountedFrameExecutionStop<'_>) {
    if let WorthUiMountedFrameExecutionStop::FrameworkTransition(transition) = stop {
        let _ = transition.into_execution();
    }
}

fn main() {}
