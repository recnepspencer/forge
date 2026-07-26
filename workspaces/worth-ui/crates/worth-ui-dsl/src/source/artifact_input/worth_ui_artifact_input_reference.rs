#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorthUiArtifactInputReference {
    authored_text: String,
}

impl WorthUiArtifactInputReference {
    pub(crate) fn new(authored_text: impl Into<String>) -> Self {
        Self {
            authored_text: authored_text.into(),
        }
    }

    pub fn authored_text(&self) -> &str {
        &self.authored_text
    }
}
