use worth_ui::facade::{WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLaunch};

fn main() {
    let _launch = WorthUiRuntimeLaunch {
        artifact: todo!(),
        frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
        diagnostic_policy: WorthUiRuntimeDiagnosticPolicy::minimal(),
    };
}
