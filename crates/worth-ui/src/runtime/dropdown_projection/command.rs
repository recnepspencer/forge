#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDropdownCommand {
    command_id: String,
    label: String,
    icon_id: Option<String>,
    shortcut: Option<String>,
}

impl WorthUiDropdownCommand {
    pub(crate) fn new(
        command_id: impl Into<String>,
        label: impl Into<String>,
        icon_id: Option<String>,
        shortcut: Option<String>,
    ) -> Self {
        Self {
            command_id: command_id.into(),
            label: label.into(),
            icon_id,
            shortcut,
        }
    }

    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn icon_id(&self) -> Option<&str> {
        self.icon_id.as_deref()
    }

    pub fn shortcut(&self) -> Option<&str> {
        self.shortcut.as_deref()
    }
}
