#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderMenuCommand {
    command_id: String,
    label: String,
    shortcut: Option<String>,
}

impl WorthUiHeaderMenuCommand {
    pub(crate) fn new(
        command_id: impl Into<String>,
        label: impl Into<String>,
        shortcut: Option<String>,
    ) -> Self {
        Self {
            command_id: command_id.into(),
            label: label.into(),
            shortcut,
        }
    }

    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn shortcut(&self) -> Option<&str> {
        self.shortcut.as_deref()
    }
}
