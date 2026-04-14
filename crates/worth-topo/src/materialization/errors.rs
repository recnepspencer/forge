#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthTopologyMaterializationError {
    message: String,
}

impl WorthTopologyMaterializationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for WorthTopologyMaterializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for WorthTopologyMaterializationError {}
