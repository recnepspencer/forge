use super::{presented_state_without_recipient, HOST_SESSION};
use crate::native::input::observation::UiNativeInputObservationDisposition;
use winit::event::ElementState;
use winit::keyboard::{Key, KeyCode, PhysicalKey};
use worth_ui_host_contract::UiHostObservationPayload;

#[test]
fn command_keyboard_is_retained_without_a_local_input_recipient() {
    let mut state = presented_state_without_recipient();
    let disposition = state.observe_keyboard_components_at(
        &Key::Character("P".into()),
        PhysicalKey::Code(KeyCode::KeyP),
        ElementState::Pressed,
        false,
        None,
        10,
    );
    assert_eq!(disposition, UiNativeInputObservationDisposition::Retained);
    let batches = state.drain(HOST_SESSION).into_batches();
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].reports().len(), 1);
    assert!(matches!(
        batches[0].reports()[0].payload(),
        UiHostObservationPayload::Keyboard { .. }
    ));
    assert_eq!(batches[0].reports()[0].input_affinity(), None);
}

#[test]
fn text_is_not_emitted_without_a_local_input_recipient() {
    let mut state = presented_state_without_recipient();
    let disposition = state.observe_keyboard_components_at(
        &Key::Character("a".into()),
        PhysicalKey::Code(KeyCode::KeyA),
        ElementState::Pressed,
        false,
        Some("a"),
        10,
    );
    assert_eq!(disposition, UiNativeInputObservationDisposition::Retained);
    let batches = state.drain(HOST_SESSION).into_batches();
    assert_eq!(batches[0].reports().len(), 1);
    assert!(matches!(
        batches[0].reports()[0].payload(),
        UiHostObservationPayload::Keyboard { .. }
    ));
}
