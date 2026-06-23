#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiCapabilityReloadFamilyKind {
    ThemeTokens,
    Commands,
    CommandProjections,
    Components,
    Density,
    Appearance,
}

impl WorthUiCapabilityReloadFamilyKind {
    pub(crate) fn token(self) -> &'static str {
        match self {
            Self::ThemeTokens => "theme_tokens",
            Self::Commands => "commands",
            Self::CommandProjections => "command_projections",
            Self::Components => "components",
            Self::Density => "density",
            Self::Appearance => "appearance",
        }
    }
}
