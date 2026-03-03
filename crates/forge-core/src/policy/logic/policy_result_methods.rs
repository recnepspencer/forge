//! Method implementations for `PolicyResult<T>`.

use crate::errors::{AmbiguousResult, KernelError};
use crate::policy::data::PolicyResult;

impl<T> PolicyResult<T> {
    /// Returns `true` if the result is `Success`.
    pub fn is_success(&self) -> bool {
        matches!(self, PolicyResult::Success(_))
    }

    /// Returns `true` if the result requires a policy decision.
    pub fn is_ambiguous(&self) -> bool {
        matches!(self, PolicyResult::Ambiguous { .. })
    }

    /// Returns `true` if the result is a hard error.
    pub fn is_hard_error(&self) -> bool {
        matches!(self, PolicyResult::HardError(_))
    }

    /// Convert to a standard `Result`, treating ambiguity as an error.
    ///
    /// Use this when the caller cannot handle ambiguity and wants to
    /// escalate it as a `KernelError::AmbiguousResult`.
    pub fn into_result_strict(self) -> Result<T, KernelError> {
        match self {
            PolicyResult::Success(v) => Ok(v),
            PolicyResult::Ambiguous { query, .. } => Err(KernelError::AmbiguousResult {
                result: AmbiguousResult {
                    location: query.location,
                    residual: query.margin,
                    context: format!("Policy decision required: {:?}", query.kind),
                },
                context: None,
            }),
            PolicyResult::HardError(e) => Err(e),
        }
    }

    /// Convert to a standard `Result`, accepting the potential value on ambiguity.
    ///
    /// Use this when the caller trusts the solver's best guess.
    pub fn into_result_accepting(self) -> Result<T, KernelError> {
        match self {
            PolicyResult::Success(v) => Ok(v),
            PolicyResult::Ambiguous {
                potential_value, ..
            } => Ok(potential_value),
            PolicyResult::HardError(e) => Err(e),
        }
    }
}

impl<T> From<T> for PolicyResult<T> {
    fn from(value: T) -> Self {
        PolicyResult::Success(value)
    }
}
