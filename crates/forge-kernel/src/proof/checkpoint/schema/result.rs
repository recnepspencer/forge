use serde::{Deserialize, Serialize};

use super::ValidationCheckpoint;

/// Result of a checkpoint validation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Which checkpoint triggered this run.
    checkpoint: ValidationCheckpoint,
    /// Whether validation passed.
    passed: bool,
    /// Error description if validation failed.
    error_detail: Option<String>,
    /// Number of entities at time of validation.
    entity_count: usize,
    /// Whether validation was skipped due to entity limit.
    skipped: bool,
    /// Whether geometric checks were included in this run.
    included_geometric: bool,
    /// Duration of the validation in microseconds.
    duration_micros: u64,
}

impl ValidationResult {
    /// Create a passing result.
    pub fn passed(
        checkpoint: ValidationCheckpoint,
        entity_count: usize,
        included_geometric: bool,
        duration_micros: u64,
    ) -> Self {
        Self {
            checkpoint,
            passed: true,
            error_detail: None,
            entity_count,
            skipped: false,
            included_geometric,
            duration_micros,
        }
    }

    /// Create a failing result.
    pub fn failed(
        checkpoint: ValidationCheckpoint,
        entity_count: usize,
        detail: String,
        included_geometric: bool,
        duration_micros: u64,
    ) -> Self {
        Self {
            checkpoint,
            passed: false,
            error_detail: Some(detail),
            entity_count,
            skipped: false,
            included_geometric,
            duration_micros,
        }
    }

    /// Create a skipped result (entity limit exceeded).
    pub fn skipped(checkpoint: ValidationCheckpoint, entity_count: usize) -> Self {
        Self {
            checkpoint,
            passed: true,
            error_detail: None,
            entity_count,
            skipped: true,
            included_geometric: false,
            duration_micros: 0,
        }
    }

    /// Whether validation passed.
    pub fn is_passed(&self) -> bool {
        self.passed
    }

    /// Whether validation was skipped.
    pub fn is_skipped(&self) -> bool {
        self.skipped
    }

    /// The error detail, if any.
    pub fn error_detail(&self) -> Option<&str> {
        self.error_detail.as_deref()
    }

    /// The checkpoint that triggered this validation.
    pub fn checkpoint(&self) -> ValidationCheckpoint {
        self.checkpoint
    }

    /// Entity count at time of validation.
    pub fn entity_count(&self) -> usize {
        self.entity_count
    }

    /// Whether geometric checks were included.
    pub fn included_geometric(&self) -> bool {
        self.included_geometric
    }

    /// Duration of the validation in microseconds.
    pub fn duration_micros(&self) -> u64 {
        self.duration_micros
    }
}
