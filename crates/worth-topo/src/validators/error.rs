#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyValidationError {
    validator: &'static str,
    message: String,
}

impl TopologyValidationError {
    pub fn new(validator: &'static str, message: impl Into<String>) -> Self {
        Self {
            validator,
            message: message.into(),
        }
    }

    pub fn validator(&self) -> &'static str {
        self.validator
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for TopologyValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.validator, self.message)
    }
}

impl std::error::Error for TopologyValidationError {}
