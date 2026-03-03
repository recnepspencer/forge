//! Conversion implementations for `KernelError`.

use crate::errors::data::{AmbiguousResult, KernelError};

impl From<forge_math::MathError> for KernelError {
    fn from(err: forge_math::MathError) -> Self {
        match err {
            forge_math::MathError::PrecisionEscalation {
                bit_length,
                threshold,
            } => KernelError::PrecisionEscalation {
                bit_length,
                threshold,
                context: None,
            },
            forge_math::MathError::InvalidInput(msg) => KernelError::InvalidInput {
                message: msg,
                context: None,
            },
            forge_math::MathError::InternalError(msg) => KernelError::InternalError {
                message: msg,
                context: None,
            },
            forge_math::MathError::Ambiguous {
                location,
                residual,
                context,
            } => KernelError::AmbiguousResult {
                result: AmbiguousResult {
                    location,
                    residual,
                    context,
                },
                context: None,
            },
        }
    }
}
