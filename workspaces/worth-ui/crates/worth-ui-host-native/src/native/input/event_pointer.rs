use winit::event::WindowEvent;

use super::{
    UiNativeInputObservationDisposition, UiNativeInputObservationState,
    UiNativePointerPositionWitness,
};

mod button;
mod capture;
mod motion;

pub(super) fn observe(
    state: &mut UiNativeInputObservationState,
    event: &WindowEvent,
    pointer_witness: UiNativePointerPositionWitness,
) -> Option<UiNativeInputObservationDisposition> {
    match event {
        WindowEvent::CursorMoved { position, .. } => Some(motion::observe(state, *position)),
        WindowEvent::MouseInput {
            state: button_state,
            button,
            ..
        } => Some(button::observe(
            state,
            *button_state,
            *button,
            pointer_witness,
        )),
        WindowEvent::CursorLeft { .. } => Some(capture::observe_exit(state)),
        _ => None,
    }
}
