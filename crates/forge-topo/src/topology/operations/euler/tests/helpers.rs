//! Shared test helpers for Euler operator tests.

use forge_core::{KernelError, log_result, OperationResult};

/// Log and unwrap an operation result for debugging.
pub fn logged_op<T>(label: &str, result: Result<OperationResult<T>, KernelError>) -> Result<T, KernelError> {
    match result {
        Ok(op_result) => {
            log_result(label, &op_result);
            Ok(op_result.into_value())
        }
        Err(e) => {
            eprintln!("[{}] ERROR: {:?}", label, e);
            Err(e)
        }
    }
}
