#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiCommandShortcutPlatform {
    Windows,
    MacOs,
    Linux,
}

impl UiCommandShortcutPlatform {
    pub(crate) const fn current_target() -> Self {
        if cfg!(target_os = "macos") {
            Self::MacOs
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else {
            Self::Linux
        }
    }
}
