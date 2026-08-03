#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum UiHostKey {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    Escape,
    Tab,
    Backspace,
    Enter,
    Space,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Copy,
    Cut,
    Paste,
    Colon,
    Comma,
    Backslash,
    Slash,
    Pipe,
    Questionmark,
    Exclamationmark,
    OpenBracket,
    CloseBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Backtick,
    Minus,
    Period,
    Plus,
    Equals,
    Semicolon,
    Quote,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
    BrowserBack,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    SuperLeft,
    SuperRight,
    IntlBackslash,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UiHostKeyboardModifiers {
    alt: bool,
    control: bool,
    shift: bool,
    mac_command: bool,
    command: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UiHostKeyTransition {
    Pressed { repeat: bool },
    Released,
}

impl UiHostKeyboardModifiers {
    pub const fn new(
        alt: bool,
        control: bool,
        shift: bool,
        mac_command: bool,
        command: bool,
    ) -> Self {
        Self {
            alt,
            control,
            shift,
            mac_command,
            command,
        }
    }

    pub const fn alt(self) -> bool {
        self.alt
    }

    pub const fn control(self) -> bool {
        self.control
    }

    pub const fn shift(self) -> bool {
        self.shift
    }

    pub const fn mac_command(self) -> bool {
        self.mac_command
    }

    pub const fn command(self) -> bool {
        self.command
    }

    pub(crate) const fn bits(self) -> u8 {
        self.alt as u8
            | (self.control as u8) << 1
            | (self.shift as u8) << 2
            | (self.mac_command as u8) << 3
            | (self.command as u8) << 4
    }
}
