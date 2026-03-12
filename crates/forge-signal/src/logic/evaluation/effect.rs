use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectVersion;
use crate::data::dependency::DependencySnapshot;
use crate::data::handle::NodeId;
use crate::data::output::{
    ArtifactContinuityToken, ChangedRegion, MemoizedResultOrigin, OutputChange, OutputIdentity,
};
use crate::data::trace::CausalityMetadata;
use crate::logic::prepared::PreparedKeyedContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuppressionReason {
    OutputIdentityUnchanged,
    ContinuityTokenUnchanged,
    ComparatorMatch,
    ValidatedClean,
    ConditionRevertedClean,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeferralReason {
    ConditionNotMet,
    OnDemandNotRequested,
    DebounceWindow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvaluationVerdict {
    Recomputed,
    Suppressed { reason: SuppressionReason },
    Deferred { reason: DeferralReason },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationEffect {
    pub node: NodeId,
    pub verdict: EvaluationVerdict,
    pub aspect_version: AspectVersion,
    pub output_change: OutputChange,
    pub output_identity: Option<OutputIdentity>,
    pub continuity_token: Option<ArtifactContinuityToken>,
    pub changed_regions: Vec<ChangedRegion>,
    pub labels: Vec<String>,
    pub dependency_snapshot: DependencySnapshot,
    pub meaningful_input_changes: u32,
    pub recomputed: bool,
    pub memoized_origin: MemoizedResultOrigin,
    pub keyed_context: Option<PreparedKeyedContext>,
    pub causality: Option<CausalityMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectComparison {
    pub output_identity_unchanged: bool,
    pub continuity_token_unchanged: bool,
    pub propagation_suppressed: bool,
    pub output_change: OutputChange,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EffectDependencyInputs {
    pub dependency_snapshot: DependencySnapshot,
    pub meaningful_input_changes: u32,
}
