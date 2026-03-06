//! Canonical, immutable configuration consumed by feature execution lifecycle.

use crate::engine::facade::AuditLevel;
use crate::proof::checkpoint::schema::ValidationConfig;

/// Per-feature execution policy resolved before pipeline evaluation starts.
#[derive(Debug, Clone)]
pub struct FeatureExecutionConfig {
    /// Feature kind used for traces, audit summaries, and diagnostics.
    pub feature_kind: &'static str,
    /// Audit emission policy for this feature execution.
    pub audit_level: AuditLevel,
    /// Post-invariant validation configuration snapshot.
    pub validation: ValidationConfig,
}
