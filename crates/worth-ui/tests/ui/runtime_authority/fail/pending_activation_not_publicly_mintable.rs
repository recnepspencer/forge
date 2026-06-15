use worth_ui::facade::{WorthUiPendingActivation, WorthUiRuntimeFrameEpoch};

fn main() {
    let _pending = WorthUiPendingActivation::for_frame_epoch(WorthUiRuntimeFrameEpoch::initial());
}
