use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::data::core_profile::CORE_STORAGE_PROFILE_ID;
use crate::data::graph::SignalGraph;
use crate::data::telemetry::RuntimeTelemetry;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::lineage::LineageRecord;
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::replay::{ReplayCursor, ReplayFrame};
use crate::diagnostics::summary::ExecutionHistorySummary;
use crate::diagnostics::{FailureSummary, RollbackDiagnostic};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
/// Stable identifier for a captured runtime-evaluation snapshot.
pub struct SignalSnapshotId(pub u64);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
/// Stable identifier for a branch-local evaluation timeline.
pub struct SignalBranchId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Public branch handle for snapshot, restore, and replay inspection APIs.
///
/// Branches are runtime-local evaluation timelines. They model branch-local
/// graph state, replay history, and lineage ancestry; they are not durable
/// relational history stores.
pub struct SignalBranchHandle {
    pub id: SignalBranchId,
    pub name: String,
    pub parent_branch_id: Option<SignalBranchId>,
    pub head_snapshot_id: Option<SignalSnapshotId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Versioned metadata for a serialized `SignalSnapshotV1`.
///
/// The snapshot body captures evaluation state, not host-managed source truth.
/// Use this metadata to inspect compatibility, active branch identity, replay
/// cursor position, and runtime policy without restoring the snapshot.
pub struct SignalSnapshotMeta {
    pub schema_version: u32,
    pub snapshot_id: SignalSnapshotId,
    pub branch_id: SignalBranchId,
    pub branch_name: String,
    pub core_storage_profile: String,
    pub replay_head: Option<ReplayCursor>,
    pub runtime_policy: SignalRuntimePolicy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Snapshot-resident diagnostics payload needed for deterministic restore,
/// replay inspection, and lineage continuity.
pub struct SignalSnapshotDiagnostics {
    pub latest_flow: Option<FlowSummary>,
    pub latest_failure: Option<FailureSummary>,
    pub latest_rollback: Option<RollbackDiagnostic>,
    pub recent_history: VecDeque<ExecutionHistorySummary>,
    pub replay_frames: VecDeque<ReplayFrame>,
    pub explanation_facts: BTreeMap<crate::data::handle::NodeId, ExplanationFact>,
    pub provenance_facts: BTreeMap<crate::data::handle::NodeId, ProvenanceFact>,
    pub lineage_records: VecDeque<LineageRecord>,
    pub branch_catalog: BTreeMap<SignalBranchId, SignalBranchHandle>,
    pub active_branch: SignalBranchId,
    pub next_replay_cursor: u64,
    pub next_snapshot_id: u64,
    pub next_branch_id: u64,
    pub next_lineage_artifact_id: u64,
    pub next_lineage_sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Versioned snapshot of `forge-signal` evaluation state.
///
/// This captures graph-local evaluation state, runtime diagnostics required for
/// deterministic replay/restore, branch metadata, and lineage/replay history.
/// It intentionally does not claim ownership of host-managed source truth.
pub struct SignalSnapshotV1 {
    pub meta: SignalSnapshotMeta,
    pub graph: SignalGraph,
    pub diagnostics: SignalSnapshotDiagnostics,
    pub graph_telemetry: RuntimeTelemetry,
    pub runtime_telemetry: Option<RuntimeTelemetry>,
}

impl SignalSnapshotMeta {
    pub const SCHEMA_VERSION: u32 = 1;

    pub fn new(
        snapshot_id: SignalSnapshotId,
        branch: &SignalBranchHandle,
        replay_head: Option<ReplayCursor>,
        runtime_policy: SignalRuntimePolicy,
    ) -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            snapshot_id,
            branch_id: branch.id,
            branch_name: branch.name.clone(),
            core_storage_profile: CORE_STORAGE_PROFILE_ID.to_string(),
            replay_head,
            runtime_policy,
        }
    }
}

impl SignalSnapshotV1 {
    /// Inspect snapshot metadata without restoring the snapshot.
    pub fn meta(&self) -> &SignalSnapshotMeta {
        &self.meta
    }

    /// Branch identity that owned the snapshot head when this snapshot was captured.
    pub fn branch_id(&self) -> SignalBranchId {
        self.meta.branch_id
    }

    /// Stable snapshot identifier for replay and lineage references.
    pub fn snapshot_id(&self) -> SignalSnapshotId {
        self.meta.snapshot_id
    }
}
