/// Shortcut display posture for a command projection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommandProjectionShortcutVisibility {
    Hidden,
    VisibleWhenCommandHasShortcut,
}

impl CommandProjectionShortcutVisibility {
    pub fn digest_basis(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::VisibleWhenCommandHasShortcut => "visible_when_command_has_shortcut",
        }
    }
}
