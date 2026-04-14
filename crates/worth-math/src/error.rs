//! Structured error taxonomy for the Worth math crate.
//!
//! This crate provides mathematical error types used by `worth-math` and
//! `worth-geom`. It reports:
//! - Invalid input (e.g., NaN, division by zero)
//! - Precision escalation (rational number too large)
//! - Geometric ambiguity (results near tolerance boundaries)

use std::fmt;

/// Errors that can occur during mathematical and geometric operations.
#[derive(Debug, Clone)]
pub enum MathError {
    /// Exact arithmetic bit-length exceeded the budget.
    PrecisionEscalation {
        /// Current bit-length of the rational number.
        bit_length: u32,
        /// Configured threshold.
        threshold: u32,
    },

    /// Invalid input provided to an operation.
    InvalidInput(String),

    /// Internal error — should never happen in correct code.
    InternalError(String),

    /// A geometric result is ambiguous and requires a policy decision.
    ///
    /// Returned by geometry solvers when a result is near a tolerance
    /// boundary. The kernel layer catches this and applies policy.
    Ambiguous {
        /// 3D location where the ambiguity occurred.
        location: [f64; 3],
        /// Geometric metric of ambiguity (e.g., residual, distance).
        residual: f64,
        /// Human-readable context describing the ambiguity.
        context: String,
    },
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
            MathError::Ambiguous {
                location,
                residual,
                context,
            } => {
                write!(
                    f,
                    "Ambiguous result at [{:.6}, {:.6}, {:.6}] (residual: {:.2e}): {}",
                    location[0], location[1], location[2], residual, context
                )
            }
        }
    }
}

impl std::error::Error for MathError {}
