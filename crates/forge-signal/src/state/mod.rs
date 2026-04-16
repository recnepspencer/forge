use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Deserializer, Serialize};

use crate::data::core_profile::CORE_STORAGE_PROFILE_ID;
use crate::data::graph::SignalGraph;
use crate::data::graph::{DependencyEdgeStore, SubscriberEdgeStore};
use crate::data::node::{CheckpointNodeImage, NodeEntry};
use crate::data::proof::{ClassifiedSnapshotBatchCommit, SnapshotBatchCommit};
use crate::data::telemetry::RuntimeTelemetry;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::lineage::LineageRecord;
use crate::diagnostics::policy::{ArtifactRetentionPolicy, SignalRuntimePolicy};
use crate::diagnostics::replay::{ReplayCursor, ReplayFrame};
use crate::diagnostics::state::DiagnosticsState;
use crate::diagnostics::summary::ExecutionHistorySummary;
use crate::diagnostics::{FailureSummary, RollbackDiagnostic};
use crate::logic::transaction::{
    ObservationBoundarySummary, ReconstructabilityProof, ReconstructabilityRecord,
};

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
    #[serde(default)]
    pub artifact_retention: SnapshotArtifactRetentionPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Explicit snapshot-time retention contract for cold explanation/provenance richness.
///
/// This is a richness policy only. Snapshot restore, identity, and replay truth
/// remain defined by the runtime and dependency contracts, not by retained
/// explanation/provenance payload presence.
pub struct SnapshotArtifactRetentionPolicy {
    pub explanation_retention: ArtifactRetentionPolicy,
    pub provenance_retention: ArtifactRetentionPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Explicit operational intent for snapshot restore.
pub struct SnapshotRestoreIntent {
    pub state: SnapshotStateRestoreMode,
    pub artifacts: SnapshotArtifactRestoreMode,
    pub dependency_state: SnapshotDependencyRestoreMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Whether restore rewinds operational runtime authority to captured state.
pub enum SnapshotStateRestoreMode {
    RewindActiveState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// How cold explanation/provenance richness should be handled during restore.
pub enum SnapshotArtifactRestoreMode {
    RestoreCapturedRetention,
    ApplyActiveRuntimePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Whether dependency state is restored as captured or used only as a recomputation seed.
pub enum SnapshotDependencyRestoreMode {
    RestoreCapturedState,
    SeedRecomputationFromSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Reason a restore currently still requires a coarse graph/state replacement boundary.
pub enum SnapshotRestoreCoarseReason {
    EntryStateRewind,
    NodeSetDifference,
    DiagnosticsHistoryRestore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CheckpointRestoreSnapshotBatch {
    classified: ClassifiedSnapshotBatchCommit,
}

impl CheckpointRestoreSnapshotBatch {
    pub(crate) fn new(classified: ClassifiedSnapshotBatchCommit) -> Self {
        Self { classified }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn classified(&self) -> &ClassifiedSnapshotBatchCommit {
        &self.classified
    }

    pub(crate) fn clone_inner(&self) -> ClassifiedSnapshotBatchCommit {
        self.classified.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreDeltaAccounting {
    dependency_snapshot_delta_node_count: u64,
}

impl RestoreDeltaAccounting {
    pub(crate) fn new(dependency_snapshot_delta_node_count: u64) -> Self {
        Self {
            dependency_snapshot_delta_node_count,
        }
    }

    pub(crate) fn dependency_snapshot_delta_node_count(self) -> u64 {
        self.dependency_snapshot_delta_node_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Proof-bearing restore plan describing how much of a restore can be lowered
/// as shared-node delta work versus still requiring a coarse replacement boundary.
pub struct SnapshotRestorePlan {
    intent: SnapshotRestoreIntent,
    shared_node_count: u64,
    current_only_node_count: u64,
    snapshot_only_node_count: u64,
    checkpoint_restore_batch: CheckpointRestoreSnapshotBatch,
    delta_accounting: RestoreDeltaAccounting,
    coarse_replacement_required: bool,
    coarse_reasons: Vec<SnapshotRestoreCoarseReason>,
}

impl SnapshotRestorePlan {
    pub(crate) fn new(
        intent: SnapshotRestoreIntent,
        shared_node_count: u64,
        current_only_node_count: u64,
        snapshot_only_node_count: u64,
        checkpoint_restore_batch: CheckpointRestoreSnapshotBatch,
        delta_accounting: RestoreDeltaAccounting,
        coarse_replacement_required: bool,
        coarse_reasons: Vec<SnapshotRestoreCoarseReason>,
    ) -> Self {
        Self {
            intent,
            shared_node_count,
            current_only_node_count,
            snapshot_only_node_count,
            checkpoint_restore_batch,
            delta_accounting,
            coarse_replacement_required,
            coarse_reasons,
        }
    }

    pub fn checkpoint_restore_batch(&self) -> &CheckpointRestoreSnapshotBatch {
        &self.checkpoint_restore_batch
    }

    pub fn intent(&self) -> SnapshotRestoreIntent {
        self.intent
    }

    pub fn shared_node_count(&self) -> u64 {
        self.shared_node_count
    }

    pub fn current_only_node_count(&self) -> u64 {
        self.current_only_node_count
    }

    pub fn snapshot_only_node_count(&self) -> u64 {
        self.snapshot_only_node_count
    }

    pub fn dependency_snapshot_delta_node_count(&self) -> u64 {
        self.delta_accounting.dependency_snapshot_delta_node_count()
    }

    pub fn coarse_replacement_required(&self) -> bool {
        self.coarse_replacement_required
    }

    pub fn coarse_reasons(&self) -> &[SnapshotRestoreCoarseReason] {
        &self.coarse_reasons
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Snapshot-resident diagnostics payload needed for deterministic restore,
/// replay inspection, and lineage continuity.
pub struct SignalSnapshotDiagnostics {
    pub latest_flow: Option<FlowSummary>,
    pub latest_failure: Option<FailureSummary>,
    pub latest_rollback: Option<RollbackDiagnostic>,
    pub latest_observation: Option<ObservationBoundarySummary>,
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
pub struct SignalCheckpointSlot {
    #[serde(
        default,
        alias = "entry",
        deserialize_with = "deserialize_checkpoint_slot_node"
    )]
    pub node: Option<CheckpointNodeImage>,
    pub generation: u32,
    pub retired: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CheckpointSlotNodeRepr {
    Image(CheckpointNodeImage),
    Legacy(NodeEntry),
}

fn deserialize_checkpoint_slot_node<'de, D>(
    deserializer: D,
) -> Result<Option<CheckpointNodeImage>, D::Error>
where
    D: Deserializer<'de>,
{
    let repr = Option::<CheckpointSlotNodeRepr>::deserialize(deserializer)?;
    Ok(repr.map(|repr| match repr {
        CheckpointSlotNodeRepr::Image(image) => image,
        CheckpointSlotNodeRepr::Legacy(entry) => entry.to_checkpoint_image(),
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCheckpointArena {
    pub slots: Vec<SignalCheckpointSlot>,
    pub free_list: Vec<u32>,
    pub active_nodes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCheckpointTopology {
    pub dependency_edges: DependencyEdgeStore,
    pub subscriber_edges: SubscriberEdgeStore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Narrow checkpoint-owned authority payload used to reconstruct operational
/// graph truth without carrying runtime observation baggage.
pub struct SignalCheckpointAuthority {
    pub(crate) arena: SignalCheckpointArena,
    pub(crate) topology: SignalCheckpointTopology,
    pub(crate) diagnostics: DiagnosticsState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonical checkpoint-carried authority image for reconstructive restore.
///
/// Supported restore paths must consume this image rather than treating the
/// entire snapshot bundle as the authority carrier.
pub struct SignalCheckpointImage {
    pub authority: SignalCheckpointAuthority,
    pub dependency_snapshot_batch: SnapshotBatchCommit,
    pub graph_telemetry: RuntimeTelemetry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Versioned snapshot of `forge-signal` evaluation state.
///
/// This captures graph-local evaluation state, runtime diagnostics required for
/// deterministic replay/restore, branch metadata, and lineage/replay history.
/// It intentionally does not claim ownership of host-managed source truth.
pub struct SignalSnapshotV1 {
    pub meta: SignalSnapshotMeta,
    pub checkpoint_image: SignalCheckpointImage,
    #[serde(alias = "graph")]
    pub diagnostic_graph: SignalGraph,
    pub diagnostics: SignalSnapshotDiagnostics,
    pub graph_telemetry: RuntimeTelemetry,
    pub runtime_telemetry: Option<RuntimeTelemetry>,
    pub reconstructability: Option<ReconstructabilityRecord>,
}

impl SignalSnapshotMeta {
    pub const SCHEMA_VERSION: u32 = 2;

    pub fn new(
        snapshot_id: SignalSnapshotId,
        branch: &SignalBranchHandle,
        replay_head: Option<ReplayCursor>,
        runtime_policy: SignalRuntimePolicy,
        artifact_retention: SnapshotArtifactRetentionPolicy,
    ) -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            snapshot_id,
            branch_id: branch.id,
            branch_name: branch.name.clone(),
            core_storage_profile: CORE_STORAGE_PROFILE_ID.to_string(),
            replay_head,
            runtime_policy,
            artifact_retention,
        }
    }
}

impl SnapshotArtifactRetentionPolicy {
    pub fn from_runtime_policy(policy: SignalRuntimePolicy) -> Self {
        Self {
            explanation_retention: policy.retention_budget.explanation_retention,
            provenance_retention: policy.retention_budget.provenance_retention,
        }
    }

    pub fn retains_explanation_facts(self) -> bool {
        matches!(self.explanation_retention, ArtifactRetentionPolicy::Retain)
    }

    pub fn retains_provenance_facts(self) -> bool {
        matches!(self.provenance_retention, ArtifactRetentionPolicy::Retain)
    }
}

impl Default for SnapshotArtifactRetentionPolicy {
    fn default() -> Self {
        Self::from_runtime_policy(SignalRuntimePolicy::default())
    }
}

impl SnapshotRestoreIntent {
    pub fn restore_runtime_truth() -> Self {
        Self {
            state: SnapshotStateRestoreMode::RewindActiveState,
            artifacts: SnapshotArtifactRestoreMode::RestoreCapturedRetention,
            dependency_state: SnapshotDependencyRestoreMode::RestoreCapturedState,
        }
    }

    pub fn restore_runtime_truth_with_active_policy() -> Self {
        Self {
            state: SnapshotStateRestoreMode::RewindActiveState,
            artifacts: SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy,
            dependency_state: SnapshotDependencyRestoreMode::RestoreCapturedState,
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

    pub fn checkpoint_image(&self) -> &SignalCheckpointImage {
        &self.checkpoint_image
    }

    /// Rich diagnostics/inspection payload captured with the snapshot.
    ///
    /// This is not restore authority. Supported restore paths must consume the
    /// checkpoint image instead.
    pub fn diagnostic_graph(&self) -> &SignalGraph {
        &self.diagnostic_graph
    }

    pub fn authority_graph(&self) -> SignalGraph {
        SignalGraph::restore_from_checkpoint_authority(&self.checkpoint_image.authority)
    }

    pub fn reconstructability_proof(
        &self,
    ) -> Result<ReconstructabilityProof, crate::data::error::SignalError> {
        let record = self.reconstructability.as_ref().ok_or_else(|| {
            crate::data::error::SignalError::incompatible_snapshot(format!(
                "snapshot `{}` is missing reconstructability record",
                self.meta.snapshot_id.0
            ))
        })?;
        Ok(record.proof())
    }
}

#[cfg(test)]
mod tests {
    use super::{SignalCheckpointAuthority, SignalCheckpointSlot};
    use crate::data::graph::{DependencyEdgeStore, SubscriberEdgeStore};
    use crate::data::node::NodeEntry;
    use crate::diagnostics::state::DiagnosticsState;

    #[derive(serde::Serialize)]
    struct LegacyCheckpointSlot {
        entry: Option<NodeEntry>,
        generation: u32,
        retired: bool,
    }

    #[derive(serde::Serialize)]
    struct LegacyCheckpointArena {
        slots: Vec<LegacyCheckpointSlot>,
        free_list: Vec<u32>,
        active_nodes: u32,
    }

    #[derive(serde::Serialize)]
    struct LegacyCheckpointTopology {
        dependency_edges: DependencyEdgeStore,
        subscriber_edges: SubscriberEdgeStore,
    }

    #[derive(serde::Serialize)]
    struct LegacyCheckpointAuthority {
        arena: LegacyCheckpointArena,
        topology: LegacyCheckpointTopology,
        diagnostics: DiagnosticsState,
    }

    #[test]
    fn checkpoint_slot_deserializes_legacy_entry_payload() {
        let legacy = LegacyCheckpointSlot {
            entry: Some(NodeEntry::new()),
            generation: 7,
            retired: false,
        };

        let encoded = serde_json::to_vec(&legacy).expect("serialize legacy checkpoint slot");
        let decoded: SignalCheckpointSlot =
            serde_json::from_slice(&encoded).expect("deserialize legacy checkpoint slot");

        assert!(
            decoded.node.is_some(),
            "legacy entry payload should be bridged"
        );
        assert_eq!(decoded.generation, 7);
        assert!(!decoded.retired);
    }

    #[test]
    fn checkpoint_authority_deserializes_legacy_entry_payloads() {
        let legacy = LegacyCheckpointAuthority {
            arena: LegacyCheckpointArena {
                slots: vec![LegacyCheckpointSlot {
                    entry: Some(NodeEntry::new()),
                    generation: 3,
                    retired: false,
                }],
                free_list: Vec::new(),
                active_nodes: 1,
            },
            topology: LegacyCheckpointTopology {
                dependency_edges: DependencyEdgeStore::default(),
                subscriber_edges: SubscriberEdgeStore::default(),
            },
            diagnostics: DiagnosticsState::default(),
        };

        let encoded = serde_json::to_vec(&legacy).expect("serialize legacy checkpoint authority");
        let decoded: SignalCheckpointAuthority =
            serde_json::from_slice(&encoded).expect("deserialize legacy checkpoint authority");

        assert_eq!(decoded.arena.active_nodes, 1);
        assert_eq!(decoded.arena.slots.len(), 1);
        assert!(decoded.arena.slots[0].node.is_some());
    }

    #[test]
    fn checkpoint_slot_serializes_new_node_image_boundary() {
        let slot = SignalCheckpointSlot {
            node: Some(NodeEntry::new().to_checkpoint_image()),
            generation: 11,
            retired: false,
        };

        let encoded = serde_json::to_value(&slot).expect("serialize checkpoint slot");

        assert!(
            encoded.get("node").is_some(),
            "current checkpoint schema must serialize the explicit node image boundary"
        );
        assert!(
            encoded.get("entry").is_none(),
            "current checkpoint schema must not emit the legacy in-memory entry field"
        );
    }
}
