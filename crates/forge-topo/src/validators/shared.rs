//! Shared utilities for validator implementations.
//!
//! DOMAIN: Common error construction used by all validator subdirectories.

use forge_core::KernelError;

/// Construct a `ValidatorFailure` error with the given validator name and detail.
///
/// All new validators should use this helper instead of constructing
/// `TopologyError::ValidatorFailure` directly.
pub(crate) fn vf(validator: &str, detail: String) -> KernelError {
    KernelError::TopologyViolation {
        err: forge_core::TopologyError::ValidatorFailure {
            validator: validator.to_string(),
            detail,
        },
        context: None,
    }
}
