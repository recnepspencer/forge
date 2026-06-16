use super::WorthUiHeaderMenuCommand;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderMenuGroup {
    title: String,
    projection_id: String,
    commands: Vec<WorthUiHeaderMenuCommand>,
}

impl WorthUiHeaderMenuGroup {
    pub(crate) fn new(
        title: impl Into<String>,
        projection_id: impl Into<String>,
        commands: Vec<WorthUiHeaderMenuCommand>,
    ) -> Self {
        Self {
            title: title.into(),
            projection_id: projection_id.into(),
            commands,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn projection_id(&self) -> &str {
        &self.projection_id
    }

    pub fn commands(&self) -> &[WorthUiHeaderMenuCommand] {
        &self.commands
    }
}
