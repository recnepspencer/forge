pub(crate) fn keyboard_stroke(
    payload: &worth_ui_host_contract::UiHostObservationPayload,
) -> Option<(super::input_stroke::UiCommandInputStroke, bool)> {
    let worth_ui_host_contract::UiHostObservationPayload::Keyboard {
        logical_key,
        physical_key,
        modifiers,
        transition,
        ..
    } = payload
    else {
        return None;
    };
    let worth_ui_host_contract::UiHostKeyTransition::Pressed { repeat } = transition else {
        return None;
    };
    let modifiers = command_modifiers(*modifiers);
    let logical =
        crate::capability::UiCommandShortcutStroke::logical(command_key(*logical_key)?, modifiers);
    let physical = physical_key.and_then(|key| {
        command_key(key)
            .map(|key| crate::capability::UiCommandShortcutStroke::physical(key, modifiers))
    });
    Some((
        super::input_stroke::UiCommandInputStroke::new(logical, physical),
        *repeat,
    ))
}

fn command_modifiers(
    host: worth_ui_host_contract::UiHostKeyboardModifiers,
) -> crate::capability::UiCommandModifierSet {
    let mut modifiers = crate::capability::UiCommandModifierSet::none();
    if host.alt() {
        modifiers = modifiers.with_alt();
    }
    if host.control() {
        modifiers = modifiers.with_control();
    }
    if host.shift() {
        modifiers = modifiers.with_shift();
    }
    if host.mac_command() || host.command() {
        modifiers = modifiers.with_meta();
    }
    modifiers
}

fn command_key(
    host: worth_ui_host_contract::UiHostKey,
) -> Option<crate::capability::UiCommandKeyCode> {
    use crate::capability::UiCommandKeyCode as Command;
    use worth_ui_host_contract::UiHostKey as Host;
    Some(match host {
        Host::ArrowDown => Command::ArrowDown,
        Host::ArrowLeft => Command::ArrowLeft,
        Host::ArrowRight => Command::ArrowRight,
        Host::ArrowUp => Command::ArrowUp,
        Host::Escape => Command::Escape,
        Host::Tab => Command::Tab,
        Host::Backspace => Command::Backspace,
        Host::Enter => Command::Enter,
        Host::Space => Command::Space,
        Host::Insert => Command::Insert,
        Host::Delete => Command::Delete,
        Host::Home => Command::Home,
        Host::End => Command::End,
        Host::PageUp => Command::PageUp,
        Host::PageDown => Command::PageDown,
        Host::Copy => Command::Copy,
        Host::Cut => Command::Cut,
        Host::Paste => Command::Paste,
        Host::Colon => Command::Colon,
        Host::Comma => Command::Comma,
        Host::Backslash => Command::Backslash,
        Host::Slash => Command::Slash,
        Host::Pipe => Command::Pipe,
        Host::Questionmark => Command::QuestionMark,
        Host::Exclamationmark => Command::ExclamationMark,
        Host::OpenBracket => Command::OpenBracket,
        Host::CloseBracket => Command::CloseBracket,
        Host::OpenCurlyBracket => Command::OpenCurlyBracket,
        Host::CloseCurlyBracket => Command::CloseCurlyBracket,
        Host::Backtick => Command::Backtick,
        Host::Minus => Command::Minus,
        Host::Period => Command::Period,
        Host::Plus => Command::Plus,
        Host::Equals => Command::Equals,
        Host::Semicolon => Command::Semicolon,
        Host::Quote => Command::Quote,
        Host::Num0 => Command::Num0,
        Host::Num1 => Command::Num1,
        Host::Num2 => Command::Num2,
        Host::Num3 => Command::Num3,
        Host::Num4 => Command::Num4,
        Host::Num5 => Command::Num5,
        Host::Num6 => Command::Num6,
        Host::Num7 => Command::Num7,
        Host::Num8 => Command::Num8,
        Host::Num9 => Command::Num9,
        Host::A => Command::A,
        Host::B => Command::B,
        Host::C => Command::C,
        Host::D => Command::D,
        Host::E => Command::E,
        Host::F => Command::F,
        Host::G => Command::G,
        Host::H => Command::H,
        Host::I => Command::I,
        Host::J => Command::J,
        Host::K => Command::K,
        Host::L => Command::L,
        Host::M => Command::M,
        Host::N => Command::N,
        Host::O => Command::O,
        Host::P => Command::P,
        Host::Q => Command::Q,
        Host::R => Command::R,
        Host::S => Command::S,
        Host::T => Command::T,
        Host::U => Command::U,
        Host::V => Command::V,
        Host::W => Command::W,
        Host::X => Command::X,
        Host::Y => Command::Y,
        Host::Z => Command::Z,
        Host::F1 => Command::F1,
        Host::F2 => Command::F2,
        Host::F3 => Command::F3,
        Host::F4 => Command::F4,
        Host::F5 => Command::F5,
        Host::F6 => Command::F6,
        Host::F7 => Command::F7,
        Host::F8 => Command::F8,
        Host::F9 => Command::F9,
        Host::F10 => Command::F10,
        Host::F11 => Command::F11,
        Host::F12 => Command::F12,
        Host::F13 => Command::F13,
        Host::F14 => Command::F14,
        Host::F15 => Command::F15,
        Host::F16 => Command::F16,
        Host::F17 => Command::F17,
        Host::F18 => Command::F18,
        Host::F19 => Command::F19,
        Host::F20 => Command::F20,
        Host::F21 => Command::F21,
        Host::F22 => Command::F22,
        Host::F23 => Command::F23,
        Host::F24 => Command::F24,
        Host::F25 => Command::F25,
        Host::F26 => Command::F26,
        Host::F27 => Command::F27,
        Host::F28 => Command::F28,
        Host::F29 => Command::F29,
        Host::F30 => Command::F30,
        Host::F31 => Command::F31,
        Host::F32 => Command::F32,
        Host::F33 => Command::F33,
        Host::F34 => Command::F34,
        Host::F35 => Command::F35,
        Host::BrowserBack => Command::BrowserBack,
        Host::IntlBackslash => Command::InternationalBackslash,
        Host::ShiftLeft
        | Host::ShiftRight
        | Host::ControlLeft
        | Host::ControlRight
        | Host::AltLeft
        | Host::AltRight
        | Host::SuperLeft
        | Host::SuperRight => return None,
    })
}
