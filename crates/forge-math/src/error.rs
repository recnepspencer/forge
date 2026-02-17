//! Structured error taxonomy for the Forge math crate.
//!
//! This crate is PURE math. It does NOT know about topology, modeling errors, or ambiguity.
//! It only reports mathematical failures:
//! - Invalid input (e.g., NaN, division by zero)
//! - Precision escalation (rational number too large)

use std::fmt;

/// Errors that can occur during pure mathematical operations.
#[derive(Debug, Clone)]
pub enum MathError {
    /// Exact arithmetic bit-length exceeded the budget (Milestone 0.2.3).
    PrecisionEscalation {
        /// Current bit-length of the rational number
        bit_length: u32,
        /// Configured threshold
        threshold: u32,
    },

    /// Invalid input provided to an operation.
    InvalidInput(String),

    /// Internal error — should never happen in correct code.
    InternalError(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::PrecisionEscalation {
                bit_length,
                threshold,
            } => {
                write!(
                    f,
                    "Precision escalation: {} bits exceeds {} bit threshold",
                    bit_length, threshold
                )
            }
            MathError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            MathError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for MathError {}
