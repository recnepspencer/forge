//! Performance and accounting metrics for an operation.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Performance and accounting metrics for an operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperationMetrics {
    /// Wall-clock duration of the operation.
    pub duration: Duration,
    /// Number of entities created during the operation.
    pub entities_created: u32,
    /// Number of entities deleted during the operation.
    pub entities_deleted: u32,
    /// Number of entities modified during the operation.
    pub entities_modified: u32,
    /// Number of exact predicate evaluations.
    pub exact_predicate_calls: u64,
    /// Number of policy-driven decisions made.
    pub policy_decisions_made: u32,
}

impl OperationMetrics {
    /// Accumulate another metrics record into this one.
    ///
    /// Adds all counters field-by-field. Used by `absorb_sub_result`
    /// and `OperationFinalizer` to roll up sub-operation metrics
    /// without manual per-field addition.
    pub fn accumulate(&mut self, other: &Self) {
        self.duration += other.duration;
        self.entities_created += other.entities_created;
        self.entities_deleted += other.entities_deleted;
        self.entities_modified += other.entities_modified;
        self.exact_predicate_calls += other.exact_predicate_calls;
        self.policy_decisions_made += other.policy_decisions_made;
    }
}
