//! Core error types: `KernelError`, `ErrorScope`, `ErrorContext`, `SuggestedFix`.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::policy::PolicyKind;

use super::ambiguous_result::AmbiguousResult;
use super::diagnostic_payload::DiagnosticPayload;
use super::merge_error::MergeError;
use super::topology_error::TopologyError;

// =========================================================================
// STRUCTURED ERROR CONTEXT
// =========================================================================

/// Identifies where an error originated in the kernel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorScope {
    /// Global kernel error not tied to a specific operation or entity.
    Global,
    /// Error occurred while processing a specific feature.
    Feature { feature_id: u64 },
    /// Error occurred on a specific topological entity.
    Entity { entity_kind: String, index: u32 },
    /// Error occurred during a specific Euler operation.
    Operation { op_name: String, invocation_id: u64 },
}

/// Machine-actionable remediation hints for an error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SuggestedFix {
    /// Increase a tolerance threshold.
    IncreaseThreshold {
        parameter: String,
        current: f64,
        suggested: f64,
    },
    /// Reduce a geometric parameter value.
    ReduceValue {
        parameter: String,
        current: f64,
        max_allowed: f64,
    },
    /// Re-run the operation with a different explicit policy.
    RetryWithPolicy { policy_kind: PolicyKind },
    /// Suggest splitting a complex operation into smaller steps.
    SplitOperation,
    /// No automatic fix available; requires manual triage.
    ManualIntervention { description: String },
}

impl fmt::Display for SuggestedFix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SuggestedFix::IncreaseThreshold {
                parameter,
                current,
                suggested,
            } => {
                write!(
                    f,
                    "Increase {} from {:.2e} to {:.2e}",
                    parameter, current, suggested
                )
            }
            SuggestedFix::ReduceValue {
                parameter,
                current,
                max_allowed,
            } => {
                write!(
                    f,
                    "Reduce {} from {:.2e} to at most {:.2e}",
                    parameter, current, max_allowed
                )
            }
            SuggestedFix::RetryWithPolicy { policy_kind } => {
                write!(f, "Retry with explicit policy: {:?}", policy_kind)
            }
            SuggestedFix::SplitOperation => {
                write!(f, "Split into smaller operations")
            }
            SuggestedFix::ManualIntervention { description } => {
                write!(f, "{}", description)
            }
        }
    }
}

/// Structured diagnostic context for AI agents and UI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Where the error happened.
    pub scope: ErrorScope,
    /// Automated remediation hints.
    pub suggested_fixes: Vec<SuggestedFix>,
    /// Human-readable detailed explanation.
    pub detail: String,
}

// =========================================================================
// KERNEL ERROR
// =========================================================================

/// The primary error type used across all Forge crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelError {
    /// A topology invariant was violated (e.g., non-manifold edge, broken loop).
    TopologyViolation {
        err: TopologyError,
        context: Option<ErrorContext>,
    },

    /// A geometric result is ambiguous and requires a policy decision.
    AmbiguousResult {
        result: AmbiguousResult,
        context: Option<ErrorContext>,
    },

    /// A tolerance threshold was exceeded during curved geometry operations.
    ToleranceExceeded {
        /// 3D location where the tolerance was exceeded
        location: [f64; 3],
        /// How close to the threshold (lower = more marginal)
        margin: f64,
        /// Description of the violation
        message: String,
        context: Option<ErrorContext>,
    },

    /// Exact arithmetic bit-length exceeded the budget.
    PrecisionEscalation {
        /// Current bit-length of the rational number
        bit_length: u32,
        /// Configured threshold
        threshold: u32,
        context: Option<ErrorContext>,
    },

    /// Invalid input provided to an operation.
    InvalidInput {
        message: String,
        context: Option<ErrorContext>,
    },

    /// Internal error — should never happen in correct code.
    InternalError {
        message: String,
        context: Option<ErrorContext>,
    },

    /// Invalid configuration parameter provided to a solver or policy.
    InvalidConfig { field: String, reason: String },

    /// A diagnostic failure wrapping another error with replay context.
    DiagnosticFailure {
        /// Structured context for replay and debugging.
        payload: DiagnosticPayload,
        /// The underlying error that triggered diagnostics.
        source: Box<KernelError>,
    },

    /// Cross-session replay architecture mismatch.
    ///
    /// The `ReplayLog` was recorded on a different target triple than
    /// the current process. FMA and other hardware differences may cause
    /// non-deterministic results.
    ReplayMismatch {
        /// The target triple the log was recorded on.
        expected: String,
        /// The target triple of the current process.
        actual: String,
        context: Option<ErrorContext>,
    },

    /// A region merge operation failed due to a topological or geometric policy violation.
    ///
    /// Carries a structured `MergeError` describing the exact failure mode.
    /// Never downgraded to `InternalError` — the typed variant is preserved through
    /// `with_phase` so callers can match on specific failure modes.
    MergeFailure(MergeError),
}
