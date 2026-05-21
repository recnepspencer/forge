//! Conversion implementations for `KernelError`.

use crate::errors::data::{AmbiguousResult, KernelError};

impl From<worth_math::MathError> for KernelError {
    fn from(err: worth_math::MathError) -> Self {
        match err {
            worth_math::MathError::PrecisionEscalation {
                bit_length,
                threshold,
            } => KernelError::PrecisionEscalation {
                bit_length,
                threshold,
                context: None,
            },
            worth_math::MathError::InvalidInput(msg) => KernelError::InvalidInput {
                message: msg,
                context: None,
            },
            worth_math::MathError::NumericContractViolation { kind, context } => {
                KernelError::InvalidInput {
                    message: format!("numeric contract violation ({kind}): {context}"),
                    context: None,
                }
            }
            worth_math::MathError::InternalError(msg) => KernelError::InternalError {
                message: msg,
                context: None,
            },
            worth_math::MathError::Ambiguous {
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
