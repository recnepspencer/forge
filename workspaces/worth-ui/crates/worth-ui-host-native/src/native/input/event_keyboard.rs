use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{Key, PhysicalKey};
use worth_ui_host_contract::UiHostObservationPayload;

use super::{
    event_outcome::input_affine_batch_fits, keyboard, UiNativeInputObservationDisposition,
    UiNativeInputObservationEventFamily, UiNativeInputObservationState,
    UiNativeInputObservationStop,
};

pub(super) fn observe(
    state: &mut UiNativeInputObservationState,
    event: &WindowEvent,
) -> Option<UiNativeInputObservationDisposition> {
    let disposition = match event {
        WindowEvent::ModifiersChanged(modifiers) => {
            state.modifiers = keyboard::modifiers(*modifiers);
            if state.completed.is_none() {
                state.record_stop(UiNativeInputObservationStop::NoPresentationBasis);
            }
            UiNativeInputObservationDisposition::Ignored
        }
        WindowEvent::KeyboardInput { event, .. } => observe_key_event(state, event),
        _ => return None,
    };
    Some(disposition)
}

fn observe_key_event(
    state: &mut UiNativeInputObservationState,
    event: &KeyEvent,
) -> UiNativeInputObservationDisposition {
    observe_components(
        state,
        &event.logical_key,
        event.physical_key,
        event.state,
        event.repeat,
        event.text.as_deref(),
    )
}

pub(super) fn observe_components(
    state: &mut UiNativeInputObservationState,
    logical_key: &Key,
    physical_key: PhysicalKey,
    key_state: ElementState,
    repeat: bool,
    text: Option<&str>,
) -> UiNativeInputObservationDisposition {
    if !state.admit_input(UiNativeInputObservationEventFamily::Keyboard) {
        return state.rejection_disposition();
    }
    let (keyboard, text) = match keyboard::translate_components(
        logical_key,
        physical_key,
        key_state,
        repeat,
        text,
        state.modifiers,
    ) {
        Ok(translated) => translated,
        Err(keyboard::UiNativeKeyboardDenial::UnsupportedKey) => {
            return state.terminal_disposition(UiNativeInputObservationStop::UnsupportedKey);
        }
    };
    let next_revision = state.next_revision;
    let text = (!state.ime_enabled && state.current_input_recipient().is_some())
        .then_some(text)
        .flatten();
    let mut payloads = vec![keyboard];
    if let Some(text) = text {
        let Some(revision) = state.take_revision() else {
            return state.terminal_disposition(UiNativeInputObservationStop::TextRevisionExhausted);
        };
        payloads.push(UiHostObservationPayload::TextInput { revision, text });
    }
    if !input_affine_batch_fits(&payloads) {
        state.next_revision = next_revision;
        return state.denial_disposition(UiNativeInputObservationStop::OverCapacityText);
    }
    let disposition = state.emit_payloads(payloads);
    if disposition != UiNativeInputObservationDisposition::Retained {
        state.next_revision = next_revision;
    }
    disposition
}
