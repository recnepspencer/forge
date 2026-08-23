use winit::event::{MouseScrollDelta, WindowEvent};
use worth_ui_host_contract::UiHostObservationPayload;

use super::{
    pointer, UiNativeInputObservationDisposition, UiNativeInputObservationEventFamily,
    UiNativeInputObservationState, UiNativeInputObservationStop,
};

pub(super) fn observe(
    state: &mut UiNativeInputObservationState,
    event: &WindowEvent,
) -> Option<UiNativeInputObservationDisposition> {
    let WindowEvent::MouseWheel { delta, .. } = event else {
        return None;
    };
    if !state.admit_input(UiNativeInputObservationEventFamily::Scroll) {
        return Some(state.rejection_disposition());
    }
    let Some(profile) = state.profile else {
        return Some(state.terminal_disposition(UiNativeInputObservationStop::MissingEventProfile));
    };
    let (x_subpixels, y_subpixels) =
        match delta {
            MouseScrollDelta::PixelDelta(delta) => {
                match pointer::logical_delta(*delta, profile.scale_factor) {
                    Ok(delta) => delta,
                    Err(pointer::UiNativePointerCoordinateDenial::NotFinite) => {
                        return Some(state.terminal_disposition(
                            UiNativeInputObservationStop::CoordinateNotFinite,
                        ));
                    }
                    Err(pointer::UiNativePointerCoordinateDenial::OutOfRange) => {
                        return Some(state.terminal_disposition(
                            UiNativeInputObservationStop::CoordinateOutOfRange,
                        ));
                    }
                }
            }
            MouseScrollDelta::LineDelta(..) => {
                return Some(state.terminal_disposition(
                    UiNativeInputObservationStop::Unsupported(
                        UiNativeInputObservationEventFamily::Scroll,
                    ),
                ));
            }
        };
    Some(state.emit_payloads([UiHostObservationPayload::ScrollDelta {
        x_subpixels,
        y_subpixels,
    }]))
}
