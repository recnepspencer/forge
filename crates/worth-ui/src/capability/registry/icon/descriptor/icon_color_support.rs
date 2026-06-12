#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconColorSupport {
    InheritsTextColor,
    ThemeTokenDriven,
    FixedColor,
    Missing,
}

impl IconColorSupport {
    pub fn inherits_text_color() -> Self {
        Self::InheritsTextColor
    }

    pub fn theme_token_driven() -> Self {
        Self::ThemeTokenDriven
    }

    pub fn fixed_color() -> Self {
        Self::FixedColor
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::Missing
    }

    pub(crate) fn is_missing(self) -> bool {
        matches!(self, Self::Missing)
    }

    pub(crate) fn requires_theme_token(self) -> bool {
        matches!(self, Self::ThemeTokenDriven)
    }

    pub(crate) fn admits_theme_posture(self, posture: IconThemePosture) -> bool {
        match posture {
            IconThemePosture::InheritsTextColor => matches!(self, Self::InheritsTextColor),
            IconThemePosture::ThemeTokenDriven => matches!(self, Self::ThemeTokenDriven),
            IconThemePosture::FixedColorSafe => matches!(self, Self::FixedColor),
            IconThemePosture::Missing => true,
        }
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::InheritsTextColor => "inherits_text_color",
            Self::ThemeTokenDriven => "theme_token_driven",
            Self::FixedColor => "fixed_color",
            Self::Missing => "missing",
        }
    }
}

use super::IconThemePosture;
