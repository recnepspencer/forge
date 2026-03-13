use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectVersion;
use crate::data::dependency::SharedDependencySnapshot;
use crate::data::handle::NodeId;
use crate::data::output::OutputChange;

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
pub struct OperationalEffect {
    pub node: NodeId,
    pub verdict: EvaluationVerdict,
    pub aspect_version: AspectVersion,
    pub output_change: OutputChange,
    pub dependency_snapshot: SharedDependencySnapshot,
    pub meaningful_input_changes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EffectDependencyInputs {
    pub dependency_snapshot: SharedDependencySnapshot,
    pub meaningful_input_changes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PendingDependencySnapshot {
    pub node: NodeId,
    pub snapshot: SharedDependencySnapshot,
}
