use worth_ui::facade::{
    WorthUiPrimitiveEventDispatchReceipt, WorthUiPrimitivePointerCaptureHostSupport,
    WorthUiPrimitivePointerCaptureState, WorthUiPrimitivePointerFrameReceipt,
    WorthUiPrimitivePointerPhase,
};

fn main() {
    let _forged = WorthUiPrimitivePointerFrameReceipt {
        dispatch: dispatch(),
        capture_state: WorthUiPrimitivePointerCaptureState::Uncaptured,
        phase: WorthUiPrimitivePointerPhase::Hover,
        host_support: WorthUiPrimitivePointerCaptureHostSupport::Certified,
    };
}

fn dispatch() -> WorthUiPrimitiveEventDispatchReceipt {
    panic!("fixture only checks pointer frame field privacy")
}
