use super::UiCommandKeyCode;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandLogicalKey(UiCommandKeyCode);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandPhysicalKey(UiCommandKeyCode);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiCommandShortcutKey {
    Logical(UiCommandLogicalKey),
    Physical(UiCommandPhysicalKey),
}

impl UiCommandLogicalKey {
    pub const fn new(code: UiCommandKeyCode) -> Self {
        Self(code)
    }

    pub const fn code(self) -> UiCommandKeyCode {
        self.0
    }
}

impl UiCommandPhysicalKey {
    pub const fn new(code: UiCommandKeyCode) -> Self {
        Self(code)
    }

    pub const fn code(self) -> UiCommandKeyCode {
        self.0
    }
}

impl UiCommandShortcutKey {
    pub const fn logical(code: UiCommandKeyCode) -> Self {
        Self::Logical(UiCommandLogicalKey::new(code))
    }

    pub const fn physical(code: UiCommandKeyCode) -> Self {
        Self::Physical(UiCommandPhysicalKey::new(code))
    }

    pub const fn code(self) -> UiCommandKeyCode {
        match self {
            Self::Logical(key) => key.code(),
            Self::Physical(key) => key.code(),
        }
    }

    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Physical(_))
    }
}
