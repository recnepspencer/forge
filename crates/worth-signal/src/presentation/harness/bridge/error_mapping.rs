//! Map boundary-level execution failures into harness-facing errors.

#[cfg(not(feature = "parallel"))]
use crate::data::error::SignalError;

#[cfg(not(feature = "parallel"))]
pub(super) fn staged_parallel_unavailable() -> SignalError {
    SignalError::invalid_input("staged-parallel execution requested without the `parallel` feature")
}

#[cfg(not(feature = "parallel"))]
pub(super) fn full_parallel_unavailable() -> SignalError {
    SignalError::invalid_input("full-parallel execution requested without the `parallel` feature")
}
