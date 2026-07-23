use worth_ui::facade::{
    WorthUiPendingActivation,
    runtime::WorthUiRuntimeFrameEpoch,
};

fn main() {
    let _pending = WorthUiPendingActivation::for_frame_epoch(WorthUiRuntimeFrameEpoch::initial());
}
