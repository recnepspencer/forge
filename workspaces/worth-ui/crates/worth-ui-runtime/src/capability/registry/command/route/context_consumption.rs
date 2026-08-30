/// Exact coherent-context axes a command route is permitted to inspect.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandContextConsumption(u8);

impl UiCommandContextConsumption {
    const FOCUS: u8 = 1 << 0;
    const SELECTION: u8 = 1 << 1;
    const PORTAL_CHAIN: u8 = 1 << 2;

    pub const fn none() -> Self {
        Self(0)
    }

    pub const fn with_focus(self) -> Self {
        Self(self.0 | Self::FOCUS)
    }

    pub const fn with_selection(self) -> Self {
        Self(self.0 | Self::SELECTION)
    }

    pub const fn with_portal_chain(self) -> Self {
        Self(self.0 | Self::PORTAL_CHAIN)
    }

    pub const fn consumes_focus(self) -> bool {
        self.0 & Self::FOCUS != 0
    }

    pub const fn consumes_selection(self) -> bool {
        self.0 & Self::SELECTION != 0
    }

    pub const fn consumes_portal_chain(self) -> bool {
        self.0 & Self::PORTAL_CHAIN != 0
    }

    pub(crate) const fn bits(self) -> u8 {
        self.0
    }
}
