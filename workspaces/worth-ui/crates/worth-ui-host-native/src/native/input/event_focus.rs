use winit::event::WindowEvent;
use worth_ui_host_contract::UiHostObservationPayload;

use super::{
    UiNativeInputObservationDisposition, UiNativeInputObservationEventFamily,
    UiNativeInputObservationState, UiNativeInputObservationStop,
};

pub(super) fn observe(
    state: &mut UiNativeInputObservationState,
    event: &WindowEvent,
) -> Option<UiNativeInputObservationDisposition> {
    let WindowEvent::Focused(focused) = event else {
        return None;
    };
    if !state.admit_input(UiNativeInputObservationEventFamily::WindowFocus) {
        return Some(state.rejection_disposition());
    }
    if !focused && state.pointer.end_capture().is_err() {
        state.record_terminal_stop(UiNativeInputObservationStop::PointerCaptureEpochExhausted);
        return Some(UiNativeInputObservationDisposition::Stopped);
    }
    let Some((_, _, presentation)) = state.completed else {
        return Some(state.rejection_disposition());
    };
    Some(state.emit_payloads([UiHostObservationPayload::WindowFocus {
        surface: presentation.host_surface(),
        focused: *focused,
    }]))
}
