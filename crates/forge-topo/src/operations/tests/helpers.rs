//! Shared test helpers for Euler operator tests.

use forge_core::{KernelError, OperationResult};

/// Unwrap an operation result for debugging.
pub fn logged_op<T>(
    label: &str,
    result: Result<OperationResult<T>, KernelError>,
) -> Result<T, KernelError> {
    match result {
        Ok(op_result) => {
            forge_core::log_result(label, &op_result);
            Ok(op_result.into_value())
        }
        Err(e) => {
            forge_core::log_error(label, &e);
            Err(e)
        }
    }
}
