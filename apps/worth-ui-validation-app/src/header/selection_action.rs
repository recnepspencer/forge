#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationHeaderSelectionAction {
    projection_id: String,
    command_id: String,
}

impl ValidationHeaderSelectionAction {
    pub fn new(projection_id: impl Into<String>, command_id: impl Into<String>) -> Self {
        Self {
            projection_id: projection_id.into(),
            command_id: command_id.into(),
        }
    }

    pub fn projection_id(&self) -> &str {
        &self.projection_id
    }

    pub fn command_id(&self) -> &str {
        &self.command_id
    }
}
