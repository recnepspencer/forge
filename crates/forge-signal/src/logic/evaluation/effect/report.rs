use serde::{Deserialize, Serialize};

use super::EvaluationVerdict;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectComparison {
    pub output_identity_unchanged: bool,
    pub continuity_token_unchanged: bool,
    pub propagation_suppressed: bool,
    pub output_change: crate::data::output::OutputChange,
    pub changed_partition_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppliedEffectReport {
    pub verdict: EvaluationVerdict,
    pub comparison: EffectComparison,
    pub suppressed_downstream: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedApplyResult {
    pub dependency_updates: u32,
    pub report: AppliedEffectReport,
    pub pending_snapshot: Option<super::PendingDependencySnapshot>,
}
