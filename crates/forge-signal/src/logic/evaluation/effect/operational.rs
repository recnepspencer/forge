use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectVersion;
use crate::data::dependency::{
    DependencySnapshotId, DependencySnapshotUpdate, SnapshotDeltaRecord,
};
use crate::data::graph::DependencySetId;
use crate::data::handle::NodeId;
use crate::data::output::OutputChange;
use crate::data::reuse::{ReuseBasis, ReuseBoundaryContext};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DependencyInputContext {
    pub dependency_set_id: DependencySetId,
    pub dependency_snapshot_id: DependencySnapshotId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalEffect {
    pub node: NodeId,
    pub verdict: EvaluationVerdict,
    pub aspect_version: AspectVersion,
    pub output_change: OutputChange,
    pub reuse_basis: ReuseBasis,
    pub reuse_boundary_context: ReuseBoundaryContext,
    pub dependency_snapshot_update: DependencySnapshotUpdate,
    pub snapshot_delta: SnapshotDeltaRecord,
    pub meaningful_input_changes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EffectDependencyInputs {
    pub context: DependencyInputContext,
    pub dependency_snapshot_update: DependencySnapshotUpdate,
    pub snapshot_delta: SnapshotDeltaRecord,
    pub meaningful_input_changes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PendingDependencySnapshot {
    pub node: NodeId,
    pub update: DependencySnapshotUpdate,
    pub delta: SnapshotDeltaRecord,
}
