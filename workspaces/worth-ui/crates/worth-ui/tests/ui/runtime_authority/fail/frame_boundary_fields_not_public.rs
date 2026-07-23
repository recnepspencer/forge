use worth_ui::facade::{
    WorthUiFrameBoundary,
    WorthUiFrameBoundaryPosture,
    runtime::WorthUiRuntimeFrameEpoch,
};

fn main() {
    let _boundary = WorthUiFrameBoundary {
        frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
        posture: WorthUiFrameBoundaryPosture::SafeToActivate,
    };
}
