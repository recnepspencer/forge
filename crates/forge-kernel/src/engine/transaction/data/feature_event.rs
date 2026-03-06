//! Typed lifecycle events for feature-pipeline transaction orchestration.

use crate::engine::facade::AuditLevel;

/// Typed identity for one feature pipeline invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeatureInvocationId(u64);

impl FeatureInvocationId {
    /// Create a typed feature invocation ID.
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Raw numeric value for diagnostics.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Typed operation event stream for one feature pipeline execution.
///
/// NOTE: payload identifiers must remain strongly typed.
#[derive(Debug, Clone)]
pub enum KernelFeatureEvent {
    OperationStarted {
        feature_kind: &'static str,
        invocation_id: FeatureInvocationId,
        audit_level: AuditLevel,
        state_hash_before: u128,
    },
    OperationCompleted {
        invocation_id: FeatureInvocationId,
        duration_micros: u64,
        state_hash_after: u128,
    },
    OperationFailed {
        invocation_id: FeatureInvocationId,
        error_summary: String,
    },
}
