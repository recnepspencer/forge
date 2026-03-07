use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecError {
    InvalidInput { message: String },
    NotFound { message: String },
    ValidationFailed { message: String },
    TransactionFinished,
}

impl SpecError {
    pub fn invalid(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
        }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationFailed {
            message: message.into(),
        }
    }
}

impl Display for SpecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput { message } => write!(f, "invalid input: {message}"),
            Self::NotFound { message } => write!(f, "not found: {message}"),
            Self::ValidationFailed { message } => write!(f, "validation failed: {message}"),
            Self::TransactionFinished => write!(f, "spec draft is already finished"),
        }
    }
}

impl std::error::Error for SpecError {}
