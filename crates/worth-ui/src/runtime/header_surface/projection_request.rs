use crate::capability::CommandProjectionId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderMenuProjectionRequest {
    title: String,
    projection_id: CommandProjectionId,
}

impl WorthUiHeaderMenuProjectionRequest {
    pub fn new(title: impl Into<String>, projection_id: CommandProjectionId) -> Self {
        Self {
            title: title.into(),
            projection_id,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn projection_id(&self) -> &CommandProjectionId {
        &self.projection_id
    }
}
