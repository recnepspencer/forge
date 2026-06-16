use crate::capability::CommandProjectionSelectionMode;

use super::WorthUiHeaderMenuCommand;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderMenuGroup {
    title: String,
    projection_id: String,
    selection_mode: CommandProjectionSelectionMode,
    commands: Vec<WorthUiHeaderMenuCommand>,
}

impl WorthUiHeaderMenuGroup {
    pub(crate) fn new(
        title: impl Into<String>,
        projection_id: impl Into<String>,
        selection_mode: CommandProjectionSelectionMode,
        commands: Vec<WorthUiHeaderMenuCommand>,
    ) -> Self {
        Self {
            title: title.into(),
            projection_id: projection_id.into(),
            selection_mode,
            commands,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn projection_id(&self) -> &str {
        &self.projection_id
    }

    pub fn selection_mode(&self) -> CommandProjectionSelectionMode {
        self.selection_mode
    }

    pub fn commands(&self) -> &[WorthUiHeaderMenuCommand] {
        &self.commands
    }
}
