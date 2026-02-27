//! Typed error cases for the pipeline infrastructure.
//!
//! DOMAIN: Pipeline-specific errors that wrap into `KernelError` with
//! `ErrorContext` pointing to the pipeline stage where the failure occurred.

use std::fmt;

use forge_core::KernelError;
use forge_core::PolicyKind;
use forge_core::errors::{ErrorContext, ErrorScope};

use crate::engine::contract::InvariantKind;

/// Pipeline-specific error cases.
///
/// Each variant maps to a specific pipeline failure mode. The `From`
/// impl wraps these into `KernelError` with stage-specific context.
#[derive(Debug)]
pub enum PipelineError {
    /// A required policy is not configured in the ModelingContext.
    PolicyNotConfigured {
        kind: PolicyKind,
        feature: String,
    },

    /// A post-execution invariant was violated.
    InvariantViolation {
        kind: InvariantKind,
        detail: String,
    },

    /// Input parsing failed (missing dependency, wrong type).
    InputParseFailure {
        expected: String,
        actual: String,
    },

    /// Input semantic validation failed.
    InputValidationFailure {
        message: String,
    },

    /// A step within the operation failed.
    StepExecutionFailed {
        step: String,
        source: Box<KernelError>,
    },
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PolicyNotConfigured { kind, feature } => {
                write!(f, "Policy {:?} not configured for feature '{}'", kind, feature)
            }
            Self::InvariantViolation { kind, detail } => {
                write!(f, "Invariant {:?} violated: {}", kind, detail)
            }
            Self::InputParseFailure { expected, actual } => {
                write!(f, "Input parse failure: expected '{}', got '{}'", expected, actual)
            }
            Self::InputValidationFailure { message } => {
                write!(f, "Input validation failed: {}", message)
            }
            Self::StepExecutionFailed { step, source } => {
                write!(f, "Step '{}' failed: {}", step, source)
            }
        }
    }
}

impl From<PipelineError> for KernelError {
    fn from(err: PipelineError) -> Self {
        let detail = err.to_string();

        KernelError::InternalError {
            message: detail.clone(),
            context: Some(ErrorContext {
                scope: ErrorScope::Global,
                suggested_fixes: Vec::new(),
                detail,
            }),
        }
    }
}
