use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedTopologyError {
    message: String,
}

impl ProjectedTopologyError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for ProjectedTopologyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ProjectedTopologyError {}
