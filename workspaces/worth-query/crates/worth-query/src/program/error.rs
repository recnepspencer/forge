#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProgramError {
    message: String,
}

impl WorthQueryProgramError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for WorthQueryProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for WorthQueryProgramError {}
