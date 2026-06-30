#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SettingScope {
    User,
    Workspace,
    Project,
    App,
    Plugin,
    Theme,
    Density,
    Keyboard,
    Accessibility,
}

impl SettingScope {
    pub fn user() -> Self {
        Self::User
    }

    pub fn workspace() -> Self {
        Self::Workspace
    }

    pub fn project() -> Self {
        Self::Project
    }

    pub fn app() -> Self {
        Self::App
    }

    pub fn plugin() -> Self {
        Self::Plugin
    }

    pub fn theme() -> Self {
        Self::Theme
    }

    pub fn density() -> Self {
        Self::Density
    }

    pub fn keyboard() -> Self {
        Self::Keyboard
    }

    pub fn accessibility() -> Self {
        Self::Accessibility
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Workspace => "workspace",
            Self::Project => "project",
            Self::App => "app",
            Self::Plugin => "plugin",
            Self::Theme => "theme",
            Self::Density => "density",
            Self::Keyboard => "keyboard",
            Self::Accessibility => "accessibility",
        }
    }
}
