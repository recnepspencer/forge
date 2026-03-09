//! Domain-free error types for forge-signal.

use std::fmt;

/// Library-native error type for signal graph operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalError {
    /// Invalid caller input or graph contract violation.
    InvalidInput {
        message: String,
        context: Option<String>,
    },
    /// Internal invariant failure.
    Internal {
        message: String,
        context: Option<String>,
    },
}

impl SignalError {
    /// Build an invalid-input error with no extra context.
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
            context: None,
        }
    }

    /// Build an internal error with no extra context.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
            context: None,
        }
    }
}

impl fmt::Display for SignalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { message, .. } => write!(f, "invalid input: {message}"),
            Self::Internal { message, .. } => write!(f, "internal error: {message}"),
        }
    }
}

impl std::error::Error for SignalError {}
