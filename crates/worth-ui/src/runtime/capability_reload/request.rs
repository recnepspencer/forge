use super::WorthUiThemeTokenReloadPackage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiCapabilityReloadRequest {
    ThemeTokens(WorthUiThemeTokenReloadPackage),
}

impl WorthUiCapabilityReloadRequest {
    pub fn from_theme_tokens(theme_tokens: WorthUiThemeTokenReloadPackage) -> Self {
        Self::ThemeTokens(theme_tokens)
    }
}
