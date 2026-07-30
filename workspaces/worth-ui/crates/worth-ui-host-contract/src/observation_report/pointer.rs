#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiHostPointerIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiHostPointerCaptureEpoch(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiHostSurfacePosition {
    x_subpixels: i64,
    y_subpixels: i64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiHostPressedPointerButtons(u8);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum UiHostPointerButton {
    Primary,
    Secondary,
    Middle,
    Extra1,
    Extra2,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum UiHostPointerButtonTransition {
    Pressed,
    Released,
}

impl UiHostPointerIdentity {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl UiHostPointerCaptureEpoch {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl UiHostSurfacePosition {
    pub const fn new(x_subpixels: i64, y_subpixels: i64) -> Self {
        Self {
            x_subpixels,
            y_subpixels,
        }
    }

    pub const fn x_subpixels(self) -> i64 {
        self.x_subpixels
    }

    pub const fn y_subpixels(self) -> i64 {
        self.y_subpixels
    }
}

impl UiHostPressedPointerButtons {
    pub const NONE: Self = Self(0);

    pub fn from_buttons(buttons: impl IntoIterator<Item = UiHostPointerButton>) -> Self {
        let mut bits = 0u8;
        for button in buttons {
            bits |= button.bit();
        }
        Self(bits)
    }

    pub const fn contains(self, button: UiHostPointerButton) -> bool {
        self.0 & button.bit() != 0
    }

    pub const fn bits(self) -> u8 {
        self.0
    }
}

impl UiHostPointerButton {
    const fn bit(self) -> u8 {
        1 << self as u8
    }
}
