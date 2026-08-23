use super::{logical_key, text_for_keypress};
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
use worth_ui_host_contract::UiHostKey;

#[test]
fn unicode_character_keeps_text_and_uses_physical_key_for_key_identity() {
    assert_eq!(
        logical_key(
            &Key::Character("é".into()),
            PhysicalKey::Code(KeyCode::KeyA),
        ),
        Some(UiHostKey::A)
    );
    assert_eq!(
        text_for_keypress(&Key::Character("é".into()), Some("é"), true).as_deref(),
        Some("é")
    );
}

#[test]
fn named_enter_never_becomes_a_second_text_observation() {
    assert_eq!(
        text_for_keypress(&Key::Named(NamedKey::Enter), Some("\r"), true),
        None
    );
}
