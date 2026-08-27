use winit::event::{MouseScrollDelta, TouchPhase, WindowEvent};
use worth_ui_host_contract::{
    UiHostObservationPayload, UiHostScrollDeltaPhase, UiHostScrollDeltaPrecision,
    UiHostScrollDeltaSource, UiHostScrollDeltaTargetAffinity,
};

use super::{
    pointer, UiNativeInputObservationDisposition, UiNativeInputObservationEventFamily,
    UiNativeInputObservationState, UiNativeInputObservationStop,
};

pub(super) fn observe(
    state: &mut UiNativeInputObservationState,
    event: &WindowEvent,
) -> Option<UiNativeInputObservationDisposition> {
    let WindowEvent::MouseWheel { delta, phase, .. } = event else {
        return None;
    };
    if !state.admit_input(UiNativeInputObservationEventFamily::Scroll) {
        return Some(state.rejection_disposition());
    }
    let Some(profile) = state.profile else {
        return Some(state.terminal_disposition(UiNativeInputObservationStop::MissingEventProfile));
    };
    let (x_subpixels, y_subpixels, precision) =
        match delta {
            MouseScrollDelta::PixelDelta(delta) => {
                match pointer::logical_delta(*delta, profile.scale_factor) {
                    Ok((x, y)) => (x, y, UiHostScrollDeltaPrecision::Pixel),
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
            MouseScrollDelta::LineDelta(x_lines, y_lines) => {
                match pointer::logical_line_delta(
                    *x_lines,
                    *y_lines,
                    profile.wheel_line_logical_units,
                ) {
                    Ok((x, y)) => (x, y, UiHostScrollDeltaPrecision::Line),
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
        };
    let Some((_, _, presentation)) = state.completed else {
        return Some(state.rejection_disposition());
    };
    Some(state.emit_payloads([UiHostObservationPayload::ScrollDelta {
        source: UiHostScrollDeltaSource::PointerWheel,
        phase: scroll_phase(*phase),
        precision,
        target: UiHostScrollDeltaTargetAffinity::presented_surface_fallback(presentation),
        x_subpixels,
        y_subpixels,
    }]))
}

const fn scroll_phase(phase: TouchPhase) -> UiHostScrollDeltaPhase {
    match phase {
        TouchPhase::Started => UiHostScrollDeltaPhase::Started,
        TouchPhase::Moved => UiHostScrollDeltaPhase::Updated,
        TouchPhase::Ended => UiHostScrollDeltaPhase::Ended,
        TouchPhase::Cancelled => UiHostScrollDeltaPhase::Cancelled,
    }
}
