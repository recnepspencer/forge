use super::{
    WorthUiCommandProjectionReloadPackage, WorthUiCommandReloadPackage,
    WorthUiThemeTokenReloadPackage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiCapabilityReloadRequest {
    ThemeTokens(WorthUiThemeTokenReloadPackage),
    Commands(WorthUiCommandReloadPackage),
    CommandProjections(WorthUiCommandProjectionReloadPackage),
}

impl WorthUiCapabilityReloadRequest {
    pub fn from_theme_tokens(theme_tokens: WorthUiThemeTokenReloadPackage) -> Self {
        Self::ThemeTokens(theme_tokens)
    }

    pub fn from_commands(commands: WorthUiCommandReloadPackage) -> Self {
        Self::Commands(commands)
    }

    pub fn from_command_projections(projections: WorthUiCommandProjectionReloadPackage) -> Self {
        Self::CommandProjections(projections)
    }
}
