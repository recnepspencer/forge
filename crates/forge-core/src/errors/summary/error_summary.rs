//! Top-level error summary envelope and kernel error summary.

use serde::{Deserialize, Serialize};

use super::diagnostic_payload_summary::DiagnosticPayloadSummary;
use super::merge_error_summary::MergeErrorSummary;
use super::topology_error_summary::TopologyErrorSummary;
use crate::errors::data::{AmbiguousResult, ErrorContext, KernelError};

/// Broad category for a serialized error summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
    Kernel,
}

/// Optional non-kernel source error captured alongside a kernel error summary.
///
/// This is reserved for future source-chain enrichment in audit artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceErrorSummary {
    Opaque { type_name: String, message: String },
}

/// Top-level serializable error summary envelope.
///
/// `human_message` is a convenience field only. Consumers must use the typed
/// variant summaries (`kernel`) for critical logic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorSummary {
    pub category: ErrorCategory,
    pub kernel: Option<KernelErrorSummary>,
    #[serde(default)]
    pub source_chain: Vec<SourceErrorSummary>,
    pub human_message: Option<String>,
}

impl From<&KernelError> for ErrorSummary {
    fn from(value: &KernelError) -> Self {
        Self {
            category: ErrorCategory::Kernel,
            kernel: Some(KernelErrorSummary::from(value)),
            source_chain: Vec::new(),
            human_message: Some(value.to_string()),
        }
    }
}

/// Serializable typed summary of `KernelError`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KernelErrorSummary {
    TopologyViolation {
        err: TopologyErrorSummary,
        context: Option<ErrorContext>,
    },
    AmbiguousResult {
        result: AmbiguousResult,
        context: Option<ErrorContext>,
    },
    ToleranceExceeded {
        location: [f64; 3],
        margin: f64,
        message: String,
        context: Option<ErrorContext>,
    },
    PrecisionEscalation {
        bit_length: u32,
        threshold: u32,
        context: Option<ErrorContext>,
    },
    InvalidInput {
        message: String,
        context: Option<ErrorContext>,
    },
    InternalError {
        message: String,
        context: Option<ErrorContext>,
    },
    InvalidConfig {
        field: String,
        reason: String,
    },
    DiagnosticFailure {
        payload: DiagnosticPayloadSummary,
        source: Box<KernelErrorSummary>,
    },
    ReplayMismatch {
        expected: String,
        actual: String,
        context: Option<ErrorContext>,
    },
    MergeFailure(MergeErrorSummary),
}

impl From<&KernelError> for KernelErrorSummary {
    fn from(value: &KernelError) -> Self {
        match value {
            KernelError::TopologyViolation { err, context } => Self::TopologyViolation {
                err: TopologyErrorSummary::from(err),
                context: context.clone(),
            },
            KernelError::AmbiguousResult { result, context } => Self::AmbiguousResult {
                result: result.clone(),
                context: context.clone(),
            },
            KernelError::ToleranceExceeded {
                location,
                margin,
                message,
                context,
            } => Self::ToleranceExceeded {
                location: *location,
                margin: *margin,
                message: message.clone(),
                context: context.clone(),
            },
            KernelError::PrecisionEscalation {
                bit_length,
                threshold,
                context,
            } => Self::PrecisionEscalation {
                bit_length: *bit_length,
                threshold: *threshold,
                context: context.clone(),
            },
            KernelError::InvalidInput { message, context } => Self::InvalidInput {
                message: message.clone(),
                context: context.clone(),
            },
            KernelError::InternalError { message, context } => Self::InternalError {
                message: message.clone(),
                context: context.clone(),
            },
            KernelError::InvalidConfig { field, reason } => Self::InvalidConfig {
                field: field.clone(),
                reason: reason.clone(),
            },
            KernelError::DiagnosticFailure { payload, source } => Self::DiagnosticFailure {
                payload: DiagnosticPayloadSummary::from(payload),
                source: Box::new(KernelErrorSummary::from(source.as_ref())),
            },
            KernelError::ReplayMismatch {
                expected,
                actual,
                context,
            } => Self::ReplayMismatch {
                expected: expected.clone(),
                actual: actual.clone(),
                context: context.clone(),
            },
            KernelError::MergeFailure(err) => Self::MergeFailure(MergeErrorSummary::from(err)),
        }
    }
}
