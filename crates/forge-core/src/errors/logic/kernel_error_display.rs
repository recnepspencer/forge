//! Display and method implementations for `KernelError`.

use std::fmt;

use crate::errors::data::{ErrorContext, ErrorScope, KernelError, MergeError};

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::TopologyViolation { err, .. } => write!(f, "Topology violation: {}", err),
            KernelError::AmbiguousResult { result, .. } => {
                write!(
                    f,
                    "Ambiguous result at [{:.6}, {:.6}, {:.6}]: {}",
                    result.location[0], result.location[1], result.location[2], result.context
                )
            }
            KernelError::ToleranceExceeded {
                location,
                margin,
                message,
                ..
            } => {
                write!(
                    f,
                    "Tolerance exceeded at [{:.6}, {:.6}, {:.6}] (margin: {:.2e}): {}",
                    location[0], location[1], location[2], margin, message
                )
            }
            KernelError::PrecisionEscalation {
                bit_length,
                threshold,
                ..
            } => {
                write!(
                    f,
                    "Precision escalation: {} bits exceeds {} bit threshold",
                    bit_length, threshold
                )
            }
            KernelError::InvalidInput { message, .. } => write!(f, "Invalid input: {}", message),
            KernelError::InternalError { message, .. } => write!(f, "Internal error: {}", message),
            KernelError::InvalidConfig { field, reason } => {
                write!(f, "Invalid configuration for '{}': {}", field, reason)
            }
            KernelError::DiagnosticFailure { payload, source } => {
                write!(
                    f,
                    "Diagnostic failure in '{}' (hash: {:#x}, seed: {}): {}",
                    payload.operation, payload.state_hash, payload.seed, source
                )
            }
            KernelError::ReplayMismatch {
                expected, actual, ..
            } => {
                write!(
                    f,
                    "Replay architecture mismatch: log recorded on '{}', current process is '{}'",
                    expected, actual
                )
            }
            KernelError::MergeFailure(err) => write!(f, "Merge failure: {}", err),
        }
    }
}

impl KernelError {
    /// Extract the structured error context, if any variant carries one.
    pub fn get_context(&self) -> Option<&ErrorContext> {
        match self {
            KernelError::TopologyViolation { context, .. }
            | KernelError::AmbiguousResult { context, .. }
            | KernelError::ToleranceExceeded { context, .. }
            | KernelError::PrecisionEscalation { context, .. }
            | KernelError::InvalidInput { context, .. }
            | KernelError::InternalError { context, .. }
            | KernelError::ReplayMismatch { context, .. } => context.as_ref(),
            KernelError::InvalidConfig { .. } => None,
            KernelError::DiagnosticFailure { source, .. } => source.get_context(),
            // MergeError fields are self-describing; no separate ErrorContext attached.
            KernelError::MergeFailure(_) => None,
        }
    }

    /// Wrap this error with a phase label, prefixing its detail string.
    ///
    /// For `MergeFailure`, the typed `MergeError` structure is preserved.
    /// `PartialMergePlanRejected` has its `reason` prefixed with the phase label.
    /// Other `MergeError` variants are re-wrapped with an outer `PartialMergePlanRejected`
    /// carrying the phase context. This ensures typed semantics are never lost.
    pub fn with_phase(mut self, phase: &str) -> Self {
        match &mut self {
            KernelError::TopologyViolation { context, .. }
            | KernelError::AmbiguousResult { context, .. }
            | KernelError::ToleranceExceeded { context, .. }
            | KernelError::PrecisionEscalation { context, .. }
            | KernelError::InvalidInput { context, .. }
            | KernelError::InternalError { context, .. }
            | KernelError::ReplayMismatch { context, .. } => {
                let ctx = context.get_or_insert_with(|| ErrorContext {
                    scope: ErrorScope::Global,
                    suggested_fixes: Vec::new(),
                    detail: String::new(),
                });
                if ctx.detail.is_empty() {
                    ctx.detail = format!("Failed during phase '{}'", phase);
                } else {
                    ctx.detail = format!("[{}] {}", phase, ctx.detail);
                }
            }
            KernelError::InvalidConfig { .. } => {
                // InvalidConfig has no ErrorContext to attach a phase to.
            }
            KernelError::DiagnosticFailure { source, .. } => {
                let new_source = (**source).clone();
                *source = Box::new(new_source.with_phase(phase));
            }
            // Preserve typed MergeError structure exactly.
            // Only PartialMergePlanRejected gets a phase prefix on its reason field —
            // it is the only general-purpose carrier. All other MergeError variants
            // already carry fully structured fields (edge_index, valence, witness, etc.)
            // that are more informative than a phase string. Do not convert them.
            KernelError::MergeFailure(merge_err) => {
                if let MergeError::PartialMergePlanRejected { reason, .. } = merge_err {
                    *reason = format!("[{}] {}", phase, reason);
                }
                // All other MergeError variants: left completely untouched.
            }
        }
        self
    }

    /// Fill in `ErrorContext` only when the error has `context: None`.
    ///
    /// Designed to be called by operation runners (e.g. `MutableDraft::execute`)
    /// so that operators don't need to manually stamp operation scope on every
    /// error site. Operators that provide their own context are left untouched.
    ///
    /// `op_debug` should be the `Debug` representation of the operator struct,
    /// which naturally contains all input entity IDs.
    pub fn ensure_operation_context(
        mut self,
        op_name: &str,
        invocation_id: u64,
        op_debug: &str,
    ) -> Self {
        match &mut self {
            KernelError::TopologyViolation { context, .. }
            | KernelError::AmbiguousResult { context, .. }
            | KernelError::ToleranceExceeded { context, .. }
            | KernelError::PrecisionEscalation { context, .. }
            | KernelError::InvalidInput { context, .. }
            | KernelError::InternalError { context, .. }
            | KernelError::ReplayMismatch { context, .. } => {
                if context.is_none() {
                    *context = Some(ErrorContext {
                        scope: ErrorScope::Operation {
                            op_name: op_name.to_string(),
                            invocation_id,
                        },
                        suggested_fixes: vec![],
                        detail: op_debug.to_string(),
                    });
                }
            }
            // These variants have no ErrorContext field or are self-describing.
            KernelError::InvalidConfig { .. }
            | KernelError::DiagnosticFailure { .. }
            | KernelError::MergeFailure(_) => {}
        }
        self
    }
}

impl std::error::Error for KernelError {}
