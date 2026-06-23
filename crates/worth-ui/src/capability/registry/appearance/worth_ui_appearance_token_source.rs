#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiAppearanceTokenSource {
    Platform,
    Application,
    Plugin,
}

impl WorthUiAppearanceTokenSource {
    pub fn platform() -> Self {
        Self::Platform
    }

    pub fn application() -> Self {
        Self::Application
    }

    pub fn plugin() -> Self {
        Self::Plugin
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Platform => "platform",
            Self::Application => "application",
            Self::Plugin => "plugin",
        }
    }
}
