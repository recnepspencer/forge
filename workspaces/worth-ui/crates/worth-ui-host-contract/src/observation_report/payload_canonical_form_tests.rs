use super::{
    UiHostImeCompositionPhase, UiHostImePreedit, UiHostKey, UiHostKeyTransition,
    UiHostKeyboardModifiers, UiHostObservationPayload, UiHostPointerButton,
    UiHostPointerButtonTransition, UiHostPointerCaptureEpoch, UiHostPointerIdentity,
    UiHostPressedPointerButtons, UiHostSurfacePosition,
};

#[test]
fn pointer_motion_identity_covers_position_and_coalescing_axes() {
    let motion = UiHostObservationPayload::PointerMotion {
        pointer: pointer(7),
        capture_epoch: capture_epoch(3),
        pressed_buttons: UiHostPressedPointerButtons::NONE,
        position: position(10, -20),
    };
    assert_eq!(motion.encoded_len(), 33);
    assert_axis_changes(
        &motion,
        [
            UiHostObservationPayload::PointerMotion {
                pointer: pointer(8),
                capture_epoch: capture_epoch(3),
                pressed_buttons: UiHostPressedPointerButtons::NONE,
                position: position(10, -20),
            },
            UiHostObservationPayload::PointerMotion {
                pointer: pointer(7),
                capture_epoch: capture_epoch(4),
                pressed_buttons: UiHostPressedPointerButtons::NONE,
                position: position(10, -20),
            },
            UiHostObservationPayload::PointerMotion {
                pointer: pointer(7),
                capture_epoch: capture_epoch(3),
                pressed_buttons: primary_pressed(),
                position: position(10, -20),
            },
            UiHostObservationPayload::PointerMotion {
                pointer: pointer(7),
                capture_epoch: capture_epoch(3),
                pressed_buttons: UiHostPressedPointerButtons::NONE,
                position: position(11, -20),
            },
            UiHostObservationPayload::PointerMotion {
                pointer: pointer(7),
                capture_epoch: capture_epoch(3),
                pressed_buttons: UiHostPressedPointerButtons::NONE,
                position: position(10, -21),
            },
        ],
    );
}

#[test]
fn pointer_button_identity_covers_exact_position_and_every_lossless_axis() {
    let button = pointer_button(
        7,
        3,
        UiHostPointerButton::Primary,
        UiHostPointerButtonTransition::Pressed,
        position(10, -20),
    );
    assert_eq!(button.encoded_len(), 34);
    assert_axis_changes(
        &button,
        [
            pointer_button(
                8,
                3,
                UiHostPointerButton::Primary,
                UiHostPointerButtonTransition::Pressed,
                position(10, -20),
            ),
            pointer_button(
                7,
                4,
                UiHostPointerButton::Primary,
                UiHostPointerButtonTransition::Pressed,
                position(10, -20),
            ),
            pointer_button(
                7,
                3,
                UiHostPointerButton::Secondary,
                UiHostPointerButtonTransition::Pressed,
                position(10, -20),
            ),
            pointer_button(
                7,
                3,
                UiHostPointerButton::Primary,
                UiHostPointerButtonTransition::Released,
                position(10, -20),
            ),
            pointer_button(
                7,
                3,
                UiHostPointerButton::Primary,
                UiHostPointerButtonTransition::Pressed,
                position(11, -20),
            ),
            pointer_button(
                7,
                3,
                UiHostPointerButton::Primary,
                UiHostPointerButtonTransition::Pressed,
                position(10, -21),
            ),
        ],
    );
}

#[test]
fn keyboard_payload_identity_covers_logical_physical_and_lawful_transition() {
    let key = keyboard(
        UiHostKey::A,
        Some(UiHostKey::A),
        UiHostKeyboardModifiers::default(),
        UiHostKeyTransition::Pressed { repeat: false },
    );
    assert_eq!(key.encoded_len(), 8);
    assert_axis_changes(
        &key,
        [
            keyboard(
                UiHostKey::B,
                Some(UiHostKey::A),
                UiHostKeyboardModifiers::default(),
                UiHostKeyTransition::Pressed { repeat: false },
            ),
            keyboard(
                UiHostKey::A,
                Some(UiHostKey::B),
                UiHostKeyboardModifiers::default(),
                UiHostKeyTransition::Pressed { repeat: false },
            ),
            keyboard(
                UiHostKey::A,
                None,
                UiHostKeyboardModifiers::default(),
                UiHostKeyTransition::Pressed { repeat: false },
            ),
            keyboard(
                UiHostKey::A,
                Some(UiHostKey::A),
                UiHostKeyboardModifiers::default(),
                UiHostKeyTransition::Pressed { repeat: true },
            ),
            keyboard(
                UiHostKey::A,
                Some(UiHostKey::A),
                UiHostKeyboardModifiers::default(),
                UiHostKeyTransition::Released,
            ),
        ],
    );
}

#[test]
fn keyboard_payload_identity_covers_each_modifier_bit() {
    let key = keyboard(
        UiHostKey::A,
        Some(UiHostKey::A),
        UiHostKeyboardModifiers::default(),
        UiHostKeyTransition::Pressed { repeat: false },
    );
    for modifiers in [
        UiHostKeyboardModifiers::new(true, false, false, false, false),
        UiHostKeyboardModifiers::new(false, true, false, false, false),
        UiHostKeyboardModifiers::new(false, false, true, false, false),
        UiHostKeyboardModifiers::new(false, false, false, true, false),
        UiHostKeyboardModifiers::new(false, false, false, false, true),
    ] {
        let modified = keyboard(
            UiHostKey::A,
            Some(UiHostKey::A),
            modifiers,
            UiHostKeyTransition::Pressed { repeat: false },
        );
        assert_ne!(digest(&key), digest(&modified));
    }
}

#[test]
fn text_and_ime_phases_have_distinct_identity_and_exact_byte_cost() {
    let text = "aé🦀z";
    let selected_preedit =
        UiHostImePreedit::from_unicode_scalar_range(text, Some(1..3)).expect("valid range");
    let other_selected_preedit =
        UiHostImePreedit::from_unicode_scalar_range(text, Some(0..1)).expect("valid range");
    let unselected_preedit =
        UiHostImePreedit::from_unicode_scalar_range(text, None).expect("valid preedit");
    let text_input = UiHostObservationPayload::TextInput {
        revision: 7,
        text: text.into(),
    };
    let selected = ime(7, UiHostImeCompositionPhase::Preedit(selected_preedit));
    let other_selected = ime(
        7,
        UiHostImeCompositionPhase::Preedit(other_selected_preedit),
    );
    let unselected = ime(7, UiHostImeCompositionPhase::Preedit(unselected_preedit));
    let committed = ime(7, UiHostImeCompositionPhase::Commit(text.into()));
    let cancelled = ime(7, UiHostImeCompositionPhase::Cancel);

    assert_eq!(text_input.encoded_len(), 8 + text.len());
    assert_eq!(selected.encoded_len(), 26 + text.len());
    assert_eq!(unselected.encoded_len(), 10 + text.len());
    assert_eq!(committed.encoded_len(), 9 + text.len());
    assert_eq!(cancelled.encoded_len(), 9);

    let digests = [
        digest(&text_input),
        digest(&selected),
        digest(&other_selected),
        digest(&unselected),
        digest(&committed),
        digest(&cancelled),
        digest(&ime(8, UiHostImeCompositionPhase::Cancel)),
    ];
    for (index, left) in digests.iter().enumerate() {
        for right in &digests[index + 1..] {
            assert_ne!(left, right, "one semantic axis must change identity");
        }
    }
}

fn assert_axis_changes<const N: usize>(
    base: &UiHostObservationPayload,
    variants: [UiHostObservationPayload; N],
) {
    for variant in variants {
        assert_ne!(
            digest(base),
            digest(&variant),
            "one semantic axis must change identity"
        );
    }
}

fn pointer(value: u64) -> UiHostPointerIdentity {
    UiHostPointerIdentity::new(value)
}

fn capture_epoch(value: u64) -> UiHostPointerCaptureEpoch {
    UiHostPointerCaptureEpoch::new(value)
}

fn position(x: i64, y: i64) -> UiHostSurfacePosition {
    UiHostSurfacePosition::new(x, y)
}

fn primary_pressed() -> UiHostPressedPointerButtons {
    UiHostPressedPointerButtons::from_buttons([UiHostPointerButton::Primary])
}

fn pointer_button(
    pointer_value: u64,
    capture_epoch_value: u64,
    button: UiHostPointerButton,
    transition: UiHostPointerButtonTransition,
    position: UiHostSurfacePosition,
) -> UiHostObservationPayload {
    UiHostObservationPayload::PointerButton {
        pointer: pointer(pointer_value),
        capture_epoch: capture_epoch(capture_epoch_value),
        button,
        transition,
        position,
    }
}

fn keyboard(
    logical_key: UiHostKey,
    physical_key: Option<UiHostKey>,
    modifiers: UiHostKeyboardModifiers,
    transition: UiHostKeyTransition,
) -> UiHostObservationPayload {
    UiHostObservationPayload::Keyboard {
        logical_key,
        physical_key,
        modifiers,
        transition,
    }
}

fn ime(revision: u64, phase: UiHostImeCompositionPhase) -> UiHostObservationPayload {
    UiHostObservationPayload::ImeComposition { revision, phase }
}

fn digest(payload: &UiHostObservationPayload) -> u64 {
    payload.integrity_digest()
}
