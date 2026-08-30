/// Canonical shortcut modifiers. `Primary` remains an authored platform alias
/// until command routing resolves it against an explicit platform.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandModifierSet(u8);

impl UiCommandModifierSet {
    const ALT: u8 = 1 << 0;
    const CONTROL: u8 = 1 << 1;
    const SHIFT: u8 = 1 << 2;
    const META: u8 = 1 << 3;
    const PRIMARY: u8 = 1 << 4;

    pub const fn none() -> Self {
        Self(0)
    }

    pub const fn with_alt(self) -> Self {
        Self(self.0 | Self::ALT)
    }

    pub const fn with_control(self) -> Self {
        Self(self.0 | Self::CONTROL)
    }

    pub const fn with_shift(self) -> Self {
        Self(self.0 | Self::SHIFT)
    }

    pub const fn with_meta(self) -> Self {
        Self(self.0 | Self::META)
    }

    pub const fn with_primary(self) -> Self {
        Self(self.0 | Self::PRIMARY)
    }

    pub const fn alt(self) -> bool {
        self.0 & Self::ALT != 0
    }

    pub const fn control(self) -> bool {
        self.0 & Self::CONTROL != 0
    }

    pub const fn shift(self) -> bool {
        self.0 & Self::SHIFT != 0
    }

    pub const fn meta(self) -> bool {
        self.0 & Self::META != 0
    }

    pub const fn primary(self) -> bool {
        self.0 & Self::PRIMARY != 0
    }

    pub(crate) const fn bits(self) -> u8 {
        self.0
    }

    pub(crate) const fn has_conflicting_primary_alias(self) -> bool {
        self.primary() && (self.control() || self.meta())
    }

    pub(crate) const fn resolved_for(self, platform: super::UiCommandShortcutPlatform) -> Self {
        let mut bits = self.0 & !Self::PRIMARY;
        if self.primary() {
            bits |= match platform {
                super::UiCommandShortcutPlatform::MacOs => Self::META,
                super::UiCommandShortcutPlatform::Windows
                | super::UiCommandShortcutPlatform::Linux => Self::CONTROL,
            };
        }
        Self(bits)
    }
}
