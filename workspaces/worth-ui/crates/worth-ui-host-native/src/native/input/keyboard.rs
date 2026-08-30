use winit::event::{ElementState, Modifiers};
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
use worth_ui_host_contract::{
    UiHostKey, UiHostKeyTransition, UiHostKeyboardModifiers, UiHostObservationPayload,
};

#[derive(Debug)]
pub(crate) enum UiNativeKeyboardDenial {
    UnsupportedKey,
}

pub(crate) fn modifiers(modifiers: Modifiers) -> UiHostKeyboardModifiers {
    let state = modifiers.state();
    UiHostKeyboardModifiers::new(
        state.alt_key(),
        state.control_key(),
        state.shift_key(),
        cfg!(target_os = "macos") && state.super_key(),
        !cfg!(target_os = "macos") && state.super_key(),
    )
}

pub(crate) fn translate_components(
    logical_key: &Key,
    physical_key: PhysicalKey,
    state: ElementState,
    repeat: bool,
    text: Option<&str>,
    modifiers: UiHostKeyboardModifiers,
) -> Result<(UiHostObservationPayload, Option<Box<str>>), UiNativeKeyboardDenial> {
    let logical = self::logical_key(logical_key, physical_key)
        .ok_or(UiNativeKeyboardDenial::UnsupportedKey)?;
    let physical = self::physical_key(physical_key);
    let transition = match state {
        ElementState::Pressed => UiHostKeyTransition::Pressed { repeat },
        ElementState::Released => UiHostKeyTransition::Released,
    };
    let text = text_for_keypress(logical_key, text, state == ElementState::Pressed);
    Ok((
        UiHostObservationPayload::Keyboard {
            logical_key: logical,
            physical_key: physical,
            modifiers,
            transition,
        },
        text,
    ))
}

fn text_for_keypress(logical_key: &Key, text: Option<&str>, pressed: bool) -> Option<Box<str>> {
    (pressed && matches!(logical_key, Key::Character(_) | Key::Dead(_)))
        .then_some(text)
        .flatten()
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
        .map(String::into_boxed_str)
}

fn logical_key(key: &Key, physical: PhysicalKey) -> Option<UiHostKey> {
    match key {
        Key::Named(named) => named_key(*named, physical),
        Key::Character(text) => text
            .chars()
            .next()
            .filter(|_| text.chars().count() == 1)
            .and_then(character_key)
            .or_else(|| physical_key(physical)),
        Key::Dead(Some(character)) => character_key(*character).or_else(|| physical_key(physical)),
        Key::Dead(None) | Key::Unidentified(_) => physical_key(physical),
    }
}

fn character_key(character: char) -> Option<UiHostKey> {
    Some(match character {
        '0' => UiHostKey::Num0,
        '1' => UiHostKey::Num1,
        '2' => UiHostKey::Num2,
        '3' => UiHostKey::Num3,
        '4' => UiHostKey::Num4,
        '5' => UiHostKey::Num5,
        '6' => UiHostKey::Num6,
        '7' => UiHostKey::Num7,
        '8' => UiHostKey::Num8,
        '9' => UiHostKey::Num9,
        'a' | 'A' => UiHostKey::A,
        'b' | 'B' => UiHostKey::B,
        'c' | 'C' => UiHostKey::C,
        'd' | 'D' => UiHostKey::D,
        'e' | 'E' => UiHostKey::E,
        'f' | 'F' => UiHostKey::F,
        'g' | 'G' => UiHostKey::G,
        'h' | 'H' => UiHostKey::H,
        'i' | 'I' => UiHostKey::I,
        'j' | 'J' => UiHostKey::J,
        'k' | 'K' => UiHostKey::K,
        'l' | 'L' => UiHostKey::L,
        'm' | 'M' => UiHostKey::M,
        'n' | 'N' => UiHostKey::N,
        'o' | 'O' => UiHostKey::O,
        'p' | 'P' => UiHostKey::P,
        'q' | 'Q' => UiHostKey::Q,
        'r' | 'R' => UiHostKey::R,
        's' | 'S' => UiHostKey::S,
        't' | 'T' => UiHostKey::T,
        'u' | 'U' => UiHostKey::U,
        'v' | 'V' => UiHostKey::V,
        'w' | 'W' => UiHostKey::W,
        'x' | 'X' => UiHostKey::X,
        'y' | 'Y' => UiHostKey::Y,
        'z' | 'Z' => UiHostKey::Z,
        ' ' => UiHostKey::Space,
        ':' => UiHostKey::Colon,
        ',' => UiHostKey::Comma,
        '\\' => UiHostKey::Backslash,
        '/' => UiHostKey::Slash,
        '|' => UiHostKey::Pipe,
        '?' => UiHostKey::Questionmark,
        '!' => UiHostKey::Exclamationmark,
        '[' => UiHostKey::OpenBracket,
        ']' => UiHostKey::CloseBracket,
        '{' => UiHostKey::OpenCurlyBracket,
        '}' => UiHostKey::CloseCurlyBracket,
        '`' => UiHostKey::Backtick,
        '-' => UiHostKey::Minus,
        '.' => UiHostKey::Period,
        '+' => UiHostKey::Plus,
        '=' => UiHostKey::Equals,
        ';' => UiHostKey::Semicolon,
        '\'' => UiHostKey::Quote,
        _ => return None,
    })
}

fn named_key(named: NamedKey, physical: PhysicalKey) -> Option<UiHostKey> {
    Some(match named {
        NamedKey::ArrowDown => UiHostKey::ArrowDown,
        NamedKey::ArrowLeft => UiHostKey::ArrowLeft,
        NamedKey::ArrowRight => UiHostKey::ArrowRight,
        NamedKey::ArrowUp => UiHostKey::ArrowUp,
        NamedKey::Escape => UiHostKey::Escape,
        NamedKey::Tab => UiHostKey::Tab,
        NamedKey::Backspace => UiHostKey::Backspace,
        NamedKey::Enter => UiHostKey::Enter,
        NamedKey::Space => UiHostKey::Space,
        NamedKey::Insert => UiHostKey::Insert,
        NamedKey::Delete => UiHostKey::Delete,
        NamedKey::Home => UiHostKey::Home,
        NamedKey::End => UiHostKey::End,
        NamedKey::PageUp => UiHostKey::PageUp,
        NamedKey::PageDown => UiHostKey::PageDown,
        NamedKey::Copy => UiHostKey::Copy,
        NamedKey::Cut => UiHostKey::Cut,
        NamedKey::Paste => UiHostKey::Paste,
        NamedKey::BrowserBack => UiHostKey::BrowserBack,
        NamedKey::F1 => UiHostKey::F1,
        NamedKey::F2 => UiHostKey::F2,
        NamedKey::F3 => UiHostKey::F3,
        NamedKey::F4 => UiHostKey::F4,
        NamedKey::F5 => UiHostKey::F5,
        NamedKey::F6 => UiHostKey::F6,
        NamedKey::F7 => UiHostKey::F7,
        NamedKey::F8 => UiHostKey::F8,
        NamedKey::F9 => UiHostKey::F9,
        NamedKey::F10 => UiHostKey::F10,
        NamedKey::F11 => UiHostKey::F11,
        NamedKey::F12 => UiHostKey::F12,
        NamedKey::F13 => UiHostKey::F13,
        NamedKey::F14 => UiHostKey::F14,
        NamedKey::F15 => UiHostKey::F15,
        NamedKey::F16 => UiHostKey::F16,
        NamedKey::F17 => UiHostKey::F17,
        NamedKey::F18 => UiHostKey::F18,
        NamedKey::F19 => UiHostKey::F19,
        NamedKey::F20 => UiHostKey::F20,
        NamedKey::F21 => UiHostKey::F21,
        NamedKey::F22 => UiHostKey::F22,
        NamedKey::F23 => UiHostKey::F23,
        NamedKey::F24 => UiHostKey::F24,
        NamedKey::F25 => UiHostKey::F25,
        NamedKey::F26 => UiHostKey::F26,
        NamedKey::F27 => UiHostKey::F27,
        NamedKey::F28 => UiHostKey::F28,
        NamedKey::F29 => UiHostKey::F29,
        NamedKey::F30 => UiHostKey::F30,
        NamedKey::F31 => UiHostKey::F31,
        NamedKey::F32 => UiHostKey::F32,
        NamedKey::F33 => UiHostKey::F33,
        NamedKey::F34 => UiHostKey::F34,
        NamedKey::F35 => UiHostKey::F35,
        NamedKey::Shift => match physical {
            PhysicalKey::Code(KeyCode::ShiftRight) => UiHostKey::ShiftRight,
            _ => UiHostKey::ShiftLeft,
        },
        NamedKey::Control => match physical {
            PhysicalKey::Code(KeyCode::ControlRight) => UiHostKey::ControlRight,
            _ => UiHostKey::ControlLeft,
        },
        NamedKey::Alt | NamedKey::AltGraph => match physical {
            PhysicalKey::Code(KeyCode::AltRight) => UiHostKey::AltRight,
            _ => UiHostKey::AltLeft,
        },
        NamedKey::Super => match physical {
            PhysicalKey::Code(KeyCode::SuperRight) => UiHostKey::SuperRight,
            _ => UiHostKey::SuperLeft,
        },
        _ => return None,
    })
}

fn physical_key(physical: PhysicalKey) -> Option<UiHostKey> {
    Some(match physical {
        PhysicalKey::Code(KeyCode::ArrowDown) => UiHostKey::ArrowDown,
        PhysicalKey::Code(KeyCode::ArrowLeft) => UiHostKey::ArrowLeft,
        PhysicalKey::Code(KeyCode::ArrowRight) => UiHostKey::ArrowRight,
        PhysicalKey::Code(KeyCode::ArrowUp) => UiHostKey::ArrowUp,
        PhysicalKey::Code(KeyCode::Escape) => UiHostKey::Escape,
        PhysicalKey::Code(KeyCode::Tab) => UiHostKey::Tab,
        PhysicalKey::Code(KeyCode::Backspace) => UiHostKey::Backspace,
        PhysicalKey::Code(KeyCode::Enter) => UiHostKey::Enter,
        PhysicalKey::Code(KeyCode::Space) => UiHostKey::Space,
        PhysicalKey::Code(KeyCode::Insert) => UiHostKey::Insert,
        PhysicalKey::Code(KeyCode::Delete) => UiHostKey::Delete,
        PhysicalKey::Code(KeyCode::Home) => UiHostKey::Home,
        PhysicalKey::Code(KeyCode::End) => UiHostKey::End,
        PhysicalKey::Code(KeyCode::PageUp) => UiHostKey::PageUp,
        PhysicalKey::Code(KeyCode::PageDown) => UiHostKey::PageDown,
        PhysicalKey::Code(KeyCode::Backquote) => UiHostKey::Backtick,
        PhysicalKey::Code(KeyCode::Backslash) => UiHostKey::Backslash,
        PhysicalKey::Code(KeyCode::BracketLeft) => UiHostKey::OpenBracket,
        PhysicalKey::Code(KeyCode::BracketRight) => UiHostKey::CloseBracket,
        PhysicalKey::Code(KeyCode::Comma) => UiHostKey::Comma,
        PhysicalKey::Code(KeyCode::Equal) => UiHostKey::Equals,
        PhysicalKey::Code(KeyCode::Minus) => UiHostKey::Minus,
        PhysicalKey::Code(KeyCode::Period) => UiHostKey::Period,
        PhysicalKey::Code(KeyCode::Quote) => UiHostKey::Quote,
        PhysicalKey::Code(KeyCode::Semicolon) => UiHostKey::Semicolon,
        PhysicalKey::Code(KeyCode::Slash) => UiHostKey::Slash,
        PhysicalKey::Code(KeyCode::Digit0 | KeyCode::Numpad0) => UiHostKey::Num0,
        PhysicalKey::Code(KeyCode::Digit1 | KeyCode::Numpad1) => UiHostKey::Num1,
        PhysicalKey::Code(KeyCode::Digit2 | KeyCode::Numpad2) => UiHostKey::Num2,
        PhysicalKey::Code(KeyCode::Digit3 | KeyCode::Numpad3) => UiHostKey::Num3,
        PhysicalKey::Code(KeyCode::Digit4 | KeyCode::Numpad4) => UiHostKey::Num4,
        PhysicalKey::Code(KeyCode::Digit5 | KeyCode::Numpad5) => UiHostKey::Num5,
        PhysicalKey::Code(KeyCode::Digit6 | KeyCode::Numpad6) => UiHostKey::Num6,
        PhysicalKey::Code(KeyCode::Digit7 | KeyCode::Numpad7) => UiHostKey::Num7,
        PhysicalKey::Code(KeyCode::Digit8 | KeyCode::Numpad8) => UiHostKey::Num8,
        PhysicalKey::Code(KeyCode::Digit9 | KeyCode::Numpad9) => UiHostKey::Num9,
        PhysicalKey::Code(KeyCode::KeyA) => UiHostKey::A,
        PhysicalKey::Code(KeyCode::KeyB) => UiHostKey::B,
        PhysicalKey::Code(KeyCode::KeyC) => UiHostKey::C,
        PhysicalKey::Code(KeyCode::KeyD) => UiHostKey::D,
        PhysicalKey::Code(KeyCode::KeyE) => UiHostKey::E,
        PhysicalKey::Code(KeyCode::KeyF) => UiHostKey::F,
        PhysicalKey::Code(KeyCode::KeyG) => UiHostKey::G,
        PhysicalKey::Code(KeyCode::KeyH) => UiHostKey::H,
        PhysicalKey::Code(KeyCode::KeyI) => UiHostKey::I,
        PhysicalKey::Code(KeyCode::KeyJ) => UiHostKey::J,
        PhysicalKey::Code(KeyCode::KeyK) => UiHostKey::K,
        PhysicalKey::Code(KeyCode::KeyL) => UiHostKey::L,
        PhysicalKey::Code(KeyCode::KeyM) => UiHostKey::M,
        PhysicalKey::Code(KeyCode::KeyN) => UiHostKey::N,
        PhysicalKey::Code(KeyCode::KeyO) => UiHostKey::O,
        PhysicalKey::Code(KeyCode::KeyP) => UiHostKey::P,
        PhysicalKey::Code(KeyCode::KeyQ) => UiHostKey::Q,
        PhysicalKey::Code(KeyCode::KeyR) => UiHostKey::R,
        PhysicalKey::Code(KeyCode::KeyS) => UiHostKey::S,
        PhysicalKey::Code(KeyCode::KeyT) => UiHostKey::T,
        PhysicalKey::Code(KeyCode::KeyU) => UiHostKey::U,
        PhysicalKey::Code(KeyCode::KeyV) => UiHostKey::V,
        PhysicalKey::Code(KeyCode::KeyW) => UiHostKey::W,
        PhysicalKey::Code(KeyCode::KeyX) => UiHostKey::X,
        PhysicalKey::Code(KeyCode::KeyY) => UiHostKey::Y,
        PhysicalKey::Code(KeyCode::KeyZ) => UiHostKey::Z,
        PhysicalKey::Code(KeyCode::F1) => UiHostKey::F1,
        PhysicalKey::Code(KeyCode::F2) => UiHostKey::F2,
        PhysicalKey::Code(KeyCode::F3) => UiHostKey::F3,
        PhysicalKey::Code(KeyCode::F4) => UiHostKey::F4,
        PhysicalKey::Code(KeyCode::F5) => UiHostKey::F5,
        PhysicalKey::Code(KeyCode::F6) => UiHostKey::F6,
        PhysicalKey::Code(KeyCode::F7) => UiHostKey::F7,
        PhysicalKey::Code(KeyCode::F8) => UiHostKey::F8,
        PhysicalKey::Code(KeyCode::F9) => UiHostKey::F9,
        PhysicalKey::Code(KeyCode::F10) => UiHostKey::F10,
        PhysicalKey::Code(KeyCode::F11) => UiHostKey::F11,
        PhysicalKey::Code(KeyCode::F12) => UiHostKey::F12,
        PhysicalKey::Code(KeyCode::F13) => UiHostKey::F13,
        PhysicalKey::Code(KeyCode::F14) => UiHostKey::F14,
        PhysicalKey::Code(KeyCode::F15) => UiHostKey::F15,
        PhysicalKey::Code(KeyCode::F16) => UiHostKey::F16,
        PhysicalKey::Code(KeyCode::F17) => UiHostKey::F17,
        PhysicalKey::Code(KeyCode::F18) => UiHostKey::F18,
        PhysicalKey::Code(KeyCode::F19) => UiHostKey::F19,
        PhysicalKey::Code(KeyCode::F20) => UiHostKey::F20,
        PhysicalKey::Code(KeyCode::F21) => UiHostKey::F21,
        PhysicalKey::Code(KeyCode::F22) => UiHostKey::F22,
        PhysicalKey::Code(KeyCode::F23) => UiHostKey::F23,
        PhysicalKey::Code(KeyCode::F24) => UiHostKey::F24,
        PhysicalKey::Code(KeyCode::F25) => UiHostKey::F25,
        PhysicalKey::Code(KeyCode::F26) => UiHostKey::F26,
        PhysicalKey::Code(KeyCode::F27) => UiHostKey::F27,
        PhysicalKey::Code(KeyCode::F28) => UiHostKey::F28,
        PhysicalKey::Code(KeyCode::F29) => UiHostKey::F29,
        PhysicalKey::Code(KeyCode::F30) => UiHostKey::F30,
        PhysicalKey::Code(KeyCode::F31) => UiHostKey::F31,
        PhysicalKey::Code(KeyCode::F32) => UiHostKey::F32,
        PhysicalKey::Code(KeyCode::F33) => UiHostKey::F33,
        PhysicalKey::Code(KeyCode::F34) => UiHostKey::F34,
        PhysicalKey::Code(KeyCode::F35) => UiHostKey::F35,
        PhysicalKey::Code(KeyCode::BrowserBack) => UiHostKey::BrowserBack,
        PhysicalKey::Code(KeyCode::ShiftLeft) => UiHostKey::ShiftLeft,
        PhysicalKey::Code(KeyCode::ShiftRight) => UiHostKey::ShiftRight,
        PhysicalKey::Code(KeyCode::ControlLeft) => UiHostKey::ControlLeft,
        PhysicalKey::Code(KeyCode::ControlRight) => UiHostKey::ControlRight,
        PhysicalKey::Code(KeyCode::AltLeft) => UiHostKey::AltLeft,
        PhysicalKey::Code(KeyCode::AltRight) => UiHostKey::AltRight,
        PhysicalKey::Code(KeyCode::SuperLeft) => UiHostKey::SuperLeft,
        PhysicalKey::Code(KeyCode::SuperRight) => UiHostKey::SuperRight,
        PhysicalKey::Code(KeyCode::IntlBackslash) => UiHostKey::IntlBackslash,
        _ => return None,
    })
}

#[cfg(test)]
#[path = "keyboard_tests.rs"]
mod tests;
