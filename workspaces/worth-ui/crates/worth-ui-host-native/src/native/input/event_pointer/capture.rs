use super::super::{
    UiNativeInputObservationDisposition, UiNativeInputObservationEventFamily,
    UiNativeInputObservationState, UiNativeInputObservationStop,
};

pub(super) fn observe_exit(
    state: &mut UiNativeInputObservationState,
) -> UiNativeInputObservationDisposition {
    if !state.admit_input(UiNativeInputObservationEventFamily::Pointer) {
        return state.rejection_disposition();
    }
    if state.pointer.end_capture().is_err() {
        state.record_terminal_stop(UiNativeInputObservationStop::PointerCaptureEpochExhausted);
        return UiNativeInputObservationDisposition::Stopped;
    }
    UiNativeInputObservationDisposition::Ignored
}
