#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconThemePosture {
    InheritsTextColor,
    ThemeTokenDriven,
    FixedColorSafe,
    Missing,
}

impl IconThemePosture {
    pub fn inherits_text_color() -> Self {
        Self::InheritsTextColor
    }

    pub fn theme_token_driven() -> Self {
        Self::ThemeTokenDriven
    }

    pub fn fixed_color_safe() -> Self {
        Self::FixedColorSafe
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::Missing
    }

    pub(crate) fn is_missing(self) -> bool {
        matches!(self, Self::Missing)
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::InheritsTextColor => "inherits_text_color",
            Self::ThemeTokenDriven => "theme_token_driven",
            Self::FixedColorSafe => "fixed_color_safe",
            Self::Missing => "missing",
        }
    }
}
